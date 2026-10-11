//! 1 Thessalonians: Encouragement while awaiting a promised return.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("1 Thessalonians");
    story.letter(
        "1",
        "Thessalonica",
        "thanksgiving for faith, love, and hope",
    );
    story.letter(
        "2",
        "Thessalonica",
        "Paul recalls his ministry and affection",
    );
    story.letter("3", "Thessalonica", "Timothy's report brings encouragement");
    story.letter(
        "4",
        "Thessalonica",
        "holiness, love, and hope concerning believers who have died",
    );
    story.letter(
        "5",
        "Thessalonica",
        "watchfulness, community care, prayer, and discernment",
    );
    story
}
