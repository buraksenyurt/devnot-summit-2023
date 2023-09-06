use std::f32::consts::PI;

fn main() {
    // 01 - Aşağıda klasik bir referans yoluyla değer atama ve kullanma söz konusu.
    let point = PI;
    let new_point = calculate_something(&point);
    let other_point = calculate_something(&point);
    println!("{new_point} , {other_point}");

    // //#02
    // let point = PI;
    // let new_point = calculate_something_v2(&point);
    // let other_point = calculate_something_v2(&point);
    // println!("{new_point} , {other_point}");
    // /*
    //     fonksiyon referansı mutable beklediği için aşağıdaki hata alınır.
    //     &mut point olarak atanmalıdır. Hatta point değişkeni de mutable olmalıdır.

    // error[E0308]: mismatched types
    //     --> src/main.rs:9:44
    //     |
    //     9  |     let new_point = calculate_something_v2(&point);
    //     |                     ---------------------- ^^^^^^ types differ in mutability
    //     |                     |
    //     |                     arguments to this function are incorrect
    //     |
    //     = note: expected mutable reference `&mut f32`
    //                         found reference `&{float}`

    //  */
}

// #01 - Borrowing Hakkında
fn calculate_something(value: &f32) -> f32 {
    let mut calculated = *value;
    calculated += 1.;
    calculated
}

// #02 01'i şöyle değiştirirsek */
fn calculate_something_v2(value: &mut f32) -> f32 {
    let mut calculated = *value;
    calculated += 1.;
    calculated
}
