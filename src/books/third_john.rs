//! 3 John: Hospitality works better without a gatekeeping maintainer.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("3 John");
    story.letter("1:1-4", "Gaius", "joy in his faithfulness to the truth");
    story.letter("1:5-8", "Gaius", "support faithful travelling brethren");
    story.letter(
        "1:9-10",
        "Gaius",
        "Diotrephes's refusal of others is condemned",
    );
    story.letter(
        "1:11-12",
        "Gaius",
        "imitate good; Demetrius receives a good report",
    );
    story.letter(
        "1:13-14",
        "Gaius",
        "a personal visit is hoped for; greetings follow",
    );
    story
}
