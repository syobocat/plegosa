use megalodon::entities::Status;

use super::Filter;

pub struct AuthorFilter {
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
}

impl AuthorFilter {
    pub fn new(include: Vec<String>, exclude: Vec<String>) -> Self {
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
        }
    }
}

impl Filter for AuthorFilter {
    fn filter(&self, status: &Status) -> Result<(), String> {
        if let Some(include) = &self.include {
            if !include.contains(&status.account.acct) {
                return Err("The author is not in user_include".to_owned());
            }
        }
        if let Some(exclude) = &self.exclude {
            if exclude.contains(&status.account.acct) {
                return Err("The author is in user_exclude".to_owned());
            }
        }
        Ok(())
    }
}
