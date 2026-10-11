//! Numbers: Forty years of integration testing.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Numbers");
    story.event(
        "1-4",
        "the camp",
        "Israel is counted and arranged; Levites receive their duties",
    );
    story.law(
        "5-6",
        "Israel",
        "purity, restitution, Nazarite vows, and the priestly blessing",
    );
    story.event(
        "7-9",
        "the tabernacle",
        "offerings, dedication, passover, and guidance by the cloud",
    );
    story.journey("10", "Israel", "from Sinai toward the promised land");
    story.event(
        "11-12",
        "the wilderness",
        "complaints, quails, elders, and Miriam's challenge",
    );
    story.event(
        "13-14",
        "the spies",
        "fear leads Israel to refuse entry; wilderness judgment follows",
    );
    story.law(
        "15",
        "Israel",
        "offerings, unintentional sin, sabbath observance, and fringes",
    );
    story.event(
        "16-17",
        "Korah's rebellion",
        "judgment falls; Aaron's rod buds",
    );
    story.law(
        "18-19",
        "Israel",
        "support the priesthood and observe purification rites",
    );
    story.event(
        "20",
        "Moses and Aaron",
        "water from the rock; Moses is barred from entry; Aaron dies",
    );
    story.event(
        "21",
        "the wilderness journey",
        "the brasen serpent and victories east of Jordan",
    );
    story.prophecy(
        "22-24",
        "Balaam",
        "blessings for Israel despite Balak's demand for curses",
    );
    story.event(
        "25",
        "Baal-peor",
        "idolatry brings a plague; Phinehas intervenes",
    );
    story.event(
        "26-27",
        "the next generation",
        "a new census, inheritance for daughters, and Joshua commissioned",
    );
    story.law("28-30", "Israel", "offerings, holy days, and vows");
    story.event(
        "31-32",
        "east of Jordan",
        "war with Midian; Reuben, Gad, and half Manasseh receive land",
    );
    story.event(
        "33-34",
        "Israel's journey",
        "stages of travel and boundaries of the promised land",
    );
    story.law(
        "35-36",
        "Israel",
        "cities of refuge and inheritance boundaries",
    );
    story
}
