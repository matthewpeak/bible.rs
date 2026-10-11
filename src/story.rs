//! A small event log. Each book is an independent narrative account.
use crate::miracles::{Demonstration, demonstrate};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Life {
    Living,
    Dead,
    Risen,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Law {
    pub audience: &'static str,
    pub instruction: &'static str,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prayer {
    pub speaker: &'static str,
    pub petition: &'static str,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wisdom {
    pub subject: &'static str,
    pub counsel: &'static str,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prophecy {
    pub speaker: &'static str,
    pub oracle: &'static str,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Letter {
    pub audience: &'static str,
    pub teaching: &'static str,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vision {
    pub witness: &'static str,
    pub image: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Create {
        thing: &'static str,
    },
    Move {
        who: &'static str,
        to: &'static str,
    },
    Appoint {
        who: &'static str,
        office: &'static str,
    },
    Build {
        structure: &'static str,
    },
    Destroy {
        structure: &'static str,
    },
    Covenant {
        name: &'static str,
        promise: &'static str,
    },
    Die {
        who: &'static str,
    },
    Rise {
        who: &'static str,
    },
    Record {
        subject: &'static str,
        outcome: &'static str,
    },
    Law(Law),
    Prayer(Prayer),
    Wisdom(Wisdom),
    Prophecy(Prophecy),
    Letter(Letter),
    Vision(Vision),
    Demonstrate {
        name: &'static str,
    },
}
impl Action {
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Create { .. } => "create",
            Self::Move { .. } => "journey",
            Self::Appoint { .. } => "appoint",
            Self::Build { .. } => "build",
            Self::Destroy { .. } => "destroy",
            Self::Covenant { .. } => "covenant",
            Self::Die { .. } => "death",
            Self::Rise { .. } => "resurrection",
            Self::Record { .. } => "event",
            Self::Law(_) => "law",
            Self::Prayer(_) => "prayer",
            Self::Wisdom(_) => "wisdom",
            Self::Prophecy(_) => "prophecy",
            Self::Letter(_) => "letter",
            Self::Vision(_) => "vision",
            Self::Demonstrate { .. } => "execute",
        }
    }
    pub fn describe(&self) -> String {
        match self {
            Self::Create { thing } => format!("Create {thing}"),
            Self::Move { who, to } => format!("{who} -> {to}"),
            Self::Appoint { who, office } => format!("{who} appointed {office}"),
            Self::Build { structure } => format!("Build {structure}"),
            Self::Destroy { structure } => format!("Destroy {structure}"),
            Self::Covenant { name, promise } => format!("{name}: {promise}"),
            Self::Die { who } => format!("{who} dies"),
            Self::Rise { who } => format!("{who} rises from the dead"),
            Self::Record { subject, outcome } => format!("{subject}: {outcome}"),
            Self::Law(l) => format!("{}: {}", l.audience, l.instruction),
            Self::Prayer(p) => format!("{}: {}", p.speaker, p.petition),
            Self::Wisdom(w) => format!("{}: {}", w.subject, w.counsel),
            Self::Prophecy(p) => format!("{}: {}", p.speaker, p.oracle),
            Self::Letter(l) => format!("{}: {}", l.audience, l.teaching),
            Self::Vision(v) => format!("{} sees {}", v.witness, v.image),
            Self::Demonstrate { name } => format!("Run {name} in Rust"),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct State {
    pub creation: BTreeSet<&'static str>,
    pub locations: BTreeMap<&'static str, &'static str>,
    pub offices: BTreeMap<&'static str, &'static str>,
    pub structures: BTreeSet<&'static str>,
    pub covenants: BTreeMap<&'static str, &'static str>,
    pub lives: BTreeMap<&'static str, Life>,
    pub outcomes: BTreeMap<&'static str, &'static str>,
    pub laws: Vec<Law>,
    pub prayers: Vec<Prayer>,
    pub wisdom: Vec<Wisdom>,
    pub prophecies: Vec<Prophecy>,
    pub letters: Vec<Letter>,
    pub visions: Vec<Vision>,
}
impl State {
    pub fn apply(&mut self, action: &Action) {
        match action {
            Action::Create { thing } => {
                self.creation.insert(thing);
            }
            Action::Move { who, to } => {
                self.locations.insert(who, to);
            }
            Action::Appoint { who, office } => {
                self.offices.insert(office, who);
                self.lives.entry(who).or_insert(Life::Living);
            }
            Action::Build { structure } => {
                self.structures.insert(structure);
            }
            Action::Destroy { structure } => {
                self.structures.remove(structure);
            }
            Action::Covenant { name, promise } => {
                self.covenants.insert(name, promise);
            }
            Action::Die { who } => {
                self.lives.insert(who, Life::Dead);
            }
            Action::Rise { who } => {
                self.lives.insert(who, Life::Risen);
            }
            Action::Record { subject, outcome } => {
                self.outcomes.insert(subject, outcome);
            }
            Action::Law(l) => self.laws.push(l.clone()),
            Action::Prayer(p) => self.prayers.push(p.clone()),
            Action::Wisdom(w) => self.wisdom.push(w.clone()),
            Action::Prophecy(p) => self.prophecies.push(p.clone()),
            Action::Letter(l) => self.letters.push(l.clone()),
            Action::Vision(v) => self.visions.push(v.clone()),
            Action::Demonstrate { .. } => {}
        }
    }
}
#[derive(Debug, Clone)]
pub struct Event {
    pub passage: &'static str,
    pub action: Action,
}
#[derive(Debug, Clone)]
pub struct Chronicle {
    pub book: &'static str,
    pub state: State,
    pub events: Vec<Event>,
    pub demonstrations: Vec<Demonstration>,
}
impl Chronicle {
    pub fn new(book: &'static str) -> Self {
        Self {
            book,
            state: State::default(),
            events: vec![],
            demonstrations: vec![],
        }
    }
    pub fn push(&mut self, passage: &'static str, action: Action) {
        self.state.apply(&action);
        self.events.push(Event { passage, action });
    }
    pub fn create(&mut self, p: &'static str, thing: &'static str) {
        self.push(p, Action::Create { thing });
    }
    pub fn journey(&mut self, p: &'static str, who: &'static str, to: &'static str) {
        self.push(p, Action::Move { who, to });
    }
    pub fn appoint(&mut self, p: &'static str, who: &'static str, office: &'static str) {
        self.push(p, Action::Appoint { who, office });
    }
    pub fn build(&mut self, p: &'static str, structure: &'static str) {
        self.push(p, Action::Build { structure });
    }
    pub fn destroy(&mut self, p: &'static str, structure: &'static str) {
        self.push(p, Action::Destroy { structure });
    }
    pub fn covenant(&mut self, p: &'static str, name: &'static str, promise: &'static str) {
        self.push(p, Action::Covenant { name, promise });
    }
    pub fn die(&mut self, p: &'static str, who: &'static str) {
        self.push(p, Action::Die { who });
    }
    pub fn rise(&mut self, p: &'static str, who: &'static str) {
        self.push(p, Action::Rise { who });
    }
    pub fn event(&mut self, p: &'static str, subject: &'static str, outcome: &'static str) {
        self.push(p, Action::Record { subject, outcome });
    }
    pub fn law(&mut self, p: &'static str, audience: &'static str, instruction: &'static str) {
        self.push(
            p,
            Action::Law(Law {
                audience,
                instruction,
            }),
        );
    }
    pub fn prayer(&mut self, p: &'static str, speaker: &'static str, petition: &'static str) {
        self.push(p, Action::Prayer(Prayer { speaker, petition }));
    }
    pub fn wisdom(&mut self, p: &'static str, subject: &'static str, counsel: &'static str) {
        self.push(p, Action::Wisdom(Wisdom { subject, counsel }));
    }
    pub fn prophecy(&mut self, p: &'static str, speaker: &'static str, oracle: &'static str) {
        self.push(p, Action::Prophecy(Prophecy { speaker, oracle }));
    }
    pub fn letter(&mut self, p: &'static str, audience: &'static str, teaching: &'static str) {
        self.push(p, Action::Letter(Letter { audience, teaching }));
    }
    pub fn vision(&mut self, p: &'static str, witness: &'static str, image: &'static str) {
        self.push(p, Action::Vision(Vision { witness, image }));
    }
    pub fn execute(&mut self, p: &'static str, name: &'static str) {
        self.demonstrations
            .push(demonstrate(name).expect("known demonstration"));
        self.push(p, Action::Demonstrate { name });
    }
    /// State can be rebuilt from the log. The four Gospels remain four accounts.
    pub fn replay(&self) -> State {
        let mut state = State::default();
        for event in &self.events {
            state.apply(&event.action);
        }
        state
    }
}
