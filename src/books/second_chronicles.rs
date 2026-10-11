//! 2 Chronicles: A second account, not a duplicate event stream.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("2 Chronicles");
    story.event("1", "Solomon", "asks for wisdom");
    story.build("2-5", "Solomon's temple");
    story.prayer(
        "6-7",
        "Solomon",
        "dedicates the temple; God answers with a promise and warning",
    );
    story.event(
        "8-9",
        "Solomon's reign",
        "works, wealth, the queen of Sheba, and death",
    );
    story.event(
        "10-12",
        "Rehoboam",
        "the kingdom divides; Jerusalem is humbled by Egypt",
    );
    story.event(
        "13-16",
        "Abijah and Asa",
        "wars, reforms, and failures of reliance",
    );
    story.event(
        "17-20",
        "Jehoshaphat",
        "teaching, compromised alliances, prayer, and deliverance",
    );
    story.event(
        "21-24",
        "Judah's succession",
        "Jehoram, Ahaziah, Athaliah, and Joash bring upheaval",
    );
    story.event(
        "25-28",
        "Judah's kings",
        "Amaziah, Uzziah, Jotham, and Ahaz rule with differing faithfulness",
    );
    story.event(
        "29-32",
        "Hezekiah",
        "restores worship and passover; faces Assyria",
    );
    story.event(
        "33",
        "Manasseh and Amon",
        "Manasseh humbles himself; Amon continues in evil",
    );
    story.event(
        "34-35",
        "Josiah",
        "the law is recovered; covenant and passover are renewed",
    );
    story.journey(
        "36:1-21",
        "Judah",
        "exile in Babylon after Jerusalem's fall",
    );
    story.event(
        "36:22-23",
        "Cyrus",
        "authorises return and the rebuilding of the temple",
    );
    story
}
