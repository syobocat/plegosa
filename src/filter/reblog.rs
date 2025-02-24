use megalodon::entities::Status;

use super::Filter;

pub struct ReblogFilter {}

impl ReblogFilter {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Filter for ReblogFilter {
    fn filter(&self, status: &Status) -> Result<(), String> {
        if status.reblog.is_some() && !status.quote {
            return Err("The status is a repeat".to_owned());
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use megalodon::entities::Status;

    use crate::filter::test::*;

    use super::*;

    #[test]
    fn reblog_test() {
        let not_reblog = Status {
            reblog: None,
            quote: false,
            ..plain_status()
        };
        let reblog = Status {
            reblog: Some(Box::new(plain_status())),
            quote: false,
            ..plain_status()
        };
        let quote = Status {
            reblog: Some(Box::new(plain_status())),
            quote: true,
            ..plain_status()
        };

        let reblog_filter = ReblogFilter::new();
        assert!(reblog_filter.filter(&not_reblog).is_ok());
        assert!(reblog_filter.filter(&reblog).is_err());
        assert!(reblog_filter.filter(&quote).is_ok());
    }
}
