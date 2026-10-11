//! Ecclesiastes: The benchmark does not tell you what life is for.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Ecclesiastes");
    story.wisdom(
        "1-2",
        "the Preacher's search",
        "wisdom, pleasure, labour, and possessions cannot overcome mortality",
    );
    story.wisdom(
        "3",
        "time",
        "life has seasons; human understanding is limited",
    );
    story.wisdom(
        "4-6",
        "life under the sun",
        "oppression, companionship, worship, wealth, and dissatisfaction",
    );
    story.wisdom(
        "7-8",
        "wisdom's limits",
        "patience, justice, authority, and unresolved wrongs",
    );
    story.wisdom(
        "9-10",
        "mortality and folly",
        "enjoy ordinary gifts; act wisely amid uncertainty",
    );
    story.wisdom(
        "11",
        "uncertainty",
        "give generously and work without complete knowledge",
    );
    story.wisdom(
        "12",
        "the conclusion",
        "remember the Creator; fear God and keep his commandments",
    );
    story
}
