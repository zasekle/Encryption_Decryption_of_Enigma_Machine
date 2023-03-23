use std::io;
use std::str;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(PartialEq)]
enum EncodeDecode {
    ENCODE,
    DECODE
}

struct InputValues {
    encode_or_decode: EncodeDecode,
    base_shift_amount: u16,
    character_comparison_strings: Vec<String>,
    in_place_modified_message: Vec<u8>,
}

//This is setup oddly for a programming problem because I was practicing some clean code stuff.
fn main() {
    let in_place_modified_message = encode_or_decode_string(InputValues::new());

    println!("{}", str::from_utf8(&in_place_modified_message).expect("failed to create utf8 string"));
}

fn encode_or_decode_string(input_values: InputValues) -> Vec<u8> {
    if input_values.encode_or_decode == EncodeDecode::ENCODE {
        encode_string(input_values)
    } else {
        decode_string(input_values)
    }
}

fn encode_string(mut input_values: InputValues) -> Vec<u8> {
    run_caesar_shift_for_encoding(&mut input_values);

    use_character_comparisons_with_rotors(input_values)
}

fn run_caesar_shift_for_encoding(input_values: &mut InputValues) {
    let mut idx: u16 = 0;
    for c in &mut input_values.in_place_modified_message {
        let total_shift_amount = input_values.base_shift_amount + idx;
        *c = calculate_character_value_for_encoding_caesar_shift(c, &total_shift_amount);
        idx += 1;
    }
}

fn calculate_character_value_for_encoding_caesar_shift(prev_char_in_bytes: &u8, total_shift_amount: &u16) -> u8 {
    let a_byte_value = 'A' as u16;
    let current_alphabet_index = *prev_char_in_bytes as u16 - a_byte_value;
    let new_alphabet_index = (current_alphabet_index + total_shift_amount) % 26;
    let new_character = new_alphabet_index + a_byte_value;

    new_character as u8
}

fn use_character_comparisons_with_rotors(mut input_values: InputValues) -> Vec<u8> {
    for character_comparison in input_values.character_comparison_strings {
        let character_comparison_as_bytes = character_comparison.as_bytes();
        for c in &mut input_values.in_place_modified_message {
            *c = calculate_character_value_for_rotor_comparison_encoding(c, character_comparison_as_bytes);
        }
    }

    input_values.in_place_modified_message
}

fn calculate_character_value_for_rotor_comparison_encoding(previous_char_in_bytes: &u8, character_comparison_as_bytes: &[u8]) -> u8 {
    let current_alphabet_index = *previous_char_in_bytes - ('A' as u8);
    character_comparison_as_bytes[current_alphabet_index as usize]
}

fn decode_string(mut input_values: InputValues) -> Vec<u8> {
    input_values.character_comparison_strings.reverse();

    reverse_caesar_shift_for_decoding(&mut input_values);

    reverse_character_comparisons_with_rotors(input_values)
}

fn reverse_caesar_shift_for_decoding(input_values: &mut InputValues) {
    for character_comparison in &input_values.character_comparison_strings {
        let character_comparison_as_bytes = character_comparison.as_bytes();
        for c in &mut input_values.in_place_modified_message {
            *c = calculate_character_value_for_reversing_caesar_shift(c, character_comparison_as_bytes);
        }
    }
}

fn calculate_character_value_for_reversing_caesar_shift(previous_char_in_bytes: &u8, character_comparison_as_bytes: &[u8]) -> u8 {
    let position_of_char = character_comparison_as_bytes
        .iter()
        .position(|&r| r == *previous_char_in_bytes)
        .unwrap();
    'A' as u8 + position_of_char as u8
}

fn reverse_character_comparisons_with_rotors(mut input_values: InputValues) -> Vec<u8> {
    let mut idx: u16 = 0;
    for c in &mut input_values.in_place_modified_message {
        let total_shift_amount = (input_values.base_shift_amount + idx) as u8;
        *c = calculate_character_value_for_rotor_comparison_decoding(&total_shift_amount, c);
        idx += 1;
    }

    input_values.in_place_modified_message
}

fn calculate_character_value_for_rotor_comparison_decoding(total_shift_amount: &u8, previous_char_in_bytes: &u8) -> u8 {
    let a_byte_value = 'A' as u8;
    let amount_to_be_subtracted = (*total_shift_amount % 26) as u8;
    let current_alphabet_index = *previous_char_in_bytes - a_byte_value;
    let addition_to_keep_value_positive = 26 + current_alphabet_index - amount_to_be_subtracted;
    a_byte_value + (addition_to_keep_value_positive % 26)
}

impl InputValues {

    pub fn new() -> Self {
        InputValues {
            encode_or_decode: Self::extract_encode_or_decode_from_stdin(),
            base_shift_amount: Self::extract_base_shift_amount_from_stdin(),
            character_comparison_strings: Self::extract_character_comparison_strings_from_stdin(),
            in_place_modified_message: Self::extract_in_place_modified_message_from_stdin(),
        }
    }

    fn extract_encode_or_decode_from_stdin() -> EncodeDecode {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let encode_decode_raw_input = input_line.trim_matches('\n').to_string();
        if encode_decode_raw_input == "ENCODE" {
            EncodeDecode::ENCODE
        } else {
            EncodeDecode::DECODE
        }
    }

    fn extract_base_shift_amount_from_stdin() -> u16 {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        parse_input!(input_line, u16)
    }

    fn extract_character_comparison_strings_from_stdin() -> Vec<String> {
        let mut character_comparison_strings = Vec::<String>::new();
        for _ in 0..3 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            character_comparison_strings.push(input_line.trim_matches('\n').to_string());
        }
        character_comparison_strings
    }

    fn extract_in_place_modified_message_from_stdin() -> Vec<u8> {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let messagein_place_modified_message = input_line.trim_matches('\n').as_bytes().to_vec();
        messagein_place_modified_message
    }

}
