use std::collections::HashMap;
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
    println!(
        "{:?}",
        encryption("나다라마 바사아자", 5, korea_arr).unwrap()
    );

    let dd = vec!['일', '이', '삼', '사', '오'];
    // println!("{:?}", encryption("fghij    abcde", 5, alphbet_arr).unwrap());
    let letters: HashMap<usize, &char> = dd.iter().enumerate().collect();
    let numbers: HashMap<char, usize> = dd
        .iter()
        .enumerate()
        .map(|(idx, chr)| (chr.clone(), idx))
        .collect();
    println!("{:?}", numbers);
}
#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test() {
        let alphbet_arr = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        assert_eq!(
            encryption("jgorevxumxgsskx", 20, alphbet_arr).unwrap(),
            String::from("dailyprogrammer")
        );
    }
    #[test]
    fn test1() {
        let alphbet_arr = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        assert_eq!(encryption("a", 1, alphbet_arr).unwrap(), String::from("b"));
    }
    #[test]
    fn test2() {
        let alphbet_arr = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        assert_eq!(
            encryption("abcz", 1, alphbet_arr).unwrap(),
            String::from("bcda")
        );
    }
    #[test]
    fn test3() {
        let alphbet_arr = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        assert_eq!(
            encryption("irk", 13, alphbet_arr).unwrap(),
            String::from("vex")
        );
    }
    #[test]
    fn test4() {
        let alphbet_arr = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        assert_eq!(
            encryption("fusion", 6, alphbet_arr).unwrap(),
            String::from("layout")
        );
    }
    #[test]
    fn test5() {
        let alphbet_arr = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        assert_eq!(
            encryption("dailyprogrammer", 6, alphbet_arr).unwrap(),
            String::from("jgorevxumxgsskx")
        );
    }
    #[test]
    fn test6() {
        let alphbet_arr = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        assert_eq!(
            encryption("jgorevxumxgsskx", 20, alphbet_arr).unwrap(),
            String::from("dailyprogrammer")
        );
    }
}
