//! Micah: The requirements fit in a verse. Meeting them is harder.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Micah");
    story.prophecy(
        "1-2",
        "Micah",
        "judgment on Samaria, Judah, and those who seize others' land",
    );
    story.prophecy(
        "3",
        "Micah",
        "corrupt rulers, priests, and prophets are condemned",
    );
    story.prophecy(
        "4",
        "Micah",
        "peace, restoration, and the nations coming to the LORD",
    );
    story.prophecy(
        "5",
        "Micah",
        "a ruler from Bethlehem and a purified remnant",
    );
    story.prophecy(
        "6",
        "Micah",
        "a covenant dispute calls for justice, mercy, and humility",
    );
    story.prayer(
        "7",
        "Micah",
        "laments corruption and trusts God's pardon and restoration",
    );
    story
}
