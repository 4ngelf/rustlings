// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // <- literals are &'static str
    string("red".to_string()); // <- str::to_string returns a String
    string(String::from("hi")); // <- String::from returns a String
    string("rust is fun!".to_owned()); // <- str::to_owned returns a String

    string_slice("nice weather".into()); // <- Could be either one
    string("nice weather".into());       //      It converts to needed type

    string(format!("Interpolation {}", "Station")); // <- This macro returns String
    string_slice(&String::from("abc")[0..1]); // <- String references to &str
    string_slice("  hello there ".trim());    // <- str::trim returns a &str
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // <- str::replace
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // <- str::to_lowercase returns String
}
