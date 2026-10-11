//! Psalms: One hundred and fifty ways to call out.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Psalms");
    story.wisdom(
        "1-2",
        "the opening psalms",
        "the righteous way and the LORD's anointed king",
    );
    story.prayer(
        "3-41",
        "Book I",
        "lament, trust, confession, praise, and deliverance",
    );
    story.prayer(
        "42-72",
        "Book II",
        "longing for God, communal distress, kingship, and hope",
    );
    story.prayer(
        "73-89",
        "Book III",
        "the prosperity of the wicked, sanctuary, ruin, and covenant questions",
    );
    story.prayer(
        "90-106",
        "Book IV",
        "human frailty, God's reign, thanksgiving, and remembered history",
    );
    story.prayer(
        "107-118",
        "Book V's opening",
        "deliverance, wisdom, kingship, and festival praise",
    );
    story.wisdom(
        "119",
        "the law of the LORD",
        "delight in God's word across an alphabetic poem",
    );
    story.prayer(
        "120-134",
        "the songs of degrees",
        "pilgrimage, help, peace, family, and worship",
    );
    story.prayer(
        "135-145",
        "praise and lament",
        "God's works, exile, deliverance, and enduring kingship",
    );
    story.prayer(
        "146-150",
        "the closing praises",
        "all creation is called to praise the LORD",
    );
    story
}
