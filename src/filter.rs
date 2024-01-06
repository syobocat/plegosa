use crate::config::TimelineSetting;
use crate::config::CONFIG;
use crate::streamer::Timeline;
use kanaria::string::UCSStr;
use megalodon::entities::StatusVisibility;
use regex::Regex;

pub fn filter(message: megalodon::entities::status::Status, tl: &Timeline) -> bool {
    let config = CONFIG.get().unwrap();
    let filter = &config.filter;

    // Remove Repeats (a.k.a. Boosts)
    if message.reblog.is_some() {
        return false;
    }

    // Remove dupicates from Home Timeline
    if matches!(tl, Timeline::Home) && matches!(message.visibility, StatusVisibility::Public) {
        match config.timelines {
            TimelineSetting { public: true, .. } => return false,
            TimelineSetting { local: true, .. } => {
                if !message.account.acct.contains('@') {
                    return false;
                }
            }
            _ => {}
        };
    }

    if !filter.user_include.is_empty() && !filter.user_include.contains(&message.account.acct) {
        return false;
    }
    if filter.user_exclude.contains(&message.account.acct) {
        return false;
    }

    let (content, include, exclude) = if filter.case_sensitive {
        (
            message.content,
            filter.include.clone(),
            filter.exclude.clone(),
        )
    } else {
        (
            UCSStr::from_str(message.content.as_str())
                .lower_case()
                .hiragana()
                .to_string(),
            filter
                .include
                .iter()
                .map(|x| UCSStr::from_str(&x).lower_case().hiragana().to_string())
                .collect(),
            filter
                .exclude
                .iter()
                .map(|x| UCSStr::from_str(&x).lower_case().hiragana().to_string())
                .collect(),
        )
    };
    if filter.use_regex {
        if !include.is_empty()
            && include
                .into_iter()
                .map(|x| Regex::new(&x).unwrap()) // We can use unwrap() here as we have already checked they're all valid regex.
                .any(|x| x.is_match(&content))
        {
            return false;
        }
        if !exclude
            .into_iter()
            .map(|x| Regex::new(&x).unwrap()) // We can use unwrap() here as we have already checked they're all valid regex.
            .any(|x| x.is_match(&content))
        {
            return false;
        }
    } else {
        if !include.is_empty() && include.into_iter().any(|x| content.contains(&x)) {
            return false;
        }
        if !exclude.into_iter().any(|x| content.contains(&x)) {
            return false;
        }
    }

    true
}
