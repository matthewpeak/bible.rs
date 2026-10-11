//! Luke: The account pays close attention to the people others overlook.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Luke");
    story.event(
        "1-2",
        "the births",
        "John and Jesus are announced and born; shepherds, Simeon, Anna, and the temple",
    );
    story.event(
        "3-4",
        "Jesus begins",
        "baptism, genealogy, temptation, Nazareth, and healing",
    );
    story.event(
        "5-6",
        "Jesus and the disciples",
        "callings, controversies, the twelve, and teaching on love",
    );
    story.event(
        "7-8",
        "Jesus' ministry",
        "healing, a widow's son, forgiveness, parables, and deliverance",
    );
    story.event(
        "9",
        "Jesus",
        "sends the twelve, feeds the multitude, is transfigured, and turns toward Jerusalem",
    );
    story.wisdom(
        "10-11",
        "discipleship",
        "the seventy, the good Samaritan, Mary and Martha, and prayer",
    );
    story.wisdom(
        "12-14",
        "the kingdom",
        "trust, readiness, repentance, humility, and the cost of discipleship",
    );
    story.wisdom(
        "15",
        "the lost are found",
        "the sheep, the coin, and the returning son",
    );
    story.wisdom(
        "16-18",
        "wealth and faithfulness",
        "stewardship, the rich man and Lazarus, gratitude, prayer, and humility",
    );
    story.event(
        "19",
        "Zacchaeus and Jerusalem",
        "a tax collector responds; Jesus enters the city and cleanses the temple",
    );
    story.event(
        "20-21",
        "Jesus in Jerusalem",
        "disputes, the widow's offering, and warnings about coming trouble",
    );
    story.event(
        "22",
        "the passion begins",
        "passover, supper, prayer, betrayal, arrest, and denial",
    );
    story.die("23", "Jesus");
    story.rise("24", "Jesus");
    story.event(
        "24:13-53",
        "the risen Jesus",
        "Emmaus, appearances to the disciples, and ascension",
    );
    story
}
