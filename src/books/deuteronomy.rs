//! Deuteronomy: Read the terms before entering.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Deuteronomy");
    story.event(
        "1-3",
        "Moses",
        "reviews the journey, the refusal to enter, and victories east of Jordan",
    );
    story.law(
        "4-5",
        "Israel",
        "remember Sinai and the ten commandments; reject idolatry",
    );
    story.law(
        "6-11",
        "Israel",
        "love the LORD, teach the children, and remember deliverance",
    );
    story.law(
        "12-16",
        "Israel",
        "instructions for worship, food, debt release, and feasts",
    );
    story.law(
        "17-20",
        "Israel",
        "justice, rulers, priests, prophets, witnesses, and warfare",
    );
    story.law(
        "21-25",
        "Israel",
        "responsibilities in family and community life",
    );
    story.law(
        "26",
        "Israel",
        "bring firstfruits and remember the people's history",
    );
    story.covenant(
        "27-28",
        "blessings and curses",
        "the covenant declares the consequences of obedience and rebellion",
    );
    story.covenant("29-30", "Moab", "renew the covenant and choose life");
    story.appoint("31", "Joshua", "successor to Moses");
    story.prayer(
        "32",
        "Moses",
        "a song witnesses to God's faithfulness and Israel's failures",
    );
    story.prophecy("33", "Moses", "blessings upon the tribes");
    story.die("34", "Moses");
    story
}
