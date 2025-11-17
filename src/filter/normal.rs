// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use megalodon::entities::Status;

use super::{Filter, FilterResult, normalize};

pub struct NormalFilter {
    include: Vec<String>,
    exclude: Vec<String>,
    case_sensitive: bool,
}

impl NormalFilter {
    pub fn new(include: &[String], exclude: &[String], case_sensitive: bool) -> Self {
        let include: Vec<String> = include
            .iter()
            .map(|inc| normalize(inc, case_sensitive))
            .collect();
        let exclude: Vec<String> = exclude
            .iter()
            .map(|exc| normalize(exc, case_sensitive))
            .collect();
        Self {
            include,
            exclude,
            case_sensitive,
        }
    }
}

impl Filter for NormalFilter {
    fn filter(&self, status: &Status) -> FilterResult {
        let content = normalize(&status.content, self.case_sensitive);
        if !self.include.is_empty() && !self.include.iter().any(|x| content.contains(x)) {
            return FilterResult::Block("The status does not contain include".to_owned());
        }
        if self.exclude.iter().any(|x| content.contains(x)) {
            return FilterResult::Block("The status contains exclude".to_owned());
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
    fn normal_test() {
        let should_match = Status {
            content: "This should match".to_owned(),
            ..plain_status()
        };
        let should_not_match = Status {
            content: "This should not match".to_owned(),
            ..plain_status()
        };
        let some_random_status = Status {
            content: "Some random status".to_owned(),
            ..plain_status()
        };

        let should_match_upper = Status {
            content: "THIS SHOULD MATCH".to_owned(),
            ..plain_status()
        };
        let should_not_match_upper = Status {
            content: "THIS SHOULD NOT MATCH".to_owned(),
            ..plain_status()
        };
        let some_random_status_upper = Status {
            content: "SOME RANDOM STATUS".to_owned(),
            ..plain_status()
        };

        let normal_filter_a = NormalFilter::new(&["should match".to_owned()], &[], true);
        assert!(normal_filter_a.filter(&should_match).passed());
        assert!(normal_filter_a.filter(&should_not_match).blocked());
        assert!(normal_filter_a.filter(&some_random_status).blocked());

        assert!(normal_filter_a.filter(&should_match_upper).blocked());
        assert!(normal_filter_a.filter(&should_not_match_upper).blocked());
        assert!(normal_filter_a.filter(&some_random_status_upper).blocked());

        let normal_filter_b = NormalFilter::new(&[], &["should not match".to_owned()], true);
        assert!(normal_filter_b.filter(&should_match).passed());
        assert!(normal_filter_b.filter(&should_not_match).blocked());
        assert!(normal_filter_b.filter(&some_random_status).passed());

        assert!(normal_filter_b.filter(&should_match_upper).passed());
        assert!(normal_filter_b.filter(&should_not_match_upper).passed());
        assert!(normal_filter_b.filter(&some_random_status_upper).passed());
    }

    #[test]
    fn normal_case_insensitive_test() {
        let should_match_upper = Status {
            content: "THIS SHOULD MATCH".to_owned(),
            ..plain_status()
        };
        let should_not_match_upper = Status {
            content: "THIS SHOULD NOT MATCH".to_owned(),
            ..plain_status()
        };
        let some_random_status_upper = Status {
            content: "SOME RANDOM STATUS".to_owned(),
            ..plain_status()
        };

        let normal_filter_a = NormalFilter::new(&["should match".to_owned()], &[], false);
        assert!(normal_filter_a.filter(&should_match_upper).passed());
        assert!(normal_filter_a.filter(&should_not_match_upper).blocked());
        assert!(normal_filter_a.filter(&some_random_status_upper).blocked());

        let normal_filter_b = NormalFilter::new(&[], &["should not match".to_owned()], false);
        assert!(normal_filter_b.filter(&should_match_upper).passed());
        assert!(normal_filter_b.filter(&should_not_match_upper).blocked());
        assert!(normal_filter_b.filter(&some_random_status_upper).passed());
    }

    #[test]
    fn normal_nfc_test() {
        let composition = Status {
            content: "\u{30AC}\u{30AE}\u{30B0}\u{30B2}\u{30B4}".to_owned(), // ガギグゲゴ, 5 unicode chars
            ..plain_status()
        };
        let decomposition = Status {
            content:
                "\u{30AB}\u{3099}\u{30AD}\u{3099}\u{30AF}\u{3099}\u{30B1}\u{3099}\u{30B3}\u{3099}"
                    .to_owned(), // ガギグゲゴ, 10 unicode chars
            ..plain_status()
        };
        let compatibility = Status {
            content:
                "\u{FF76}\u{FF9E}\u{FF77}\u{FF9E}\u{FF78}\u{FF9E}\u{FF79}\u{FF9E}\u{FF7A}\u{FF9E}"
                    .to_owned(), // ｶﾞｷﾞｸﾞｹﾞｺﾞ, 10 unicode chars
            ..plain_status()
        };

        let normal_filter_a = NormalFilter::new(&["ガギグゲゴ".to_owned()], &[], true);
        assert!(normal_filter_a.filter(&composition).passed());
        assert!(normal_filter_a.filter(&decomposition).passed());
        assert!(normal_filter_a.filter(&compatibility).blocked());

        let normal_filter_b = NormalFilter::new(&["がぎぐげご".to_owned()], &[], true);
        assert!(normal_filter_b.filter(&composition).blocked());
        assert!(normal_filter_b.filter(&decomposition).blocked());
        assert!(normal_filter_b.filter(&compatibility).blocked());
    }

    #[test]
    fn normal_nfkc_test() {
        let composition = Status {
            content: "\u{30AC}\u{30AE}\u{30B0}\u{30B2}\u{30B4}".to_owned(), // ガギグゲゴ, 5 unicode chars
            ..plain_status()
        };
        let decomposition = Status {
            content:
                "\u{30AB}\u{3099}\u{30AD}\u{3099}\u{30AF}\u{3099}\u{30B1}\u{3099}\u{30B3}\u{3099}"
                    .to_owned(), // ガギグゲゴ, 10 unicode chars
            ..plain_status()
        };
        let compatibility = Status {
            content:
                "\u{FF76}\u{FF9E}\u{FF77}\u{FF9E}\u{FF78}\u{FF9E}\u{FF79}\u{FF9E}\u{FF7A}\u{FF9E}"
                    .to_owned(), // ｶﾞｷﾞｸﾞｹﾞｺﾞ, 10 unicode chars
            ..plain_status()
        };

        let normal_filter = NormalFilter::new(&["ガギグゲゴ".to_owned()], &[], false);
        assert!(normal_filter.filter(&composition).passed());
        assert!(normal_filter.filter(&decomposition).passed());
        assert!(normal_filter.filter(&compatibility).passed());

        let normal_filter_b = NormalFilter::new(&["がぎぐげご".to_owned()], &[], false);
        assert!(normal_filter_b.filter(&composition).passed());
        assert!(normal_filter_b.filter(&decomposition).passed());
        assert!(normal_filter_b.filter(&compatibility).passed());
    }
}
