//! Revelation: A breaking change with no rollback.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

use crate::miracles::{Former, Renewed, World};

/// Revelation 21:1-5. The former world is moved, not patched in place.
pub fn all_things_new(former: World<Former>) -> World<Renewed> {
    former.make_all_things_new()
}

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("Revelation");
    story.vision("1", "John", "the risen Christ among seven candlesticks");
    story.letter(
        "2-3",
        "the seven churches",
        "commendations, rebukes, calls to repent, and promises",
    );
    story.vision(
        "4-5",
        "John",
        "the heavenly throne and the Lamb worthy to open the book",
    );
    story.vision(
        "6-7",
        "John",
        "six seals, the sealed servants, and a great multitude",
    );
    story.vision("8-9", "John", "the seventh seal and six trumpets");
    story.vision(
        "10-11",
        "John",
        "the little book, two witnesses, and the seventh trumpet",
    );
    story.vision(
        "12-14",
        "John",
        "the woman, dragon, beasts, Lamb, proclamations, and harvest",
    );
    story.vision(
        "15-16",
        "John",
        "seven angels pour out seven vials of judgment",
    );
    story.vision("17-18", "John", "Babylon's character and fall");
    story.vision(
        "19",
        "John",
        "heavenly praise, the marriage supper, and the victorious rider",
    );
    story.vision(
        "20",
        "John",
        "the binding and final defeat of Satan and the great white throne",
    );
    story.execute("21:1-5", "revelation");
    story.vision("21", "John", "a new heaven and earth and the holy city");
    story.vision(
        "22",
        "John",
        "the river and tree of life, final invitations, and the promise of coming",
    );
    story
}
