struct User {
    username: String,
    email: String,
    sing_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

//struct seem like a interface or type of TS

fn main() {
    let mut user1 = User {
        email: String::from("arkar@gmail.com"),
        username: String::from("Arkar"),
        active: true,
        sing_in_count: 1,
    };

    let name = user1.username;
    let email = user1.email;
    let sing_in_count = user1.sing_in_count;
    let is_active = user1.active;
    println!("{}, {}, {}, {}", name, email, sing_in_count, is_active);
    user1.username = String::from("Brkar");
    let user2 = build_user(String::from("crkar@gmail.com"), String::from("Crkar"));
    let user3 = User {
        email: String::from("drkar@gmail.com"),
        username: String::from("Drkar"),
        ..user2 // this ..user2 will be used as a active: true, sing_in_count: 1
    };
    println!("{}", user3.username);

    //------------------------------------------- Tuple Struct ------------------------------------------------------//
    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);
    //--------------------------------------- Use Cases  ----------------------------------------------//
    let rect = Rectangle {
        width: 30,
        height: 60,
    };

    let rect_2 = Rectangle {
        width: 40,
        height: 50,
    };
    let area_of_rectangle = area(&rect);
    println!("{:?}", rect);
    println!("Area of rect {}", area_of_rectangle);
    println!("Area of rect {}", rect.area()); // print by impl
    println!("Rect 2 is {} to handle Rect 1", rect_2.can_hold(rect));

    let square = Rectangle::square(30);
    println!("Our square is {:?}", square);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sing_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
