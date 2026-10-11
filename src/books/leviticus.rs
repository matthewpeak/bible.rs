//! Leviticus: The specification is very explicit.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Leviticus");
    story.law(
        "1-7",
        "Israel",
        "instructions for burnt, meat, peace, sin, and trespass offerings",
    );
    story.appoint("8-9", "Aaron and his sons", "priests of Israel");
    story.event("10", "Nadab and Abihu", "unauthorised fire brings judgment");
    story.law(
        "11-15",
        "Israel",
        "clean and unclean foods, childbirth, skin conditions, and discharges",
    );
    story.law(
        "16",
        "the high priest",
        "observe the annual day of atonement",
    );
    story.law(
        "17-20",
        "Israel",
        "respect blood, sexual boundaries, neighbours, and holiness",
    );
    story.law(
        "21-22",
        "the priests",
        "maintain holiness in service and offerings",
    );
    story.law("23", "Israel", "observe the appointed feasts and holy days");
    story.law(
        "24",
        "Israel",
        "care for the lamps and bread; administer justice impartially",
    );
    story.law(
        "25",
        "Israel",
        "keep sabbatical years and jubilee; provide redemption and release",
    );
    story.covenant(
        "26",
        "blessings and judgments",
        "obedience and rebellion have consequences; restoration remains possible",
    );
    story.law(
        "27",
        "Israel",
        "fulfil vows and distinguish what is dedicated to the LORD",
    );
    story
}
