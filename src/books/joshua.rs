//! Joshua: The migration finally reaches production.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Joshua");
    story.appoint("1", "Joshua", "leader after Moses");
    story.event(
        "2",
        "Rahab",
        "hides the spies and seeks protection for her household",
    );
    story.journey("3-4", "Israel", "through Jordan on dry ground");
    story.event(
        "5",
        "Gilgal",
        "circumcision, passover, and encounter with the captain of the LORD's host",
    );
    story.event(
        "6",
        "Jericho",
        "the walls fall; Rahab's household is spared",
    );
    story.event(
        "7-8",
        "Ai",
        "Achan's trespass is exposed; a second campaign succeeds",
    );
    story.covenant(
        "9",
        "Gibeon",
        "Israel honours an oath made without seeking counsel",
    );
    story.event(
        "10-12",
        "the campaigns",
        "southern and northern kings are defeated",
    );
    story.event(
        "13-21",
        "the inheritance",
        "land is allotted; refuge and Levitical cities are appointed",
    );
    story.event(
        "22",
        "the eastern tribes",
        "an altar dispute is resolved as a witness, not a rival sacrifice",
    );
    story.covenant("23-24", "Shechem", "Joshua calls Israel to serve the LORD");
    story.die("24:29-33", "Joshua");
    story
}
