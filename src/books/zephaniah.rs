//! Zephaniah: Judgment is not the final line.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Zephaniah");
    story.prophecy(
        "1",
        "Zephaniah",
        "the day of the LORD approaches Judah and Jerusalem",
    );
    story.prophecy("2:1-3", "Zephaniah", "seek righteousness and meekness");
    story.prophecy("2:4-15", "Zephaniah", "judgment on neighbouring nations");
    story.prophecy(
        "3:1-8",
        "Zephaniah",
        "Jerusalem's corrupt leaders and refusal of correction",
    );
    story.prophecy(
        "3:9-20",
        "Zephaniah",
        "purified worship, a humble remnant, and restoration",
    );
    story
}
