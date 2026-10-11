//! Jude: Contend for the faith. Review incoming changes.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Jude");
    story.letter(
        "1:1-4",
        "the called",
        "contend for the faith against corrupt intruders",
    );
    story.letter(
        "1:5-16",
        "the called",
        "examples and warnings concerning judgment and rebellion",
    );
    story.letter(
        "1:17-23",
        "the called",
        "remember the apostles, build one another up, and show discerning mercy",
    );
    story.prayer(
        "1:24-25",
        "Jude",
        "praise to God who is able to keep and present his people",
    );
    story
}
