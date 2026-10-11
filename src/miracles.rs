//! The bits that would have been rejected in an ordinary code review.
//!
//! These are literary models, not assertions about how miracles work.
use std::{fmt, marker::PhantomData, sync::mpsc, thread};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Creation {
    pub day: u8,
    pub light: bool,
    pub firmament: bool,
    pub dry_land: bool,
    pub plants: bool,
    pub sun_moon_stars: bool,
    pub fish_and_birds: bool,
    pub animals_and_humankind: bool,
    pub rested: bool,
    pub revisions: u8,
}

impl Creation {
    pub fn in_the_beginning() -> Self {
        Self::default()
    }

    /// Genesis 1:1–2:3. Light ships before the solar infrastructure.
    pub fn next_day(&mut self) -> Result<(), &'static str> {
        match self.day + 1 {
            1 => self.light = true,
            2 => self.firmament = true,
            3 => {
                self.dry_land = true;
                self.plants = true;
            }
            4 => self.sun_moon_stars = true,
            5 => self.fish_and_birds = true,
            6 => self.animals_and_humankind = true,
            7 => self.rested = true,
            _ => return Err("creation week is complete"),
        }
        self.day += 1;
        if self.day <= 6 {
            self.revisions += 1;
        }
        Ok(())
    }
}

