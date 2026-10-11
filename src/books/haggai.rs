//! Haggai: Stop polishing your own house. The shared project is stalled.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Haggai");
    story.prophecy(
        "1:1-11",
        "Haggai",
        "the neglected temple exposes the people's priorities",
    );
    story.build("1:12-15", "the resumed temple works");
    story.prophecy(
        "2:1-9",
        "Haggai",
        "encouragement for the builders and a promise of future glory",
    );
    story.prophecy(
        "2:10-19",
        "Haggai",
        "holiness, defilement, and the promise of blessing",
    );
    story.prophecy(
        "2:20-23",
        "Haggai",
        "Zerubbabel is likened to a signet amid shaken kingdoms",
    );
    story
}
