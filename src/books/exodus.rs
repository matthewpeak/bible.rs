//! Exodus: The original escape from vendor lock-in.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

use crate::miracles::RedSea;

/// Exodus 14:21-31. The sea cannot be closed while its passage is borrowed.
pub fn cross_the_red_sea() -> RedSea {
    let mut sea = RedSea::default();
    {
        let mut passage = sea.part();
        passage.cross("Israel");
    } // Drop closes the sea. The people remain on the other side.
    sea
}

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Exodus");
    story.event(
        "1-2",
        "Israel in Egypt",
        "oppression increases; Moses is preserved, then flees to Midian",
    );
    story.appoint("3-4", "Moses", "messenger to Pharaoh");
    story.event(
        "5-6",
        "Pharaoh",
        "rejects the demand to release Israel and increases their labour",
    );
    story.event(
        "7-11",
        "Egypt",
        "nine plagues precede the announced death of the firstborn",
    );
    story.law(
        "12",
        "Israel",
        "keep the passover and remember deliverance from Egypt",
    );
    story.journey(
        "12:29-13:22",
        "Israel",
        "out of Egypt, led by cloud and fire",
    );
    story.execute("14:21-31", "exodus");
    story.prayer(
        "15",
        "Moses and Israel",
        "a song celebrates deliverance through the sea",
    );
    story.event(
        "16-17",
        "the wilderness",
        "manna, quails, water from the rock, and victory over Amalek",
    );
    story.event("18", "Jethro", "advises Moses to share the work of judging");
    story.law(
        "19-20",
        "Israel",
        "receive the covenant at Sinai and the ten commandments",
    );
    story.law(
        "21-23",
        "Israel",
        "rules for justice, restitution, worship, and care for the vulnerable",
    );
    story.covenant("24", "Sinai", "Israel ratifies the covenant");
    story.law(
        "25-31",
        "Moses",
        "instructions for the tabernacle, priesthood, and sabbath",
    );
    story.event(
        "32",
        "the golden calf",
        "Israel makes an idol; Moses breaks the tablets and intercedes",
    );
    story.covenant(
        "33-34",
        "renewed tablets",
        "the covenant is renewed after Israel's rebellion",
    );
    story.build("35-39", "the tabernacle");
    story.event("40", "the tabernacle", "the glory of the LORD fills it");
    story
}
