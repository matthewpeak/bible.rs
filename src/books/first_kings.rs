//! 1 Kings: One kingdom. Two forks. Many breaking changes.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("1 Kings");
    story.appoint("1-2", "Solomon", "king after David");
    story.event("3-4", "Solomon", "asks for wisdom and orders the kingdom");
    story.build("5-8", "Solomon's temple");
    story.event(
        "9-10",
        "Solomon's reign",
        "a warning, extensive works, wealth, and the queen of Sheba",
    );
    story.event(
        "11",
        "Solomon",
        "foreign worship divides his heart; division of the kingdom is announced",
    );
    story.event(
        "12",
        "Israel and Judah",
        "Rehoboam's answer leads to division; Jeroboam establishes calf worship",
    );
    story.prophecy(
        "13-16",
        "the prophets",
        "judgment is announced against unfaithful kings",
    );
    story.event(
        "17",
        "Elijah",
        "ravens, the widow's provision, and her son's restoration",
    );
    story.event("18", "mount Carmel", "fire answers Elijah; rain returns");
    story.event(
        "19",
        "Elijah at Horeb",
        "receives a commission and calls Elisha",
    );
    story.event(
        "20",
        "Ahab",
        "wars with Syria and receives prophetic rebuke",
    );
    story.event(
        "21",
        "Naboth's vineyard",
        "Ahab and Jezebel's injustice brings judgment",
    );
    story.die("22", "Ahab");
    story
}
