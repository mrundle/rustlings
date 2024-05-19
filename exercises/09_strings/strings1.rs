// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn current_favorite_color() -> String {
    /* option 1 */
    //return String::from("blue");

    /* option 2 */
    //return "blue".to_string();

    /* option 3 */
    //let mut s = String::new();
    //s.insert_str(s.len(), "blue");
    //return s;

    /* option 4 */
    //return format!("{}", "blue");

    /* option 5 (diabolical) */
    return "blue".to_string().to_string().to_string();
}
