use pyo3::prelude::*;

/// Remove non-printing characters from a string.
/// Valid characters are whose UTF9 codes are between U+0020 and U+007E.
#[pyfunction]
fn remove_non_printing_char(string: String) -> PyResult<String> {
    // contains the string without non-printing characters
    let mut new_string = String::new();
    let mut current_char:u32 = 0;

    // UTF8 code for space character
    const SPACE_UTF8_CODE:u32 = 32;

    // UTF8 code for tilde character
    const TILDE_UTF8_CODE:u32 = 126;

    for character in string.chars() {
        current_char = character as u32;

        // append characters that are between the range of space and tilde.
        if current_char >= SPACE_UTF8_CODE && current_char <= TILDE_UTF8_CODE  {
            new_string.push(character);
        }
    }
    Ok(new_string)
}

/// Validate al number using modulus 10.
#[pyfunction]
fn validate_modulus_10(number: String)->PyResult<bool> {

    // controls whether a digit have to be doubled
    let mut index:i32=-1;
    
    // accrues the sum of all digits of the `number`
    let mut total:i32=0;

    // indicates the 'base' to which a number will be converted
    const RADIX:u32 = 10;

    for character in number.chars().rev() {
        match character.to_digit(RADIX) {
            Some(digit) => {
                // double digit every second digit
                if index % 2 == 0{
                
                    // if doubling a digit yields two digits, subtract 9
                    if digit*2 > 9 {
                        total += digit as i32 * 2 - 9;
                    }
                    else {
                        total += digit as i32 * 2;
                    }
                }
                else {
                    total += digit as i32;
                }
            },
            // character is not a valid digit
            None => return Ok(false),
        };
        index += 1;
    }

    // if the result of modulus is 0 then the number is valid
    if total % 10 == 0{
        return Ok(true)
    }

    Ok(false)
}

/// A Python module implemented in Rust.
#[pymodule]
fn ary(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(remove_non_printing_char,m)?)?;
    m.add_function(wrap_pyfunction!(validate_modulus_10,m)?)?;
    Ok(())
}