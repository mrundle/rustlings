// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];
    let vec1 = fill_vec(vec0);
    assert_eq!(vec1, vec![22, 44, 66, 88]);

    //let vec0 = vec![22, 44, 66];
    //let vec1 = fill_vec_from_ref(&vec0);
    //assert_eq!(vec1, vec![22, 44, 66, 88]);

    //let mut vec0 = vec![22, 44, 66];
    //fill_vec_mut(&mut vec0);
    //assert_eq!(vec0, vec![22, 44, 66, 88]);
}

/* pass by reference and then create copy */
fn fill_vec_from_ref(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec2: Vec<i32> = [].to_vec();
    for i in vec.iter() {
        vec2.push(*i);
    }
    vec2.push(88);
    return vec2;
}

/* mutable reference, modify original */
fn fill_vec_mut(vec: &mut Vec<i32>) {
    vec.push(88);
}

/*
 * the actual solution; pass by copy on the stack?
 */
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(88);
    vec
}
