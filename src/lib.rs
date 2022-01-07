use std::cmp::max;
use std::fmt::{Display, Formatter, Write};

mod char; // private module

const DEFAULT_DELTA: usize = 3;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SortKey(Vec<u8>);

impl SortKey {
    fn new(v: Vec<u8>) -> Self {
        assert!(!v.is_empty(), "`SortKey` shouldn't be empty");
        for i in v.iter() {
            assert!(
                char::is_valid_u8(*i),
                "`SortKey` shouldn't contain invalid u8: {}",
                i
            );
        }
        assert_ne!(
            v.last(),
            Some(&char::MIN),
            "`SortKey` shouldn't end with {}",
            char::MIN
        );

        Self(v)
    }

    pub fn is_valid_str(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }

        if s.chars().any(|c| !char::is_valid_char(c)) {
            return false;
        }

        if matches!(s.chars().last(), Some(char::MIN_CHAR)) {
            return false;
        }

        true
    }

    pub fn before(&self) -> Self {
        self.before_with_delta(DEFAULT_DELTA)
    }

    /// # Panics
    ///
    /// - if delta is zero
    pub fn before_with_delta(&self, delta: usize) -> Self {
        assert!(delta > 0);
        let mut res = Vec::new();

        for at in 0..self.0.len() {
            let ch = self.0[at];
            res.push(ch);
            if ch > char::MIN {
                break;
            }
        }

        for _ in 0..delta {
            let ch = *self.0.get(res.len()).unwrap_or(&char::MIN);
            res.push(ch);
        }

        for i in 0..=delta {
            let at = res.len() - i - 1;

            match (i, res[at]) {
                (0, char::NEXT_MIN) | (_, char::MIN) => {
                    res[at] = char::MAX;
                    continue;
                }
                (_, ch) => {
                    res[at] = ch - 1;
                    break;
                }
            }
        }

        assert!(res < self.0, "`before` should satisfy post-condition");
        Self::new(res)
    }

    pub fn after(&self) -> Self {
        self.after_with_delta(DEFAULT_DELTA)
    }

    /// # Panics
    ///
    /// - if delta is zero
    pub fn after_with_delta(&self, delta: usize) -> Self {
        assert!(delta > 0);
        let mut res = Vec::new();

        for at in 0..self.0.len() {
            let ch = self.0[at];
            res.push(ch);
            if ch < char::MAX {
                break;
            }
        }

        for _ in 0..delta {
            let ch = *self.0.get(res.len()).unwrap_or(&char::MIN);
            res.push(ch);
        }

        for i in 0..=delta {
            let at = res.len() - i - 1;

            match (i, res[at]) {
                (0, char::MAX) => {
                    res[at] = char::NEXT_MIN;
                    continue;
                }
                (_, char::MAX) => {
                    res[at] = char::MIN;
                    continue;
                }
                (_, ch) => {
                    res[at] = ch + 1;
                    break;
                }
            }
        }

        assert!(res > self.0, "`after` should satisfy post-condition");
        Self::new(res)
    }

    /// # Panics
    ///
    /// - if self and other are equal
    pub fn between(&self, other: &Self) -> Self {
        assert_ne!(
            self, other,
            "`between` should satisfy pre-condition: `self` and `other` shouldn't be equal"
        );

        let max_len = max(self.0.len(), other.0.len());
        let ten = char::MAX - char::MIN + 1;
        let mut res = Vec::new();
        res.resize(max_len + 1, char::MIN);

        // self + other
        for at in (0..max_len).rev() {
            res[at] += *self.0.get(at).unwrap_or(&char::MIN) - char::MIN;
            res[at] += *other.0.get(at).unwrap_or(&char::MIN) - char::MIN;
            if at > 0 && res[at] > char::MAX {
                res[at] -= ten;
                res[at - 1] += 1;
            }
        }

        // / 2
        for at in 0..res.len() {
            if at < res.len() - 1 && res[at] % 2 == 1 {
                res[at + 1] += ten;
            }
            res[at] /= 2;
        }

        // truncate
        let (low, high) = if self < other {
            (self, other)
        } else {
            (other, self)
        };
        for at in 0..max_len {
            if res[at] > *low.0.get(at).unwrap_or(&char::MIN) {
                res.truncate(at + 1);
                break;
            }
        }

        assert!(
            res > low.0,
            "`between` should satisfy post-condition: {} {} {:?}",
            low,
            high,
            &res
        );
        assert!(
            res < high.0,
            "`between` should satisfy post-condition: {} {} {:?}",
            low,
            high,
            &res
        );
        Self::new(res)
    }

    pub fn try_between(&self, other: &Self) -> Option<Self> {
        if self == other {
            None
        } else {
            Some(self.between(other))
        }
    }
}

