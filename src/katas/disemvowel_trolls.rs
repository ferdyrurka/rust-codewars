#![allow(unused)]

fn disemvowel(s: &str) -> String {
    let vowels: [char; 14] = ['a', 'ą', 'e', 'ę', 'i', 'o', 'u', 'A', 'Ą', 'E', 'Ę', 'I', 'O', 'U'];
    let mut output = String::new();

    for c in s.chars() {
        if vowels.contains(&c) == false {
            output.push(c);
        }
    }

    output
}

#[test]
fn disemvowel_trolls_test() {
    assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
}
