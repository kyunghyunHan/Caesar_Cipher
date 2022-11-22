use Caesar::alphbet_caesar;
use Caesar::korean_caesar;
fn main() {
    println!("{:?}", alphbet_caesar("fghij  abcde", 5).unwrap());

    let test = alphbet_caesar("a", 1).unwrap();
    let test2 = korean_caesar("가나다라마바사", 7).unwrap();
    println!("{}", test);
    println!("{}", test2);
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            alphbet_caesar("jgorevxumxgsskx", 20).unwrap(),
            String::from("dailyprogrammer")
        );
    }
    #[test]
    fn test1() {
        assert_eq!(alphbet_caesar("a", 1).unwrap(), String::from("b"));
    }
    #[test]
    fn test2() {
        assert_eq!(alphbet_caesar("abcz", 1).unwrap(), String::from("bcda"));
    }
    #[test]
    fn test3() {
        assert_eq!(alphbet_caesar("irk", 13).unwrap(), String::from("vex"));
    }
    #[test]
    fn test4() {
        assert_eq!(alphbet_caesar("fusion", 6).unwrap(), String::from("layout"));
    }
    #[test]
    fn test5() {
        assert_eq!(
            alphbet_caesar("dailyprogrammer", 6).unwrap(),
            String::from("jgorevxumxgsskx")
        );
    }
    #[test]
    fn test6() {
        assert_eq!(
            alphbet_caesar("jgorevxumxgsskx", 20).unwrap(),
            String::from("dailyprogrammer")
        );
    }
}
