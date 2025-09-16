struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // this is an associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn draw_rect_in_terminal(&self) {
        for idx in 0..self.height {
            let row = if self.width > 1 && idx != 0 && idx != self.height - 1 {
                format!("#{:width$}#", "", width = (self.width - 2) as usize)
            } else if idx == 0 || idx == self.height - 1 {
                "#".repeat(self.width as usize)
            } else {
                "#".to_string()
            };
            println!("{}", row);
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 100,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 20,
    };

    let rect3 = Rectangle::square(3);

    if rect1.can_hold(&rect2) {
        println!("rect1 can hold rect2");
    } else {
        println!("rect1 can't hold rect2");
    }

    println!(
        "The area if the rectangle is {} square pixels.",
        rect.area()
    );

    println!("The rect is {:?}", rect);

    println!("rect: ");
    rect.draw_rect_in_terminal();
    println!("rect 1: ");
    rect1.draw_rect_in_terminal();
    println!("rect 2: ");
    rect2.draw_rect_in_terminal();
    println!("rect 3: ");
    rect3.draw_rect_in_terminal();
}

fn user_example() {
    let mut user1 = User {
        email: String::from("jbenhod@gmail.com"),
        username: String::from("jbenhod"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("wallace123");

    let user2 = build_user(String::from("unclebenspiderman"), String::from("uncleben"));

    let user3 = User {
        email: String::from("james@gmail.com"),
        username: String::from("james123"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
