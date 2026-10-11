//! John: The Word has an eternal lifetime.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

use crate::miracles::{Alive, Dead, Person, Water, Wine};

/// John 2:1-11. Six waterpots, one extremely surprising From implementation.
pub fn water_into_wine(water: [Water; 6]) -> [Wine; 6] {
    water.map(Wine::from)
}

/// John 19:30-20:29. Each transition consumes the previous state.
pub fn resurrection() -> Person<Alive> {
    let jesus = Person::<Alive>::new("Jesus");
    let buried: Person<Dead> = jesus.die();
    let risen: Person<Alive> = buried.resurrect();
    risen
}

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("John");
    story.event(
        "1",
        "the Word made flesh",
        "John bears witness; the first disciples follow Jesus",
    );
    story.execute("2:1-11", "cana");
    story.event(
        "2:13-25",
        "Jesus",
        "cleanses the temple and speaks of its raising",
    );
    story.wisdom(
        "3",
        "Nicodemus and John's witness",
        "new birth, God's love, and the Son's authority",
    );
    story.event(
        "4",
        "Samaria and Galilee",
        "the woman at the well and an official's healed son",
    );
    story.event(
        "5",
        "Bethesda",
        "a healing leads to a dispute about the sabbath and the Son's authority",
    );
    story.event(
        "6",
        "the bread of life",
        "feeding, walking on the sea, and teaching that divides hearers",
    );
    story.event(
        "7-8",
        "Jesus at the feast",
        "disputes about his identity, origin, and authority",
    );
    story.event(
        "9",
        "the man born blind",
        "receives sight and faces questioning",
    );
    story.wisdom(
        "10",
        "the good shepherd",
        "Jesus describes his care for the sheep and unity with the Father",
    );
    story.die("11", "Lazarus");
    story.rise("11", "Lazarus");
    story.event(
        "12",
        "Jesus approaches death",
        "anointing, entry into Jerusalem, and a final public appeal",
    );
    story.law(
        "13",
        "the disciples",
        "Jesus washes their feet and commands mutual love",
    );
    story.letter(
        "14-16",
        "the disciples",
        "comfort, the coming Comforter, abiding, and coming sorrow turned to joy",
    );
    story.prayer(
        "17",
        "Jesus",
        "prays for his disciples and those who will believe",
    );
    story.event(
        "18-19",
        "the passion",
        "arrest, denial, trial, crucifixion, and burial",
    );
    story.die("19:30", "Jesus");
    story.rise("20", "Jesus");
    story.execute("20:1-29", "resurrection");
    story.event(
        "21",
        "the risen Jesus",
        "breakfast by the sea and Peter's commission",
    );
    story
}
