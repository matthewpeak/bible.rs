//! Obadiah: Do not celebrate another team's disaster.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Obadiah");
    story.prophecy("1:1-9", "Obadiah", "Edom's pride will be brought low");
    story.prophecy(
        "1:10-14",
        "Obadiah",
        "violence and opportunism against Jacob are condemned",
    );
    story.prophecy(
        "1:15-16",
        "Obadiah",
        "the day of the LORD brings recompense among the nations",
    );
    story.prophecy(
        "1:17-21",
        "Obadiah",
        "deliverance on mount Zion and the LORD's kingdom",
    );
    story
}
