//! Acts: The message survives every attempt to terminate the process.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Acts");
    story.event("1", "the apostles", "Jesus ascends; Matthias is chosen");
    story.event(
        "2",
        "Pentecost",
        "the Holy Ghost comes; hearers understand; many join the believers",
    );
    story.event(
        "3-5",
        "the Jerusalem church",
        "healing, testimony, opposition, shared goods, and internal judgment",
    );
    story.event(
        "6-7",
        "Stephen",
        "seven are appointed; Stephen testifies and is killed",
    );
    story.journey(
        "8",
        "the gospel witnesses",
        "Samaria and the Ethiopian traveller",
    );
    story.event(
        "9",
        "Saul",
        "meets Jesus, is baptised, and begins preaching",
    );
    story.event(
        "10-11",
        "Cornelius and Antioch",
        "Gentiles receive the Holy Ghost; the church recognises their inclusion",
    );
    story.event(
        "12",
        "Herod and Peter",
        "James is killed; Peter escapes prison; Herod dies",
    );
    story.journey(
        "13-14",
        "Paul and Barnabas",
        "Cyprus and cities of Asia Minor",
    );
    story.event(
        "15",
        "the Jerusalem council",
        "Gentile inclusion is affirmed without requiring circumcision",
    );
    story.journey(
        "16-18",
        "Paul and his companions",
        "Macedonia, Athens, and Corinth",
    );
    story.event(
        "19-20",
        "Paul's ministry",
        "Ephesus, an uproar, further journeys, and farewell to the elders",
    );
    story.event(
        "21-23",
        "Paul in Jerusalem",
        "arrest, defence, and transfer under guard",
    );
    story.event(
        "24-26",
        "Paul's hearings",
        "testimony before Felix, Festus, and Agrippa",
    );
    story.event("27", "the voyage", "a storm and shipwreck spare all aboard");
    story.journey("28", "Paul", "Rome, where he teaches under guard");
    story
}
