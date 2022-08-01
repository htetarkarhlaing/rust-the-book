#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
}

impl IpAddrKind {
    fn printer() {
        println!("Hello From Enum");
    }
}

fn main() {
    let localhost = IpAddrKind::V4(127, 0, 0, 0);
    println!("{:?}", localhost);
    println!("{:?}", IpAddrKind::printer());
    let integer = 5;
    // let optional_integer: Option<i8> = Some(5);
    let optional_integer: Option<i8> = None;
    println!("{}", integer);
    println!("{:?}", optional_integer);

    let sum = integer + optional_integer.unwrap_or(0);
    println!("{}", sum);

    //---------------------------------------- ENUM & Match operator -------------------------------------------------//
    value_in_cents(Coin::Quarter(UsState::Alabama));

    let five = Some(5);
    println!("{:?}", plus_one(five));
    println!("None {:?}", plus_one(None));

    let some_value = Some(3);
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_value {
        println!("three")
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1), // we need to wrap with some
        // None => None,
        _ => None,
    }
}
