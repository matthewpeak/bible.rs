//! 2 Kings: Ignoring the warnings eventually breaks production.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("2 Kings");
    story.event(
        "1-2",
        "Elijah and Elisha",
        "Ahaziah is judged; Elijah is taken up; Elisha succeeds him",
    );
    story.event(
        "3-7",
        "Elisha",
        "water, provision, healing, a raised child, and deliverance from siege",
    );
    story.event(
        "8-10",
        "Hazael and Jehu",
        "new rulers arise; Ahab's house falls",
    );
    story.event(
        "11-12",
        "Joash",
        "Athaliah is overthrown and the temple is repaired",
    );
    story.event(
        "13-16",
        "Israel and Judah",
        "alternating reigns, wars, and persistent idolatry",
    );
    story.journey("17", "the northern kingdom", "exile in Assyria");
    story.event(
        "18-19",
        "Hezekiah",
        "reforms worship; Jerusalem is delivered from Assyria",
    );
    story.event("20", "Hezekiah", "healing and a warning about Babylon");
    story.event(
        "21",
        "Manasseh and Amon",
        "idolatry and violence spread in Judah",
    );
    story.event(
        "22-23",
        "Josiah",
        "the book of the law is found; worship is reformed",
    );
    story.journey("24-25", "Judah", "exile in Babylon");
    story.destroy("25:8-17", "the temple at Jerusalem");
    story.event("25:27-30", "Jehoiachin", "released from prison in Babylon");
    story
}
