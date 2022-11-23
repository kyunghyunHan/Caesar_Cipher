use Caesar::caesar;

fn main() {
    let korea_arr = vec![
        '가', '나', '다', '라', '마', '바', '사', '아', '자', '차', '타', '파', '하',
    ];
    let alphbet_arr = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    println!("{:?}", caesar("fghij  abcde", 5, korea_arr).unwrap());
    println!("{:?}", caesar("fghij  abcde", 5, alphbet_arr).unwrap());
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
            caesar("jgorevxumxgsskx", 20, alphbet_arr).unwrap(),
            String::from("dailyprogrammer")
        );
    }
    #[test]
    fn test1() {
        let alphbet_arr = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        assert_eq!(caesar("a", 1, alphbet_arr).unwrap(), String::from("b"));
    }
    #[test]
    fn test2() {
        let alphbet_arr = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        assert_eq!(
            caesar("abcz", 1, alphbet_arr).unwrap(),
            String::from("bcda")
        );
    }
    #[test]
    fn test3() {
        let alphbet_arr = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        assert_eq!(caesar("irk", 13, alphbet_arr).unwrap(), String::from("vex"));
    }
    #[test]
    fn test4() {
        let alphbet_arr = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        assert_eq!(
            caesar("fusion", 6, alphbet_arr).unwrap(),
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
            caesar("dailyprogrammer", 6, alphbet_arr).unwrap(),
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
            caesar("jgorevxumxgsskx", 20, alphbet_arr).unwrap(),
            String::from("dailyprogrammer")
        );
    }
}
