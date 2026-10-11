//! Judges: The same regression keeps coming back.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Judges");
    story.event(
        "1-2",
        "Israel",
        "incomplete possession and a cycle of rebellion, oppression, and deliverance",
    );
    story.appoint("3", "Othniel, Ehud, and Shamgar", "deliverers of Israel");
    story.event(
        "4-5",
        "Deborah and Barak",
        "Sisera is defeated; Deborah and Barak sing",
    );
    story.event(
        "6-8",
        "Gideon",
        "a reduced army defeats Midian; the ephod becomes a snare",
    );
    story.event(
        "9",
        "Abimelech",
        "a violent bid for kingship ends in his death",
    );
    story.event(
        "10-12",
        "Jephthah",
        "Ammon is defeated; a rash vow and civil conflict bring tragedy",
    );
    story.event(
        "13-16",
        "Samson",
        "conflict with the Philistines ends in his death at their temple",
    );
    story.event(
        "17-18",
        "Micah and Dan",
        "idolatry spreads through a private shrine and a stolen priesthood",
    );
    story.event(
        "19-21",
        "Gibeah and Benjamin",
        "an atrocity leads to civil war and further violence",
    );
    story
}
