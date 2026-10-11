//! Philemon: A person cannot be reduced to a property type.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Philemon");
    story.letter(
        "1:1-7",
        "Philemon",
        "Paul gives thanks for Philemon's faith and love",
    );
    story.letter(
        "1:8-14",
        "Philemon",
        "Paul appeals for Onesimus through love rather than compulsion",
    );
    story.letter(
        "1:15-16",
        "Philemon",
        "receive Onesimus as a beloved brother",
    );
    story.letter(
        "1:17-21",
        "Philemon",
        "welcome him as Paul and charge any debt to Paul",
    );
    story.letter(
        "1:22-25",
        "Philemon",
        "prepare a lodging; greetings and grace",
    );
    story
}
