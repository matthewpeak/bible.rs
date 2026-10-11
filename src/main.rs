use bible_rs::{
    canon::{self, BOOKS, Book, Testament},
    miracles::{self, DEMOS},
    scripture,
};
use std::{
    env,
    error::Error,
    io::{self, IsTerminal, Write},
    process::ExitCode,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> ExitCode {
    let args: Vec<_> = env::args().skip(1).collect();
    let color = io::stdout().is_terminal() && env::var_os("NO_COLOR").is_none();
    let mut out = io::BufWriter::new(io::stdout().lock());
    let result = run(&args, &mut out, color).and_then(|()| out.flush().map_err(Into::into));
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error)
            if error
                .downcast_ref::<io::Error>()
                .is_some_and(|e| e.kind() == io::ErrorKind::BrokenPipe) =>
        {
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("bible: {error}");
            ExitCode::from(2)
        }
    }
}

fn run(args: &[String], out: &mut impl Write, color: bool) -> Result<()> {
    let command = args.first().map(String::as_str).unwrap_or("--help");
    match command {
        "--help" | "-h" if args.len() <= 1 => {
            heading(out, "bible.rs", color)?;
            writeln!(out, "66 books. Zero dependencies. One eternal lifetime.\n")?;
            writeln!(out, "  bible genesis          Read and execute a book")?;
            writeln!(
                out,
                "  bible 1-samuel         Numbered books use hyphens or quotes"
            )?;
            writeln!(out, "  bible list             Browse all 66 books")?;
            writeln!(
                out,
                "  bible all              Run every book; print a compact ledger"
            )?;
            writeln!(out, "  bible all --full       Print every adapted scene")?;
            writeln!(
                out,
                "  bible demo             Run all eight Rust demonstrations"
            )?;
            writeln!(
                out,
                "  bible demo cana        Water -> Wine, with no unsafe code"
            )?;
            writeln!(
                out,
                "  bible source john      Print the actual Rust for a book"
            )?;
            writeln!(
                out,
                "  bible check            Validate all references and replay state"
            )?;
            writeln!(out, "\nDirect quotations: King James Version (KJV).")?;
            writeln!(out, "Narration: original, selective literary adaptation.")?;
        }
        "--version" | "-V" if args.len() == 1 => {
            writeln!(out, "bible.rs {}", env!("CARGO_PKG_VERSION"))?
        }
        "list" if args.len() == 1 => {
            for testament in [Testament::Old, Testament::New] {
                heading(
                    out,
                    if testament == Testament::Old {
                        "OLD TESTAMENT"
                    } else {
                        "NEW TESTAMENT"
                    },
                    color,
                )?;
                for book in BOOKS.iter().filter(|b| b.testament == testament) {
                    writeln!(
                        out,
                        "  {:>2}. {:<20} {}",
                        book.number, book.slug, book.tagline
                    )?;
                }
                writeln!(out)?;
            }
        }
        "all" if args.len() == 1 || (args.len() == 2 && args[1] == "--full") => {
            let full = args.len() == 2;
            let mut total = 0;
            if !full {
                heading(out, "THE WHOLE CANON", color)?;
            }
            for book in &BOOKS {
                let story = (book.run)();
                total += story.events.len();
                if full {
                    render_book(out, book, color)?;
                } else {
                    writeln!(
                        out,
                        "  {:>2}  {:<18} {:>3} scenes  {:>3} chapters",
                        book.number,
                        book.title,
                        story.events.len(),
                        book.chapters()
                    )?;
                }
            }
            writeln!(
                out,
                "\n66 books executed. {total} adapted scenes. 8 demonstrations."
            )?;
            writeln!(
                out,
                "Accounts run independently. Chapter counts describe the source books."
            )?;
        }
        "demo" if args.len() <= 2 => {
            let names: Vec<&str> = if args.len() == 1 || args[1] == "all" {
                DEMOS.to_vec()
            } else {
                vec![&args[1]]
            };
            for name in names {
                let demo = miracles::demonstrate(name)
                    .ok_or_else(|| format!("unknown demo '{name}'; choose {}", DEMOS.join(", ")))?;
                heading(out, &demo.name.to_uppercase(), color)?;
                writeln!(out, "{demo}")?;
            }
        }
        "source" if args.len() >= 2 => {
            let input = args[1..].join(" ");
            let book = canon::find(&input).ok_or_else(|| format!("unknown book '{input}'"))?;
            write!(out, "{}", book.source)?;
        }
        "check" if args.len() == 1 => {
            let mut count = 0;
            for book in &BOOKS {
                count += scripture::validate(book)?;
            }
            writeln!(out, "PASS  66 books execute and replay deterministically")?;
            writeln!(
                out,
                "PASS  {count} passage anchors fit KJV chapter and verse bounds"
            )?;
            writeln!(out, "PASS  66 KJV quotations have valid references")?;
            writeln!(
                out,
                "Quotation text was verified against the source at authoring time; see SOURCES.md."
            )?;
        }
        _ => {
            let input = args.join(" ");
            let book = canon::find(&input)
                .ok_or_else(|| format!("unknown book or command '{input}'; run 'bible --help'"))?;
            render_book(out, book, color)?;
        }
    }
    Ok(())
}

fn heading(out: &mut impl Write, text: &str, color: bool) -> io::Result<()> {
    if color {
        writeln!(out, "\x1b[1;33m{text}\x1b[0m")
    } else {
        writeln!(out, "{text}")
    }
}
fn render_book(out: &mut impl Write, book: &Book, color: bool) -> Result<()> {
    let story = (book.run)();
    heading(
        out,
        &format!("{:02} / {}", book.number, book.title.to_uppercase()),
        color,
    )?;
    writeln!(out, "{}\n", book.tagline)?;
    writeln!(
        out,
        "KJV / {} {}:{}",
        book.title, book.quote.chapter, book.quote.verse
    )?;
    // Line wrapping changes whitespace only, never the words of the quotation.
    let mut column = 2;
    write!(out, "  ")?;
    for (index, word) in book.quote.text.split_whitespace().enumerate() {
        if column + word.chars().count() + 1 > 88 {
            write!(out, "\n  ")?;
            column = 2;
        } else if index > 0 {
            write!(out, " ")?;
            column += 1;
        }
        write!(out, "{word}")?;
        column += word.chars().count();
    }
    writeln!(out, "\n\nADAPTATION / {} scenes", story.events.len())?;
    for event in &story.events {
        writeln!(
            out,
            "  {:<13} {:<12} {}",
            event.passage,
            event.action.kind(),
            event.action.describe()
        )?;
    }
    for demo in &story.demonstrations {
        writeln!(out, "\n{demo}")?;
    }
    writeln!(out)?;
    Ok(())
}
