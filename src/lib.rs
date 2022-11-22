use std::collections::HashMap;
const Korea: [char; 13] = [
    '가', '나', '다', '라', '마', '바', '사', '아', '자', '차', '타', '파', '하',
];
const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn alphbet_caesar(string: &str, shifts: u8) -> Result<String, Box<dyn std::error::Error>> {
    let letters: HashMap<usize, &char> = ALPHABET.iter().enumerate().collect();
    let numbers: HashMap<char, usize> = ALPHABET
        .iter()
        .enumerate()
        .map(|(idx, chr)| (chr.clone(), idx))
        .collect();

    Ok(string
        .chars()
        .map(|c| {
            if c == ' ' {
                c
            } else {
                let shift = numbers[&c] + shifts as usize;
                if shift > 25 {
                    *letters[&(shift % 26)]
                } else {
                    *letters[&shift]
                }
            }
        })
        .collect::<String>())
}
pub fn korean_caesar(string: &str, shifts: u8) -> Result<String, Box<dyn std::error::Error>> {
    let letters: HashMap<usize, &char> = Korea.iter().enumerate().collect();
    let numbers: HashMap<char, usize> = Korea
        .iter()
        .enumerate()
        .map(|(idx, chr)| (chr.clone(), idx))
        .collect();

    Ok(string
        .chars()
        .map(|c| {
            if c == ' ' {
                c
            } else {
                let shift = numbers[&c] + shifts as usize;
                if shift > 12 {
                    *letters[&(shift % 13)]
                } else {
                    *letters[&shift]
                }
            }
        })
        .collect::<String>())
}
