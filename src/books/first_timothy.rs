//! 1 Timothy: Instructions for maintaining the local assembly.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("1 Timothy");
    story.letter("1", "Timothy", "oppose false teaching and remember mercy");
    story.letter("2", "Timothy", "prayer and conduct in worship");
    story.letter("3", "Timothy", "qualifications for overseers and deacons");
    story.letter("4", "Timothy", "resist false asceticism and set an example");
    story.letter(
        "5",
        "Timothy",
        "care for widows, elders, and others in the community",
    );
    story.letter(
        "6",
        "Timothy",
        "sound teaching, contentment, wealth, and faithfulness",
    );
    story
}
