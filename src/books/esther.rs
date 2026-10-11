//! Esther: The reversal nobody in the court saw coming.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Esther");
    story.event(
        "1",
        "Vashti",
        "removed as queen after refusing the king's command",
    );
    story.appoint("2", "Esther", "queen of Persia");
    story.event("2:21-23", "Mordecai", "uncovers a plot against the king");
    story.event("3", "Haman", "obtains a decree against the Jews");
    story.event(
        "4",
        "Esther and Mordecai",
        "the community fasts; Esther resolves to approach the king",
    );
    story.event("5", "Esther", "invites the king and Haman to banquets");
    story.event(
        "6",
        "Mordecai",
        "honoured by the king through an unwilling Haman",
    );
    story.event("7", "Haman", "his plot is exposed and he is executed");
    story.event(
        "8-9",
        "the Jews",
        "a counter-decree allows defence; deliverance is commemorated at Purim",
    );
    story.appoint("10", "Mordecai", "second to king Ahasuerus");
    story
}
