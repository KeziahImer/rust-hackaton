pub struct Animal {
    race: String,
    name: String,
    age: i32,
}
  
fn main() {
    let cat = Animal {
        name: "Fluppy".to_string(),
        race: "Cat".to_string(),
        age: 2,
    };
  
    println!("This animal is a {} and his name is {}", cat.get_race(), cat.get_name());
}
