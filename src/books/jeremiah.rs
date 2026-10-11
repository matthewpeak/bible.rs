//! Jeremiah: The warnings were in the logs the whole time.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Jeremiah");
    story.appoint("1", "Jeremiah", "prophet to the nations");
    story.prophecy(
        "2-6",
        "Jeremiah",
        "Israel's unfaithfulness and danger from the north",
    );
    story.prophecy(
        "7-10",
        "Jeremiah",
        "the temple cannot shield injustice and idolatry",
    );
    story.event(
        "11-20",
        "Jeremiah's ministry",
        "covenant warnings, enacted signs, opposition, and personal laments",
    );
    story.prophecy(
        "21-25",
        "Jeremiah",
        "judgment on rulers and prophets; Babylon and seventy years",
    );
    story.event(
        "26-29",
        "contested prophecy",
        "Jeremiah faces trial; false reassurance is opposed; exiles receive a letter",
    );
    story.covenant(
        "30-31",
        "the new covenant promised",
        "restoration and God's law written in the heart",
    );
    story.event(
        "32",
        "Jeremiah",
        "buys a field during the siege as a sign of future restoration",
    );
    story.prophecy(
        "33",
        "Jeremiah",
        "restoration and a righteous Branch are promised",
    );
    story.event(
        "34-36",
        "Judah's response",
        "broken release of servants, faithful Rechabites, and a burned prophetic scroll",
    );
    story.event(
        "37-39",
        "Jeremiah and Jerusalem",
        "imprisonment, rescue from a dungeon, and the city's fall",
    );
    story.journey(
        "40-44",
        "the remnant and Jeremiah",
        "Egypt after Gedaliah's assassination, against prophetic counsel",
    );
    story.prophecy(
        "45",
        "Jeremiah to Baruch",
        "a warning against seeking greatness and a promise of preservation",
    );
    story.prophecy(
        "46-51",
        "Jeremiah",
        "oracles against nations, including Babylon",
    );
    story.event(
        "52",
        "Jerusalem and Jehoiachin",
        "destruction, deportation, and the exiled king's later release",
    );
    story
}
