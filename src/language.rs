use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::languages::typescript::TypescriptLang;

pub trait Language {
    fn get_name(&self) -> &str;
    fn get_version(&self) -> &str;
    fn get_features(&self) -> HashSet<&str>;
}

pub trait LanguageFeature {
    fn get_name(&self) -> &str;
    fn get_description(&self) -> Option<&str>;
    fn get_dependency_type(&self) -> LanguageFeatureType;
}

const LANGS: [&str; 8] = [
    "c++",
    "go",
    "javascript",
    "lua",
    "python",
    "ruby",
    "rust",
    "typescript",
];

pub struct Container {}

impl Container {
    pub fn create_default_languages() -> HashMap<String, Box<dyn Language>> {
        let mut languages = HashMap::new();

        for lang_key in LANGS {
            let language = match lang_key {
                "typescript" => Box::new(TypescriptLang::default()) as Box<dyn Language>,
                _ => unimplemented!(),
            };
            languages.insert(lang_key.to_string(), language);
        }

        languages
    }
}

//
// #[derive(Serialize, Deserialize)]
// pub struct Language {
//     name: String,
//     version: Option<String>,
//     features: HashMap<String, LanguageFeature>,
// }
//
// impl Language {
//     fn new(name: &str, version: Option<&str>) -> Self {
//         Self {
//             name: name.to_string(),
//             version: version.map(|v| v.to_string()),
//             features: HashMap::new(),
//         }
//     }
//     fn add_feature(&mut self, feature: LanguageFeature) {
//         self.features.insert(feature.name.clone(), feature);
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LanguageFeatureType {
    Dev,
    General,
}

// #[derive(Serialize, Deserialize)]
// struct LanguageFeature {
//     name: String,
//     description: Option<String>,
//     dependency_type: LanguageFeatureType,
// }
//
// impl LanguageFeature {
//     fn new(name: &str, description: Option<&str>, dependency_type: LanguageFeatureType) -> Self {
//         Self {
//             name: name.to_string(),
//             description: description.map(|d| d.to_string()),
//             dependency_type,
//         }
//     }
// }
//
// pub fn create_default_languages() -> HashMap<String, Language> {
//     let mut languages = HashMap::new();
//
//     for lang_key in LANGS {
//         let language = Language::new(lang_key, None);
//         languages.insert(lang_key.to_string(), language);
//     }
//
//     languages
// }
