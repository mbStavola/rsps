use std::{borrow::Cow, convert::TryFrom};

use emboss::EmbossingOptions;
use object::{Object, ObjectSection};

#[derive(Default)]
pub struct RsInfo {
    build_time: Option<String>,
    program_version: Option<String>,
    rust_version: Option<String>,
    build_profile: Option<String>,
    cargo_features: Option<String>,
}

impl RsInfo {
    pub fn build_time(&self) -> Option<&String> {
        self.build_time.as_ref()
    }

    pub fn program_version(&self) -> Option<&String> {
        self.program_version.as_ref()
    }

    pub fn rust_version(&self) -> Option<&String> {
        self.rust_version.as_ref()
    }

    pub fn build_profile(&self) -> Option<&String> {
        self.build_profile.as_ref()
    }

    pub fn cargo_features(&self) -> Option<&String> {
        self.cargo_features.as_ref()
    }

    pub fn has_content(&self) -> bool {
        self.build_time
            .as_ref()
            .or(self.program_version.as_ref())
            .or(self.rust_version.as_ref())
            .or(self.build_profile.as_ref())
            .or(self.cargo_features.as_ref())
            .is_some()
    }
}

impl TryFrom<&object::File<'_>> for RsInfo {
    type Error = anyhow::Error;

    fn try_from(value: &object::File<'_>) -> Result<Self, Self::Error> {
        let section = value.section_by_name(emboss::DEFAULT_SECTION_NAME);
        if section.is_none() {
            return Ok(RsInfo::default());
        }

        let section = section.unwrap();
        let data = section.data()?;
        let text = String::from_utf8_lossy(data).to_string();

        let metadata = emboss::extract::extract_metadata(&text, EmbossingOptions::default())?;

        Ok(RsInfo {
            build_time: metadata.get("VERGEN_BUILD_TIMESTAMP").map(Cow::to_string),
            program_version: metadata.get("VERGEN_BUILD_SEMVER").map(Cow::to_string),
            rust_version: metadata.get("VERGEN_RUSTC_SEMVER").map(Cow::to_string),
            build_profile: metadata.get("VERGEN_CARGO_PROFILE").map(Cow::to_string),
            cargo_features: metadata.get("VERGEN_CARGO_FEATURES").map(Cow::to_string),
        })
    }
}
