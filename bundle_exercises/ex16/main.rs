fn is_none(option: Option<i32>) -> bool {
    match option {
        Some(_p) => return false,
        None => return true
    }
}

fn get_str(slice: &[String], guess: &str) -> Option<String> {
    for word in slice {
        if word == guess {
            return Some(word.to_string());
        }
    }
    return None;
}

fn main() {
    println!("{}", is_none(Some(1)));
    println!("{}", is_none(None));

    println!("{:?}", get_str(&["Hello".to_string(), "World".to_string()], "World"));
}
