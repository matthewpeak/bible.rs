//! Malachi: The final review before a long silence.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Malachi");
    story.prophecy(
        "1",
        "Malachi",
        "God's love is questioned and polluted offerings are rebuked",
    );
    story.prophecy(
        "2",
        "Malachi",
        "priests and the people are charged with covenant unfaithfulness",
    );
    story.prophecy(
        "3:1-6",
        "Malachi",
        "a messenger prepares the way; refining and judgment follow",
    );
    story.prophecy(
        "3:7-18",
        "Malachi",
        "return, faithful giving, and a distinction between service and rebellion",
    );
    story.prophecy(
        "4",
        "Malachi",
        "the coming day, remembrance of Moses, and Elijah's promised return",
    );
    story
}
