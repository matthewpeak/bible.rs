//! Jonah: The worker retries in the wrong direction.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Jonah");
    story.journey("1:1-3", "Jonah", "toward Tarshish instead of Nineveh");
    story.event(
        "1:4-17",
        "the storm",
        "Jonah is cast into the sea and swallowed by a great fish",
    );
    story.prayer("2", "Jonah", "gives thanks from the fish and is delivered");
    story.journey("3", "Jonah", "Nineveh");
    story.event(
        "3:5-10",
        "Nineveh",
        "repents and is spared the announced destruction",
    );
    story.event(
        "4",
        "Jonah",
        "resentment is challenged through a plant and God's concern for Nineveh",
    );
    story
}
