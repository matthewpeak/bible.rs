//! Mark: The narrative has very little idle time.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Mark");
    story.event(
        "1",
        "Jesus' ministry begins",
        "John, baptism, temptation, disciples, and healing",
    );
    story.event(
        "2-3",
        "Jesus",
        "forgives, heals, faces disputes, and appoints twelve",
    );
    story.wisdom(
        "4",
        "the kingdom",
        "parables of seed and growth; Jesus calms a storm",
    );
    story.event(
        "5",
        "Jesus",
        "delivers the possessed man, heals a woman, and raises Jairus's daughter",
    );
    story.event(
        "6",
        "Jesus' ministry",
        "rejection, sending the twelve, John's death, feeding, and walking on the sea",
    );
    story.event(
        "7-8",
        "Jesus",
        "teaches on defilement, heals, feeds, and hears Peter's confession",
    );
    story.event(
        "9",
        "Jesus and the disciples",
        "transfiguration, deliverance, and teaching about humility",
    );
    story.wisdom(
        "10",
        "discipleship",
        "marriage, children, wealth, service, and Bartimaeus's healing",
    );
    story.event(
        "11-13",
        "Jesus in Jerusalem",
        "entry, temple conflict, disputes, and teaching about coming trouble",
    );
    story.event(
        "14",
        "the passion begins",
        "anointing, supper, prayer, arrest, trial, and Peter's denial",
    );
    story.die("15", "Jesus");
    story.rise("16", "Jesus");
    story.appoint(
        "16:15-20",
        "the disciples",
        "preach the gospel; the KJV includes these closing verses",
    );
    story
}
