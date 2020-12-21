use std::println;


#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}
fn main() {
    println!("Hello, world!");
    println!("Rust types:");
    println!("Integer:");
    println!("\ti8: {}, i16: {}, i32: {}, i64: {}, i128: {}", -11i8, 0b10011001i16, 0o100i32, -4i64, -5i128);
    println!("\tu8: {}, u16: {}, u32: {}, u64: {}, u128: {}", b'*', 2u16, 3u32, 100_001u64, 0xdeadbeefu128);
    println!("\tisize: {}, usize: {}", -10isize, 10usize);
    println!("Float:");
    println!("\tf32: {}, f64: {}", -1.2345, 1.2345);
    println!("bool:");
    println!("\t{}, {}", true, false);
    println!("char:");
    println!("\t{}, {}, {}", '$', 'a', '@');
    println!("Tuple:");
    println!("\t(char, u8, i32): {:?}", ('%', 0x7f, -1));
    println!("unit: empty tuple:");
    println!("{:?}", ());
    let p1 = Point{x: 10.1, y: 10.2};
    println!("{:?}", p1);
    let v = build_vector();
    for elem in &v {
        println!("{}", elem);
    }
}

fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10i16);
    v.push(11i16);
    v.push(12i16);
    v
}
