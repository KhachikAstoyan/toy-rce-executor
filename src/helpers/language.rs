use core::fmt;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Debug)]
pub enum Language {
    #[serde(rename = "javascript")]
    Javascript,
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "typescript")]
    Typescript,
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "c")]
    C,
    #[serde(rename = "cpp")]
    Cpp,
    #[serde(rename = "python")]
    Python,
    #[serde(rename = "java")]
    Java,
    #[serde(rename = "bash")]
    Bash,
}

impl Language {
    pub fn get_extension(&self) -> String {
        match LANGUAGE_EXTENSIONS.get(self) {
            Some(extension) => extension.to_string(),
            None => self.to_string().clone(),
        }
    }

    pub fn get_file_name(&self) -> String {
        match LANGUAGE_FILE_NAMES.get(self) {
            Some(file_name) => file_name.to_string(),
            None => String::from("solution"),
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

static LANGUAGE_EXTENSIONS: Lazy<HashMap<Language, &'static str>> = Lazy::new(|| {
    HashMap::from([
        (Language::Javascript, "js"),
        (Language::Rust, "rs"),
        (Language::Typescript, "ts"),
        (Language::Go, "go"),
        (Language::C, "c"),
        (Language::Cpp, "cpp"),
        (Language::Python, "py"),
        (Language::Java, "java"),
        (Language::Bash, "sh"),
    ])
});

static LANGUAGE_FILE_NAMES: Lazy<HashMap<Language, &'static str>> =
    Lazy::new(|| HashMap::from([(Language::Java, "Main.java")]));
