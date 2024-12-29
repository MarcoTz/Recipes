use recipes::Tag;

pub fn parse_tags(input: String) -> Vec<Tag> {
    input
        .split(",")
        .map(|tag| Tag(tag.trim().to_owned()))
        .collect()
}
