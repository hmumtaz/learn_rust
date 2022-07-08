fn main() {
    let s0 = String::from("ball");
    let ret_s0 = pig_latin(&s0);
    println!("{} in pig latin is {}", s0, ret_s0);
    let s1 = String::from("apple");
    let ret_s1 = pig_latin(&s1);
    println!("{} in pig latin is {}", s1, ret_s1);
}

fn pig_latin(s: &str) -> String {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let first = s.chars().nth(0).unwrap();
    let mut ret = String::from(s);
    for vowel in VOWELS {
        if first == vowel {
            ret.push_str("-hay");
            return ret;
        }
    }
    let mut ret = String::from(&s[1..]);
    ret.push('-');
    ret.push_str(&s[0..1]);
    ret.push_str("ay");
    return ret;
}
