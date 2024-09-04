use std::collections::{HashMap, HashSet};

use crate::language::{Language, LanguageFeature};

#[derive(Default)]
pub struct TypescriptLang {
    version: Option<String>,
    features: HashMap<String, Box<dyn LanguageFeature>>,
}


impl Language for TypescriptLang {
    fn get_name(&self) -> &str {
        "TypeScript"
    }

    fn get_version(&self) -> &str {
        self.version.as_deref().unwrap_or("latest")
    }

    fn get_features(&self) -> HashSet<&str> {
        self.features.keys().map(|k| k.as_str()).collect()
    }
}
