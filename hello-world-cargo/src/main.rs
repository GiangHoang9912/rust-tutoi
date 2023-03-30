extern crate rand;
use rand::Rng;

fn learn_params() {
    /*
       here is copied from tutoi.rs
    */
    let mut x = 45;
    println!("Hello, Giang! {}", x);
    x = 60;
    println!("Hello, Giang! {}", x);
    let y: i64 = 100;
    println!("Hello, Giang! {}", y);
}

fn learn_if_else() {
    let n = 30;
    if n == 30 {
        println!("n is less than 30");
    } else if n < 40 {
        println!("n is less than 40");
    } else {
        println!("n is greater than or equal to 40");
    }
}

fn learn_loop() {
    let mut n = 0;

    // for loop
    loop {
        n += 1;

        if n > 10 {
            break;
        }

        println!("n is {}", n);
    }
}

fn learn_while() {
    let mut n = 0;

    // while loop
    while n < 10 {
        n += 1;
        println!("n is {}", n);
    }
}

fn learn_for_loop() {
    for i in 0..10 {
        println!("i is {}", i);
    }

    // loops array
    let numbers = 0..20;

    for number in numbers {
        println!("number is {}", number);
    }

    // vec
    let animals = vec!["dog", "cat", "bird", "fish"];

    for animal in animals {
        println!("animal is {}", animal);
    }
}

// enum

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn learn_direction() {
    let mut player_direction: Direction = Direction::Up;

    check_match(player_direction);

    player_direction = Direction::Down;

    check_match(player_direction);

    player_direction = Direction::Left;

    check_match(player_direction);

    player_direction = Direction::Right;

    check_match(player_direction);
}

fn check_match(player_direction: Direction) {
    match player_direction {
        Direction::Up => println!("Player is moving up"),
        Direction::Down => println!("Player is moving down"),
        Direction::Left => println!("Player is moving left"),
        Direction::Right => println!("Player is moving right"),
    }
}

fn learn_const() {
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS is {}", MAX_POINTS);
}

fn learn_tuples() {
    let tup1 = (20, 25, 30, 35, "rust", (1, 2, 3));

    println!("tup1.2 is {}", tup1.2);
    println!("tup1.4 is {}", tup1.4);
    println!("tup1.5.2 is {}", (tup1.5).2);
}

fn check_even(number: i32) -> bool {
    if number % 2 == 0 {
        return true;
    }
    false
}

