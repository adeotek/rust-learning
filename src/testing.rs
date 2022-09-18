use std::io;

pub fn primitives_testing() {
    println!("----------------Primitives----------------");

    let int_val: u32 = 123;
    println!("Hello, int: {int_val}");
    println!("Modulo {int_val} / 10 = {}", int_val % 10);

    let div_val: f64 = int_val as f64 / 10.000;
    println!("Div {0} / 10 = {1}", int_val, div_val);

    let num_val: f32 = 123.456;
    println!("Hello, float: {:.5}", num_val / 22.11);

    let char_val: char = 'X';
    println!("Hello, char: {char_val}");

    let special_char_val: char = '\u{261D}';
    println!("Hello, special char: {special_char_val}");
}

pub fn arrays_testing() {
    println!("----------------Arrays----------------");

    let int_array = [ 1, 2, 4, 5 ];
    println!("Hello, array element 1: {}", int_array[1]);
    println!("Array length: {}", int_array.len());

    let mut explicit_array: [char; 3] = [' '; 3];
    explicit_array[1] = 'b';
    println!("Hello, explicit array element 1: {}", explicit_array[1]);

    let multi_array: [[i32;3];2] = [ [ 11, 12, 13 ], [ 21, 22, 23 ] ];
    println!("Hello, multi array element 1/1: {}", multi_array[1][1]);
}

pub fn tuples_testing() {
    println!("----------------Tuples----------------");

    let stuff = ('A', 12, rand::random::<f64>());
    println!("Hello, tuple: {} | {} | {}", stuff.0, stuff.1, stuff.2);

    let (a, b, c) = stuff;
    println!("Hello, var from tuple: {} | {} | {}", a, b, c);
}

pub fn strings_testing() {
    println!("----------------Strings----------------");

    let str_ref_val: &str = "ref test";
    println!("Hello, string: {}", str_ref_val);

    let str_val: String = String::from("\tStruct string! ");
    println!("Hello, string: {}", str_val);
    println!("Hello, trimmed string: {}", str_val.trim());
}

pub fn datetime_testing() {
    println!("----------------Date and Time----------------");

    // let ts = time_t::from(194566211);
    // println!("Hello, time: {}", ts);
    // Date & time ???
    // let current_ts = time_t;
}

pub fn control_flow_testing() {
    println!("----------------Control Flow----------------");

    if 1 == 0 {
        println!("1 is equal to 0");
    // } else if  { }
    } else {
        println!("1 is not equal to 0");
    }

    let x = if 1 == 0 {1} else {2};
    println!("x is: {x}");

    // let var = loop {
    //
    //     break "value";
    // };

    // while 1 == 0 {
    //
    // }

    let mut count: i32 = 0;
    for element in [0;10] {
        count += 1 + element;
    }
    println!("Count is: {count}");

    let mut sum: i32 = 0;
    count = 0;
    for i in 0..15 {
        count += 1;
        sum += i;
    }
    println!("Count is: {count}");
    println!("Sum is: {sum}");
}

pub fn stdin_testing() {
    println!("----------------STDIN----------------");

    let mut input = String::new();
    println!("Input data:");
    match io::stdin().read_line(&mut input) {
        Err(_e) => println!("Error: {}", _e),
        Ok(_o) => ()
    }
    println!("The input is: {}", input.trim());

    let mut num_input = String::new();
    println!("Input numeric data:");
    match io::stdin().read_line(&mut num_input) {
        Err(_e) => println!("Error: {}", _e),
        Ok(_o) => ()
    }
    let num: f64 = num_input.trim().parse().unwrap();
    println!("The numeric is: {}", num);
}