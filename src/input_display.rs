use crate::tax_calculator::get_tax_deductions;
use std::fmt::Formatter;
use std::io::{self, Write};

pub enum InputError {
    IoError(String),
    ParseError,
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::IoError(msg) => write!(f, "I/O error: {}", msg),
            InputError::ParseError => {
                write!(f, "Parse error: invalid input. Please enter a valid number")
            }
        }
    }
}

pub fn get_amount_input() -> Result<f64, InputError> {
    loop {
        print!("Enter amount before tax: ");
        io::stdout()
            .flush()
            .map_err(|e| InputError::IoError(e.to_string()))?;

        let mut input: String = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            return Err(InputError::IoError(e.to_string()));
        }

        match input.trim().parse::<f64>() {
            Ok(num) => return Ok(num),
            Err(_) => {
                println!("{}", InputError::ParseError);
                continue;
            }
        }
    }
}

pub fn display_results(amount: f64) {
    let (
        fed_tax_deduction,
        social_security_tax_deduction,
        medicare_tax_deduction,
        california_tax_deduction,
        california_sdi_deduction,
    ): (f64, f64, f64, f64, f64) = get_tax_deductions(amount);

    let total_deductions: f64 = fed_tax_deduction
        + social_security_tax_deduction
        + medicare_tax_deduction
        + california_tax_deduction
        + california_sdi_deduction;

    let monthly_income_after_tax: f64 = (amount - total_deductions) / 12.0;

    println!();
    println!(
        "Total Annual Income After Tax:                    ${:.2}",
        amount - total_deductions
    );
    println!();
    println!(
        "Annual Federal Withholding's:                     ${:.2}",
        fed_tax_deduction
    );
    println!(
        "Annual Social Security Withholding's:             ${:.2}",
        social_security_tax_deduction
    );
    println!(
        "Annual Medicare Withholding's:                    ${:.2}",
        medicare_tax_deduction
    );
    println!(
        "Annual California State Tax Withholding's:        ${:.2}",
        california_tax_deduction
    );
    println!(
        "Annual California SDI Tax Withholding's:          ${:.2}",
        california_sdi_deduction
    );
    println!();
    println!(
        "Monthly Income After Tax:                         ${:.2}",
        monthly_income_after_tax
    );
    println!();
    println!(
        "Monthly Federal Withholding's:                    ${:.2}",
        fed_tax_deduction / 12.0
    );
    println!(
        "Monthly Social Security Withholding's:            ${:.2}",
        social_security_tax_deduction / 12.0
    );
    println!(
        "Monthly Medicare Withholding's:                   ${:.2}",
        medicare_tax_deduction / 12.0
    );
    println!(
        "Monthly California State Tax Withholding's:       ${:.2}",
        california_tax_deduction / 12.0
    );
    println!(
        "Monthly California SDI Tax Withholding's:         ${:.2}",
        california_sdi_deduction / 12.0
    );
    println!();
    println!(
        "Total Federal Tax Percentage:                     ${:.4}",
        fed_tax_deduction / amount * 100.0
    );
    println!(
        "Total Social Security Tax Percentage:             ${:.4}",
        social_security_tax_deduction / amount * 100.0
    );
    println!(
        "Total Medicare Tax Percentage:                    ${:.4}",
        medicare_tax_deduction / amount * 100.0
    );
    println!(
        "Total California State Tax Percentage:            ${:.4}",
        california_tax_deduction / amount * 100.0
    );
    println!(
        "Total California SDI Tax Percentage:              ${:.4}",
        california_sdi_deduction / amount * 100.0
    )
}
