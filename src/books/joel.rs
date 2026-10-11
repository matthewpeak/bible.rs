//! Joel: The incident report becomes a promise.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Joel");
    story.prayer(
        "1",
        "Joel",
        "calls for lament over a devastating locust plague",
    );
    story.prophecy(
        "2:1-17",
        "Joel",
        "the day of the LORD approaches; return with wholehearted repentance",
    );
    story.prophecy(
        "2:18-32",
        "Joel",
        "restoration and the pouring out of God's spirit",
    );
    story.prophecy(
        "3",
        "Joel",
        "judgment among the nations and security for Judah",
    );
    story
}
