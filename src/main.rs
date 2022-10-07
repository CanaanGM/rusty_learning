struct Human{
    name: String,
    age: i8,
    current_thought: String
}

impl Human{
    fn new (input_name: &str, input_age: i8) -> Human{
        return Human{
            name: input_name.to_string(),
            age: input_age,
            current_thought: String::from("nothing")
        }
    }

    fn with_thought(mut self, thought: &str) -> Human{
        self.current_thought = thought.to_string();
        return self
    }

    fn speak(&self) -> () {
        println!("Greetings! my name is {} and I'm {} yrs old.", &self.name, &self.age);
    }
}


use std::collections::{HashMap};
enum AllowedData{
    S(String),
    I(i8)
}

struct CustomMap{
    body : HashMap<String, AllowedData>
}

impl CustomMap {

    fn new() -> CustomMap{ // constructor
        return CustomMap { body: HashMap::new() }
    }

    fn get (&self, key: &str) -> &AllowedData {
        return self.body.get(key).unwrap()
    }
    
    // basically "mut" is giving the function "read/write" previlages
    fn insert (&mut self, key: &str, value: AllowedData) -> () {
        self.body.insert(key.to_string(), value);
    }

    fn display(&self, key: &str) -> (){
        match self.get(key){
            AllowedData::I(value) => println!("{}", value),
            AllowedData::S(value) => println!("{}", value),
        }
    }

}

macro_rules! capitalize {
    ($a: expr) => {
        let mut v: Vec<char> = $a.chars().collect();
        v[0]= v[0].to_uppercase().nth(0).unwrap();

        $a = v.into_iter().collect();
    };
}

fn main(){
    // struct example
    let developer = Human::new("Canaan", 28);
    developer.speak();
    println!("currently i'm thinking {}", developer.current_thought);
    let new_dev = Human::new("Dante", 30).with_thought("i want pizza");
    new_dev.speak();
    println!("currently i'm thinking {}", new_dev.current_thought);
    // struct example end

    // hashmap w/ struct
    let mut map = CustomMap::new();

    map.insert("test", AllowedData::I(8));
    map.insert("testing", AllowedData::S("test value".to_string()));

    map.display("test");
    map.display("testing");


    let mut canaan = String::from("canaan");
    capitalize!(canaan);
    println!("{}", canaan);
}
