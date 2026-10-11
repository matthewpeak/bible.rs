//! 1 John: The public interface includes love.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("1 John");
    story.letter(
        "1",
        "the community",
        "the witnessed life, fellowship, light, and confession",
    );
    story.letter(
        "2",
        "the community",
        "Christ as advocate, obedience, love, and warnings about deceivers",
    );
    story.letter(
        "3",
        "the community",
        "God's children practise righteousness and active love",
    );
    story.letter(
        "4",
        "the community",
        "test the spirits and love one another",
    );
    story.letter(
        "5",
        "the community",
        "faith, testimony concerning the Son, assurance, and prayer",
    );
    story
}
