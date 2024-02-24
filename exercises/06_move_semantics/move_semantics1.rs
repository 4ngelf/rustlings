// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    // assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88, 99]);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    // let vec = vec;

    vec.push(88);
    vec.push(99);

    vec
}
