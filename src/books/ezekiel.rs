//! Ezekiel: A new heart is not a patch to the old one.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Ezekiel");
    story.vision(
        "1-3",
        "Ezekiel",
        "the glory of God, a scroll, and a watchman's commission",
    );
    story.prophecy(
        "4-7",
        "Ezekiel",
        "enacted siege and warnings announce Jerusalem's judgment",
    );
    story.vision(
        "8-11",
        "Ezekiel",
        "idolatry in the temple and the departure of the glory",
    );
    story.prophecy(
        "12-17",
        "Ezekiel",
        "signs, false prophets, and parables expose unfaithfulness",
    );
    story.prophecy(
        "18-20",
        "Ezekiel",
        "personal responsibility and the history of rebellion",
    );
    story.prophecy(
        "21-24",
        "Ezekiel",
        "the sword, corruption, siege, and the sign of the prophet's bereavement",
    );
    story.prophecy("25-32", "Ezekiel", "oracles against surrounding nations");
    story.prophecy(
        "33-34",
        "Ezekiel",
        "the watchman's duty and judgment on unfaithful shepherds",
    );
    story.prophecy(
        "35-36",
        "Ezekiel",
        "judgment on Edom and restoration with a new heart and spirit",
    );
    story.vision(
        "37",
        "Ezekiel",
        "dry bones live and divided sticks become one",
    );
    story.prophecy(
        "38-39",
        "Ezekiel",
        "Gog's assault ends in judgment and God's vindication",
    );
    story.vision(
        "40-46",
        "Ezekiel",
        "a measured temple, returning glory, worship, and ordered service",
    );
    story.vision(
        "47-48",
        "Ezekiel",
        "life-giving waters, restored land, and a city of God's presence",
    );
    story
}
