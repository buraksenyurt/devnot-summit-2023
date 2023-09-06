/*

    Multiple-Reader Single-Writer (MSRW) mevzusu.
*/
use std::f32::consts::PI;

fn main() {
    let mut point = PI;
    do_something(&mut point, &point);
}

fn do_something(x_value: &mut f32, y_value: &f32) {
    *x_value += y_value;
}

/*
error[E0502]: cannot borrow `point` as immutable because it is also borrowed as mutable
 --> src/main.rs:3:30
  |
3 |     do_something(&mut point, &point);
  |     ------------ ----------  ^^^^^^ immutable borrow occurs here
  |     |            |
  |     |            mutable borrow occurs here
  |     mutable borrow later used by call

For more information about this error, try `rustc --explain E0502`.
*/
