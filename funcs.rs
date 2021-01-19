use rand::Rng;
use std::char;

pub fn get_session_key() -> String {
    //generate 10 char random string
    let mut result = String::new();
    let mut rng = rand::thread_rng();

    for _i in 0..10 {
        let ch = char::from_digit(rng.gen_range(1, 10), 10).unwrap();
        result.push(ch);
    }

    return result;
}

pub fn get_hash_str() -> String {
    //calculate initial hash string
    let mut li = String::new();
    let mut rng = rand::thread_rng();

    for _i in 0..5 {
        let ch = char::from_digit(rng.gen_range(1, 7), 10).unwrap();
        li.push(ch);
    }

    return li;
}

pub fn next_session_key(hash_str: &str, session_key: &str) -> String {
    //generate next session key
    if hash_str.is_empty() {
        return "Hash code is empty".to_string()
    }

    for idx in hash_str.chars() {
        if !idx.is_ascii_digit() {
            return "Hash code contains non-digit letter".to_string()
        }
    }

    let mut result = 0;

    for idx in hash_str.chars() {
        let l = idx.to_string();
        result += calc_hash(session_key.to_string(), l.parse::<u64>().unwrap()).parse::<u64>().unwrap();
    }

    return result.to_string();
}

pub fn calc_hash(session_key: String, val: u64) -> String {
    if val == 1 { 
        let result = "00".to_string() + &(session_key[0..5].parse::<u64>().unwrap() % 97).to_string();
        return result[result.len() - 2..result.len()].to_string()
    } 
    else if val == 2 {
        let result = session_key.chars().rev().collect::<String>();
        return result + &session_key.chars().nth(0).unwrap().to_string()
    } 
    else if val == 3 {
        return session_key[session_key.len() - 5..session_key.len()].to_string() + &session_key[0..5].to_string()
    } 
    else if val == 4 {
        let mut result = 0;
        for _i in 1..9 {
            result += session_key.chars().nth(_i).unwrap().to_digit(10).unwrap() as u64 + 41;
        }
        return result.to_string()
    } 
    else if val == 5 {
        let mut ch: char;
        let mut result = 0;

        for _i in 0..session_key.len() {
            ch = ((session_key.chars().nth(_i).unwrap() as u8) ^ 43) as char;
            if !ch.is_ascii_digit() {
                ch = (ch as u8) as char;
            }
            result += ch as u64;
        }
        return result.to_string()
    }
    else {
        return (session_key.parse::<u64>().unwrap() + val).to_string()
    }
}