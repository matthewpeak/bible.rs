//! James: A declaration without implementation is insufficient.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("James");
    story.letter(
        "1",
        "the scattered tribes",
        "trials, wisdom, temptation, and doing the word",
    );
    story.letter(
        "2",
        "the scattered tribes",
        "reject partiality; living faith is shown through works",
    );
    story.letter(
        "3",
        "the scattered tribes",
        "the tongue and wisdom from above",
    );
    story.letter(
        "4",
        "the scattered tribes",
        "conflict, humility, repentance, and uncertain plans",
    );
    story.letter(
        "5",
        "the scattered tribes",
        "injustice, patience, truthful speech, prayer, and restoration",
    );
    story
}
