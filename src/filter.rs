use author::AuthorFilter;
use kanaria::string::UCSStr;
use megalodon::entities::Status;
use normal::NormalFilter;
use reblog::ReblogFilter;
use regex::RegexFilter;
use unicode_normalization::UnicodeNormalization;

use crate::config::FilterConfig;

mod author;
mod normal;
mod reblog;
mod regex;

fn normalize(content: &str, case_sensitive: bool) -> String {
    if case_sensitive {
        content.nfc().collect()
    } else {
        UCSStr::from_str(content)
            .lower_case()
            .hiragana()
            .to_string()
            .nfkc()
            .collect()
    }
}

trait Filter {
    fn filter(&self, status: &Status) -> Result<(), String>;
}

pub struct Filters {
    reblog: ReblogFilter,
    author: Option<AuthorFilter>,
    normal: Option<NormalFilter>,
    regex: Option<RegexFilter>,
}

impl Filters {
    pub fn new(config: FilterConfig) -> Self {
        let reblog = ReblogFilter::new();
        let author = if !config.user_include.is_empty() || !config.user_exclude.is_empty() {
            Some(AuthorFilter::new(config.user_include, config.user_exclude))
        } else {
            None
        };
        let (normal, regex) = if config.use_regex {
            (
                None,
                Some(RegexFilter::new(
                    config.include,
                    config.exclude,
                    config.case_sensitive,
                )),
            )
        } else {
            (
                Some(NormalFilter::new(
                    config.include,
                    config.exclude,
                    config.case_sensitive,
                )),
                None,
            )
        };
        Self {
            reblog,
            author,
            normal,
            regex,
        }
    }

    pub fn filter(&self, status: &Status) -> Result<(), String> {
        self.reblog.filter(status)?;
        if let Some(author) = &self.author {
            author.filter(status)?;
        }
        if let Some(normal) = &self.normal {
            normal.filter(status)?;
        }
        if let Some(regex) = &self.regex {
            regex.filter(status)?;
        }
        Ok(())
    }
}
