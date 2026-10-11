//! Song of Solomon: Some things resist technical documentation.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Song of Solomon");
    story.event(
        "1",
        "the lovers",
        "desire and mutual admiration open the song",
    );
    story.event(
        "2",
        "the lovers",
        "delight in one another and celebrate spring",
    );
    story.event(
        "3",
        "the woman",
        "searches for her beloved; a royal procession appears",
    );
    story.event(
        "4",
        "the beloved",
        "praises the woman's beauty with garden imagery",
    );
    story.event(
        "5",
        "the woman",
        "a missed encounter becomes a search and a description of her beloved",
    );
    story.event("6", "the lovers", "recognition and renewed praise");
    story.event(
        "7",
        "the lovers",
        "admiration leads to an invitation into the countryside",
    );
    story.wisdom(
        "8",
        "love",
        "enduring devotion cannot be purchased or extinguished",
    );
    story
}
