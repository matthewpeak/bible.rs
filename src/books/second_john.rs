//! 2 John: Truth and love belong in the same module.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("2 John");
    story.letter(
        "1:1-3",
        "the elect lady and her children",
        "greeting in truth and love",
    );
    story.letter(
        "1:4-6",
        "the elect lady and her children",
        "walk in truth, love, and the commandment",
    );
    story.letter(
        "1:7-11",
        "the elect lady and her children",
        "guard against deceivers who deny Jesus Christ come in the flesh",
    );
    story.letter(
        "1:12-13",
        "the elect lady and her children",
        "hope for a personal visit and shared joy",
    );
    story
}
