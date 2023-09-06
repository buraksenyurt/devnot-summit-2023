use std::fmt::{Display, Formatter};
/*
    version_1 den farklı olarak burada işin içerisine &str(literal string) türü ve
    dolayısıyla lifetime kullanımı girer. Örneğin çevrimine version_1'den başlanır.
    nick_name alanı String türünden &str türüne çevrilir ve alınan hatalar üzerinden ilerlenir.
*/

//#[derive(Debug)]
struct Player<'a> {
    id: i32,
    nick_name: &'a str,
    point: i32,
    level: Option<Level>,
}

//#[derive(Debug)]
enum Level {
    Mid,
    Pro(u8),
}

impl<'a> Player<'a> {
    fn new(id: i32, nick_name: &'a str, point: i32, level: Option<Level>) -> Self {
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

impl<'a> Display for Player<'a> {
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
    let mut pop = Player::new(13, "Prince of Persia", 80, Some(Level::Pro(3)));
    pop.apply_promotion(10);
    println!("{}", pop);

    let pop = Player::new(15, "John Doe", 55, None);
    println!("{}", pop);
}
