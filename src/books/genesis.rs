//! Genesis: An unusually ambitious initial commit.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

use crate::miracles::{AnimalKind, Ark, Babel, Creation, EdenError, ForbiddenFruit, Human};

/// Genesis 1:1-2:3. Six days of creation. One day without a commit.
pub fn creation_week() -> [Creation; 7] {
    let mut universe = Creation::in_the_beginning();
    std::array::from_fn(|_| {
        universe.next_day().expect("creation has seven days");
        universe.clone()
    })
}

pub struct Fall {
    pub eve: Human,
    pub adam: Human,
    pub eve_result: Result<(), EdenError>,
    pub adam_result: Result<(), EdenError>,
}

/// Genesis 3:1-7. The fruit is shared in sequence, never copied.
/// Memory safety turns out to be a narrower guarantee than moral safety.
pub fn the_fall() -> Fall {
    let fruit = ForbiddenFruit::pick();
    let mut eve = Human::new("Eve");
    let mut adam = Human::new("Adam");

    eve.receive(fruit).expect("empty hands");
    let eve_result = eve.eat();
    adam.receive(eve.give().expect("a portion remains"))
        .expect("empty hands");
    let adam_result = adam.eat();

    Fall {
        eve,
        adam,
        eve_result,
        adam_result,
    }
}

/// Genesis 7:2-3. A sample manifest with the actual categorical distinction.
pub fn noahs_ark() -> Ark {
    let mut ark = Ark::default();
    ark.embark("sheep", AnimalKind::Clean).expect("new species");
    ark.embark("lions", AnimalKind::Unclean)
        .expect("new species");
    ark.embark("doves", AnimalKind::Bird).expect("new species");
    ark
}

/// Genesis 11:1-9. Workers still run. They no longer understand the instruction.
pub fn babel() -> (Babel, Babel) {
    (Babel::build(false), Babel::build(true))
}

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Genesis");
    story.execute("1:1-2:3", "creation");
    story.create("1:1-2:3", "the heaven and the earth");
    story.create("2:4-25", "Adam, Eve, and the garden of Eden");
    story.execute("3:1-7", "eden");
    story.journey("3:8-24", "Adam and Eve", "outside Eden");
    story.event(
        "4",
        "Cain and Abel",
        "Cain kills Abel and goes east; Seth is born",
    );
    story.event(
        "5",
        "Adam's descendants",
        "the genealogy reaches Noah; Enoch is taken by God",
    );
    story.build("6:1-22", "Noah's ark");
    story.execute("7:2-3", "ark");
    story.event(
        "7-8",
        "the flood",
        "the waters rise, then recede; Noah leaves the ark",
    );
    story.covenant(
        "9:1-17",
        "Noah",
        "the bow marks God's promise against another flood destroying all flesh",
    );
    story.event(
        "9:18-10:32",
        "Noah's family",
        "the sons of Noah become the nations",
    );
    story.execute("11:1-9", "babel");
    story.journey("11:10-12:9", "Abram", "Canaan");
    story.event(
        "12:10-14:24",
        "Abram and Lot",
        "Egypt, separation, the rescue of Lot, and Melchizedek's blessing",
    );
    story.covenant("15", "Abram", "descendants and a land promised");
    story.event(
        "16",
        "Hagar",
        "Ishmael is born after Hagar encounters the angel of the LORD",
    );
    story.covenant(
        "17",
        "Abraham",
        "circumcision; Abraham and Sarah receive new names",
    );
    story.event(
        "18-19",
        "Sodom",
        "Abraham intercedes; Lot escapes the cities' destruction",
    );
    story.event(
        "20-21",
        "Abraham's household",
        "Abimelech is warned; Isaac is born; Hagar and Ishmael depart",
    );
    story.event("22", "Isaac", "a ram is provided in place of Abraham's son");
    story.event(
        "23-24",
        "Sarah and Rebekah",
        "Sarah is buried; Rebekah becomes Isaac's wife",
    );
    story.event(
        "25-27",
        "Jacob and Esau",
        "birthright is sold; Jacob receives Isaac's blessing by deception",
    );
    story.vision("28", "Jacob", "a ladder reaching to heaven at Bethel");
    story.event(
        "29-31",
        "Jacob's household",
        "Leah, Rachel, their handmaids, and children; departure from Laban",
    );
    story.event(
        "32-33",
        "Jacob",
        "wrestles, is named Israel, and meets Esau in peace",
    );
    story.event(
        "34-36",
        "Israel's family",
        "violence at Shechem; return to Bethel; Rachel and Isaac die; Esau's descendants",
    );
    story.journey("37", "Joseph", "Egypt, sold by his brothers");
    story.event(
        "38",
        "Judah and Tamar",
        "Tamar exposes Judah's failure to provide for her",
    );
    story.event(
        "39-40",
        "Joseph",
        "serves Potiphar, is imprisoned, and interprets prisoners' dreams",
    );
    story.appoint("41", "Joseph", "governor over Egypt under Pharaoh");
    story.event(
        "42-45",
        "Joseph's brothers",
        "famine brings them to Egypt; Joseph reveals himself",
    );
    story.journey("46-47", "Israel's household", "Goshen in Egypt");
    story.event(
        "48-49",
        "Jacob",
        "blesses Joseph's sons and the twelve sons of Israel",
    );
    story.event(
        "50",
        "Joseph",
        "forgives his brothers and asks that his bones return to the promised land",
    );
    story
}
