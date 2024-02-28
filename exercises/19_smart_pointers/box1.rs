// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: the value of the current item and
// the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.


#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(0, Box::new(List::Nil))
}

impl List {
    pub fn new() -> List {
        List::Nil
    }

    pub fn push(&mut self, value: i32) {
        let mut child = self; // <- child: &mut List
        while let List::Cons(_, next) = child { 
            child = &mut **next;
            // next:           &Box<List>
            // *next:          Box<List>
            // **next:         List
            // &**next:        &List
            // &mut **next:    &mut List
        } 

        // child equals List::Nil after previous loop
        *child = List::Cons(value, Box::new(List::Nil));
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use super::List::{Nil, Cons};

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }

    #[test]
    fn test_list_new(){
        assert_eq!(Nil, List::new())
    }

    #[test]
    fn test_list_push_one(){
        let mut list = List::new();
        List::push(&mut list, 63);
        assert_eq!(
            Cons(63, Box::new(Nil)),
            list
            )
    }

    #[test]
    fn test_list_push_five(){
        let b = Box::new;
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);

        assert_eq!(
            Cons(1, b(Cons(2, b(Cons(3, b(Cons(4, b(Cons(5, b(Nil)))))))))),
            list
            );
    }
}
