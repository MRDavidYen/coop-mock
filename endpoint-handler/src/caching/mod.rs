use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::endpoint_template::TemplateImpl;

pub struct TemplateCachingImpl {
    pub templates: Arc<RwLock<HashMap<String, Vec<TemplateImpl>>>>,
}

pub trait TemplateCaching: Sync + Send {
    fn get(&self, path: &str) -> Option<Vec<TemplateImpl>>;
    fn add_range(&self, first_scope: &str, templates: Vec<TemplateImpl>);
}

impl Default for TemplateCachingImpl {
    fn default() -> Self {
        Self {
            templates: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl TemplateCaching for TemplateCachingImpl {
    fn get(&self, first_scope: &str) -> Option<Vec<TemplateImpl>> {
        let template_hash = self.templates.read().unwrap();

        match template_hash.get(first_scope).cloned() {
            Some(templates) => Some(templates),
            None => None,
        }
    }

    fn add_range(&self, first_scope: &str, templates: Vec<TemplateImpl>) {
        let mut template_hash = self.templates.write().unwrap();

        template_hash.insert(first_scope.to_string(), templates);
    }
}
