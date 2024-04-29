// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

//很显然要先定义，在使用
fn main() {
    my_macro!();
}

