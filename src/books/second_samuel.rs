//! 2 Samuel: A successful deployment does not make the maintainer infallible.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("2 Samuel");
    story.prayer("1", "David", "laments Saul and Jonathan");
    story.event(
        "2-4",
        "the divided kingdom",
        "David reigns in Judah while Saul's house declines",
    );
    story.appoint("5", "David", "king over all Israel");
    story.journey("6", "the ark", "Jerusalem");
    story.covenant("7", "David", "a lasting house and kingdom are promised");
    story.event(
        "8-10",
        "David's kingdom",
        "victories and kindness to Mephibosheth",
    );
    story.event(
        "11",
        "David, Bathsheba, and Uriah",
        "David commits adultery and arranges Uriah's death",
    );
    story.event(
        "12",
        "Nathan",
        "confronts David; David confesses; the child dies",
    );
    story.event(
        "13-14",
        "Amnon and Absalom",
        "sexual violence, murder, exile, and return divide the household",
    );
    story.event(
        "15-18",
        "Absalom's rebellion",
        "David flees; Absalom is defeated and killed",
    );
    story.event(
        "19-20",
        "David's return",
        "the kingdom is restored but another rebellion follows",
    );
    story.event(
        "21",
        "Gibeon and the Philistines",
        "a famine is addressed and further conflicts are recorded",
    );
    story.prayer("22", "David", "a song thanks the LORD for deliverance");
    story.event(
        "23",
        "David's last words and mighty men",
        "a ruler's justice and warriors' deeds are recalled",
    );
    story.event(
        "24",
        "the census",
        "judgment follows David's census; an altar is built",
    );
    story
}
