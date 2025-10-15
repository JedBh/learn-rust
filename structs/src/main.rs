#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    rectangles: Vec<Rectangle>,
}

impl User {
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
            rectangles: Vec::new(),
        }
    }

    fn add_rectangle(&mut self, mut rect: Rectangle) {
        rect.owner = Some(self.username.clone());
        self.rectangles.push(rect);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    owner: Option<String>,
}

impl Rectangle {
    // this is an associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
            owner: None,
        }
    }

    fn create_rectangle(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
            owner: None,
        }
    }

    fn _area(&self) -> u32 {
        self.width * self.height
    }

    fn draw_rect_in_terminal(&self) {
        let text = self.owner.as_deref().unwrap_or("");
        for idx in 0..self.height {
            let row = if idx == 0 || idx == self.height - 1 {
                "#".repeat(self.width as usize)
            } else if self.height / 2 == idx && text.len() <= (self.width - 2) as usize {
                format!("#{:width$}#", text, width = (self.width - 2) as usize)
            } else {
                format!("#{:width$}#", "", width = (self.width - 2) as usize)
            };
            println!("{}", row);
        }
    }

    fn _can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut jbenhod = User::build_user(String::from("jbenhod@gmail.com"), String::from("jbenhod"));

    let rect1 = Rectangle::square(9);

    let rect2 = Rectangle::create_rectangle(12, 3);

    let rect3 = Rectangle::square(3);

    jbenhod.add_rectangle(rect1);
    jbenhod.add_rectangle(rect2);
    jbenhod.add_rectangle(rect3);

    for rect in &jbenhod.rectangles {
        rect.draw_rect_in_terminal();
        println!("{:#?}", rect);
    }
}
