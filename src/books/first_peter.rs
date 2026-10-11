//! 1 Peter: Hope under hostile operating conditions.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("1 Peter");
    story.letter(
        "1",
        "the scattered believers",
        "living hope, tested faith, and holy conduct",
    );
    story.letter(
        "2",
        "the scattered believers",
        "a spiritual house and faithful conduct amid suffering",
    );
    story.letter(
        "3",
        "the scattered believers",
        "household conduct, blessing others, and answering with meekness",
    );
    story.letter(
        "4",
        "the scattered believers",
        "service, love, and suffering as Christians",
    );
    story.letter(
        "5",
        "the scattered believers",
        "elders, humility, vigilance, and steadfastness",
    );
    story
}
