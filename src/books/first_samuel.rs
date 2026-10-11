//! 1 Samuel: The first king passes the wrong acceptance tests.
//!
//! Original literary adaptation. Direct quotations are in the KJV catalog.
use crate::story::Chronicle;

pub fn run() -> Chronicle {
    let mut story = Chronicle::new("1 Samuel");
    story.prayer(
        "1-2",
        "Hannah",
        "asks for a son, dedicates Samuel, and gives thanks",
    );
    story.appoint("3", "Samuel", "prophet of the LORD");
    story.event(
        "4-6",
        "the ark",
        "captured by the Philistines and returned after judgment",
    );
    story.event(
        "7",
        "Mizpeh",
        "Israel repents; Samuel judges; the Philistines are defeated",
    );
    story.appoint(
        "8-10",
        "Saul",
        "king of Israel after the people request a king",
    );
    story.event(
        "11-12",
        "Saul and Samuel",
        "Jabesh-gilead is rescued; Samuel charges the people",
    );
    story.event(
        "13-15",
        "Saul",
        "disobedience brings rejection of his continuing kingship",
    );
    story.appoint("16", "David", "anointed future king");
    story.event("17", "David", "defeats Goliath");
    story.covenant(
        "18-20",
        "David and Jonathan",
        "friendship endures Saul's hostility",
    );
    story.event(
        "21-26",
        "David in flight",
        "Nob, the wilderness, Abigail, and two refusals to kill Saul",
    );
    story.event(
        "27-30",
        "David at Ziklag",
        "lives among the Philistines and rescues the captives",
    );
    story.die("31", "Saul and Jonathan");
    story
}
