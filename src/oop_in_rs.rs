pub fn object_orient_program_rs() {
    let mut average_struc: AverageOfNumber = AverageOfNumber {
        list_of_number: vec![0],
        average: 0.0,
    };
    average_struc.add_numbers(2);
    average_struc.add_numbers(4);
    average_struc.add_numbers(5);
    let average = average_struc.get_average();

    println!("The average number is {}", average);
}

// Public
pub struct AverageOfNumber {
    pub list_of_number: Vec<i32>,
    pub average: f64,
}

impl AverageOfNumber {
    pub fn add_numbers(&mut self, value: i32) {
        self.list_of_number.push(value);
        self.calculate_average();
    }

    pub fn get_average(&self) -> f64 {
        self.average
    }

    pub fn calculate_average(&mut self) {
        let total: i32 = self.list_of_number.iter().sum();
        self.average = total as f64 / self.list_of_number.len() as f64;
    }
}
