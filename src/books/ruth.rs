//! Ruth: A redemption story with no unnecessary abstractions.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Ruth");
    story.journey("1", "Ruth and Naomi", "Bethlehem");
    story.event(
        "2",
        "Ruth",
        "gleans in the field of Boaz and receives protection",
    );
    story.event(
        "3",
        "Ruth and Boaz",
        "Ruth asks Boaz to act as a kinsman redeemer",
    );
    story.covenant(
        "4:1-12",
        "Boaz and Ruth",
        "Boaz redeems the inheritance and marries Ruth",
    );
    story.event(
        "4:13-22",
        "Obed",
        "Ruth's son becomes the grandfather of David",
    );
    story
}
