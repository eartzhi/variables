// use std::net::Ipv4Addr;

fn main() {
    // let mut x = 5;
    // println!("Значение x равно {}", x);
    // x = 6;
    // println!("Значение x равно {}", x);
    // const MAX_POINTS: u32 = 100_000;
    // println!("Значение MAX_POINTS = {}", MAX_POINTS);
    // let y = 5;
    // let y = y + 1;
    // let y = y*2;
    // println!("Значение y равно {}", y);

    // let spaces = " ";
    // let spaces = spaces.len();

    // let x = 2.0 //64
    // let y: f32 = 3.0 //32

    // //сложение 
    // let sum = 5 + 10;

    // // вычитание 
    // let diffrrence = 95.5 - 4.3;

    // // умножение
    // let product = 56.7 / 32.2;

    // // остаток
    // let remain = 43 % 5;

    // let t = true;

    // let f: bool = false;

    // let x = 'z';

    // let z = 'ƶ';

    // let heart_eyed_cat = '😻';

    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let tup = (500, 6.4, 1);

    // let (x, y, z) = tup;

    // println!("Значение y равно {}", y);

    // let five_hundred = tup.0;

    // let six_poin_four = tup.1;

    // let one = tup.2;

    // let a = [1, 2, 3, 4 ,5];

    // let months = ["Январь", "Февраль", "Март", "Апрель", "Май", "Июнь", "Июль",
    // "Август", "Сентябрь", "Октябрь", "Ноябрь", " Декабрь"];

    // let a: [i32; 5] = [1, 2, 3, 4, 5];

    // let a = [3; 5];

    // let a = [1, 2, 3, 4, 5];

    // let first = a[0];

    // let second = a[2];

    // println!("Значение first равно {}", first);

    // let index = 10;

    // let element = a[index];

    // println!("Значение элемента равно {}", element);

    // println!("Hello, World!");

    // another_function(5);
    // another_function_2(5, 6);

    // expression();

    // let x = five();

    // let x= plus_one(5);

    // println!("Значение x равно {}", x)    

    // let number =7;

    // if number < 5 {
    //     println!("True");
    // }
    // else {
    //     println!("False");
    // }

    // if number != 0 {
    //     println!("Not null")
    // }

    // let number =6;

    // if number % 4 == 0 {
    //     println!("Div 4");
    // } else if number % 3 == 0 {
    //     println!("Div 3");
    // } else if number % 3 == 0 {
    //     println!("Div 2");
    // } else {
    //     println!("Not div 4, 3, 2")
    // }

    // let comdition =true;
    // let number = if comdition {
    //     5
    // } else {
    //     6
    // };

    // println!("Значение числа равно {}", number);

    // loop {
    //     println!("One more time");
    //     break;
    // }

    // let mut counter =0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break  counter*2;
    //     }
    // };
    // println!("Result  is {}", result);


    // let mut number =3;

    // while number != 0 {
    //     println!("{}!", number);

    //     number = number - 1;
    // }
    // println!("Go!");

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("Value is {}\n", a[index]);

    //     index =  index +1
    // }

    // let a = [10, 20, 30, 40, 50];

    // for element in a.iter() {
    //     println!("Value is {}", element);
    // }

    // for number in (1..4).rev() {
    //     println!("{}!", number);
    // }

    // println!("Go!\n");

    // let s = "hello";

    // let mut s = String::from("hello");
    
    // s.push_str(", world!"); // push_str() добавляет литерал к экземпляру типа String

    // println!("{}",s);

    // let x =5;
    // let y = x;

    // println!("x = {}, y = {}", x, y);

    // // let s1 = String::from("hello");
    // // let s2 = s1;

    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {}, s2 = {}", s1, s2);

    // let s = String::from("hello"); // s входит в область видимости
    // takes_ownership(s); // значение s перемещается в функцию...
    // // ... и поэтому больше здесь не действует
    // let x = 5; // x входит в область видимости
    // makes_copy(x); // x переместится в функцию, но
    // // i32 копируема, поэтому нормально,
    // // если x будет использоваться после этого
    // // println!("{}",s);

    // let s2 = String::from("hello"); // s2 входит в область видимости
    // let s3 = takes_and_gives_back(s2); // s2 перемещается в
    // takes_and_gives_back, которая также
    // перемещает свое возвращаемое
    // значение в s3
//     // println!("{}",s3);

//     let s = "Hello, world!";

//     let my_string = String::from("hello world!");

//     // first_word работает на срезах экземпляров типа `String`
//     let world = first_word(&my_string[..]);

//     let my_string_literal = "hello world";

//     // first_word работает на срезах строковых литералов
//     let world = first_word(&my_string_literal[..]);

//     // Так как строковые литералы уже *являются* строковыми срезами,
//     // это также работает без срезового синтаксиса!

//     let word = first_word(my_string_literal);

//     let a = [1, 2, 3, 4, 5];

//     let slice = &a[1..3];

//     println!("{}", slice[1]);
    
//     struct User {
//         username: String,
//         email: String,
//         sign_in_count: u64,
//         active:bool,
//     }


//     let mut user1 =User {
//         email: String::from("someone@examplr.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     user1.email = String::from("anotheremail@example.com");


// fn build_user (email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true, 
//         sign_in_count: 1,
//     }
// }
 
//     let user2 = User {
//         email: String::from("another@example.com"),
//         username: String::from("anotherusername567"),
//         active: user1.active,
//         sign_in_count: user1.sign_in_count,
//     };

//     let user2 = User {
//         email: String::from("another@example.com"),
//         username: String::from("anotherusername"),
//         ..user1
//     };

//     struct Color(i32, i32, i32);
//     struct Point(i32, i32, i32);

//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);

//     println!("{}", black.1)


    // let width1 = 30;
    // let heigh1 = 50;
    // let rect1 = (30, 50);

    // println!("Площадь прямоугольника {} квадратным пикселям", area(rect1));

    // #[derive(Debug)]
    // struct Rectangle {
    //     width: u32,
    //     height: u32,
    // }

    // impl Rectangle {
    //     fn area (&self) -> u32 {
    //         self.width * self.height
    //     }

    //     fn can_hold (&self, rect2: &Rectangle) -> bool {
    //         (self.height > rect2.height) && (self.width > rect2.width)

    //     }

    //     fn squere (size: u32) -> Rectangle {
    //         Rectangle { width: size, height: size }
    //     }
        
    // }


    // let rect1 = &Rectangle { width: 30, height: 50 };
    // let rect2 = &Rectangle { width: 10, height: 40 };
    // let rect3 = &Rectangle { width: 60, height: 45 };
    // let sq = Rectangle::squere(3);

    // println!("Может ли rect1 содержать в себе rect2? {}", rect1.can_hold(&rect2));
    // println!("Может ли rect1 содержать в себе rect3? {}", rect1.can_hold(&rect3));
    // println!("Может ли rect1 содержать в себе sq? {}", rect1.can_hold(&sq));





// fn area (rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// // }
//     println!("rect1 равен {:?}", rect1);

//     println!("rect1 равен {:#?}", rect1);

//     println!("Площадь прямоугольника {} квадратным пикселям", rect1.area());

    // enum IpAddkind {
    //     V4, 
    //     V6,
        
    // }

    // struct IpAddr {
    //     kind: IpAddkind,
    //     adress: String,
    // }

    // struct Ipv4Addr {

    // }

    // struct Ipv6Addr {

    // }

    // enum IpAddr {
    //     V4(Ipv4Addr),
    //     V6(Ipv6Addr),
    // }

    // enum Message {
    //     Quit, 
    //     Move {x: i32, y: i32},
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }

    // impl Message {
    //     fn call (&self) {
    //         //etghrthg
    //     }
        
    // }

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // let m = Message::Write(String::from("hello"));
    // m.call();

    // let some_number = Some(5);
    // let some_string = Some("literal");

    // let absent_number: Option<i32> = None;

    // let four = IpAddkind::V4;
    // let six = IpAddkind::V6;

    // let home = IpAddr {
    //     kind: IpAddkind::V4,
    //     adress: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddkind::V6,
    //     adress: String::from("::1"),
    // };

    // fn route (ip_kind: IpAddkind) {

    // }

    // route(IpAddkind::V4);
    // route(IpAddkind::V6);

    // let hello = String::from(" ");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from(" ");
    // let hello = String::from(" ");
    // let hello = String::from(" ");
    // let hello = String::from(" ");
    // let hello = String::from(" ");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello: String = String::from("Hola");

    // enum UsState  {
    //     Alabama,
    //     Alaska,
        
    // }

    // enum Coin {
    //     Penny, 
    //     Nickel,
    //     Dime, 
    //     Quarter(UsState),
    // }



    // fn value_in_cents (coin: Coin) -> u8 {
    //     match coin {   
    //     Coin::Penny => 1, 
    //     Coin::Nickel => 5, 
    //     Coin::Dime => 10,
    //     Coin::Quarter(state) => {
    //         println!("Четвертак из штата {:?}!", state);
    //     25},
    //     }
    // }

    // value_in_cents(Coin::Quarter(UsState::Alaska));

    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match  x {
    //         None => None,
    //         Some(i) => Some(i + 1),
            
    //     }
    // }

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None); 

    // let some_u8_value = 0u8;
    // match  some_u8_value {
    //     1 => println!("one"),
    //     2 => println!("three"),
    //     5 => println!("five"),
    //     7 => println!("seven"),
    //     _ => (),
        
    // }

    // let some_u8_value = Some(0u8);
    // match  some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => (),
        
    // }

    // if let Some(3) = some_u8_value {
    //     println!("three");
    // } else {
    //     println!("not three");
    // }

    // let v: Vec<i32> = Vec::new();
    // let v =vec![1, 2, 3];
    // let mut v = Vec::new();

    // v.push(5);
    // v.push(6);
    // v.push(7);

    // let v = vec![1,2,3,4,5];

    // let third: &i32 = &v[2];
    // println!("Третий элемент равен {}", third);

    // match v.get(2) {
    //     Some(third) => println!("Третий элемент равен {}", third),
    //     None => println!("Третий элемент отсутствует."),
    // }

    // let mut v = vec![100, 32, 57];
    // for i in  &mut v {
    //     *i += 50;
    //     println!("{}", i)
    // }

    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    //     ];

    // let mut s = String::new();
    // let data = "начальное содержимое"; 
    // let s = data.to_string();

    // let s = "начальное содержимое".to_string();
    // let s = String::from("начальное содержимое");

    // let mut s = String::from("foo");

    // s.push_str("bar");

    // println!("{}", s);

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world");
    // let s3 = s1 + &s2;

    // println!("{}", s3);

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s3 = format!("{}-{}-{}", s1, s2, s3);

    // println!("{}", s3);

    // for c in "коллекции".chars() {
    //     println!("{}", c);
    // }

    // for b in "коллекции".bytes() {
    //     println!("{}", b);
    // }

    // use::std::collections::HashMap;

    // let mut scores = HashMap::new();

    // scores.insert(String::from("blue"), 10);
    // scores.insert(String::from("yellow"), 50);

    // let teams = vec![String::from("blue"), String::from("yellow")];
    // let initial_scores = vec![10, 50];

    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // println!("{:?}", scores);

    // let field_name = String::from("Любимый цвет");

    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    // // println!("{}", field_name);

    // let team_name = String::from("blue");

    // let score = scores.get(&team_name);

    // println!("{:?}", score);

    // for (key, value) in &scores {
    //     println!("{} : {}", key, value);
    // }

    // scores.insert(String::from("blue"), 25);

    // println!("{:?}", scores);

    // scores.entry(String::from("yellow")).or_insert(50);
    // scores.entry(String::from("blue")).or_insert(50);

    // println!("{:?}", scores);

    // let text = "здравствуй мир чудесный мир";

    // let mut map = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    //     println!("{}", *count);
    // }

    // println!("{:?}", map);

    let v = vec![1, 3, 5, 10, 90, 8, 80];
    let mut v_sorted = v.clone();
    v_sorted.sort();

    println!("{:?}", v_sorted);








    






    



}





// fn another_function(x: i32) {
//     println!("Значение x равно {}", x);
// }

// fn another_function_2 (x: i32, y: i32) {
//     println!("Значение x равно {}", x);
//     println!("Значение y равно {}", y);
// }

// fn expression() {

//     let x = 5;

//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("Значение y равно {}", y);
// }

// fn five() -> i32 {
//     5
// }

// fn plus_one(x: i32) ->i32 {
//     x+1
// }

// fn takes_ownership(some_string: String) { // some_string входит
//     // в область видимости
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) { // some_integer входит в область видимости
//     println!("{}", some_integer);
// } // Здесь some_integer выходит из области видимости.
// // Ничего особенного не происходит.

// fn takes_and_gives_back(a_string: String) -> String { // a_string приходит
// // в область видимости
//     a_string // a_string возвращается и выносится в вызывающую функцию
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//         return &s[0..i];
//         }
//     }
//     &s[..]
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }


