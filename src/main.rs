use std::io::stdin;

fn is_digit(ch: u8) -> bool {
    '0' as u8 <= ch && ch <= '9' as u8
}
fn to_digit(ch: u8) -> u64 {
    (ch - '0' as u8) as u64
}

fn try_get_english_digit(s: &[u8]) -> Option<u64> {
    return Some(if s.starts_with(b"one") {
        1
    } else if s.starts_with(b"two") {
        2
    } else if s.starts_with(b"three") {
        3
    } else if s.starts_with(b"four") {
        4
    } else if s.starts_with(b"five") {
        5
    } else if s.starts_with(b"six") {
        6
    } else if s.starts_with(b"seven") {
        7
    } else if s.starts_with(b"eight") {
        8
    } else if s.starts_with(b"nine") {
        9
    } else {
        return None;
    });
}

fn first_and_last<T: Clone>(mut it: impl Iterator<Item = T>) -> Option<(T, T)> {
    let first = it.next()?;
    let last = it.last().unwrap_or(first.clone());

    return Some((first, last));
}
fn get_entry(x: &[u8]) -> u64 {
    let size = x.len();
    let digits = (0..size)
        .into_iter()
        .map(|i| x[i..].as_ref())
        .filter_map(|substr| {
            if is_digit(substr[0]) {
                return Some(to_digit(substr[0]));
            }
            try_get_english_digit(substr)
        });
    let (first_digit, last_digit) =
        first_and_last(digits).expect("Expected string to contain at least one digit");

    10 * first_digit + last_digit
}
fn main() {
    let result = stdin()
        .lines()
        .map(|x| {
            return get_entry(x.expect("Error reading standard input").as_bytes());
        })
        .sum::<u64>();

    println!("{result}");
}
