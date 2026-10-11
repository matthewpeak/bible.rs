//! Hebrews: The new covenant changes the mediation layer.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Hebrews");
    story.letter(
        "1-2",
        "the hearers",
        "the Son's supremacy and solidarity with humanity",
    );
    story.letter(
        "3-4",
        "the hearers",
        "faithfulness, warning against unbelief, and promised rest",
    );
    story.letter(
        "5-7",
        "the hearers",
        "Christ's priesthood and the order of Melchisedec",
    );
    story.letter(
        "8-10",
        "the hearers",
        "a better covenant and Christ's once-for-all offering",
    );
    story.letter(
        "11",
        "the hearers",
        "examples of faith across Israel's history",
    );
    story.letter(
        "12",
        "the hearers",
        "endurance, discipline, and approach to mount Sion",
    );
    story.letter(
        "13",
        "the hearers",
        "love, hospitality, fidelity, leadership, and final greetings",
    );
    story
}
