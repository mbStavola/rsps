use vergen::{vergen, Config};

fn main() -> Result<(), ()> {
    let config = Config::default();
    vergen(config).unwrap();

    Ok(())
}
