use Caesar::encryption;
fn main() {
    //다음반복 횟수와 다음 값을 제공하는 반복자
    // let letters: HashMap<usize, &char> = tes2.iter().enumerate().collect();

    let korea_arr = vec![
        '가', '나', '다', '라', '마', '바', '사', '아', '자', '차', '카', '타', '파', '하',
    ];
    let alphbet_arr = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let upper_alphbet_arr = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let upper_korea_arr = vec![
        '갸', '냐', '댜', '랴', '먀', '뱌', '샤', '야', '쟈', '챠', '캬', '탸', '퍄', '햐',
    ];
    println!(
        "{:?}",
        encryption("나다라마 바사아자", 5, korea_arr, upper_korea_arr).unwrap()
    );
    println!(
        "{:?}",
        encryption("traue nie dem brutus", 3, alphbet_arr, upper_alphbet_arr).unwrap()
    );
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let alphbet_arr = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        let upper_alphbet_arr = vec![
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];
        assert_eq!(
            encryption("traue nie dem brutus", 3, alphbet_arr, upper_alphbet_arr).unwrap(),
            "WUDXH QLH GHP EUXWXV"
        )
    }
}
