//! Ezra: Restore from exile. Rebuild from source.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Ezra");
    story.journey(
        "1-2",
        "the returning exiles",
        "Jerusalem under Cyrus's decree",
    );
    story.build("3", "the altar and temple foundation");
    story.event("4", "opposition", "adversaries obstruct the rebuilding");
    story.build("5-6", "the second temple");
    story.journey("7-8", "Ezra and his company", "Jerusalem");
    story.prayer("9", "Ezra", "confesses the community's unfaithfulness");
    story.event(
        "10",
        "the assembly",
        "a covenant response addresses the disputed marriages",
    );
    story
}
