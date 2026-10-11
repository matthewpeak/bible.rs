//! Hosea: A covenant survives a long history of broken promises.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Hosea");
    story.event(
        "1-3",
        "Hosea's household",
        "marriage and restoration become signs concerning Israel",
    );
    story.prophecy(
        "4-6",
        "Hosea",
        "faithlessness, corrupt worship, and shallow repentance are exposed",
    );
    story.prophecy(
        "7-10",
        "Hosea",
        "political schemes and idolatry bring judgment",
    );
    story.prophecy(
        "11",
        "Hosea",
        "God's love for Israel confronts persistent turning away",
    );
    story.prophecy(
        "12-13",
        "Hosea",
        "Israel's history and false confidence are recalled",
    );
    story.prophecy(
        "14",
        "Hosea",
        "return to the LORD and receive healing and restoration",
    );
    story
}
