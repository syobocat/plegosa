use megalodon::entities::Status;
use regex::Regex;

use super::{normalize, Filter};

pub struct RegexFilter {
    include: Option<Vec<Regex>>,
    exclude: Option<Vec<Regex>>,
    case_sensitive: bool,
}

impl RegexFilter {
    pub fn new(include: Vec<String>, exclude: Vec<String>, case_sensitive: bool) -> Self {
        Self {
            include: if include.is_empty() {
                None
            } else {
                Some(
                    include
                        .into_iter()
                        .map(|exp| Regex::new(&exp).unwrap())
                        .collect(),
                )
            },
            exclude: if exclude.is_empty() {
                None
            } else {
                Some(
                    exclude
                        .into_iter()
                        .map(|exp| Regex::new(&exp).unwrap())
                        .collect(),
                )
            },
            case_sensitive,
        }
    }
}

impl Filter for RegexFilter {
    fn filter(&self, status: &Status) -> Result<(), String> {
        let content = normalize(&status.content, self.case_sensitive);
        if let Some(include) = &self.include {
            if !include.iter().any(|regex| regex.is_match(&content)) {
                return Err("The status does not contain include".to_owned());
            }
        }
        if let Some(exclude) = &self.exclude {
            if exclude.iter().any(|regex| regex.is_match(&content)) {
                return Err("The status contains exclude".to_owned());
            }
        }
        Ok(())
    }
}
