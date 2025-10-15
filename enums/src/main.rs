enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_function() {
        println!("let's get Rusty!");
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddrKind::V4(127, 0, 0, 1);

    // value_in_cents(Coin::Quarter(UsState::Alabama));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn route(ip_kind: IpAddrKind) {}

// now let's talk about the Option type in Rust
// it is defined as follows:
// enum Option<T> {
//     None,
//     Some(T),
// }

// here's an example of using Option<T>
// instead of using null values
// which can lead to errors
// we use Option<T> to represent a value that can be either something or nothing
// this is a safer way to handle absence of values in Rust
fn option_example() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // this will not compile because we cannot add i8 and Option<i8>
    // we need to handle the Option type properly
    // one way to do this is to use pattern matching or methods like unwrap_or
    // here we use unwrap_or to provide a default value of 0 if y is None
    let sum = x + y.unwrap_or(0); // using unwrap_or to provide a default value
    println!("The sum is: {}", sum);
}

// let's talk about the match expression in Rust
// it allows us to compare a value against a series of patterns and execute code based on which pattern matches
// here's an example of using match with the IpAddrKind enum

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None,
    }
}
