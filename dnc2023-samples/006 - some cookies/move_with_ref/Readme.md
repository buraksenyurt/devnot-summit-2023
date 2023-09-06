# Referans ile Sahipliği Taşımak

Örneğin başlangıç konumu şöyledir.

```rust
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

fn transfer_points(src_player: &Player, trg_player: &Player) {
    trg_player.point += src_player.point
}

fn main() {
    let baltazar = Player::new(13, "baltazar the pirate".to_string(), 18);
    let dufrey = Player::new(32, "endi dufrey".to_string(), 5);
    transfer_points(&baltazar, &dufrey);
}
```

Kodun bu halinde mutable olma haliyle ilgili ihlaller söz konusudur.

```text
error[E0594]: cannot assign to `trg_player.point`, which is behind a `&` reference
  --> src/main.rs:14:5
   |
14 |     trg_player.point += src_player.point
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `trg_player` is a `&` reference, so the data it refers to cannot be written
   |
help: consider changing this to be a mutable reference
   |
13 | fn transfer_points(src_player: &Player, trg_player: &mut Player) {
   |                                                      +++
```

Sonrasında kodda gerekli düzenlemeler yapılır. mutable olma, referansın mutable olması hali vs gösterilerek ilerlenir.

```rust
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
```

Kodun sonraki aşamasında trait konusuna atıfta bulunulabilir. Bunun için kod aşağıdaki şekilde değiştirilir.

```rust
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
```

Display trait kullanımı irdelendikten sonra cargo expand ile üretilen rust kodunun nasıl olduğu gösterilmelidir. Burada makroların neye dönüştüğü belirtilir ve write! makrosunun kaynak koduna gidilerek AST gibi kavramlar irdelenir.

expand çıktısı ile elde edilecek rust kodu aşağıdaki gibidir.

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
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
        f.write_fmt(
            format_args!("{0} - {1} ({2})", & self.id, & self.title, & self.point),
        )
    }
}
fn transfer_points(src_player: &Player, trg_player: &mut Player) {
    trg_player.point += src_player.point;
}
fn main() {
    let baltazar = Player::new(13, "baltazar the pirate".to_string(), 18);
    let mut dufrey = Player::new(32, "endi dufrey".to_string(), 5);
    transfer_points(&baltazar, &mut dufrey);
    {
        ::std::io::_print(format_args!("{0}\n", baltazar));
    };
    {
        ::std::io::_print(format_args!("{0}\n", dufrey));
    };
}
```

