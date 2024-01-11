

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
pub fn area_with_tuple(dimensions:(u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}



// Refactoring with Structs 


pub struct Rectangle {
   pub width: u32,
  pub  height: u32,
}


pub fn area_with_struct(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}

