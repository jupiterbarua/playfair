use std::collections::HashSet;


/// Position type. Used to store an X, Y value for use in a matrix.
pub type Position = (usize, usize);

#[derive(Debug, PartialEq)]
struct Playfair {
    key_matrix: [[char;5];5]
}

//remove duplicates string
fn remove_duplicates(mut s: String) -> String {
    let mtx_str = "abcdefghiklmnopqrstuvwxyz";
        // Ensure we only take the alphabetic parts of the input string and
        // remove any instance of 'j'.
        let mut parsed: String = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic() && *c != 'j')
            .collect();
        parsed.push_str(mtx_str);

    let mut no_dup = HashSet::new();
    parsed.retain(|c| no_dup.insert(c));
    parsed
}
//make alphabetic, replace double with x & odd ad x
fn format_input(input: &str) -> Vec<(char,char)> {
    let mut buffer = vec![];

    // Ensure the input is only alphabetic
    let mut input: String = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    // Loop over the characters of the input 2 at a time, checking that there is a next one.
    // If there are duplicates insert an 'x' to seperate the duplicates.
    for idx in (0..input.len()).step_by(2) {
        let a = input.chars().nth(idx).unwrap();

        if let Some(b) = input.chars().nth(idx + 1) {
            if a == b {
                input.insert(idx + 1, 'x');
            }
        }
    }

    // If we are still at an odd length, append a 0 at the end of the input.
    if input.len() % 2 != 0 {
        input.push('x');
    }

    // Again loop over the pairs, this time we are guarenteed that it is an even length so we
    // don't need the `if let Some(_)` check.
    for idx in (0..input.len()).step_by(2) {
        let a = input.chars().nth(idx).unwrap();
        let b = input.chars().nth(idx + 1).unwrap();

        buffer.push((a, b));
    }

    // Return the buffer
    buffer
}
//create tuples
fn create_tuples(s: &str)-> Vec<(char,char)>{
    let chr = s.chars().collect::<Vec<char>>();
    let mut vec = Vec::new();
    for i in (0..chr.len()).step_by(2){
        if i + 1 < chr.len(){
            vec.push(((chr[i],chr[i+1])));
        }
    }
    vec
}

impl Playfair {
    fn new(key: &str) -> Self {
        let mut playfair = Playfair { key_matrix: [['\0'; 5]; 5] };
        
        playfair.create_key_matrix(key);
        playfair
        
    }
    fn create_key_matrix(&mut self, key: &str) {
        
        //remove duplicates
        let parsed = remove_duplicates(key.to_string());
        println!("{}", parsed);
        // code to create key matrix
        for(idx, chr) in parsed.char_indices(){
            let x = idx%5;
            let y = idx/5 ;
            self.key_matrix[x][y] = chr;
        }
        
    }

    fn encrypt(&self, plaintext: &str) -> String {
        let mut buffer = String::new();
        let bigrams = format_input(plaintext);

        // Loop over each bigram
        for bigram in bigrams {
            // Get the positions of the characters, needed in performing the operations on swapping
            // or incrementing x & y values.
            let a_pos: Position = self.get_position_in_matrix(&bigram.0);
            let b_pos: Position = self.get_position_in_matrix(&bigram.1);

            if a_pos.0 == b_pos.0 {
                // Case 1: They are in the same column. In this case, we increment (with wrapping)
                // their y-values by 1.
                buffer.push(self.key_matrix[a_pos.0][(a_pos.1 + 1) % 5]);
                buffer.push(self.key_matrix[b_pos.0][(b_pos.1 + 1) % 5]);
            } else if a_pos.1 == b_pos.1 {
                // Case 2: They are in the same row. In this case, we increment (with wrapping)
                // their x-values by 1.
                buffer.push(self.key_matrix[(a_pos.0 + 1) % 5][a_pos.1]);
                buffer.push(self.key_matrix[(b_pos.0 + 1) % 5][b_pos.1]);
            } else {
                // Case 3: They are in different rows and columns, In this case, we swap the
                // x-values of each position and keep the same y-values.
                buffer.push(self.key_matrix[b_pos.0][a_pos.1]);
                buffer.push(self.key_matrix[a_pos.0][b_pos.1]);
            }
        }

        buffer

    }

