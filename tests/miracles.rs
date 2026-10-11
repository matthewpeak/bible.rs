use bible_rs::{books, miracles::*};
use std::panic::{AssertUnwindSafe, catch_unwind};

#[test]
fn light_exists_before_the_sun() {
    let days = books::genesis::creation_week();
    assert!(days[0].light);
    assert!(!days[0].sun_moon_stars);
    assert!(!days[2].sun_moon_stars);
    assert!(days[3].sun_moon_stars);
}
#[test]
fn the_seventh_day_does_not_mutate_creation() {
    let days = books::genesis::creation_week();
    let mut sixth = days[5].clone();
    let seventh = &days[6];
    assert_eq!(sixth.revisions, 6);
    assert_eq!(seventh.revisions, 6);
    sixth.day = 7;
    sixth.rested = true;
    assert_eq!(&sixth, seventh);
}
#[test]
fn an_eighth_creation_day_does_not_sneak_in() {
    let mut final_day = books::genesis::creation_week()[6].clone();
    let before = final_day.clone();
    assert!(final_day.next_day().is_err());
    assert_eq!(before, final_day);
}
#[test]
fn the_fruit_moves_and_both_people_eat() {
    let mut eve = Human::new("Eve");
    let mut adam = Human::new("Adam");
    eve.receive(ForbiddenFruit::pick()).unwrap();
    assert_eq!(eve.eat(), Err(EdenError::Disobedience));
    adam.receive(eve.give().unwrap()).unwrap();
    assert!(matches!(eve.give(), Err(EdenError::NoFruit)));
    assert_eq!(adam.eat(), Err(EdenError::Disobedience));
    assert_eq!(adam.eat(), Err(EdenError::NoFruit));
    assert!(eve.fallen && adam.fallen);
}
#[test]
fn an_empty_hand_does_not_cause_the_fall() {
    let mut adam = Human::new("Adam");
    assert_eq!(adam.eat(), Err(EdenError::NoFruit));
    assert!(!adam.fallen);
}
#[test]
fn a_rejected_fruit_is_returned_to_its_owner() {
    let mut eve = Human::new("Eve");
    eve.receive(ForbiddenFruit::pick()).unwrap();
    let returned = eve.receive(ForbiddenFruit::pick()).unwrap_err();
    let mut adam = Human::new("Adam");
    assert!(adam.receive(returned).is_ok());
}
#[test]
fn clean_animals_are_not_limited_to_one_pair() {
    assert_eq!(AnimalKind::Clean.pairs(), 7);
    assert_eq!(AnimalKind::Bird.pairs(), 7);
    assert_eq!(AnimalKind::Unclean.pairs(), 1);
    assert_eq!(books::genesis::noahs_ark().animals(), 30);
}
#[test]
fn the_ark_does_not_double_count_a_species() {
    let mut ark = Ark::default();
    ark.embark("sheep", AnimalKind::Clean).unwrap();
    assert!(ark.embark("sheep", AnimalKind::Clean).is_err());
    assert_eq!(ark.animals(), 14);
}
#[test]
fn babel_loses_communication_without_losing_threads() {
    let (before, after) = books::genesis::babel();
    assert_eq!(
        before,
        Babel {
            understood: 3,
            scattered: false
        }
    );
    assert_eq!(
        after,
        Babel {
            understood: 0,
            scattered: true
        }
    );
}
#[test]
fn the_red_sea_closes_when_the_passage_leaves_scope() {
    let sea = books::exodus::cross_the_red_sea();
    assert!(!sea.is_parted());
    assert_eq!(sea.crossed, vec!["Israel"]);
}
#[test]
fn the_sea_also_closes_during_unwinding() {
    let mut sea = RedSea::default();
    let result = catch_unwind(AssertUnwindSafe(|| {
        let mut passage = sea.part();
        passage.cross("Israel");
        panic!("the scope exits unexpectedly");
    }));
    assert!(result.is_err());
    assert!(!sea.is_parted());
    assert_eq!(sea.crossed, vec!["Israel"]);
}
#[test]
fn cana_preserves_all_six_waterpots_and_their_volumes() {
    let water = [2, 3, 2, 3, 2, 3].map(|firkins| Water { firkins });
    let wine = books::john::water_into_wine(water);
    assert_eq!(wine.map(|w| w.firkins), [2, 3, 2, 3, 2, 3]);
}
#[test]
fn resurrection_preserves_identity() {
    let risen: Person<Alive> = books::john::resurrection();
    assert_eq!(risen.name, "Jesus");
    let lazarus = Person::<Alive>::new("Lazarus").die().resurrect();
    assert_eq!(lazarus.name, "Lazarus");
}
#[test]
fn the_new_creation_has_no_death_sorrow_sea_or_tears() {
    let renewed = books::revelation::all_things_new(World::<Former>::default());
    assert!(!renewed.death && !renewed.sorrow && !renewed.sea);
    assert_eq!(renewed.tears, 0);
}
#[test]
fn sinai_has_ten_distinct_commandments() {
    assert_eq!(COMMANDMENTS.len(), 10);
    for (i, commandment) in COMMANDMENTS.iter().enumerate() {
        assert!(!COMMANDMENTS[..i].contains(commandment));
    }
}
#[test]
fn every_advertised_demo_executes() {
    for name in DEMOS {
        let demo = demonstrate(name).unwrap();
        assert!(!demo.output.is_empty());
        assert_eq!(demo.name, name);
    }
    assert!(demonstrate("prosperity").is_none());
}
