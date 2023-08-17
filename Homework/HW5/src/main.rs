fn main() {
    println!("{:?}", count_vowel_v2("gghgaaaa ioioidsf"));
    println!("{:?}", split_grade(vec!["A", "B", "F", "A+", "D", "C"]));
    println!("{:?}", split_score(vec![10, 23, 45, 60, 80, 98]));
    println!(
        "{:?}",
        extract_quoted_word_r("C  ** *C++*   *Java *Python* Rust*")
    );
}

fn count_vowel(v: &str) -> i64 {
    let mut result: i64 = 0;
    for i in v.split("") {
        result += match i {
            "a" | "A" | "e" | "E" | "i" | "I" | "o" | "O" | "u" | "U" => 1,
            _ => 0,
        }
    }
    result
}

fn count_vowel_r(v: &str) -> i64 {
    if v.is_empty() {
        return 0;
    }
    let mut result: i64 = 0;
    let text: &str = &v[0..1];
    match text {
        "a" | "A" | "e" | "E" | "i" | "I" | "o" | "O" | "u" | "U" => result += 1,
        _ => result += 0,
    };

    result + count_vowel_r(&v[1..])
}

fn count_vowel_v2(v: &str) -> Vec<(&str, i64)> {
    let mut result: Vec<(&str, i64)> = Vec::new();
    let v = v.split_whitespace();
    for i in v {
        let mut count: i64 = 0;
        for a in i.split("") {
            count += match a {
                "a" | "A" | "e" | "E" | "i" | "I" | "o" | "O" | "u" | "U" => 1,
                _ => 0,
            }
        }
        result.push((i, count))
    }
    result
}

fn split_grade(input: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let mut a: Vec<&str> = Vec::new();
    let mut d: Vec<&str> = Vec::new();
    for i in input {
        match i {
            "A+" | "A" | "B" | "C" => a.push(i),
            "D" | "F" => d.push(i),
            _ => continue,
        };
    }
    (a, d)
}

fn split_score(input: Vec<i64>) -> (Vec<(String, i64)>, Vec<(String, i64)>) {
    let mut a: Vec<(String, i64)> = Vec::new();
    let mut d: Vec<(String, i64)> = Vec::new();
    for i in input {
        match i {
            0..=49 => d.push(("F".to_string(), i)),
            50..=60 => d.push(("D".to_string(), i)),
            61..=70 => a.push(("C".to_string(), i)),
            71..=80 => a.push(("B".to_string(), i)),
            81..=94 => a.push(("A".to_string(), i)),
            95..=100 => a.push(("A+".to_string(), i)),
            _ => continue,
        }
    }
    (a, d)
}

fn extract_quoted_word(input: &str) -> Vec<&str> {
    let text: Vec<&str> = input.split(" ").collect();
    let mut result: Vec<&str> = Vec::new();
    for i in text {
        let mut count = 0;
        for s in i.chars() {
            if s == '*' {
                count += 1;
            }
        }
        if count == 2 {
            result.push(&i[1..i.len() - 1]);
        }
    }
    result
}

fn extract_quoted_word_r<'a>(input: &'a str) -> Vec<&'a str> {
    fn inner<'a>(words: &[&'a str]) -> Vec<&'a str> {
        if words.is_empty() {
            return Vec::new();
        }
        
        let mut result: Vec<&'a str> = Vec::new();
        let mut count = 0;
        
        for s in words[0].chars() {
            if s == '*' {
                count += 1;
            }
        }
        
        if count == 2 {
            result.push(&words[0][1..words[0].len() - 1]);
        }
        
        let mut remaining_result = inner(&words[1..]);
        result.append(&mut remaining_result);
        
        result
    }

    let text: Vec<&str> = input.split(" ").collect();
    inner(&text)
}

#[test]
fn test_count_vowel() {
    assert_eq!(count_vowel(""), 0);
    assert_eq!(count_vowel("hello"), 2);
    assert_eq!(count_vowel("Rust"), 1);
    assert_eq!(count_vowel("world"), 1);
}

#[test]
fn test_count_vowel_r() {
    assert_eq!(count_vowel_r(""), 0);
    assert_eq!(count_vowel_r("hello"), 2);
    assert_eq!(count_vowel_r("Rust"), 1);
    assert_eq!(count_vowel_r("world"), 1);
}

#[test]
fn test_count_vowel_v2() {
    assert_eq!(count_vowel_v2(""), vec![]);
    assert_eq!(
        count_vowel_v2("hello world"),
        vec![("hello", 2), ("world", 1)]
    );
    assert_eq!(
        count_vowel_v2("Rust programming is not fun"),
        vec![("Rust", 1), ("programming", 3), ("is", 1), ("not", 1), ("fun", 1)]
    );
}

#[test]
fn test_split_grade() {
    assert_eq!(
        split_grade(vec![]),
        (vec![], vec![])
    );
    assert_eq!(
        split_grade(vec!["A+", "B", "F", "C", "D", ""]),
        (vec!["A+", "B", "C"], vec!["F", "D"])
    );
}

#[test]
fn test_split_score() {
    assert_eq!(
        split_score(vec![]),
        (vec![],vec![])
    );
    assert_eq!(
        split_score(vec![80, 86, 20, 60, 95, 42, 1000]),
        (
            vec![("B".to_string(), 80),("A".to_string(), 86), ("A+".to_string(), 95)],
            vec![
                ("F".to_string(), 20),
                ("D".to_string(), 60),
                ("F".to_string(), 42)
            ]
        )
    );
}

#[test]
fn test_extract_quoted_word() {
    let empty_array: Vec<&str> = vec![];
    assert_eq!(
        extract_quoted_word(""),
        empty_array
    );
    assert_eq!(
        extract_quoted_word("C  ** *C++*   *Java *Python* Rust*"),
        vec!["", "C++", "Python"]
    );
    assert_eq!(
        extract_quoted_word("*****"),
        empty_array
    );
}

#[test]
fn test_extract_quoted_word_r() {
    let empty_array: Vec<&str> = vec![];
    assert_eq!(
        extract_quoted_word(""),
        empty_array
    );
    assert_eq!(
        extract_quoted_word("C  ** *C++*   *Java *Python* Rust*"),
        vec!["", "C++", "Python"]
    );
    assert_eq!(
        extract_quoted_word("*****"),
        empty_array
    );
}