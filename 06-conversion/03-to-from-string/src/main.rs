
/// To String

use std::fmt;

#[derive(Debug)]
struct Circle {
    radius: i32,
}

// impl fmt::Display for Circle {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Circle of radius {}", self.radius)
//     }
// }

// fn main() {
//     let circle = Circle { radius: 10 };
//     println!("{}", circle);
//     println!("{}", circle.to_string());
// }

/// Parse Int

// fn main() {
//     let parse: i32 = "10".parse().unwrap();
//     let turbo_parse = "15".parse::<i32>().unwrap();

//     let sum = parse + turbo_parse;
//     println!("{}", parse);
//     println!("{}", turbo_parse);
//     println!("{}", sum)
// }


/// Parse Into Under Defined Types

use std::str::FromStr;

impl FromStr for Circle {
    type Err = ();
    fn from_str(s: &str) -> Result<Circle, ()> {
        match s.trim().parse() {
            Ok(num) => Ok(Circle { radius: num }),
            Err(_) => Err(())
        }
    }
}

fn main() {
    let radius = "    3 ";
    let circle: Circle = radius.parse().unwrap();
    println!("{:?}", circle);
}