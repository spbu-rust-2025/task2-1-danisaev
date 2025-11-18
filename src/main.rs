use std::io;

fn main() {
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Cannot read the input");
    println!("{}", pallindrome_search(&mut input_string));
}

fn pallindrome_search(input: &str) -> String {
    let start_string: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    let mut special_string = String::from('|');
    for symbol in start_string.chars() {
        special_string.push(symbol);
        special_string.push('|');
    }
    let special_string_symbols = special_string.as_bytes();
    let length_of_special_string = special_string_symbols.len();
    let mut array_pallindrome_length = vec![0; length_of_special_string];
    let mut center = 0;
    let mut right_border = 0;
    let mut length_of_the_longest_pallindrome = 0;
    let mut center_of_the_longest_pallindrome = 0;
    for i in 0..length_of_special_string {
        let mirror_element_index = if i < right_border { 2 * center - i } else { 0 };
        if i < right_border {
            array_pallindrome_length[i] =
                (right_border - i).min(array_pallindrome_length[mirror_element_index]);
        }

        while (i + array_pallindrome_length[i] + 1 < length_of_special_string)
            && (i >= array_pallindrome_length[i] + 1)
            && (special_string_symbols[i + array_pallindrome_length[i] + 1]
                == special_string_symbols[i - array_pallindrome_length[i] - 1])
        {
            array_pallindrome_length[i] += 1;
        }

        if i + array_pallindrome_length[i] > right_border {
            center = i;
            right_border = i + array_pallindrome_length[i];
        }

        if array_pallindrome_length[i] > length_of_the_longest_pallindrome {
            length_of_the_longest_pallindrome = array_pallindrome_length[i];
            center_of_the_longest_pallindrome = i;
        }
    }

    let start_index_in_special =
        center_of_the_longest_pallindrome - length_of_the_longest_pallindrome;
    let end_index_in_special =
        center_of_the_longest_pallindrome + length_of_the_longest_pallindrome;

    let mut pre_final_pallindrome = String::new();
    for i in start_index_in_special..=end_index_in_special {
        pre_final_pallindrome.push(special_string_symbols[i] as char);
    }
    let final_pallindrome: String = pre_final_pallindrome.split_terminator('|').collect();
    final_pallindrome
}
