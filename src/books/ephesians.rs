//! Ephesians: Previously separated communities share one body.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Ephesians");
    story.letter(
        "1",
        "Ephesus",
        "blessing in Christ and prayer for understanding",
    );
    story.letter(
        "2",
        "Ephesus",
        "salvation by grace and reconciliation of Jew and Gentile",
    );
    story.letter(
        "3",
        "Ephesus",
        "the revealed mystery and a prayer for strength and love",
    );
    story.letter(
        "4",
        "Ephesus",
        "unity, gifts, maturity, and renewed conduct",
    );
    story.letter(
        "5",
        "Ephesus",
        "walk in love and light; household relationships",
    );
    story.letter(
        "6",
        "Ephesus",
        "household duties, spiritual armour, and prayer",
    );
    story
}
