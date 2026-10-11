//! 1 Chronicles: Genealogies are dependency graphs with consequences.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("1 Chronicles");
    story.event(
        "1-9",
        "the genealogies",
        "families from Adam through Israel and the returned community",
    );
    story.die("10", "Saul");
    story.appoint("11-12", "David", "king over all Israel");
    story.journey(
        "13-16",
        "the ark",
        "Jerusalem after an interrupted first attempt",
    );
    story.covenant("17", "David", "a house and kingdom are promised");
    story.event(
        "18-20",
        "David's campaigns",
        "victories establish the kingdom",
    );
    story.event(
        "21",
        "David's census",
        "judgment ends at the future temple site",
    );
    story.event(
        "22",
        "David's preparations",
        "materials and instructions are given for Solomon's temple",
    );
    story.appoint(
        "23-27",
        "Levites, priests, singers, gatekeepers, and officers",
        "ordered service in worship and government",
    );
    story.event(
        "28-29",
        "David's charge",
        "Solomon receives the plans; the people give; David blesses God and dies",
    );
    story
}
