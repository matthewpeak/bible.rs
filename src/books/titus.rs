//! Titus: Sound teaching should produce visible behaviour.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Titus");
    story.letter(
        "1",
        "Titus",
        "appoint elders in Crete and oppose corrupt teaching",
    );
    story.letter(
        "2",
        "Titus",
        "teach conduct fitting the gospel across the community",
    );
    story.letter(
        "3:1-8",
        "Titus",
        "mercy, renewal, peaceable conduct, and good works",
    );
    story.letter(
        "3:9-15",
        "Titus",
        "avoid unprofitable disputes and attend to practical needs",
    );
    story
}
