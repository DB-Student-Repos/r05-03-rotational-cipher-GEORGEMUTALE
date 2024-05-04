pub fn rotate(input: &str, key: i8) -> String {
    //  size of the alphabet
    let alphabet_size = 26;

    // Convert the key to a positive value within the range of the alphabet size
    let key = (key % alphabet_size as i8 + alphabet_size as i8) % alphabet_size as i8;

    // Creating Empty string 
    let mut ciphertext = String::new();

    // Moving over every character
    for c in input.chars() {
        // Is it capital
        if c.is_ascii_alphabetic() && c.is_ascii_uppercase() {
            // Changing character to ASCII value,
            //putting it back
            let rotated_ascii = (((c as u8 - b'A') as i8 + key) % alphabet_size as i8 + b'A' as i8) as u8;
            // Convert the rotated ASCII value back to a character and append it to the ciphertext
            ciphertext.push(rotated_ascii as char);
        } else if c.is_ascii_alphabetic() && c.is_ascii_lowercase() {
            // Then for lowercase characters
            let rotated_ascii = (((c as u8 - b'a') as i8 + key) % alphabet_size as i8 + b'a' as i8) as u8;
            ciphertext.push(rotated_ascii as char);
        } else {
            // save if not an alphabetic
            ciphertext.push(c);
        }
    }

    // Return the ciphertext
    ciphertext
}

