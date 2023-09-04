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

fn transfer_points(src_player: &Player, trg_player: &mut Player) {
    trg_player.point += src_player.point
}

fn main() {
    let baltazar = Player::new(13, "baltazar the pirate".to_string(), 18);
    let mut dufrey = Player::new(32, "endi dufrey".to_string(), 5);
    transfer_points(&baltazar, &mut dufrey);
}
