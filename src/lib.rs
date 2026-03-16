// mod front_of_house;

// pub use crate::front_of_house::hosting;

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }

//     fn cook_order() {}

//     pub struct Breakfast {
//         pub toast: String,
//         sessional_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast { toast: String::from(toast),
//                  sessional_fruit: String::from("peaches"),
//                  }
//         }
        
//     }

// }

// pub use self::front_house::hosting;
// use std::fmt::Result;
// use std::io::Result as IoResult;
// use std::{io, cmp::Ordering};

// use std::collections::*;

// fn function_1() -> Result {
//     // 

// }

// fn function_2() -> IoResult<()> {
//     // 
// }

// pub fn eat_at_restaurant() {
//     //Абсолютный путь
//     crate::front_house::hosting::add_to_waitlist();

//     // Относительный путь
//     front_house::hosting::add_to_waitlist();

//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();

//     // Заказать летом завтрак с ржаным тостом
//     let mut meal = back_of_house::Breakfast::summer("rye");

//     // Изменить мнение о том, какой хлеб мы бы хотели
//     meal.toast = String::from("wheat");
//     println!("Я бы хотел {} тост, пожалуйста", meal.toast);

//     // Следующая строка не будет компилироваться, если мы раскомментируем ее;
//     // нам запрещено просматривать или изменять сезонные фрукты,
//     // которые принесут с едой
//     // meal.seasonal_fruit = String::from("черника");



// }

// fn serve_order() {}

