use super::{InvalidPerson, Person, PersonIter, SmartIter};
use std::convert::TryFrom;

#[test]
fn iter() {
    let smart_iter = SmartIter::from("hello");
    assert_eq!(smart_iter.collect::<String>(), "hello");

    let smart_iter = SmartIter::from("");
    assert_eq!(smart_iter.collect::<String>(), "");

    let mut smart_iter = SmartIter::from("");
    assert_eq!(smart_iter.next(), None);
}

#[test]
fn take_until() {
    let mut smart_iter = SmartIter::from("hello");
    assert_eq!(smart_iter.take_until('h'), Some(""));
    assert_eq!(smart_iter.take_until('h'), Some(""));
    assert_eq!(smart_iter.take_until('%'), None);
    assert_eq!(smart_iter.take_until('o'), Some("hell"));
    assert_eq!(smart_iter.take_until('o'), Some(""));
    assert_eq!(smart_iter.next(), Some('o'));
    assert_eq!(smart_iter.as_str(), "");

    assert_eq!(smart_iter.take_until('h'), None);
    assert_eq!(smart_iter.take_until(""), Some(""));
}

#[test]
fn take_until_or_end() {
    let mut smart_iter = SmartIter::from("hello");
    assert_eq!(smart_iter.take_until_or_end('h'), "");
    assert_eq!(smart_iter.take_until_or_end('h'), "");
    assert_eq!(smart_iter.take_until_or_end('%'), "hello");
    assert_eq!(smart_iter.as_str(), "");
    assert_eq!(smart_iter.take_until_or_end('o'), "");
}

#[test]
fn take_word() {
    let mut smart_iter = SmartIter::from("");
    assert_eq!(smart_iter.take_word(), None);

    let mut smart_iter = SmartIter::from("    ");
    assert_eq!(smart_iter.take_word(), None);

    let mut smart_iter = SmartIter::from("hello ");
    assert_eq!(smart_iter.take_word(), Some("hello"));
    assert_eq!(smart_iter.as_str(), " ");

    let mut smart_iter = SmartIter::from("5hello ");
    assert_eq!(smart_iter.take_word(), None);

    let mut smart_iter = SmartIter::from("he!lo ");
    assert_eq!(smart_iter.take_word(), Some("he"));
    assert_eq!(smart_iter.as_str(), "!lo ");
}

#[test]
fn skip_whitespace() {
    let mut smart_iter = SmartIter::from("    ");
    smart_iter.skip_whitespace();
    assert_eq!(smart_iter.as_str(), "");

    let mut smart_iter = SmartIter::from("f    ");
    smart_iter.skip_whitespace();
    assert_eq!(smart_iter.as_str(), "f    ");

    let mut smart_iter = SmartIter::from(" \tf");
    smart_iter.skip_whitespace();
    assert_eq!(smart_iter.as_str(), "f");

    let mut smart_iter = SmartIter::from(" \tf  ");
    smart_iter.skip_whitespace();
    assert_eq!(smart_iter.as_str(), "f  ");
}

#[test]
fn try_from() {
    let mut smart_iter = SmartIter::from("");
    assert_eq!(Person::try_from(&mut smart_iter), Err(InvalidPerson));

    let mut smart_iter = SmartIter::from("Reiger");
    assert_eq!(Person::try_from(&mut smart_iter), Err(InvalidPerson));

    let mut smart_iter = SmartIter::from("Reiger Alex,");
    assert_eq!(Person::try_from(&mut smart_iter), Err(InvalidPerson));

    let mut smart_iter = SmartIter::from("Reiger Alex, Elaine");
    assert_eq!(Person::try_from(&mut smart_iter), Err(InvalidPerson));

    let mut smart_iter = SmartIter::from("Reiger, Alex");
    assert_eq!(
        Person::try_from(&mut smart_iter),
        Ok(Person {
            first_name: "Alex",
            last_name:  "Reiger",
        })
    );
    assert_eq!(smart_iter.as_str(), "");

    let mut smart_iter = SmartIter::from("Reiger,Alex");
    assert_eq!(
        Person::try_from(&mut smart_iter),
        Ok(Person {
            first_name: "Alex",
            last_name:  "Reiger",
        })
    );
    assert_eq!(smart_iter.as_str(), "");

    let mut smart_iter = SmartIter::from("Reiger ,Alex");
    assert_eq!(
        Person::try_from(&mut smart_iter),
        Ok(Person {
            first_name: "Alex",
            last_name:  "Reiger",
        })
    );
    assert_eq!(smart_iter.as_str(), "");

    let mut smart_iter = SmartIter::from("Reiger ,\tAlex");
    assert_eq!(
        Person::try_from(&mut smart_iter),
        Ok(Person {
            first_name: "Alex",
            last_name:  "Reiger",
        })
    );
    assert_eq!(smart_iter.as_str(), "");

    let mut smart_iter = SmartIter::from("\t\t Reiger ,\tAlex");
    assert_eq!(
        Person::try_from(&mut smart_iter),
        Ok(Person {
            first_name: "Alex",
            last_name:  "Reiger",
        })
    );
    assert_eq!(smart_iter.as_str(), "");

    let mut smart_iter = SmartIter::from("\t\t Reiger ,\tAlex ");
    assert_eq!(
        Person::try_from(&mut smart_iter),
        Ok(Person {
            first_name: "Alex",
            last_name:  "Reiger",
        })
    );
    assert_eq!(smart_iter.as_str(), " ");

    let mut smart_iter = SmartIter::from("\t\t Reiger ,\tAlex Elaine");
    assert_eq!(
        Person::try_from(&mut smart_iter),
        Ok(Person {
            first_name: "Alex",
            last_name:  "Reiger",
        })
    );
    assert_eq!(smart_iter.as_str(), " Elaine");
}

