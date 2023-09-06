use std::fmt::{Display, Formatter};

//#[derive(Debug)]
struct Player {
    id: i32,
    nick_name: String,
    point: i32,
    level: Option<Level>,
}

//#[derive(Debug)]
enum Level {
    Mid,
    Pro(u8),
}

impl Player {
    fn new(id: i32, nick_name: String, point: i32, level: Option<Level>) -> Self {
        Self {
            id,
            nick_name,
            point,
            level,
        }
    }
    fn apply_promotion(&mut self, amount: i32) {
        self.point += amount;
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let level = match self.level {
            Some(Level::Mid) => "Yeni Nesil".to_string(),
            Some(Level::Pro(value)) => format!("{}. seviyeden profesyonel", value),
            None => "Bilinmeyen seviye".to_string(),
        };
        write!(f, "{} - {}", self.nick_name, level)
    }
}

fn main() {
    let mut pop = Player::new(13, "Prince of Persia".to_string(), 80, Some(Level::Pro(3)));
    pop.apply_promotion(10);
    println!("{}", pop);

    let pop = Player::new(15, "John Doe".to_string(), 55, None);
    println!("{}", pop);
}