fn references() {
    let mut x = 20;
    {
        let dom = &mut x;

        *dom += 1;
    }

    println!("x is {}", x);
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn learn_struct() {
    let bg = Color {
        red: 255,
        green: 255,
        blue: 255,
    };

    println!("bg is {} {} {}", bg.red, bg.green, bg.blue);
}

struct ColorNew(u8, u8, u8);

fn learn_tuples_struct() {
    let mut bg = ColorNew(255, 255, 255);

    bg.0 = 0;

    println!("bg is {} {} {}", bg.0, bg.1, bg.2);
}

fn print_color(c: &Color) {
    println!("rgb({},{},{})", c.red, c.green, c.blue);
}

fn print_color_new(c: Color) {
    println!("rgb({},{},{})", c.red, c.green, c.blue);
}

fn learn_pass_by_ref() {
    let bg = Color {
        red: 1,
        green: 2,
        blue: 3,
    };
    print_color(&bg);
    print_color_new(bg);
}

fn learn_array() {
    // 1,2,3,4,5
    let array: [i32; 5] = [3; 5];
    for number in array.iter() {
        println!("number is {}", number);
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn print_description(&self) {
        println!("Rectangle is {} x {}", self.width, self.height);
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn learn_impl_struct() {
    let my_rect = Rectangle {
        width: 30,
        height: 30,
    };

    my_rect.print_description();
    println!("Is my_rect a square? {}", my_rect.is_square());
}

fn learn_string() {
    let mut my_string = String::from("Hello, world!");

    // print len
    println!("my_string len is {}", my_string.len());

    //print is_empty
    println!("my_string is empty? {}", my_string.is_empty());

    println!("my_string is {}", my_string);

    // check whitespace count word
    for token in my_string.split_whitespace() {
        println!("token is {}", token);
    }

    // check string contains
    println!("my_string contains world? {}", my_string.contains("world"));

    // merging string
    my_string.push_str(" I am learning Rust");

    println!("my_string is {}", my_string);
}

struct Person {
    name: String,
    age: u8,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("{} is {} years old", self.name, self.age);
    }
}

fn learn_implementing_traits() {
    let person = Person {
        name: String::from("Giang"),
        age: 24,
    };

    println!("person is {}", person.to_string());
}

fn learn_vector() {
    let mut my_vector = vec![1, 2, 3, 4, 5];

    my_vector.push(6);
    my_vector.remove(1); // remove index 1

    for number in my_vector.iter() {
        println!("number is {}", number);
    }
}

// import module
use std::fs::File;
use std::io::prelude::*;

fn learn_file() {
    let mut file =
        File::open("/Users/gianghoang/Documents/tutoi/introduce.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    println!("contents is {}", contents);
}

use std::env;

fn learn_cl_args() {
    let args: Vec<String> = env::args().collect();

    println!("args is {:?}", args);
}

fn write_file() {
    let mut file = File::create("test.txt").expect("Unable to create file");

    file.write_all(b"Hello, world!")
        .expect("Unable to write data");
}

trait HasVoiceBox {
    fn speak(&self);
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person {
    fn speak(&self) {
        println!("{} says hello", self.name);
    }

    fn can_speak(&self) -> bool {
        if self.age > 3 {
            return true;
        }
        return false;
    }
}

fn learn_defined_traits() {
    let ps = Person {
        name: String::from("Giang"),
        age: 24,
    };

    println!("can speak {}", ps.can_speak());
    ps.speak()
}

use std::io;

fn learn_reading_user_input() -> String {
    println!("Please input subject: ");
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("your name is: {}", input);
        }
        Err(error) => println!("error: {}", error),
    }

    return input;
}

// learn hashmap
use std::collections::HashMap;

fn learn_hash_map(subject: &String) {
    let search = String::from(subject.replace("\n", ""));
    println!("subject: {}", search);

    let mut marks = HashMap::new();

    //add value
    marks.insert(String::from("math"), 9);
    marks.insert(String::from("music"), 8);
    marks.insert(String::from("english"), 7);
    marks.insert(String::from("physical"), 6);

    // len of marks
    println!("len of marks is {}", marks.len());

    match marks.get(&search) {
        Some(mark) => println!("mark of {} is {}", search, mark),
        None => println!("{} is not in the list", search),
    }

    // remove value
    marks.remove(&search);
    println!("len of marks is {}", marks.len());
    match marks.get(&search) {
        Some(mark) => println!("mark of {} is {}", search, mark),
        None => println!("{} is not in the list", search),
    }
}

fn rand_number() -> u32 {
    let rand_num = rand::thread_rng().gen_range(1..100);
    println!("rand num is {}", rand_num);

    return rand_num;
}

fn string_methods() {
    {
        let my_string = String::from("Hello, world!");
        print!("my_string is {}", my_string);
    }

    {
        // new line
        let my_string: String = String::from("Hello, world! \nhello, world!");

        for line in my_string.lines() {
            println!("line is {}", line);
        }
    }

    {
        let my_string = String::from("Hello, world!");
        let tokens: Vec<&str> = my_string.split(" ").collect();

        println!("tokens is {:?}", tokens);
    }

    {
        let before_trim = String::from("   Hello, world!   ");
        let after_trim = before_trim.trim();

        println!("before trim: {}", before_trim);
        println!("after trim: {}", after_trim);
    }

    {
        let my_string = String::from("Hello, world!");
        println!("{}", my_string[3..7].to_string());

        match my_string.chars().nth(4) {
            Some(c) => println!("char at index 4 is {}", c),
            None => println!("char at index 4 is not found"),
        }
    }
}

mod person_mod;

fn learn_mul_source_file() {
    // call print fn from main.rs
    person_mod::print_message();
}

mod person {
    struct Person {
        name: String,
        age: u8,
    }

    pub mod private {
        pub struct PrivatePerson {
            pub name: String,
            pub age: u8,
        }

        impl PrivatePerson {
            pub fn new(name: String, age: u8) -> PrivatePerson {
                PrivatePerson { name, age }
            }
        }

        pub fn print_message() {
            let giang = PrivatePerson::new(String::from("Giang"), 24);
            println!("private person: name: {}, age: {}", giang.name, giang.age);
        }
    }

    impl Person {
        fn new(name: String, age: u8) -> Person {
            Person { name, age }
        }

        fn to_string(&self) -> String {
            format!("name: {}, age: {}", self.name, self.age)
        }
    }

    pub fn print_message() {
        let giang = Person::new(String::from("Giang"), 24);
        println!("{}", giang.to_string());
    }
}

fn learn_module() {
    person::print_message();

    person::private::print_message();
}

fn learn_enum() {
    let name = String::from("Giang");

    let index_search = 2;

    println!(
        "char at index {} is: {}",
        index_search,
        match name.chars().nth(index_search) {
            Some(c) => c.to_string(),
            None => format!("No char at index {}", index_search),
        }
    );
}

mod http;
async fn learn_http_request() {
    print!("http request: ");
    http::http::http_server().await;
}

enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    fn is_weekend(&self) -> bool {
        match self {
            Day::Saturday | Day::Sunday => true,
            _ => false,
        }
    }
}

fn learn_enums_methods() {
    let mut today = Day::Sunday;
    print!("today is: {}", today.is_weekend());
    today = Day::Monday;
    print!("today is: {}", today.is_weekend());
    today = Day::Tuesday;
    print!("today is: {}", today.is_weekend());
    today = Day::Wednesday;
    print!("today is: {}", today.is_weekend());
    today = Day::Thursday;
    print!("today is: {}", today.is_weekend());
    today = Day::Friday;
    print!("today is: {}", today.is_weekend());
    today = Day::Saturday;
    print!("today is: {}", today.is_weekend());
    today = Day::Sunday;

    println!("is today weekend? {}", today.is_weekend());
}

use std::process::Command;

fn learn_exec_command() {
    let mut cmd = Command::new("ls");
    cmd.arg("-la");

    match cmd.output() {
        Ok(output) => {
            println!("output: {}", String::from_utf8_lossy(&output.stdout));
        }
        Err(error) => println!("error: {}", error),
    }
}

#[tokio::main]
async fn main() {
    learn_params();
    learn_if_else();
    learn_loop();
    learn_while();
    learn_for_loop();
    learn_direction();
    learn_const();
    learn_tuples();
    let check = check_even(2);

    if check {
        println!("2 is even");
    } else {
        println!("2 is odd");
    }

    references();

    learn_struct();

    learn_tuples_struct();
    learn_pass_by_ref();
    learn_array();
    learn_impl_struct();
    learn_string();
    learn_implementing_traits();
    learn_vector();
    learn_file();
    learn_cl_args();
    write_file();
    learn_defined_traits();
    learn_reading_user_input();

    let mut user_input = learn_reading_user_input();
    while user_input.trim() != "exit" {
        user_input = learn_reading_user_input();
        learn_hash_map(&user_input)
    }

    rand_number();
    string_methods();
    learn_enum();
    learn_mul_source_file();

    learn_module();
    learn_http_request().await;
    learn_enums_methods();
    learn_exec_command();
}

#[cfg(test)]
mod giang_test {
    #[test]
    #[should_panic]
    fn test() {
        assert_eq!(1, 1);

        panic!("Oh no!")
    }

    #[test]
    fn test_random() {
        assert_eq!(super::rand_number(), 1);
    }
}
