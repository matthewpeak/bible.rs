//! Validate passage anchors against the KJV's actual chapter and verse lengths.
use crate::canon::Book;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Verse {
    pub chapter: u16,
    pub verse: u16,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Passage {
    pub start: Verse,
    pub end: Verse,
}

pub fn parse(book: &Book, reference: &str) -> Result<Passage, String> {
    let invalid = || format!("invalid KJV reference: {} {reference}", book.title);
    let length = |chapter: u16| -> Result<u16, String> {
        chapter
            .checked_sub(1)
            .and_then(|i| book.verse_counts.get(usize::from(i)))
            .copied()
            .ok_or_else(invalid)
    };
    let number = |s: &str| s.parse::<u16>().ok().filter(|n| *n > 0).ok_or_else(invalid);
    let mut sides = reference.split('-');
    let first = sides.next().ok_or_else(invalid)?;
    let last = sides.next();
    if sides.next().is_some() {
        return Err(invalid());
    }
    let explicit = first.contains(':');
    let start = if let Some((chapter, verse)) = first.split_once(':') {
        Verse {
            chapter: number(chapter)?,
            verse: number(verse)?,
        }
    } else {
        Verse {
            chapter: number(first)?,
            verse: 1,
        }
    };
    let end = match last {
        Some(last) => {
            if let Some((chapter, verse)) = last.split_once(':') {
                Verse {
                    chapter: number(chapter)?,
                    verse: number(verse)?,
                }
            } else if explicit {
                Verse {
                    chapter: start.chapter,
                    verse: number(last)?,
                }
            } else {
                let chapter = number(last)?;
                Verse {
                    chapter,
                    verse: length(chapter)?,
                }
            }
        }
        None if explicit => start,
        None => Verse {
            chapter: start.chapter,
            verse: length(start.chapter)?,
        },
    };
    if start > end || start.verse > length(start.chapter)? || end.verse > length(end.chapter)? {
        return Err(invalid());
    }
    Ok(Passage { start, end })
}

pub fn validate(book: &Book) -> Result<usize, String> {
    let story = (book.run)();
    if story.book != book.title || story.events.is_empty() {
        return Err(format!("{} has no valid account", book.title));
    }
    for event in &story.events {
        parse(book, event.passage)?;
    }
    parse(
        book,
        &format!("{}:{}", book.quote.chapter, book.quote.verse),
    )?;
    if book.quote.text.trim().is_empty() {
        return Err(format!("{} has an empty quotation", book.title));
    }
    if story.state != story.replay() {
        return Err(format!("{} failed replay", book.title));
    }
    Ok(story.events.len())
}
