// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let cat = ("Furry McFurson", 3.5);

    /* typing not strictly necessary, but \_o_/ */
    let name: &str;
    let age: f32;

    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}
