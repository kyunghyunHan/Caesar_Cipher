use std::collections::HashMap;
pub fn encryption(
    string: &str,
    shifts: u8,
    lang_arr: Vec<char>,
    lang_arr2: Vec<char>,
) -> Result<String, Box<dyn std::error::Error>> {
    //현재 반복 횟수와 다음 값을 제공하는 반복자를 만듬.
    //반환된 반복자는 i,val생성
    let letters: HashMap<usize, &char> = lang_arr2.iter().enumerate().collect();
    let numbers: HashMap<char, usize> = lang_arr
        .iter()
        .enumerate()
        .map(|(idx, chr)| (chr.clone(), idx))
        .collect();
    //chars:문자열 조각의 [char]s에 대한 반복자를 반환
    Ok(string
        .chars()
        .map(|c| {
            //c가 띄우쓰기면 그대로 반환
            if c == ' ' {
                c
            } else {
                //shifts를 usize로 변경후 number  shifts 만큼이동
                //Hashmap에서 "a"를 찾아서 usize만큼 이동
                let shift = numbers[&c] + shifts as usize;
                //shift가 배열의 -1보다 크면
                //정렬
                if shift > lang_arr.len() - 1 {
                    //letter shift % 배열의 크기만큼으로 변경
                    *letters[&(shift % lang_arr.len())]
                } else {
                    //아니면 그위치
                    *letters[&shift]
                }
            }
        })
        //스트링으로 반환
        .collect::<String>())
}
