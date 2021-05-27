use std::{cell::RefCell, collections::HashMap, io::Write, rc::Rc};

use ansi_term::Color;
use anyhow::Result;
use clap::Clap;
use rayon::prelude::*;
use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};
use tabwriter::TabWriter;

use crate::{commands::RspsSubcommand, util};

const T_JUNCTION: char = '├';
const L_CORNER: char = '└';
const LINE: char = '─';
const BAR: char = '│';

#[derive(Clap)]
pub struct TreeCommand;

struct Tree<'a> {
    nodes: Vec<Rc<RefCell<TreeNode<'a>>>>,
}

#[derive(Clone, Debug)]
enum TreeNode<'a> {
    Branch {
        process: &'a Process,
        children: Vec<Rc<RefCell<TreeNode<'a>>>>,
    },
    Leaf {
        process: &'a Process,
    },
}

impl RspsSubcommand for TreeCommand {
    fn exec(&self, system: &mut System, tw: &mut TabWriter<Vec<u8>>) -> Result<()> {
        let processes = system
            .get_processes()
            .values()
            .par_bridge()
            .filter_map(|process| {
                util::is_process_rusty(process)
                    .ok()
                    .flatten()
                    .map(|info| (process, info))
            })
            .collect::<Vec<_>>();

        if processes.is_empty() {
            return Ok(());
        }

        tw.write_all("@\n".as_bytes())?;

        let mut lookup = HashMap::<Pid, Rc<RefCell<TreeNode>>>::with_capacity(processes.len());

        for (process, _) in processes.iter() {
            lookup.insert(
                process.pid(),
                Rc::new(RefCell::new(TreeNode::Leaf { process })),
            );
        }

        let mut nodes = vec![];
        for (process, _) in processes {
            let node = lookup
                .get(&process.pid())
                .cloned()
                .expect("Process should exist");

            match process.parent() {
                Some(parent) if lookup.contains_key(&parent) => {
                    let parent = lookup
                        .get(&parent)
                        .cloned()
                        .map(|node| {
                            let replacement = if let TreeNode::Leaf { process } = *node.borrow() {
                                Some(TreeNode::Branch {
                                    process,
                                    children: vec![],
                                })
                            } else {
                                None
                            };

                            if let Some(replacement) = replacement {
                                node.replace(replacement);
                            }

                            node
                        })
                        .expect("Parent should exist");

                    let mut current = (*parent).borrow_mut();
                    match *current {
                        TreeNode::Branch {
                            ref mut children, ..
                        } => children.push(node),
                        _ => panic!("Node must be converted to branch"),
                    }
                }
                // Doesn't have a parent or parent isn't rusty
                Some(_) | None => nodes.push(node),
            }
        }

        let tree = Tree { nodes };

        let last_node = tree.nodes.len() - 1;
        for (i, node) in tree.nodes.iter().enumerate() {
            let current = node.borrow();
            print_tree(tw, &*current, 0, i == last_node, "")?;
        }

        Ok(())
    }
}

fn print_tree(
    tw: &mut TabWriter<Vec<u8>>,
    current: &TreeNode,
    depth: usize,
    is_last: bool,
    prefix: &str,
) -> Result<()> {
    tw.write_all(prefix.as_bytes())?;

    let start = if is_last {
        format!("{}{}{} ", L_CORNER, LINE, LINE)
    } else {
        format!("{}{}{} ", T_JUNCTION, LINE, LINE)
    };

    tw.write_all(start.as_bytes())?;

    match current {
        TreeNode::Branch { process, children } => {
            let header = format!(
                "{}\t[{}]\n",
                Color::Cyan.paint(process.pid().to_string()),
                Color::Yellow.paint(process.name()),
            );
            tw.write_all(header.as_bytes())?;

            let prefix = if is_last {
                format!("{}\t", prefix)
            } else {
                format!("{}{}\t", prefix, BAR)
            };

            let last_node = children.len() - 1;
            for (i, node) in children.iter().enumerate() {
                let current = node.borrow();
                print_tree(tw, &*current, depth + 1, i == last_node, &prefix)?;
            }
        }
        TreeNode::Leaf { process } => {
            let header = format!(
                "{}\t[{}]\n",
                Color::Cyan.paint(process.pid().to_string()),
                Color::Yellow.paint(process.name()),
            );
            tw.write_all(header.as_bytes())?;
        }
    }

    Ok(())
}
