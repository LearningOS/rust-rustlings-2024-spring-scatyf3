// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y), 
        //如果不加入ref?
        //会发生所有权的转移，即通过模式匹配，将y对应数据的所有权转移给p
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
