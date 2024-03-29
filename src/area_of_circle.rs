// Normal function
pub fn area(width: u32, height: u32) -> u32 {
    width * height
}

// With Tuples

// In one way, this program is better.
// Tuples let us add a bit of structure, and we’re now passing
// just one argument. But in another way, this version is
// less clear: tuples don’t name their elements,
// so we have to index into the parts of the tuple, making our calculation less obvious.
pub fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Refactoring with Structs

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

pub fn area_with_struct(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}

/// Method in Rust
/// Methods are similar to functions: we declare them with the fn keyword and a name, they can
//have parameters and a return value, and they contain some code that’s run when the method
//is called from somewhere else. Unlike functions, methods are defi ned within the context of a
//struct (or an enum or a trait object, which we cover in Chapter 6 and Chapter 17, respectively),
//and their first parameter is always self , which represents the instance of the struct the
//method is being called on.

pub struct RectangleM {
    pub width: u32,
    pub height: u32,
}

impl RectangleM {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Associated Functions

// All functions defi ned within an impl block are called associated functions because they’re
// associated with the type named after the impl . We can defi ne associated functions that don’t
// have self as their first parameter (and thus are not methods) because they don’t need an
// instance of the type to work with. We’ve already used one function like this: the String::from
// function that’s defi ned on the String type.

//  Multiple impl Blocks
//Each struct is allowed to have multiple impl blocks. 



impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
