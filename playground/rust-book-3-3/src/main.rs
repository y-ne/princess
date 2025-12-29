fn main() {
    another_function(5);

    print_labeled_measurement(5, 'h');

    let _y = 6;

    let y = {
        let x = 3;
        x + 1
    };

    println!("y : {y}");

    let x = five();
    println!("x : {x}");

    let x = plus_one(5);
    println!("x : {x}");
}

fn another_function(x: i32) {
    println!("value of x : {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("measurement : {value} {unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // same as return x + 1;
}