/// Intentionally neither Copy nor Clone. One fruit, one owner.
///
/// ```compile_fail
/// use bible_rs::miracles::{ForbiddenFruit, Human};
/// let fruit = ForbiddenFruit::pick();
/// let mut eve = Human::new("Eve");
/// let mut adam = Human::new("Adam");
/// eve.receive(fruit).unwrap();
/// adam.receive(fruit).unwrap(); // error[E0382]: use of moved value
/// ```
#[derive(Debug)]
pub struct ForbiddenFruit {
    portions: u8,
}
impl ForbiddenFruit {
    pub fn pick() -> Self {
        Self { portions: 2 }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EdenError {
    NoFruit,
    Disobedience,
}
#[derive(Debug)]
pub struct Human {
    pub name: &'static str,
    pub fallen: bool,
    fruit: Option<ForbiddenFruit>,
}
impl Human {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            fallen: false,
            fruit: None,
        }
    }
    pub fn receive(&mut self, fruit: ForbiddenFruit) -> Result<(), ForbiddenFruit> {
        if self.fruit.is_some() {
            return Err(fruit);
        }
        self.fruit = Some(fruit);
        Ok(())
    }
    pub fn give(&mut self) -> Result<ForbiddenFruit, EdenError> {
        self.fruit.take().ok_or(EdenError::NoFruit)
    }
    pub fn eat(&mut self) -> Result<(), EdenError> {
        let fruit = self.fruit.as_mut().ok_or(EdenError::NoFruit)?;
        fruit.portions -= 1;
        if fruit.portions == 0 {
            self.fruit.take();
        }
        self.fallen = true;
        Err(EdenError::Disobedience)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimalKind {
    Clean,
    Unclean,
    Bird,
}
impl AnimalKind {
    /// Genesis 7:2–3: "by sevens" is rendered here as seven male/female pairs.
    pub const fn pairs(self) -> u8 {
        match self {
            Self::Clean | Self::Bird => 7,
            Self::Unclean => 1,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Passenger {
    pub species: &'static str,
    pub pairs: u8,
}
#[derive(Debug, Default)]
pub struct Ark {
    pub manifest: Vec<Passenger>,
}
impl Ark {
    pub fn embark(&mut self, species: &'static str, kind: AnimalKind) -> Result<(), &'static str> {
        if self.manifest.iter().any(|p| p.species == species) {
            return Err("already aboard");
        }
        self.manifest.push(Passenger {
            species,
            pairs: kind.pairs(),
        });
        Ok(())
    }
    pub fn animals(&self) -> u32 {
        self.manifest.iter().map(|p| u32::from(p.pairs) * 2).sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tongue {
    Shared,
    First,
    Second,
    Third,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Babel {
    pub understood: usize,
    pub scattered: bool,
}
impl Babel {
    /// Three real threads, one message protocol. Genesis 11:1–9.
    /// Language names are abstract because the passage does not name them.
    pub fn build(confused: bool) -> Self {
        let tongues = if confused {
            [Tongue::First, Tongue::Second, Tongue::Third]
        } else {
            [Tongue::Shared; 3]
        };
        let understood = thread::scope(|scope| {
            let workers: Vec<_> = tongues
                .into_iter()
                .map(|language| {
                    let (sender, receiver) = mpsc::channel();
                    let worker = scope.spawn(move || {
                        let message = receiver.recv().expect("a building instruction");
                        usize::from(language == message)
                    });
                    sender
                        .send(Tongue::Shared)
                        .expect("worker receives the instruction");
                    worker
                })
                .collect();
            workers
                .into_iter()
                .map(|w| w.join().expect("Babel worker panicked"))
                .sum()
        });
        Self {
            understood,
            scattered: understood < 3,
        }
    }
}

/// The passage owns the mutable borrow. Nobody can close the sea underneath it.
///
/// ```compile_fail
/// use bible_rs::miracles::RedSea;
/// let mut sea = RedSea::default();
/// let mut passage = sea.part();
/// sea.part(); // error[E0499]: cannot borrow as mutable more than once
/// passage.cross("Israel");
/// ```
#[derive(Debug, Default)]
pub struct RedSea {
    parted: bool,
    pub crossed: Vec<&'static str>,
}
impl RedSea {
    pub fn is_parted(&self) -> bool {
        self.parted
    }
    pub fn part(&mut self) -> Passage<'_> {
        self.parted = true;
        Passage { sea: self }
    }
}
pub struct Passage<'a> {
    sea: &'a mut RedSea,
}
impl Passage<'_> {
    pub fn cross(&mut self, people: &'static str) {
        self.sea.crossed.push(people);
    }
}
impl Drop for Passage<'_> {
    fn drop(&mut self) {
        self.sea.parted = false;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Water {
    pub firkins: u8,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Wine {
    pub firkins: u8,
}
impl From<Water> for Wine {
    fn from(water: Water) -> Self {
        Self {
            firkins: water.firkins,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Alive;
#[derive(Debug, PartialEq, Eq)]
pub struct Dead;
#[derive(Debug, PartialEq, Eq)]
pub struct Person<State> {
    pub name: &'static str,
    state: PhantomData<State>,
}
impl Person<Alive> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            state: PhantomData,
        }
    }
    pub fn die(self) -> Person<Dead> {
        Person {
            name: self.name,
            state: PhantomData,
        }
    }
}
impl Person<Dead> {
    /// ```compile_fail
    /// use bible_rs::miracles::{Person, Alive};
    /// let living = Person::<Alive>::new("Lazarus");
    /// living.resurrect(); // resurrection requires the Dead state
    /// ```
    pub fn resurrect(self) -> Person<Alive> {
        Person {
            name: self.name,
            state: PhantomData,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Former;
#[derive(Debug, PartialEq, Eq)]
pub struct Renewed;
#[derive(Debug, PartialEq, Eq)]
pub struct World<State> {
    pub death: bool,
    pub sorrow: bool,
    pub sea: bool,
    pub tears: u64,
    state: PhantomData<State>,
}
impl Default for World<Former> {
    fn default() -> Self {
        Self {
            death: true,
            sorrow: true,
            sea: true,
            tears: 1,
            state: PhantomData,
        }
    }
}
impl World<Former> {
    pub fn make_all_things_new(self) -> World<Renewed> {
        World {
            death: false,
            sorrow: false,
            sea: false,
            tears: 0,
            state: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Commandment {
    NoOtherGods,
    NoGravenImages,
    DoNotTakeTheNameInVain,
    RememberTheSabbath,
    HonourFatherAndMother,
    DoNotKill,
    DoNotCommitAdultery,
    DoNotSteal,
    DoNotBearFalseWitness,
    DoNotCovet,
}
/// Exodus 20:3–17. No setters were requested.
pub const COMMANDMENTS: [Commandment; 10] = [
    Commandment::NoOtherGods,
    Commandment::NoGravenImages,
    Commandment::DoNotTakeTheNameInVain,
    Commandment::RememberTheSabbath,
    Commandment::HonourFatherAndMother,
    Commandment::DoNotKill,
    Commandment::DoNotCommitAdultery,
    Commandment::DoNotSteal,
    Commandment::DoNotBearFalseWitness,
    Commandment::DoNotCovet,
];

#[derive(Debug, Clone)]
pub struct Demonstration {
    pub name: &'static str,
    pub reference: &'static str,
    pub mechanism: &'static str,
    pub output: Vec<String>,
}
impl fmt::Display for Demonstration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} / {}", self.name, self.reference)?;
        writeln!(f, "  Rust: {}", self.mechanism)?;
        for line in &self.output {
            writeln!(f, "  {line}")?;
        }
        Ok(())
    }
}

pub const DEMOS: [&str; 8] = [
    "creation",
    "eden",
    "ark",
    "babel",
    "exodus",
    "cana",
    "resurrection",
    "revelation",
];

pub fn demonstrate(name: &str) -> Option<Demonstration> {
    let (reference, mechanism, output) = match name {
        "creation" => {
            let mut output = Vec::new();
            let changes = [
                "light",
                "firmament",
                "land + plants",
                "sun + moon + stars",
                "fish + birds",
                "animals + humankind",
                "rest",
            ];
            for (i, (change, universe)) in changes
                .iter()
                .zip(crate::books::genesis::creation_week())
                .enumerate()
            {
                output.push(format!(
                    "day {}  {:<23} revisions={}",
                    i + 1,
                    change,
                    universe.revisions
                ));
            }
            (
                "Genesis 1:1-2:3",
                "state transitions; day seven makes no creation changes",
                output,
            )
        }
        "eden" => {
            let crate::books::genesis::Fall {
                eve,
                adam,
                eve_result,
                adam_result,
            } = crate::books::genesis::the_fall();
            (
                "Genesis 3:1-7",
                "move semantics and Result; memory safety cannot enforce obedience",
                vec![
                    format!("Eve eats: {eve_result:?}"),
                    "Eve transfers the remaining fruit to Adam.".into(),
                    format!("Adam eats: {adam_result:?}"),
                    format!("fallen: Eve={}, Adam={}", eve.fallen, adam.fallen),
                    "The compiler permitted this. The commandment did not.".into(),
                ],
            )
        }
        "ark" => {
            let ark = crate::books::genesis::noahs_ark();
            let mut output: Vec<_> = ark
                .manifest
                .iter()
                .map(|p| {
                    format!(
                        "{:<8} {} pairs / {} animals",
                        p.species,
                        p.pairs,
                        p.pairs * 2
                    )
                })
                .collect();
            output.push(format!(
                "Sample manifest: {} animals. Genesis 7 has special cases.",
                ark.animals()
            ));
            (
                "Genesis 7:2-3",
                "typed animal categories; a sample manifest, not every species",
                output,
            )
        }
        "babel" => {
            let (before, after) = crate::books::genesis::babel();
            (
                "Genesis 11:1-9",
                "scoped threads and incompatible message protocols",
                vec![
                    format!(
                        "shared tongue:   {}/3 workers understand",
                        before.understood
                    ),
                    format!(
                        "confused tongues: {}/3 workers understand",
                        after.understood
                    ),
                    format!("scattered={}; construction halted", after.scattered),
                ],
            )
        }
        "exodus" => {
            let sea = crate::books::exodus::cross_the_red_sea();
            (
                "Exodus 14:21-31",
                "RAII: the sea closes when the passage leaves scope",
                vec![
                    format!("crossed: {}", sea.crossed.join(", ")),
                    format!("passage dropped; sea parted={}", sea.is_parted()),
                ],
            )
        }
        "cana" => {
            // John 2:6 says two or three firkins per pot. Use two for this model.
            let jars = std::array::from_fn::<_, 6, _>(|_| Water { firkins: 2 });
            let wine = crate::books::john::water_into_wine(jars);
            (
                "John 2:1-11",
                "From<Water> for Wine; ownership transfers without unsafe",
                vec![
                    format!("{} waterpots converted", wine.len()),
                    format!(
                        "{} firkins of wine",
                        wine.iter().map(|w| u32::from(w.firkins)).sum::<u32>()
                    ),
                    "No transmute. No dependencies. Unexpectedly good vintage.".into(),
                ],
            )
        }
        "resurrection" => {
            let risen = crate::books::john::resurrection();
            (
                "John 19:30-20:29",
                "typestate: Alive -> Dead -> Alive, consuming each prior state",
                vec![
                    format!("{}: Alive -> Dead -> Alive", risen.name),
                    "The previous state cannot be used after the transition.".into(),
                ],
            )
        }
        "revelation" => {
            let former = World::<Former>::default();
            let renewed = crate::books::revelation::all_things_new(former);
            (
                "Revelation 21:1-5",
                "a consuming migration from World<Former> to World<Renewed>",
                vec![
                    format!(
                        "death={} sorrow={} sea={}",
                        renewed.death, renewed.sorrow, renewed.sea
                    ),
                    format!("tears={}", renewed.tears),
                    "Former world consumed. No rollback implemented.".into(),
                ],
            )
        }
        _ => return None,
    };
    // All names returned here are drawn from DEMOS, keeping the public data static.
    let name = *DEMOS.iter().find(|candidate| **candidate == name)?;
    Some(Demonstration {
        name,
        reference,
        mechanism,
        output,
    })
}
