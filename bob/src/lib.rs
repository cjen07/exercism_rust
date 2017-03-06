pub fn reply(words: &str) -> &'static str {
    if words.ends_with("?") {
        "Sure."
    } else if words == "" {
        "Fine. Be that way!"
    } else if words.find(char::is_lowercase) == None {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}