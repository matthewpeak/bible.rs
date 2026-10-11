//! Colossians: Christ is not an optional dependency.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Colossians");
    story.letter(
        "1",
        "Colosse",
        "thanksgiving, Christ's supremacy, and reconciliation",
    );
    story.letter(
        "2",
        "Colosse",
        "remain rooted in Christ; reject teachings that diminish his sufficiency",
    );
    story.letter(
        "3",
        "Colosse",
        "seek things above and put on renewed conduct",
    );
    story.letter(
        "4",
        "Colosse",
        "prayer, wise speech, messengers, and greetings",
    );
    story
}
