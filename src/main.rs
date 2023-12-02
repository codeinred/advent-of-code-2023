use std::io::stdin;

fn is_digit(ch: u8) -> bool {
    '0' as u8 <= ch && ch <= '9' as u8
}

fn to_digit(ch: u8) -> u64 {
    (ch - '0' as u8) as u64
}

fn get_digit(s: &[u8]) -> Option<u64> {
    let first = *s.first()?;
    is_digit(first).then(|| to_digit(first))
}

fn try_get_english_digit(s: &[u8]) -> Option<u64> {
    const DIGITS: [&[u8]; 10] = [
        b"zero", b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];
    DIGITS
        .iter()
        .enumerate()
        .filter_map(|(i, word)| s.starts_with(word).then_some(i as u64))
        .next()
}

fn try_get_any_digit(s: &[u8]) -> Option<u64> {
    get_digit(s).or_else(|| try_get_english_digit(s))
}

fn first_and_last<T: Clone>(mut it: impl Iterator<Item = T>) -> Option<(T, T)> {
    let first = it.next()?;
    let last = it.last().unwrap_or(first.clone());

    return Some((first, last));
}

fn get_entry(x: &[u8]) -> Option<u64> {
    let digits = (0..x.len())
        .into_iter()
        .map(|i| x[i..].as_ref())
        .filter_map(try_get_any_digit);
    let (first, last) = first_and_last(digits)?;

    Some(10 * first + last)
}
fn main() {
    let result = stdin()
        .lines()
        .map(Result::unwrap)
        .filter_map(|s| get_entry(s.as_bytes()))
        .sum::<u64>();

    println!("{result}");
}