impl Default for SortKey {
    fn default() -> Self {
        Self::new(vec![(char::MIN + char::MAX + 1) / 2])
    }
}

impl Display for SortKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in &self.0 {
            let c = char::to_char(*i);
            f.write_char(c)?;
        }
        Ok(())
    }
}

impl TryFrom<String> for SortKey {
    type Error = TryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if SortKey::is_valid_str(&value) {
            let chars = value.chars().map(char::to_u8).collect();
            Ok(SortKey::new(chars))
        } else {
            Err(TryFromStringError(value))
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("SortKey invalid format: {0}")]
pub struct TryFromStringError(String);

#[cfg(test)]
mod tests {
    use crate::SortKey;

    #[test]
    fn is_valid_str() {
        assert!(SortKey::is_valid_str("012"));
        assert!(SortKey::is_valid_str("abc"));
        assert!(SortKey::is_valid_str(
            "0123456789abcdefghijklmnopqrstuvwxyz"
        ));
        assert!(!SortKey::is_valid_str(""));
        assert!(!SortKey::is_valid_str("A"));
        assert!(!SortKey::is_valid_str("120"));
    }

    #[test]
    fn default() {
        let k = SortKey::default();
        assert_eq!(&k.to_string(), "i");
    }

    #[test]
    fn display() {
        let k = SortKey::try_from("012".to_owned()).unwrap();
        assert_eq!(&k.to_string(), "012");
    }

    #[test]
    fn try_from_string() {
        assert!(SortKey::try_from("012".to_owned()).is_ok());
        assert!(SortKey::try_from("120".to_owned()).is_err());
    }

    #[test]
    fn before_normal_case() {
        assert_before("abc");
    }

    #[test]
    fn before_carry_down_case() {
        assert_before("ab1");
    }

    #[test]
    fn before_underflow_case() {
        assert_before("01");
    }

    fn assert_before(input: &str) {
        let input = SortKey::try_from(input.to_owned()).unwrap();
        let got = input.before();
        assert!(got < input);
    }

    #[test]
    fn after_normal_case() {
        assert_after("abc");
    }

    #[test]
    fn after_carry_up_case() {
        assert_after("abz");
    }

    #[test]
    fn after_multi_carry_up_case() {
        assert_after("abzzz");
    }

    #[test]
    fn after_overflow_case() {
        assert_after("zzz");
    }

    fn assert_after(input: &str) {
        let input = SortKey::try_from(input.to_owned()).unwrap();
        let got = input.after();
        assert!(got > input);
    }

    #[test]
    fn between_simple_case() {
        assert_between("3", "j");
    }

    #[test]
    fn between_neighbor_case() {
        assert_between("3", "4");
    }

    #[test]
    fn between_difficult_case01() {
        assert_between("x", "x1");
    }

    #[test]
    fn between_difficult_case02() {
        assert_between("x", "x01");
    }

    #[test]
    fn between_difficult_case03() {
        assert_between("xz", "y");
    }

    #[test]
    fn between_difficult_case04() {
        assert_between("abc3", "abd");
    }

    #[test]
    fn between_difficult_case05() {
        assert_between("abc", "abd3");
    }

    #[test]
    fn between_difficult_case06() {
        assert_between("ab2z", "ab3");
    }

    #[test]
    fn between_difficult_case07() {
        assert_between("aaxaaaaaaaaaa", "aaybbbbbbbbbb");
    }

    #[test]
    fn between_difficult_case08() {
        assert_between("hzzzz", "hzzzzi");
    }

    #[test]
    fn between_difficult_case09() {
        assert_between("hzzzzb", "hzzzzci");
    }

    fn assert_between(low: &str, high: &str) {
        let low = SortKey::try_from(low.to_owned()).unwrap();
        let high = SortKey::try_from(high.to_owned()).unwrap();
        let got = low.between(&high);
        dbg!(&low, &got, &high);
        assert!(&low < &got);
        assert!(&got < &high);
    }
}
