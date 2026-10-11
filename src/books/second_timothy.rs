//! 2 Timothy: The handover is personal.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("2 Timothy");
    story.letter("1", "Timothy", "guard the entrusted message without fear");
    story.letter(
        "2",
        "Timothy",
        "teach faithful people, endure, and avoid quarrels",
    );
    story.letter(
        "3",
        "Timothy",
        "recognise difficult times and continue in the scriptures",
    );
    story.letter(
        "4",
        "Timothy",
        "preach faithfully; Paul's departure approaches; personal requests follow",
    );
    story
}
