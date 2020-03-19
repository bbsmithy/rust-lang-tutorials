

//return the first word in a string

pub fn first_word(word: &String) -> usize {
    let word_bytes = word.as_bytes();

    for(i, &character) in word_bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }

    word_bytes.len()
}


let result: usize = first_word("Hello there")
println!("Position of first space {}", result)
