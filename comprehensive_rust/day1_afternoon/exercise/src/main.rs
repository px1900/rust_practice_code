pub fn luhn(cc_number: &str) -> bool {
    // println!("cc_number: {}", cc_number);
    let mut sum = 0;
    let mut cnt = 0;
    for val in cc_number.chars().rev().filter(|ch| !ch.is_whitespace()) {
        // println!("val: {}", val);
        // if this character is not a digit, return false
        if !val.is_digit(10) {
            return false;
        }
        cnt += 1;
        // if this count is even, double the digit
        if cnt % 2 == 0 {
            let double_value = val.to_digit(10).unwrap() * 2;
            if double_value >= 10 {
                // println!("added {}", 1 + double_value - 10);
                sum += 1 + double_value - 10;
            } else {
                // println!("added {}", double_value);
                sum += double_value;
            }
        } else { // if this count is odd, add the digit
            // println!("added {}", val.to_digit(10).unwrap());
            sum += val.to_digit(10).unwrap();
        }
    }

    // for cnt == 2: Deal with empty string or string with only whitespace
    // for cnt == 1: Deal with single digit
    if cnt <= 1 {
        return false;
    }
    // println!("sum: {}", sum);
    sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}