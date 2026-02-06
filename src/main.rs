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

    let number =7;

    if number < 5 {
        println!("True");
    }
    else {
        println!("False");
    }

    if number != 0 {
        println!("Not null")
    }

    let number =6;

    if number % 4 == 0 {
        println!("Div 4");
    } else if number % 3 == 0 {
        println!("Div 3");
    } else if number % 3 == 0 {
        println!("Div 2");
    } else {
        println!("Not div 4, 3, 2")
    }

    let comdition =true;
    let number = if comdition {
        5
    } else {
        6
    };

    println!("Значение числа равно {}", number);

    loop {
        println!("One more time");
        break;
    }

    let mut counter =0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break  counter*2;
        }
    };
    println!("Result  is {}", result);


    let mut number =3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("Go!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("Value is {}\n", a[index]);

        index =  index +1
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("Value is {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("Go!\n");
    
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