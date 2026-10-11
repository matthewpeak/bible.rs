use bible_rs::{
    books,
    canon::{self, BOOKS, Testament},
    scripture::{self, Verse},
    story::{Action, Life},
};
use std::collections::BTreeSet;

#[test]
fn the_canon_has_39_old_and_27_new_testament_books() {
    assert_eq!(
        BOOKS
            .iter()
            .filter(|b| b.testament == Testament::Old)
            .count(),
        39
    );
    assert_eq!(
        BOOKS
            .iter()
            .filter(|b| b.testament == Testament::New)
            .count(),
        27
    );
    assert_eq!(BOOKS.iter().map(|b| b.chapters()).sum::<usize>(), 1189);
    assert_eq!(
        BOOKS
            .iter()
            .flat_map(|b| b.verse_counts)
            .map(|n| usize::from(*n))
            .sum::<usize>(),
        31102
    );
    assert_eq!(
        BOOKS.iter().map(|b| b.slug).collect::<BTreeSet<_>>().len(),
        66
    );
}
#[test]
fn every_book_runs_with_valid_kjv_passage_anchors() {
    for book in &BOOKS {
        assert!(scripture::validate(book).unwrap_or_else(|e| panic!("{e}")) > 0);
    }
}
#[test]
fn all_accounts_replay_without_state_drift() {
    for book in &BOOKS {
        let story = (book.run)();
        assert_eq!(story.state, story.replay(), "{}", book.title);
    }
}
#[test]
fn quotations_match_the_verified_source_fixture() {
    let fixture = include_str!("../assets/kjv-quotes.tsv");
    assert_eq!(fixture.lines().count(), 66);
    for (book, line) in BOOKS.iter().zip(fixture.lines()) {
        let (reference, text) = line.split_once('\t').unwrap();
        assert_eq!(
            reference,
            format!("{} {}:{}", book.title, book.quote.chapter, book.quote.verse)
        );
        assert_eq!(book.quote.text, text, "{} quotation changed", book.title);
    }
}
#[test]
fn the_king_james_version_keeps_charity() {
    let quote = canon::find("1-corinthians").unwrap().quote;
    assert_eq!(
        quote.text,
        "And now abideth faith, hope, charity, these three; but the greatest of these is charity."
    );
    assert_eq!(canon::find("john").unwrap().quote.text, bible_rs::THE_WORD);
}
#[test]
fn passage_ranges_respect_chapters_and_verses() {
    let genesis = canon::find("genesis").unwrap();
    let range = scripture::parse(genesis, "1:31-2:3").unwrap();
    assert_eq!(
        range.start,
        Verse {
            chapter: 1,
            verse: 31
        }
    );
    assert_eq!(
        range.end,
        Verse {
            chapter: 2,
            verse: 3
        }
    );
    assert_eq!(
        scripture::parse(genesis, "1-2").unwrap().end,
        Verse {
            chapter: 2,
            verse: 25
        }
    );
    assert_eq!(
        scripture::parse(genesis, "1:1-3").unwrap().end,
        Verse {
            chapter: 1,
            verse: 3
        }
    );
}
#[test]
fn invalid_passages_are_rejected_instead_of_silently_truncated() {
    let genesis = canon::find("genesis").unwrap();
    for reference in [
        "0", "51", "1:0", "1:32", "2-1", "1:8-3", "1-2-3", "", "x", "1::2", "1:1-51:1",
    ] {
        assert!(
            scripture::parse(genesis, reference).is_err(),
            "accepted {reference}"
        );
    }
}
#[test]
fn numbered_books_accept_readable_names_and_slugs() {
    for book in &BOOKS {
        assert_eq!(canon::find(book.title).unwrap().number, book.number);
        assert_eq!(canon::find(book.slug).unwrap().number, book.number);
        assert_eq!(
            canon::find(&book.number.to_string()).unwrap().number,
            book.number
        );
    }
    assert_eq!(
        canon::find("Song of Songs").unwrap().title,
        "Song of Solomon"
    );
    assert!(canon::find("67").is_none());
    assert!(canon::find("not-a-book").is_none());
}
#[test]
fn parallel_gospels_do_not_mutate_each_other() {
    for story in [
        books::matthew::run(),
        books::mark::run(),
        books::luke::run(),
        books::john::run(),
    ] {
        assert_eq!(story.state.lives.get("Jesus"), Some(&Life::Risen));
        let deaths = story
            .events
            .iter()
            .filter(|e| matches!(e.action, Action::Die { who: "Jesus" }))
            .count();
        assert_eq!(deaths, 1);
    }
}
#[test]
fn non_narrative_books_preserve_their_literary_forms() {
    assert!(!books::psalms::run().state.prayers.is_empty());
    assert!(!books::proverbs::run().state.wisdom.is_empty());
    assert!(!books::leviticus::run().state.laws.is_empty());
    assert!(!books::isaiah::run().state.prophecies.is_empty());
    assert!(!books::romans::run().state.letters.is_empty());
    assert!(!books::revelation::run().state.visions.is_empty());
}
