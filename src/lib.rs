#[cfg(test)]
#[macro_use]
extern crate quickcheck;

pub const INSERTION_COST:    usize = 1;
pub const DELETION_COST:     usize = 1;
pub const SUBSTITUTION_COST: usize = 1;
pub const IDENTITY_COST:     usize = 0;

pub fn distance(lhs: &str, rhs: &str) -> usize {
    // Thanks, Mr Wales!

    if lhs == rhs { return 0; }

    if lhs.is_empty() || rhs.is_empty() {
        return lhs.chars().chain(rhs.chars()).count();
    }

    let mut v0 = (0..(lhs.chars().count() + 1)).collect::<Vec<_>>();
    let mut v1 = v0.clone();

    for (r_index, r_char) in rhs.chars().enumerate() {
        v1[0] = r_index + INSERTION_COST;

        for (l_index, l_char) in lhs.chars().enumerate() {
            let cost = if l_char == r_char {
                IDENTITY_COST
            } else {
                SUBSTITUTION_COST
            };

            use ::std::cmp::min;

            v1[l_index + 1] = min(
                v0[l_index] + cost,
                min(
                    v1[l_index]     + DELETION_COST,
                    v0[l_index + 1] + INSERTION_COST,
                )
            );
        }

        v0.clone_from_slice(&v1);
    }

    *v1.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sitting_kitten() {
        assert_eq!(distance("sitting", "kitten"), SUBSTITUTION_COST * 2 + INSERTION_COST);
    }

    #[test]
    fn sunday_saturday() {
        assert_eq!(distance("Saturday", "Sunday"), INSERTION_COST * 2 + SUBSTITUTION_COST);
    }

    #[test]
    fn saturday_saturday() {
        assert_eq!(distance("Saturday", "Saturday"), 0);
    }

    #[test]
    fn null_eighty() {
        assert_eq!(distance("\u{0}", "\u{80}"), SUBSTITUTION_COST);
        assert_eq!(distance("\u{80}", "\u{0}"), SUBSTITUTION_COST);
    }

    // Invariants:
    //
    // - The edit distance between any string and itself is 0.
    //   I.e.: `distance(a, a) == 0`
    quickcheck! {
        fn one_string(string: String) -> bool {
            distance(&string, &string) == 0
        }
    }

    // - The edit distance is symmetric.
    //   I.e.: `distance(a, b) == distance(b, a)`
    quickcheck! {
        fn two_strings(string1: String, string2: String) -> bool {
            distance(&string1, &string2) == distance(&string2, &string1)
        }
    }
}
