// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
    return input.trim().to_string();
}

fn compose_me(input: &str) -> String {
    /* option 1 */
    //let mut s = input.to_string();
    //s.insert_str(s.len(), " world!");
    //return s;

    /* option 2 */
    return format!("{} world!", input);
}

fn replace_me(input: &str) -> String {
    //return input.to_string().replace("cars", "balloons");
    return input.replace("cars", "balloons");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
