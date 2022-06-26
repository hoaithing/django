use crate::garden::vegetables::Plant;

pub mod garden;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i64
}

impl User {
    fn show_info(&self) {
        println!("{} - {}", self.username, self.email)
    }

    fn can_login(&self) -> bool {
        self.sign_in_count <= 10
    }

    fn show_can_login(&self) {
        if self.can_login() {
            println!("{} can login", self.username)
        } else {
            println!("{} can not login", self.username)
        }
    }
}

fn check(i: i32) -> Result<bool, i32> {
    let devide = i % 2;
    if devide == 0 {
        Ok(true)
    } else {
        Err(devide)
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: "hahungkk".to_string(),
        email: "hahungkk@gmail.com".to_string(),
        sign_in_count: 1
    };
    user1.show_info();
    user1.show_can_login();
    let user2 = User{sign_in_count: 20, ..user1 };
    user2.show_can_login();
    let plant = Plant {
        name: String::from("cherry"),
    };
    println!("{}", plant);

    for i in 0..10 {
        let result = check(i);
        if result.is_ok() {
            println!("{} is modulus", i)
        } else {
            println!("{} mod 2 = {}", i, result.unwrap_err())
        }
    }
}
