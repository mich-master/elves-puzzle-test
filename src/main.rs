fn utf8_to_digit(code: u8) -> Option<u32> {
    match code {
        48 => Some(0),
        49 => Some(1),
        50 => Some(2),
        51 => Some(3),
        52 => Some(4),
        53 => Some(5),
        54 => Some(6),
        55 => Some(7),
        56 => Some(8),
        57 => Some(9),
        _ => None,
    }
}

fn elves_line_value(line: &str) -> u32 {
    line.find(|c: char| c.is_ascii_digit())
        .and_then(|first_digit_index| utf8_to_digit(line.as_bytes()[first_digit_index]))
        .and_then(|first_digit| {
            line.rfind(|c: char| c.is_ascii_digit())
                .map(|second_digit_index| (first_digit, second_digit_index))
        })
        .and_then(|(first_digit, second_digit_index)| {
            utf8_to_digit(line.as_bytes()[second_digit_index])
                .map(|second_digit| first_digit * 10 + second_digit)
        })
        .unwrap_or(0)
}

fn elves_puzzle(text: &str) -> u32 {
    text.lines()
        .fold(0, |sum, line| elves_line_value(line) + sum)
}

fn main() {
    let text = "5adfgb6
    pq34stuvw0x
    a2be5f
    trebuc6het";
    let r: u32 = elves_puzzle(text);
    println!("{}", r);
}

#[test]
fn test_elves_puzzle() {
    let text = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    assert_eq!(elves_puzzle(text), 142);
}