#[test]
fn people_iter() {
    let people = PersonIter::from("").collect::<Vec<_>>();
    assert_eq!(people, []);

    let people = PersonIter::from("\t   ").collect::<Vec<_>>();
    assert_eq!(people, []);

    let people = PersonIter::from("$%#@").collect::<Vec<_>>();
    assert_eq!(people, [Err(InvalidPerson)]);

    let people = PersonIter::from("Reiger, Alex").collect::<Vec<_>>();
    assert_eq!(
        people,
        [Ok(Person {
            first_name: "Alex",
            last_name:  "Reiger",
        })]
    );

    let people = PersonIter::from(" Reiger,Alex").collect::<Vec<_>>();
    assert_eq!(
        people,
        [Ok(Person {
            first_name: "Alex",
            last_name:  "Reiger",
        })]
    );

    let people = PersonIter::from(" Reiger,Alex;").collect::<Vec<_>>();
    assert_eq!(
        people,
        [Ok(Person {
            first_name: "Alex",
            last_name:  "Reiger",
        })]
    );

    let people = PersonIter::from("Reiger Alex").collect::<Vec<_>>();
    assert_eq!(people, [Err(InvalidPerson)]);

    let people = PersonIter::from("\tReiger ,Alex;Nardo").collect::<Vec<_>>();
    assert_eq!(
        people,
        [
            Ok(Person {
                first_name: "Alex",
                last_name:  "Reiger",
            }),
            Err(InvalidPerson)
        ]
    );

    let people = PersonIter::from("\tReiger ,Alex Nardo").collect::<Vec<_>>();
    assert_eq!(people, [Err(InvalidPerson)]);

    let people = PersonIter::from("\tReiger ,Alex;Nardo,Elaine  ").collect::<Vec<_>>();
    assert_eq!(
        people,
        [
            Ok(Person {
                first_name: "Alex",
                last_name:  "Reiger",
            }),
            Ok(Person {
                first_name: "Elaine",
                last_name:  "Nardo",
            })
        ]
    );

    let people = PersonIter::from(
        "\tReiger ,Alex;DePalma,Louie        ;Nardo,Elaine; Gravas   ,Latka;Banta, Tony;      ",
    )
    .collect::<Vec<_>>();
    assert_eq!(
        people,
        [
            Ok(Person {
                first_name: "Alex",
                last_name:  "Reiger",
            }),
            Ok(Person {
                first_name: "Louie",
                last_name:  "DePalma",
            }),
            Ok(Person {
                first_name: "Elaine",
                last_name:  "Nardo",
            }),
            Ok(Person {
                first_name: "Latka",
                last_name:  "Gravas",
            }),
            Ok(Person {
                first_name: "Tony",
                last_name:  "Banta",
            })
        ]
    );

    let people = PersonIter::from(
        "\tReiger ,Alex;DePalma,Louie        ;N@rdo,E!aine; Gravas   ,Latka;Banta, Tony;      ",
    )
    .collect::<Vec<_>>();
    assert_eq!(
        people,
        [
            Ok(Person {
                first_name: "Alex",
                last_name:  "Reiger",
            }),
            Ok(Person {
                first_name: "Louie",
                last_name:  "DePalma",
            }),
            Err(InvalidPerson),
            Ok(Person {
                first_name: "Latka",
                last_name:  "Gravas",
            }),
            Ok(Person {
                first_name: "Tony",
                last_name:  "Banta",
            })
        ]
    );
}
