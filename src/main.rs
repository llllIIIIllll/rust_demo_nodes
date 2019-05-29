
struct Cli {
    s:String,
    num:u32,
}

fn test() {
    println!("another print");
}

fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("number of x: {}", x);
    x = 6;
    println!("number of x: {}", x);
    let mut _x = Cli {s: "str".to_string(), num: 50};
    println!("number of _x: {} and {}", _x.s, _x.num);
    
    test();
}
