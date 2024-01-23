//In Struct Definitions
#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}


//  In Method Definitions
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Multi gen in Struct
#[derive(Debug)]
pub struct MPoint<T, U> {
    pub x: T,
    pub y: U,
}

pub fn generics_and_traits_in_rs() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);

    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);

    println!("The largest char is {}", result);

    // In Struct Definitions
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let text = Point {
        x: String::from("Hello"),
        y: String::from("World"),
    };
    print!(
        "integer {:?}, Float {:?} ,String or Text {:?}",
        integer, float, text
    );

    // Multiple generics in struct
    let integer_m = MPoint { x: 5.0, y: 10 };
    let float_m = MPoint {
        x: String::from("four"),
        y: 4.0,
    };
    let text_m = Point {
        x: String::from("Hello"),
        y: String::from("World"),
    };
    print!(
        "integer {:?}, Float {:?} ,String or Text {:?}",
        integer_m, float_m, text_m
    );
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
