//! Nahum: Nineveh receives a different announcement this time.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Nahum");
    story.prophecy(
        "1",
        "Nahum",
        "God's power, justice, and refuge for those who trust him",
    );
    story.prophecy("2", "Nahum", "Nineveh's siege and collapse");
    story.prophecy("3:1-7", "Nahum", "violence and exploitation are exposed");
    story.prophecy(
        "3:8-19",
        "Nahum",
        "Nineveh cannot rely on strength to escape judgment",
    );
    story
}
