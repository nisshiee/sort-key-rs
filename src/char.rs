pub const MIN: u8 = 0;
pub const NEXT_MIN: u8 = MIN + 1;
pub const MAX: u8 = 35;
pub const MIN_CHAR: char = '0';

pub fn is_valid_char(c: char) -> bool {
    ('0'..='9').contains(&c) || ('a'..='z').contains(&c)
}

pub fn is_valid_u8(i: u8) -> bool {
    (MIN..=MAX).contains(&i)
}

pub fn to_char(i: u8) -> char {
    if (MIN..10).contains(&i) {
        char::from_u32('0' as u32 + i as u32).expect("'0' to '9' are valid unicode")
    } else if (10..=MAX).contains(&i) {
        char::from_u32('a' as u32 + i as u32 - 10).expect("'a' to 'z' are valid unicode")
    } else {
        panic!("`to_char` must receive valid value: {} is invalid", i);
    }
}

pub fn to_u8(c: char) -> u8 {
    if ('0'..='9').contains(&c) {
        c as u8 - b'0'
    } else if ('a'..='z').contains(&c) {
        c as u8 - b'a' + 10
    } else {
        panic!("`to_u8` must receive valid char: '{}' is invalid", c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_test() {
        let patterns = [
            (0u8, '0'),
            (1, '1'),
            (2, '2'),
            (3, '3'),
            (4, '4'),
            (5, '5'),
            (6, '6'),
            (7, '7'),
            (8, '8'),
            (9, '9'),
            (10, 'a'),
            (11, 'b'),
            (12, 'c'),
            (13, 'd'),
            (32, 'w'),
            (33, 'x'),
            (34, 'y'),
            (35, 'z'),
        ];

        for (i, c) in patterns {
            assert!(is_valid_char(c));
            assert_eq!(to_char(i), c);
            assert_eq!(to_u8(c), i);
        }
    }

    #[test]
    #[should_panic]
    fn convert_fail01() {
        to_char(36);
    }

    #[test]
    #[should_panic]
    fn convert_fail02() {
        to_u8('A');
    }
}
