//! 2 Peter: A delayed promise is not a failed promise.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("2 Peter");
    story.letter(
        "1",
        "the believers",
        "grow in virtue and remember the apostolic witness",
    );
    story.letter(
        "2",
        "the believers",
        "false teachers and destructive conduct face judgment",
    );
    story.letter(
        "3:1-10",
        "the believers",
        "answer scoffers with God's patience and the coming day",
    );
    story.letter(
        "3:11-18",
        "the believers",
        "live in holiness and grow in grace and knowledge",
    );
    story
}
