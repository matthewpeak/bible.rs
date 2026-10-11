//! 1 Corinthians: Many gifts. One body. Several unresolved issues.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("1 Corinthians");
    story.letter(
        "1-4",
        "Corinth",
        "divisions, the cross, wisdom, and the role of ministers",
    );
    story.letter(
        "5-6",
        "Corinth",
        "sexual wrongdoing, disputes, and the body",
    );
    story.letter(
        "7",
        "Corinth",
        "marriage, singleness, and differing circumstances",
    );
    story.letter(
        "8-10",
        "Corinth",
        "food offered to idols, freedom, and care for others",
    );
    story.letter(
        "11",
        "Corinth",
        "worship practices and abuses at the Lord's supper",
    );
    story.letter("12", "Corinth", "spiritual gifts serve one body");
    story.letter("13", "Corinth", "love gives gifts and actions their worth");
    story.letter(
        "14",
        "Corinth",
        "intelligibility and order in gathered worship",
    );
    story.letter(
        "15",
        "Corinth",
        "Christ's resurrection and the resurrection of the dead",
    );
    story.letter(
        "16",
        "Corinth",
        "the collection, travel plans, and final encouragement",
    );
    story
}
