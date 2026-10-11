//! Romans: Grace cannot be earned by passing enough tests.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Romans");
    story.letter(
        "1-3",
        "Rome",
        "Jew and Gentile alike need God's righteousness",
    );
    story.letter(
        "4",
        "Rome",
        "Abraham illustrates righteousness through faith",
    );
    story.letter(
        "5",
        "Rome",
        "peace through Christ; Adam and Christ are contrasted",
    );
    story.letter(
        "6-7",
        "Rome",
        "new life, sin, the law, and the struggle with disobedience",
    );
    story.letter(
        "8",
        "Rome",
        "life in the Spirit, adoption, hope, and God's enduring love",
    );
    story.letter(
        "9-11",
        "Rome",
        "Israel, the Gentiles, mercy, and God's faithfulness",
    );
    story.letter(
        "12-13",
        "Rome",
        "living service, love, civic duties, and readiness",
    );
    story.letter(
        "14-15",
        "Rome",
        "welcome one another amid differing practices; Paul's mission and plans",
    );
    story.letter("16", "Rome", "greetings, warnings, and concluding praise");
    story
}
