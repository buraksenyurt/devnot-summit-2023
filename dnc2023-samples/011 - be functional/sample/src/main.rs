#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Player {
    id: u32,
    nick_name: String,
    bonus: f64,
    level: u32,
}

fn main() {
    let mut players = vec![
        Player {
            id: 18,
            nick_name: String::from("Persival"),
            bonus: 100.0,
            level: 90,
        },
        Player {
            id: 22,
            nick_name: String::from("Tiriniti"),
            bonus: 150.0,
            level: 95,
        },
        Player {
            id: 3,
            nick_name: String::from("Kapitan Marvıl"),
            bonus: 80.0,
            level: 120,
        },
        Player {
            id: 6,
            nick_name: String::from("M.Jey"),
            bonus: 125.0,
            level: 98,
        },
    ];
    println!("Promosyondan önce\n {:#?}", players);

    let updated_players: Vec<&mut Player> = players
        .iter_mut()
        .filter(|player| player.level > 100)
        .map(|player| {
            player.bonus *= 1.10;
            player
        })
        .collect();

    println!(
        "Promosyondan sonra\n {:#?}",
        updated_players
            .iter()
            .filter(|player| player.level > 100)
            .collect::<Vec<_>>()
    );
}
