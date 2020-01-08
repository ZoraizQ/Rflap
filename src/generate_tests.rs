use rand::Rng;
use std::collections::HashSet;
use std::convert::TryInto;

pub fn generate_tests(
    alphabet: HashSet<char>,
    min_length: u8,
    max_length: u8,
    include_empty: bool,
    size: u8,
    random: bool,
) -> Vec<String> {
    assert!(min_length <= max_length);

    let mut return_vec: Vec<String> = [].to_vec();

    if include_empty {
        return_vec.push("".to_string());
    }

    let alphabet_vec: Vec<char> = alphabet.iter().cloned().collect();

    if random {
        while return_vec.len() < size.try_into().unwrap() {
            let mut rng = rand::thread_rng();

            let string_length: u8 = rng.gen_range(min_length, max_length);

            let new_test_string: String = (0..string_length)
                .map(|_| {
                    let idx: usize = rng.gen_range(0, alphabet_vec.len()).try_into().unwrap();
                    alphabet_vec[idx] as char
                })
                .collect();

            return_vec.push(new_test_string);
        }
    } else {
        // implement trie and flattening
    }

    return_vec
}
