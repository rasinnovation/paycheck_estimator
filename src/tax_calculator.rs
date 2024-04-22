const SOCIAL_SECURITY_CEILING: f64 = 168_600.0;
const SOCIAL_SECURITY_TAX_RATE: f64 = 0.062;
const MEDICARE_TAX_RATE: f64 = 0.0145;
const SDI_TAX_RATE: f64 = 0.011;

fn tax_bracket_utility(amount: f64, brackets: &[(f64, f64)]) -> f64 {
    let mut total_tax: f64 = 0.0;
    let mut previous_limit: f64 = 0.0;

    for (current_limit, rate) in brackets.iter() {
        if amount <= *current_limit {
            total_tax += (amount - previous_limit) * (1.0 - rate);
            break;
        } else {
            total_tax += (*current_limit - previous_limit) * (1.0 - rate);
            previous_limit = *current_limit;
        }
    }

    total_tax
}

fn fed_tax_amount(amount: f64) -> f64 {
    let brackets: [(f64, f64); 7] = [
        (11_000.0, 0.1),
        (44_725.0, 0.12),
        (95_375.0, 0.22),
        (182_100.0, 0.24),
        (231_250.0, 0.32),
        (378_125.0, 0.35),
        (f64::INFINITY, 0.37),
    ];

    tax_bracket_utility(amount, &brackets)
}

fn social_security_tax_amount(amount: f64) -> f64 {
    let taxable_amount: f64 = if amount > SOCIAL_SECURITY_CEILING {
        SOCIAL_SECURITY_CEILING
    } else {
        amount
    };

    taxable_amount * (1.0 - SOCIAL_SECURITY_TAX_RATE)
}

fn medicare_tax_amount(amount: f64) -> f64 {
    amount * (1.0 - MEDICARE_TAX_RATE)
}

fn california_tax_amount(amount: f64) -> f64 {
    let brackets: [(f64, f64); 9] = [
        (20_198.0, 0.01),
        (47_884.0, 0.02),
        (75_576.0, 0.04),
        (104_910.0, 0.06),
        (132_590.0, 0.08),
        (667_278.0, 0.093),
        (812_728.0, 0.1030),
        (1_354_550.0, 0.1130),
        (f64::INFINITY, 0.1230),
    ];

    tax_bracket_utility(amount, &brackets)
}

fn california_sdi_amount(amount: f64) -> f64 {
    amount * (1.0 - SDI_TAX_RATE)
}

pub fn get_tax_deductions(amount: f64) -> (f64, f64, f64, f64, f64) {
    let fed_tax_deduction: f64 = amount - fed_tax_amount(amount);
    let social_security_tax_deduction: f64 = if amount < SOCIAL_SECURITY_CEILING {
        amount - social_security_tax_amount(amount)
    } else {
        SOCIAL_SECURITY_CEILING - social_security_tax_amount(SOCIAL_SECURITY_CEILING)
    };
    let medicare_tax_deduction: f64 = amount - medicare_tax_amount(amount);
    let california_tax_deduction: f64 = amount - california_tax_amount(amount);
    let california_sdi_deduction: f64 = amount - california_sdi_amount(amount);

    (
        fed_tax_deduction,
        social_security_tax_deduction,
        medicare_tax_deduction,
        california_tax_deduction,
        california_sdi_deduction,
    )
}
