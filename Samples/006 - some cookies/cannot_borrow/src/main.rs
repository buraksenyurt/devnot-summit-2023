fn main() {}

struct Game {
    players: Vec<Fighter>,
}
impl Game {
    fn new() -> Self {
        Self {
            players: vec![Fighter::default(), Fighter::default()],
        }
    }

    /*

    error[E0502]: cannot borrow `self.players` as mutable because it is also borrowed as immutable
      --> src/main.rs:29:30
       |
    28 |         let fighter_1 = &self.players[0];
       |                          ------------ immutable borrow occurs here
    29 |         let fighter_2 = &mut self.players[1];
       |                              ^^^^^^^^^^^^ mutable borrow occurs here
    30 |         fighter_2.take_the_shoot(fighter_1.hit_count);
       |                                  ------------------- immutable borrow later used here

    For more information about this error, try `rustc --explain E0502`.
    error: could not compile `cannot_borrow` (bin "cannot_borrow") due to previous error

     */

    fn fight(&mut self) {
        let fighter_1 = &self.players[0];
        let fighter_2 = &mut self.players[1];
        fighter_2.take_the_shoot(fighter_1.hit_count);
    }
}

struct Fighter {
    health: i32,
    hit_count: i32,
    location: (f32, f32),
}

impl Default for Fighter {
    fn default() -> Self {
        Self {
            health: 100,
            hit_count: 0,
            location: (0., 10.),
        }
    }
}

impl Fighter {
    fn take_the_shoot(&mut self, amount: i32) {
        self.hit_count += amount;
    }
}
