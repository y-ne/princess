use std::io;

fn main() {
    let _guess: u32 = "42".parse().expect("Not a number");

    let _x = 2.0;
    let _y: f32 = 3.0;

    let _sum = 5 + 10;

    let _difference = 95.5 - 4.3;

    let _product = 4 * 30;

    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3;

    let _remainder = 43 % 5;

    let _c = 'z';
    let _z: char = 'ℤ';
    let _heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;
    println!("y : {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let _a = [3; 5]; // same as let a = [3, 3, 3, 3, 3];

    let a = [1, 2, 3, 4, 5];

    let _first = a[0];
    let _second = a[1];

    let a = [1, 2, 3, 4, 5];
    println!("please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index.trim().parse().expect("index was not number");

    let element = a[index];

    println!("value of element at index {index} is : {element}");
}
