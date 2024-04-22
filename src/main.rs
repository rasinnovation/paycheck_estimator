use crate::input_display::{display_results, get_amount_input};

mod input_display;
mod tax_calculator;

fn main() {
    let amount: f64 = match get_amount_input() {
        Ok(num) => num,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    
    display_results(amount);
}
