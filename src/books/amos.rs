//! Amos: The worship service passes. The justice checks fail.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Amos");
    story.prophecy("1-2", "Amos", "judgment on nations, Judah, and Israel");
    story.prophecy(
        "3-4",
        "Amos",
        "privilege increases responsibility; repeated warnings were ignored",
    );
    story.prophecy(
        "5-6",
        "Amos",
        "seek the LORD and justice; reject empty worship and complacency",
    );
    story.vision(
        "7",
        "Amos",
        "locusts, fire, and a plumbline; conflict with Amaziah",
    );
    story.vision(
        "8",
        "Amos",
        "summer fruit signals the end and a famine of hearing God's words",
    );
    story.prophecy(
        "9",
        "Amos",
        "inescapable judgment gives way to a promise of restoration",
    );
    story
}