    /// Get the position of a given character withing the matrix. Returns a [Position] type, which is an
    /// (x, y) pair of where the character is in the function. Since i = j in this implementation,
    /// whenever the letter 'j' is searched for, just search for 'i' instead.
    fn get_position_in_matrix(&self, to_search: &char) -> Position {
        // Loop over each column and item.
        for (idx, column) in self.key_matrix.iter().enumerate() {
            // Check if what we are searching for is in the Matrix. This seems to be marginally
            // faster than just another for loop and comparison.
            if let Some(jdx) = column.iter().position(|&chr| chr == *to_search) {
                // Return the position we found.
                return (idx, jdx);
            }
        }

        // If no position was found, we were probably searching for a 'j', which in our current
        // implementation, i = j, so  return the result for searching for 'i'.
        self.get_position_in_matrix(&'i')
    }

    fn decrypt(&self, ciphertext: &str) -> String {
        let mut buffer = String::new();
        let bigrams  = format_input(ciphertext);

        // Loop over the bigrams
        for bigram in bigrams {
            // Get the positions of the characters, needed in performing the operations on swapping
            // or decrementing x & y values.
            let a_pos: Position = self.get_position_in_matrix(&bigram.0);
            let b_pos: Position = self.get_position_in_matrix(&bigram.1);

            if a_pos.0 == b_pos.0 {
                // Case 1: They are in the same column. In this case, we increment (with wrapping)
                // their y-values by 1.

                // Subtract 1, producing an optional with the value from the operation. If we try
                // to subtract 1 from 0, .checked_sub() would result in a None being returned, in
                // which case .unwrap_or() will give us a 4, effectively giving us this 'reverse'
                // modular arithmetic
                let a_y = a_pos.1.checked_sub(1).unwrap_or(4);
                let b_y = b_pos.1.checked_sub(1).unwrap_or(4);

                buffer.push(self.key_matrix[a_pos.0][a_y]);
                buffer.push(self.key_matrix[b_pos.0][b_y]);
            } else if a_pos.1 == b_pos.1 {
                // Case 2: They are in the same row. In this case, we increment (with wrapping)
                // their x-values by 1.

                // Subtract 1, producing an optional with the value from the operation. If we try
                // to subtract 1 from 0, .checked_sub() would result in a None being returned, in
                // which case .unwrap_or() will give us a 4, effectively giving us this 'reverse'
                // modular arithmetic
                let a_x = a_pos.0.checked_sub(1).unwrap_or(4);
                let b_x = b_pos.0.checked_sub(1).unwrap_or(4);

                buffer.push(self.key_matrix[a_x][a_pos.1]);
                buffer.push(self.key_matrix[b_x][b_pos.1]);
            } else {
                // Case 3: They are in different rows and columns, In this case, we swap the
                // x-values of each position and keep the same y-values.
                buffer.push(self.key_matrix[b_pos.0][a_pos.1]);
                buffer.push(self.key_matrix[a_pos.0][b_pos.1]);
            }
        }

        buffer
   // code to decrypt ciphertext
    }

}

fn main() {
    let playfair =  Playfair::new("Jupiter");

    // Encrypt a given string reference
    let res = playfair.encrypt("Jupiter Barua");
    println!("{res}");
    // Decrypt a given string reference
    let dec = playfair.decrypt(&res);
    println!("{dec}"); 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_no_dups() {
        
        let test_key = remove_duplicates("abcdefg".to_string());

        assert_eq!(test_key.len(), 25);
        assert_eq!(test_key, "abcdefghiklmnopqrstuvwxyz");
    }

    #[test]
    fn test_keyword_with_dups() {
        let test_key = remove_duplicates("aabbccddee".to_string());

        assert_eq!(test_key.len(), 25);
        assert_eq!(test_key, "abcdefghiklmnopqrstuvwxyz");
    }

}
