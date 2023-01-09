extern crate adder;

#[test]
fn larger_can_hold_smaller() {
    let larger = adder::Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = adder::Rectangle {
        width: 5,
        height: 1,
    };
    assert!(larger.can_hold(&smaller));
}

#[test]
fn it_adds_two() {
    let x = 5;
    assert_eq!(7, adder::add_two(x));
}

#[test]
fn it_greets() {
    let name = String::from("ognis1205");
    //        let name = String::from("Carol");
    let result = adder::greeting(name.as_str());
    assert!(
        result.contains("ognis1205"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}

#[test]
#[should_panic(expected = "Guess value must be between 1 and 100")]
fn greater_than_100() {
    let guess = adder::Guess::new(101);
}
