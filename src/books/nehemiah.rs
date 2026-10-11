//! Nehemiah: The wall has maintainers now.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Nehemiah");
    story.prayer(
        "1",
        "Nehemiah",
        "confesses and asks for favour to restore Jerusalem",
    );
    story.journey("2", "Nehemiah", "Jerusalem with royal permission");
    story.build("3-4", "Jerusalem's wall amid opposition");
    story.event(
        "5",
        "Nehemiah",
        "confronts debt exploitation and restores property",
    );
    story.event("6", "the wall", "completed despite threats and deception");
    story.event(
        "7",
        "the register",
        "families and returning exiles are counted",
    );
    story.event(
        "8",
        "Ezra and the assembly",
        "the law is read and explained; the feast of tabernacles is kept",
    );
    story.covenant(
        "9-10",
        "the returned community",
        "confession and renewed commitments to the law",
    );
    story.event(
        "11-12",
        "Jerusalem",
        "resettlement and dedication of the wall",
    );
    story.event(
        "13",
        "Nehemiah's reforms",
        "temple duties, sabbath, and marriage practices are addressed",
    );
    story
}
