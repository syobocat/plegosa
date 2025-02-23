use megalodon::entities::Status;

use super::{normalize, Filter};

pub struct NormalFilter {
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
    case_sensitive: bool,
}

impl NormalFilter {
    pub fn new(include: Vec<String>, exclude: Vec<String>, case_sensitive: bool) -> Self {
        Self {
            include: if include.is_empty() {
                None
            } else {
                Some(include)
            },
            exclude: if exclude.is_empty() {
                None
            } else {
                Some(exclude)
            },
            case_sensitive,
        }
    }
}

impl Filter for NormalFilter {
    fn filter(&self, status: &Status) -> Result<(), String> {
        let content = normalize(&status.content, self.case_sensitive);
        if let Some(include) = &self.include {
            if !include.iter().any(|x| content.contains(x)) {
                return Err("The status does not contain include".to_owned());
            }
        }
        if let Some(exclude) = &self.exclude {
            if exclude.iter().any(|x| content.contains(x)) {
                return Err("The status contains exclude".to_owned());
            }
        }
        Ok(())
    }
}
