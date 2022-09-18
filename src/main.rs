mod challenges;
mod testing;

use std::io;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    println!("================This is Rust================");

    let function_map = build_function_map();
    loop {
        println!("--------Input test identifier:");
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Err(_e) => println!("Error: {}", _e),
            Ok(_o) => ()
        }
        let test_id = user_input.trim();

        if test_id == "q" || test_id == "Q" {
            break;
        }

        match function_map.get(test_id) {
            Some(f) => f(),
            None => {
                println!("Invalid test identifier: {}", test_id);
            }
        }
    }

    Ok(())
}

fn build_function_map() -> HashMap<&'static str, Box<dyn Fn()>> {
    let mut function_map : HashMap<&str, Box<dyn Fn()>> = HashMap::new();

    function_map.insert("c1", Box::new(|| challenges::challenge_1()));
    function_map.insert("c2", Box::new(|| challenges::challenge_2()));
    function_map.insert("c3", Box::new(|| challenges::challenge_3()));
    function_map.insert("c4", Box::new(|| challenges::challenge_4()));
    function_map.insert("c5", Box::new(|| challenges::challenge_5()));

    function_map.insert("prim", Box::new(|| testing::primitives_testing()));
    function_map.insert("arr", Box::new(|| testing::arrays_testing()));
    function_map.insert("tup", Box::new(|| testing::tuples_testing()));
    function_map.insert("str", Box::new(|| testing::strings_testing()));
    function_map.insert("dt", Box::new(|| testing::datetime_testing()));
    function_map.insert("cf", Box::new(|| testing::control_flow_testing()));
    function_map.insert("io", Box::new(|| testing::stdin_testing()));

    function_map
}
