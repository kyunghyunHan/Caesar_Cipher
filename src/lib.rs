use std::collections::HashMap;
pub fn caesar(
    string: &str,
    shifts: u8,
    lang_arr: Vec<char>,
) -> Result<String, Box<dyn std::error::Error>> {
    let letters: HashMap<usize, &char> = lang_arr.iter().enumerate().collect();
    let numbers: HashMap<char, usize> = lang_arr
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
                if shift > lang_arr.len() - 1 {
                    *letters[&(shift % lang_arr.len())]
                } else {
                    *letters[&shift]
                }
            }
        })
        .collect::<String>())
}
