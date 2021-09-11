fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = 6;
    println!("The value of x is: {}", x);
    println!("😻");

    let tup: (i32, f64, u8 ) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("y i {}", y);

    let five_hundred = tup.0;

    println!("{}", tup.1);

    let a = [1, 2, 3, 4, 5];

    println!("{}", a[3]);

    another_function(8);
}

fn another_function(x: i32) {
    println!("x is: {}", x);
}