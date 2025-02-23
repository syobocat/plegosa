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
