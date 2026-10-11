//! Daniel: The host operating system keeps changing.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Daniel");
    story.event(
        "1",
        "Daniel and his companions",
        "serve in Babylon while maintaining their convictions",
    );
    story.vision(
        "2",
        "Nebuchadnezzar",
        "a great image and a kingdom that endures",
    );
    story.event(
        "3",
        "Shadrach, Meshach, and Abed-nego",
        "survive the fiery furnace",
    );
    story.event("4", "Nebuchadnezzar", "is humbled and later restored");
    story.event(
        "5",
        "Belshazzar",
        "the writing on the wall announces his kingdom's end",
    );
    story.event(
        "6",
        "Daniel",
        "continues praying and survives the lions' den",
    );
    story.vision(
        "7",
        "Daniel",
        "four beasts, the Ancient of days, and one like the Son of man",
    );
    story.vision(
        "8",
        "Daniel",
        "the ram, goat, and conflict over the sanctuary",
    );
    story.prayer(
        "9",
        "Daniel",
        "confesses and receives the prophecy of seventy weeks",
    );
    story.vision(
        "10-12",
        "Daniel",
        "conflict among kingdoms, deliverance, and resurrection",
    );
    story
}
