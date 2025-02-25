use megalodon::entities::Status;

use super::{normalize, Filter};

pub struct NormalFilter {
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
    case_sensitive: bool,
}

impl NormalFilter {
    pub fn new(include: Vec<String>, exclude: Vec<String>, case_sensitive: bool) -> Self {
        let include: Vec<String> = include
            .into_iter()
            .map(|inc| normalize(&inc, case_sensitive))
            .collect();
        let exclude: Vec<String> = exclude
            .into_iter()
            .map(|exc| normalize(&exc, case_sensitive))
            .collect();
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

        let normal_filter_a = NormalFilter::new(vec!["should match".to_owned()], Vec::new(), true);
        assert!(normal_filter_a.filter(&should_match).is_ok());
        assert!(normal_filter_a.filter(&should_not_match).is_err());
        assert!(normal_filter_a.filter(&some_random_status).is_err());

        assert!(normal_filter_a.filter(&should_match_upper).is_err());
        assert!(normal_filter_a.filter(&should_not_match_upper).is_err());
        assert!(normal_filter_a.filter(&some_random_status_upper).is_err());

        let normal_filter_b =
            NormalFilter::new(Vec::new(), vec!["should not match".to_owned()], true);
        assert!(normal_filter_b.filter(&should_match).is_ok());
        assert!(normal_filter_b.filter(&should_not_match).is_err());
        assert!(normal_filter_b.filter(&some_random_status).is_ok());

        assert!(normal_filter_b.filter(&should_match_upper).is_ok());
        assert!(normal_filter_b.filter(&should_not_match_upper).is_ok());
        assert!(normal_filter_b.filter(&some_random_status_upper).is_ok());
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

        let normal_filter_a = NormalFilter::new(vec!["should match".to_owned()], Vec::new(), false);
        assert!(normal_filter_a.filter(&should_match_upper).is_ok());
        assert!(normal_filter_a.filter(&should_not_match_upper).is_err());
        assert!(normal_filter_a.filter(&some_random_status_upper).is_err());

        let normal_filter_b =
            NormalFilter::new(Vec::new(), vec!["should not match".to_owned()], false);
        assert!(normal_filter_b.filter(&should_match_upper).is_ok());
        assert!(normal_filter_b.filter(&should_not_match_upper).is_err());
        assert!(normal_filter_b.filter(&some_random_status_upper).is_ok());
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

        let normal_filter_a = NormalFilter::new(vec!["ガギグゲゴ".to_owned()], Vec::new(), true);
        assert!(normal_filter_a.filter(&composition).is_ok());
        assert!(normal_filter_a.filter(&decomposition).is_ok());
        assert!(normal_filter_a.filter(&compatibility).is_err());

        let normal_filter_b = NormalFilter::new(vec!["がぎぐげご".to_owned()], Vec::new(), true);
        assert!(normal_filter_b.filter(&composition).is_err());
        assert!(normal_filter_b.filter(&decomposition).is_err());
        assert!(normal_filter_b.filter(&compatibility).is_err());
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

        let normal_filter = NormalFilter::new(vec!["ガギグゲゴ".to_owned()], Vec::new(), false);
        assert!(normal_filter.filter(&composition).is_ok());
        assert!(normal_filter.filter(&decomposition).is_ok());
        assert!(normal_filter.filter(&compatibility).is_ok());

        let normal_filter_b = NormalFilter::new(vec!["がぎぐげご".to_owned()], Vec::new(), false);
        assert!(normal_filter_b.filter(&composition).is_ok());
        assert!(normal_filter_b.filter(&decomposition).is_ok());
        assert!(normal_filter_b.filter(&compatibility).is_ok());
    }
}
