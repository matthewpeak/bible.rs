//! Galatians: Do not add circumcision as a dependency of grace.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Galatians");
    story.letter(
        "1",
        "Galatia",
        "Paul defends the gospel and recounts his calling",
    );
    story.letter(
        "2",
        "Galatia",
        "Gentile inclusion and justification by faith are defended",
    );
    story.letter("3", "Galatia", "Abraham, promise, law, and faith");
    story.letter(
        "4",
        "Galatia",
        "adoption and freedom contrasted with bondage",
    );
    story.letter(
        "5",
        "Galatia",
        "stand in liberty, serve in love, and walk in the Spirit",
    );
    story.letter(
        "6",
        "Galatia",
        "restore gently, bear burdens, and sow faithfully",
    );
    story
}
