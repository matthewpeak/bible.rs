//! 2 Corinthians: Weakness is not hidden from the status report.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("2 Corinthians");
    story.letter(
        "1-2",
        "Corinth",
        "comfort, changed plans, forgiveness, and reconciliation",
    );
    story.letter(
        "3-5",
        "Corinth",
        "new covenant ministry, fragile vessels, resurrection hope, and reconciliation",
    );
    story.letter(
        "6-7",
        "Corinth",
        "holiness, restored affection, and constructive sorrow",
    );
    story.letter("8-9", "Corinth", "generous giving for the saints");
    story.letter(
        "10-11",
        "Corinth",
        "Paul defends his ministry against boastful rivals",
    );
    story.letter(
        "12",
        "Corinth",
        "visions, weakness, sufficient grace, and concern for the church",
    );
    story.letter("13", "Corinth", "examine yourselves and seek restoration");
    story
}
