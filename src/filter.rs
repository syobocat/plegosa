use crate::config::TimelineSetting;
use crate::streamer::Timeline;
use kanaria::string::UCSStr;
use megalodon::entities::StatusVisibility;
use regex::Regex;

pub fn filter(message: megalodon::entities::status::Status, tl: Timeline) -> bool {
    let filter = crate::config::FILTER.get().unwrap();
    let timeline_setting = crate::config::TIMELINES.get().unwrap();

    // Remove Repeats (a.k.a. Boosts)
    if message.reblogged.unwrap_or_default() {
        return false;
    }

    // Remove dupicates from Home Timeline
    if matches!(tl, Timeline::Home) && message.visibility == StatusVisibility::Public {
        match timeline_setting {
            TimelineSetting { public: true, .. } => return false,
            TimelineSetting { local: true, .. } => return message.account.acct.contains('@'),
            _ => {}
        };
    }

    if !filter.user_include.is_empty() && !filter.user_include.contains(&message.account.acct) {
        return false;
    }
    if filter.user_exclude.contains(&message.account.acct) {
        return false;
    }
    let content = if filter.is_case_sensitive {
        message.content
    } else {
        UCSStr::from_str(message.content.as_str())
            .lower_case()
            .hiragana()
            .to_string()
    };
    if !filter.include.is_empty()
        && filter
            .include
            .clone()
            .into_iter()
            .filter(|x| content.contains(x))
            .collect::<Vec<String>>()
            .is_empty()
    {
        return false;
    }
    if !filter
        .exclude
        .clone()
        .into_iter()
        .filter(|x| content.contains(x))
        .collect::<Vec<String>>()
        .is_empty()
    {
        return false;
    }
    if !filter.include_regex.is_empty()
        && filter
            .include_regex
            .clone()
            .into_iter()
            .filter(|x| x.is_match(&content))
            .collect::<Vec<Regex>>()
            .is_empty()
    {
        return false;
    }
    if !filter
        .exclude_regex
        .clone()
        .into_iter()
        .filter(|x| x.is_match(&content))
        .collect::<Vec<Regex>>()
        .is_empty()
    {
        return false;
    }
    true
}
