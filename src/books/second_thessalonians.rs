//! 2 Thessalonians: No, the release date has not been announced.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("2 Thessalonians");
    story.letter(
        "1",
        "Thessalonica",
        "endurance under persecution and hope of justice",
    );
    story.letter(
        "2",
        "Thessalonica",
        "do not be unsettled by claims that the day has already come",
    );
    story.letter(
        "3:1-5",
        "Thessalonica",
        "pray for the word's spread and remain steadfast",
    );
    story.letter(
        "3:6-18",
        "Thessalonica",
        "correct disorderly idleness while treating people as brethren",
    );
    story
}
