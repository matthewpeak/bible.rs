//! Job: The test suite must not assume suffering proves guilt.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Job");
    story.event(
        "1-2",
        "Job",
        "loses family, possessions, and health; friends arrive",
    );
    story.prayer("3", "Job", "laments his birth and suffering");
    story.wisdom(
        "4-14",
        "the first debate",
        "friends explain suffering as punishment; Job disputes their account",
    );
    story.wisdom(
        "15-21",
        "the second debate",
        "accusations intensify; Job challenges simple explanations",
    );
    story.wisdom(
        "22-27",
        "the third debate",
        "Job maintains his integrity and seeks a hearing",
    );
    story.wisdom(
        "28",
        "wisdom's source",
        "human skill cannot purchase wisdom; the fear of the Lord is central",
    );
    story.prayer(
        "29-31",
        "Job",
        "remembers former life and declares his integrity",
    );
    story.wisdom(
        "32-37",
        "Elihu",
        "offers another account of suffering and God's greatness",
    );
    story.event(
        "38-41",
        "the LORD",
        "answers Job from the whirlwind with questions about creation",
    );
    story.event(
        "42",
        "Job and his friends",
        "Job responds; the friends are rebuked; Job prays and is restored",
    );
    story
}
