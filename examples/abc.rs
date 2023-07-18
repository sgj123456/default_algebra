use procedure_macro::*;
#[watch]
#[default_algebra(i32:10,i64:20)]
#[derive(Debug)]
#[watch]
enum ABC {
    A(Nil),
    B(i64),
    C,
    D,
    E,
    F,
    G,
}

fn main() {
    println!("{:?}", ABC::A);
    println!("{:?}", ABC::b_default());
    println!("{:?}", ABC::c_default());
    println!("{:?}", ABC::d_default());
    println!("{:?}", ABC::e_default());
    println!("{:?}", ABC::f_default());
    println!("{:?}", ABC::g_default());
}
