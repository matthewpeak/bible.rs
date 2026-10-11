//! Zechariah: More visions than the architecture document anticipated.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Zechariah");
    story.prophecy(
        "1:1-6",
        "Zechariah",
        "return to the LORD and heed the former prophets",
    );
    story.vision(
        "1:7-6:15",
        "Zechariah",
        "night visions encourage restoration and the crowning of Joshua",
    );
    story.prophecy(
        "7-8",
        "Zechariah",
        "true fasting, justice, and Jerusalem's future joy",
    );
    story.prophecy(
        "9-11",
        "Zechariah",
        "a humble king, deliverance, and rejected shepherding",
    );
    story.prophecy(
        "12-13",
        "Zechariah",
        "Jerusalem, mourning, cleansing, and a struck shepherd",
    );
    story.vision(
        "14",
        "Zechariah",
        "the day of the LORD and holiness throughout Jerusalem",
    );
    story
}
