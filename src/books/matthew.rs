//! Matthew: The kingdom has a different definition of greatness.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Matthew");
    story.event(
        "1-2",
        "Jesus' birth",
        "genealogy, Joseph's dream, wise men, Egypt, and return to Nazareth",
    );
    story.event(
        "3",
        "John the Baptist",
        "prepares the way and baptises Jesus",
    );
    story.event(
        "4",
        "Jesus",
        "is tempted, begins preaching, and calls disciples",
    );
    story.law(
        "5-7",
        "the disciples",
        "the sermon on the mount teaches kingdom righteousness",
    );
    story.event(
        "8-9",
        "Jesus' ministry",
        "healing, deliverance, a calmed storm, and the calling of Matthew",
    );
    story.appoint("10", "the twelve", "sent to preach and heal");
    story.event(
        "11-12",
        "responses to Jesus",
        "John's question, warnings, sabbath disputes, and growing opposition",
    );
    story.wisdom(
        "13",
        "parables of the kingdom",
        "the sower, tares, mustard seed, leaven, treasure, pearl, and net",
    );
    story.event(
        "14-15",
        "Jesus' ministry",
        "John is killed; multitudes are fed; Jesus walks on the sea",
    );
    story.event(
        "16-17",
        "the disciples",
        "Peter confesses; Jesus foretells his death; the transfiguration",
    );
    story.law(
        "18",
        "the disciples",
        "humility, care for the little ones, correction, and forgiveness",
    );
    story.wisdom(
        "19-20",
        "following Jesus",
        "marriage, wealth, reward, service, and the vineyard labourers",
    );
    story.event(
        "21-23",
        "Jesus in Jerusalem",
        "entry, temple cleansing, disputes, and rebuke of hypocrisy",
    );
    story.prophecy(
        "24-25",
        "Jesus",
        "judgment, watchfulness, and readiness for the Son of man",
    );
    story.event(
        "26",
        "the passion begins",
        "anointing, supper, Gethsemane, betrayal, arrest, and Peter's denial",
    );
    story.die("27", "Jesus");
    story.rise("28", "Jesus");
    story.appoint("28:16-20", "the disciples", "make disciples of all nations");
    story
}
