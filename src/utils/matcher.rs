use regex::Regex;
use rosu_v2::model::GameMods;
use std::{borrow::Cow, str::FromStr};

pub fn is_custom_emote(msg: &str) -> bool {
    EMOJI_MATCHER.is_match(msg)
}

enum MentionType {
    Channel,
    Role,
    User,
}

pub fn get_mention_channel(msg: &str) -> Option<u64> {
    get_mention(MentionType::Channel, msg)
}

pub fn get_mention_role(msg: &str) -> Option<u64> {
    get_mention(MentionType::Role, msg)
}

pub fn get_mention_user(msg: &str) -> Option<u64> {
    get_mention(MentionType::User, msg)
}

fn get_mention(mention_type: MentionType, msg: &str) -> Option<u64> {
    if let Ok(id) = msg.parse() {
        return Some(id);
    }

    let captures = match mention_type {
        MentionType::Channel => CHANNEL_ID_MATCHER.captures(msg),
        MentionType::Role => ROLE_ID_MATCHER.captures(msg),
        MentionType::User => MENTION_MATCHER.captures(msg),
    };

    captures
        .and_then(|c| c.get(1))
        .and_then(|c| c.as_str().parse().ok())
}

#[allow(dead_code)]
pub fn get_osu_user_id(msg: &str) -> Option<u32> {
    OSU_URL_USER_MATCHER
        .captures(msg)
        .and_then(|c| c.get(1))
        .and_then(|c| c.as_str().parse::<u32>().ok())
}

pub fn get_osu_match_id(msg: &str) -> Option<u32> {
    if let Ok(id) = msg.parse::<u32>() {
        return Some(id);
    }

    OSU_URL_MATCH_MATCHER
        .captures(msg)
        .and_then(|c| c.get(1))
        .and_then(|c| c.as_str().parse::<u32>().ok())
}

pub fn get_youtube_id(msg: &str) -> Option<&str> {
    YOUTUBE_LINK_MATCHER
        .captures(msg)
        .and_then(|c| c.get(1))
        .map(|c| c.as_str())
}

#[allow(dead_code)]
pub fn is_hit_results(msg: &str) -> bool {
    HIT_RESULTS_MATCHER.is_match(msg)
}

pub fn is_guest_diff(msg: &str) -> bool {
    OSU_DIFF_MATCHER.is_match(msg)
}

pub fn tourney_badge(description: &str) -> bool {
    !IGNORE_BADGE_MATCHER.is_match_at(description, 0)
}

pub fn highlight_funny_numeral(content: &str) -> Cow<str> {
    SEVEN_TWO_SEVEN.replace_all(content, "__${num}__")
}

lazy_static! {
    static ref ROLE_ID_MATCHER: Regex = Regex::new(r"<@&(\d+)>").unwrap();

    static ref CHANNEL_ID_MATCHER: Regex = Regex::new(r"<#(\d+)>").unwrap();

    static ref MENTION_MATCHER: Regex = Regex::new(r"<@!?(\d+)>").unwrap();

    static ref OSU_URL_USER_MATCHER: Regex = Regex::new(r"https://osu.ppy.sh/users/(\d+)").unwrap();

    static ref OSU_URL_MAP_NEW_MATCHER: Regex = Regex::new(
        r"https://osu.ppy.sh/beatmapsets/(\d+)(?:(?:#(?:osu|mania|taiko|fruits)|<#\d+>)/(\d+))?"
    )
    .unwrap();

    static ref OSU_URL_MAP_OLD_MATCHER: Regex =
        Regex::new(r"https://osu.ppy.sh/b(?:eatmaps)?/(\d+)").unwrap();
    static ref OSU_URL_MAPSET_OLD_MATCHER: Regex =
        Regex::new(r"https://osu.ppy.sh/s/(\d+)").unwrap();

    static ref OSU_URL_MATCH_MATCHER: Regex =
        Regex::new(r"https://osu.ppy.sh/(?:community/matches|mp)/(\d+)").unwrap();

    static ref MOD_PLUS_MATCHER: Regex = Regex::new(r"^\+(\w+)!?$").unwrap();
    static ref MOD_MINUS_MATCHER: Regex = Regex::new(r"^-(\w+)!$").unwrap();

    static ref HIT_RESULTS_MATCHER: Regex = Regex::new(r".*\{(\d+/){2,}\d+}.*").unwrap();

    static ref OSU_DIFF_MATCHER: Regex =
        Regex::new(".*'s? (easy|normal|hard|insane|expert|extra|extreme|emotions|repetition)")
            .unwrap();

    static ref EMOJI_MATCHER: Regex = Regex::new(r"<(a?):([^:\n]+):(\d+)>").unwrap();

    static ref IGNORE_BADGE_MATCHER: Regex = Regex::new(r"^((?i)contrib|nomination|assessment|global|moderation|beatmap|spotlight|map|pending|aspire|elite|monthly|exemplary|outstanding|longstanding|idol[^@]+)").unwrap();

    static ref SEVEN_TWO_SEVEN: Regex = Regex::new("(?P<num>7[.,]?2[.,]?7)").unwrap();

    static ref YOUTUBE_LINK_MATCHER: Regex = Regex::new("http(?:s?)://(?:www\\.)?youtu(?:be\\.com/watch\\?v=|\\.be/)([\\w\\-_]*)(&(amp;)?‌​[\\w\\?‌​=]*)?").unwrap();
}
