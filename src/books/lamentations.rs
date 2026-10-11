//! Lamentations: Five poems after a catastrophic outage.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Lamentations");
    story.prayer(
        "1",
        "desolate Jerusalem",
        "mourns abandonment, suffering, and sin",
    );
    story.prayer(
        "2",
        "the lamenter",
        "grieves the city's destruction and calls for tears",
    );
    story.prayer(
        "3",
        "the sufferer",
        "moves from affliction toward hope in God's mercies",
    );
    story.prayer(
        "4",
        "the lamenter",
        "contrasts former splendour with siege and ruin",
    );
    story.prayer(
        "5",
        "the community",
        "asks the LORD to remember and restore",
    );
    story
}
