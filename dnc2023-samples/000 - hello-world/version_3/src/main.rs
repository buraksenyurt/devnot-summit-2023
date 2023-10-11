use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct Player<'a> {
    id: i32,
    title: &'a str,
    point: f32,
    level: Level,
}

impl<'a> Player<'a> {
    fn new(id: i32, title: &'a str, point: f32, level: Level) -> Self {
        Self {
            id,
            title,
            point,
            level,
        }
    }
}
impl<'a> Display for Player<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let level_info = match self.level {
            Level::Rookie | Level::Pro => "Standart oyuncu".to_string(),
            Level::Elit(dan) => format!("{}. dandan elit oyuncu", dan),
        };
        write!(f, "{}-({}). {}", self.title, self.point, level_info)
    }
}

fn apply_promotion(player: &mut Player, amount: f32) {
    player.point += amount;
}
#[derive(Debug)]
enum Level {
    Rookie,
    Pro,
    Elit(u8),
}
fn main() {
    let mut maryo = Player::new(32, "Maryo Can Cey", 6.25, Level::Elit(3));
    println!("{}", maryo);
    apply_promotion(&mut maryo, 1.5);
    println!("{}", maryo);
}
