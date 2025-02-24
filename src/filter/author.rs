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

#[cfg(test)]
mod test {
    use megalodon::entities::{Account, Status};

    use crate::filter::test::*;

    use super::*;

    #[test]
    fn author() {
        let should_match = Status {
            account: Account {
                acct: "should_match".to_owned(),
                ..plain_account()
            },
            ..plain_status()
        };
        let should_not_match = Status {
            account: Account {
                acct: "should_not_match".to_owned(),
                ..plain_account()
            },
            ..plain_status()
        };
        let some_random_man = Status {
            account: Account {
                acct: "some_random_man".to_owned(),
                ..plain_account()
            },
            ..plain_status()
        };

        let author_filter_a = AuthorFilter::new(vec!["should_match".to_owned()], Vec::new());
        assert!(author_filter_a.filter(&should_match).is_ok());
        assert!(author_filter_a.filter(&should_not_match).is_err());
        assert!(author_filter_a.filter(&some_random_man).is_err());

        let author_filter_b = AuthorFilter::new(Vec::new(), vec!["should_not_match".to_owned()]);
        assert!(author_filter_b.filter(&should_match).is_ok());
        assert!(author_filter_b.filter(&should_not_match).is_err());
        assert!(author_filter_b.filter(&some_random_man).is_ok());
    }
}
