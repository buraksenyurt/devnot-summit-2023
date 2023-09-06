use std::fmt::{Display, Formatter};

struct Player {
    id: i32,
    title: String,
    point: i32,
}

impl Player {
    fn new(id: i32, title: String, point: i32) -> Self {
        Self { id, title, point }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} ({})", self.id, self.title, self.point)
    }
}

fn transfer_points(src_player: &Player, trg_player: &mut Player) {
    trg_player.point += src_player.point
}

fn main() {
    let baltazar = Player::new(13, "baltazar the pirate".to_string(), 18);
    let mut dufrey = Player::new(32, "endi dufrey".to_string(), 5);
    transfer_points(&baltazar, &mut dufrey);
    println!("{}", baltazar);
    println!("{}", dufrey);
}
