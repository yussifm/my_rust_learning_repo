pub fn object_orient_program_rs() {}

// Public
pub struct AverageOfNumber {
    pub list_Of_number: Vec<i32>,
    pub average: f64,
}

impl AverageOfNumber {
    pub fn add_numbers(&mut self, value: i32) {
        self.list_Of_number.push(value);
        self.calculate_average();
    }

    pub fn get_average(&self) -> f64 {
        self.average
    }

    pub fn calculate_average(&mut self) {
        let total = self.list_Of_number.iter().sum();
        self.average = total as f64 / self.list_Of_number.len() as f64;
    }
}
