# Scripture and coverage

Direct scripture quotations use the **King James Version**, using the
standardised 1769 text supplied by [eBible.org](https://ebible.org/kjv/).
Narration, code comments, taglines, and Rust metaphors are original adaptations
unless explicitly identified as scripture. They are not modernised KJV quotes.

## Source record

- [Edition information](https://ebible.org/Scriptures/details.php?id=eng-kjv)
- [USFX source archive](https://ebible.org/Scriptures/eng-kjv_usfx.zip)
- Retrieved: **2026-10-11**
- Source member: `eng-kjv_usfx.xml`
- SHA-256 of that XML: `9f21efb28694a3c9441186da8d792c2ec194e9bf6a1710856a3c26b5cc9ef25a`

The source contains additional books. This project's canon is the **66-book
Protestant collection**, with 1,189 chapters and 31,102 verses in its reference
metadata. Verse counts describe the source, not the number of verses adapted.

The 66 selected quotations in [assets/kjv-quotes.tsv](assets/kjv-quotes.tsv)
were extracted from the source with footnotes, cross references, markup, and
paragraph marks removed. Whitespace was normalised. Words and punctuation were
preserved. The catalog is checked against this fixture by the test suite.
The source publisher identifies the KJV as public domain outside the UK and
provides the edition's rights information on the linked source page.

## What has been adapted

Every book has a Rust module, for a total of **600 selected scenes**. A scene may
represent a single event or condense several chapters of laws, genealogies,
poetry, prophecy, narrative, or teaching. This is a literary adaptation at the
story and teaching level, not a verse-by-verse implementation. The event log is a
partial model of each book's account, not a complete model of its world.

Eight scenes also have dedicated Rust demonstrations. Other scenes update typed
narrative state or collect typed laws, prayers, wisdom, prophecies, letters, and
visions. Each book begins in its own narrative context. Parallel accounts in
Samuel, Kings, Chronicles, and the Gospels are not forced into one chronology.

`cargo run -- check` validates that every passage anchor fits the KJV's chapter
and verse bounds, that quotations have valid references, and that each account's
state can be reproduced from its event log. It does not judge theology or prove
that a summary captures every detail of a passage.

## Explicit modeling choices

- Genesis 7:2-3's clean animals and birds taken "by sevens" are represented as
  seven male/female pairs. Unclean animals are represented as one pair. The ark
  demo is a three-species sample manifest.
- The fruit in Genesis 3 is not identified as an apple. The ownership model uses
  one fruit with two portions so Eve can eat and pass the remaining portion to Adam.
- Babel's language names are abstract. The passage does not identify them.
- John 2:6 gives two or three firkins per waterpot. The demo chooses two; the
  conversion preserves each pot's volume.
- Resurrection uses consuming type transitions. The program does not simulate
  elapsed days or claim to explain the miracle.
- Revelation's demo models the stated absence of sea, death, sorrow, and tears
  in Revelation 21:1-5. Its types are a literary rendering of the vision.
- Mark 16 includes the KJV's closing verses. Passage wording and numbering follow
  this requested edition.

## Contributions

Keep passage anchors accurate. Use verified KJV wording for direct quotes and
identify original paraphrases as narration. Add tests for new executable
behaviour. Keep the joke aimed at the unnecessary software rewrite.
