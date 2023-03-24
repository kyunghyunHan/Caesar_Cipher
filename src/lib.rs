use std::collections::HashMap;

pub fn encryption(
    string: &str,//암호화할 문자열
    shifts: u8,//시저암호에서 사용할 이동거리
    lang_arr: Vec<char>,//사용할 문자집합
    lang_arr_upper: Vec<char>,//대문자로 변경할 문자 집합
) -> Result<String, Box<dyn std::error::Error>> {//함수는 Result<String, Box<dyn std::error::Error>>를 반환
     //1. letters 변수에 lang_arr2를 열거하면서 각 문자의 인덱스를 저장하는 HashMap을 만듭니다.
    

    let letters: HashMap<usize, &char> = lang_arr_upper.iter().enumerate().collect();
    //2. numbers 변수에 lang_arr를 열거하면서 각 문자와 인덱스를 저장하는 HashMap을 만듭니다.
    let numbers: HashMap<char, usize> = lang_arr
        .iter()
        .enumerate()
        .map(|(idx, chr)| (chr.clone(), idx))
        .collect();

    //3. string을 chars() 메서드를 사용하여 char 타입의 반복자로 변환합니다.  
    Ok(string
        .chars()
        .map(|c| {
       //    * map() 메서드를 사용하여 문자열의 각 문자에 대해 시저 암호를 수행합니다.
            if c == ' ' {//    * 문자가 공백인 경우 그대로 반환합니다.
                c
            } else {
                // * 문자가 알파벳인 경우, numbers HashMap을 사용하여 문자의 인덱스를 얻고 shifts 만큼 이동합니다.
                //shifts를 usize로 변경후 number  shifts 만큼이동
                //Hashmap에서 "a"를 찾아서 usize만큼 이동
                // 5                  0            5
                let shift = numbers[&c] + shifts as usize;
        
                //* 이동 후 인덱스가 lang_arr의 범위를 벗어나는 경우, letters HashMap을 사용하여 알파벳을 순환시킵니다.
                if shift > lang_arr.len() - 1 {
                    //letter shift % 배열의 크기만큼으로 변경
                    *letters[&(shift % lang_arr.len())]
                } else {
                    //순환시키지 않아도 되는 경우, letters HashMap을 사용하여 인덱스를 문자로 변환합니다
                    *letters[&shift]
                }
            }
        })
        //4. collect() 메서드를 사용하여 암호화된 문자열을 생성합니다.
        .collect::<String>())
      
}
