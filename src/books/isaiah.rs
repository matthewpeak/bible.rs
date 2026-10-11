//! Isaiah: Warnings, promises, and a very long release horizon.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Isaiah");
    story.prophecy(
        "1-5",
        "Isaiah",
        "Judah's injustice and false worship face judgment; restoration is promised",
    );
    story.vision(
        "6",
        "Isaiah",
        "the LORD enthroned; cleansing and a prophetic commission",
    );
    story.prophecy(
        "7-12",
        "Isaiah",
        "signs to Ahaz, coming judgment, and hope through a righteous ruler",
    );
    story.prophecy(
        "13-23",
        "Isaiah",
        "oracles concerning Babylon and surrounding nations",
    );
    story.prophecy(
        "24-27",
        "Isaiah",
        "worldwide judgment, praise, resurrection hope, and restoration",
    );
    story.prophecy(
        "28-35",
        "Isaiah",
        "warnings against false security; a renewed Zion and wilderness",
    );
    story.event(
        "36-37",
        "Hezekiah",
        "Jerusalem is threatened by Assyria and delivered",
    );
    story.event(
        "38-39",
        "Hezekiah",
        "healing is followed by a warning of Babylonian exile",
    );
    story.prophecy(
        "40-48",
        "Isaiah",
        "comfort, God's uniqueness, release from Babylon, and Cyrus's role",
    );
    story.prophecy(
        "49-55",
        "Isaiah",
        "the servant, suffering, restoration, and an invitation to mercy",
    );
    story.prophecy(
        "56-59",
        "Isaiah",
        "justice, true fasting, inclusion, and the people's sins",
    );
    story.prophecy("60-62", "Isaiah", "Zion's restoration and good tidings");
    story.prayer(
        "63-64",
        "the people",
        "remember mercy and plead for God's intervention",
    );
    story.vision(
        "65-66",
        "Isaiah",
        "judgment, renewed creation, and worship among the nations",
    );
    story
}
