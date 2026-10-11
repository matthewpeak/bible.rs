//! Habakkuk: A complaint, a response, and a harder follow-up question.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Habakkuk");
    story.prayer(
        "1:1-11",
        "Habakkuk",
        "questions unchecked violence and hears of the Chaldeans",
    );
    story.prayer(
        "1:12-2:1",
        "Habakkuk",
        "asks why a more wicked nation should judge another",
    );
    story.prophecy(
        "2:2-20",
        "the LORD",
        "the appointed vision, living by faith, and woes against oppression",
    );
    story.prayer(
        "3",
        "Habakkuk",
        "remembers God's mighty works and resolves to rejoice amid loss",
    );
    story
}
