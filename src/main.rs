use std::io::stdin;
use std::char::from_digit;
use std::collections::HashMap;
const ALPHABET : [char; 32] = ['#', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'Ñ', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '1', '2', '3', '4'];
fn search(ch : char) -> Option<usize>{
    for i in 0..32{
        if ch == ALPHABET[i]{
            return Some(i);
        }
    }
    None
}
fn fill(word : &String, bytes : &mut Vec<u8>){
    for ch in word.chars(){
        match search(ch) {
            Some(i) => bytes.push(i as u8),
            None => (),
        }
    }
}
fn to_bits(n : u8, max_bits : u8) -> Vec<u8>{
    let mut bits : Vec<u8> = Vec::new();
    for i in 0..max_bits{
        match n & 0x1 << (max_bits - 1 - i) {
            0 => bits.push(0),
            _ => bits.push(1),
        }
    }
    bits
}
fn encrypt(text_bits : &Vec<u8>, final_bits : &Vec<u8>) -> Vec<u8>{
    let mut text : Vec<u8> = Vec::new();
    for i in 0..text_bits.len(){
        text.push(text_bits[i] ^ final_bits[i]);
    }
    text
}

fn to_ascii(encrypted : &Vec<u8>){
    let mut start = 0;
    let mut end = 5;
    let base : u8 = 2;
    for _ in 0..encrypted.len()/5{
        let word = &encrypted[start..end];
        start = end;
        end += 5;
        let mut sum = 0;
        for j in 0..5{
            sum += word[j]*(base.pow((4-j) as u32));
        }
        print!("{}", ALPHABET[sum as usize]);

    }
    println!();
}
fn main() {
    let text: String = String::from("HOLAATODOSESTAESUNAPRUEBA");
    let key : String = String::from("RANDOMKEY1234");
    let mut init_state: Vec<u8> = Vec::new();
    let mut text_bits : Vec<u8> = Vec::new();
    let iterations;
    let mut states = HashMap::new();
    {
        let mut key_bytes : Vec<u8> = Vec::new();
        let mut text_bytes : Vec<u8> = Vec::new();
        fill(&key, &mut key_bytes);
        fill(&text, &mut text_bytes);
        iterations = text.len()* 5;
        for bit in text_bytes{
            text_bits.append(&mut to_bits(bit, 5));
        }
        for bit in key_bytes{
            init_state.append(&mut to_bits(bit, 5));
        }
        let rule_30 = to_bits(30, 8);
        let aux = ["111", "110", "101", "100", "011", "010", "001",
            "000"];
        for i in 0..rule_30.len(){
            states.insert(aux[i], rule_30[i]);
        }
    }
    let cell = (init_state.len()-1)/2;
    let mut final_bits : Vec<u8> = Vec::new();
    let mut aux = init_state.clone();
    for _ in 0..iterations{
        final_bits.push(init_state[cell]);
        for i in 1..init_state.len() - 1{
            let mut next : String = String::new();
            for j in i-1..i+2{
                match from_digit(init_state[j] as u32, 10){
                    Some(ch) => next.push(ch),
                    None => (),
                }
            }
            match states.get(next.as_str()){
                Some(value) => aux[i] = *value,
                None => println!("Error"),
            }
        }
        init_state = aux.clone();
    }
    let encrypted = encrypt(&text_bits, &final_bits);
    to_ascii(&encrypted);
    to_ascii(&encrypt(&encrypted, &final_bits));
}
