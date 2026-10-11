//! Philippians: A prison letter with an unexpected amount of joy.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Philippians");
    story.letter(
        "1",
        "Philippi",
        "thanksgiving, imprisonment, and courage in the gospel",
    );
    story.letter(
        "2",
        "Philippi",
        "humility patterned after Christ; Timothy and Epaphroditus",
    );
    story.letter(
        "3",
        "Philippi",
        "confidence in Christ and pressing toward the calling",
    );
    story.letter(
        "4",
        "Philippi",
        "unity, prayer, contentment, and thanks for support",
    );
    story
}
