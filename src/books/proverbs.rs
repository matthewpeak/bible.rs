//! Proverbs: Wisdom is not a total function over every circumstance.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Proverbs");
    story.wisdom("1-4", "the learner", "seek wisdom and receive instruction");
    story.wisdom(
        "5-7",
        "fidelity",
        "avoid adultery, folly, and destructive enticement",
    );
    story.wisdom(
        "8-9",
        "wisdom and folly",
        "contrasting invitations call for a choice",
    );
    story.wisdom(
        "10-15",
        "daily conduct",
        "speech, diligence, justice, wealth, and correction",
    );
    story.wisdom(
        "16-22",
        "human plans",
        "humility, leadership, friendship, and dependence on the LORD",
    );
    story.wisdom(
        "23-24",
        "the words of the wise",
        "restraint, truth, perseverance, and justice",
    );
    story.wisdom(
        "25-29",
        "Hezekiah's collected proverbs",
        "rulers, restraint, disputes, and discipline",
    );
    story.wisdom(
        "30",
        "Agur",
        "humility, sufficiency, and observations about the world",
    );
    story.wisdom(
        "31:1-9",
        "Lemuel's instruction",
        "rulers should judge justly and defend the needy",
    );
    story.wisdom(
        "31:10-31",
        "the virtuous woman",
        "wisdom, diligence, generosity, and the fear of the LORD",
    );
    story
}
