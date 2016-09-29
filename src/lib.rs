pub const INSERTION_COST:    usize = 1;
pub const DELETION_COST:     usize = 1;
pub const SUBSTITUTION_COST: usize = 1;
pub const IDENTITY_COST:     usize = 0;

pub fn distance(lhs: &str, rhs: &str) -> usize {
    43usize
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
}
