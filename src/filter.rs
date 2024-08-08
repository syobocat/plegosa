use crate::config::TimelineSetting;
use crate::config::CONFIG;
use crate::streamer::Timeline;
use kanaria::string::UCSStr;
use megalodon::entities::{Status, StatusVisibility};
use regex::Regex;
use unicode_normalization::UnicodeNormalization;

pub fn filter(message: &Status, tl: &Timeline) -> Result<(), String> {
    let timelines = &CONFIG.timelines;
    let filter = &CONFIG.filter;

    // Remove Repeats (a.k.a. Boosts)
    if message.reblog.is_some() {
        return Err("The message is the repeat".to_owned());
    }

    // Remove dupicates from Home Timeline
    if matches!(tl, &Timeline::Home) && matches!(message.visibility, StatusVisibility::Public) {
        match timelines {
            TimelineSetting { public: true, .. } => {
                return Err("The message arrived at both Home and Public Timeline".to_owned())
            }
            TimelineSetting { local: true, .. } => {
                if !message.account.acct.contains('@') {
                    return Err("The message arrived at both Home and Local Timeline".to_owned());
                }
            }
            _ => {}
        };
    }

    if !filter.user_include.is_empty() && !filter.user_include.contains(&message.account.acct) {
        return Err("The author is not in user_include".to_owned());
    }
    if filter.user_exclude.contains(&message.account.acct) {
        return Err("The author is in user_exclude".to_owned());
    }

    let (content, include, exclude): (String, Vec<_>, Vec<_>) = if filter.case_sensitive {
        (
            message.content.nfc().collect(),
            filter.include.clone(),
            filter.exclude.clone(),
        )
    } else {
        (
            UCSStr::from_str(message.content.as_str())
                .lower_case()
                .hiragana()
                .to_string()
                .nfkc()
                .collect(),
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
            && !include
                .into_iter()
                .map(|x| Regex::new(&x).unwrap()) // We can use unwrap() here as we have already checked they're all valid regex.
                .any(|x| x.is_match(&content))
        {
            return Err("The message does not contain include".to_owned());
        }
        if exclude
            .into_iter()
            .map(|x| Regex::new(&x).unwrap()) // We can use unwrap() here as we have already checked they're all valid regex.
            .any(|x| x.is_match(&content))
        {
            return Err("The message contains exclude".to_owned());
        }
    } else {
        if !include.is_empty() && !include.into_iter().any(|x| content.contains(&x)) {
            return Err("The message does not contain include".to_owned());
        }
        if exclude.into_iter().any(|x| content.contains(&x)) {
            return Err("The message contains exclude".to_owned());
        }
    }

    Ok(())
}
