static SS_PREFIX: &str = " · Supersonic";

static TITLE_SPOTIFY_PREMIUM: &str = "spotify premium";
static TITLE_SPOTIFY: &str = "spotify";
static CLASS_SPOTIFY: &str = "spotify";

/// a tuple of a matcher function and a cleaner function
pub const MATCHERS: [(fn(&str, &str) -> bool, fn(&str) -> String); 2] = [
    (supersonic_matcher, supersonic_cleaner),
    (spotify_matcher, spotify_cleaner),
];

pub fn supersonic_matcher(title: &str, _class: &str) -> bool {
    return title.ends_with(SS_PREFIX);
}

pub fn supersonic_cleaner(title: &str) -> String {
    return title.replace(SS_PREFIX, "");
}

pub fn spotify_matcher(title: &str, class: &str) -> bool {
    if title.is_empty() {
        return false;
    }

    let title_lower = title.to_lowercase();

    if title_lower == TITLE_SPOTIFY_PREMIUM {
        return false;
    }

    if title_lower == TITLE_SPOTIFY{
        return false;
    }

    return class.starts_with(CLASS_SPOTIFY);
}

pub fn spotify_cleaner(title: &str) -> String {
    return String::from(title);
}
