use megalodon::entities::Status;
use regex::Regex;

use super::{normalize, Filter, FilterResult};

pub struct RegexFilter {
    include: Vec<Regex>,
    exclude: Vec<Regex>,
    case_sensitive: bool,
}

impl RegexFilter {
    pub fn new(include: Vec<String>, exclude: Vec<String>, case_sensitive: bool) -> Self {
        Self {
            include: include.iter().map(|exp| Regex::new(exp).unwrap()).collect(),
            exclude: exclude.iter().map(|exp| Regex::new(exp).unwrap()).collect(),
            case_sensitive,
        }
    }
}

impl Filter for RegexFilter {
    fn filter(&self, status: &Status) -> FilterResult {
        let content = normalize(&status.content, self.case_sensitive);
        if !self.include.is_empty() && !self.include.iter().any(|regex| regex.is_match(&content)) {
            return FilterResult::Block("The status does not contain include".to_owned());
        }
        if self.exclude.iter().any(|regex| regex.is_match(&content)) {
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
    fn regex_test() {
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

        let regex_filter_a = RegexFilter::new(vec!["this.*d match".to_owned()], Vec::new(), false);
        assert!(regex_filter_a.filter(&should_match).passed());
        assert!(regex_filter_a.filter(&should_not_match).blocked());
        assert!(regex_filter_a.filter(&some_random_status).blocked());

        let regex_filter_b = RegexFilter::new(Vec::new(), vec!["this.*not".to_owned()], false);
        assert!(regex_filter_b.filter(&should_match).passed());
        assert!(regex_filter_b.filter(&should_not_match).blocked());
        assert!(regex_filter_b.filter(&some_random_status).passed());
    }
}
