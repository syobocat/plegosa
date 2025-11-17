// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use megalodon::entities::Status;

use super::{Filter, FilterResult};

pub struct ReblogFilter {}

impl ReblogFilter {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Filter for ReblogFilter {
    fn filter(&self, status: &Status) -> FilterResult {
        if status.reblog.is_some() && !status.quote {
            return FilterResult::Block("The status is a repeat".to_owned());
        }
        FilterResult::Pass
    }
}

#[cfg(test)]
mod test {
    use megalodon::entities::Status;

    use crate::filter::tests::*;

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
        assert!(reblog_filter.filter(&not_reblog).passed());
        assert!(reblog_filter.filter(&reblog).blocked());
        assert!(reblog_filter.filter(&quote).passed());
    }
}
