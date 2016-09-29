// (Used in tests)
#[allow(unused)]
pub const INSERTION_COST:    usize = 1;
#[allow(unused)]
pub const DELETION_COST:     usize = 1;
#[allow(unused)]
pub const SUBSTITUTION_COST: usize = 1;
#[allow(unused)]
pub const IDENTITY_COST:     usize = 0;

use ::std::cmp::min;

fn min3<T: Ord>(a: T, b: T, c: T) -> T {
    min(a, min(b, c))
}

use ::std::collections::BTreeSet;
use ::std::ops::Deref;

pub fn all_within<T: Deref<Target=str> + ::std::cmp::Ord>(items: &[T], target: &str, max: usize) -> Vec<String> {
    let mut found = BTreeSet::new();

    for candidate in items {
        if let Some(dist) = distance_within(candidate, target, max) {
            found.insert((dist, candidate.clone()));
        }
    }

    found.into_iter()
        .map(|(_, elmt)| String::from(elmt.deref()))
        .collect()
}

// Originally translated from the Wikipedia article on Levenshtein distance
pub fn distance(lhs: &str, rhs: &str) -> usize {
    let rhs_len = rhs.chars().count();
    if lhs.is_empty() { return rhs_len; }

    let lhs_len = lhs.chars().count();
    if rhs.is_empty() { return lhs_len; }

    if lhs == rhs { return 0; }

    let mut v0 = (0..(lhs_len + 1)).collect::<Vec<usize>>();
    let mut v1 = v0.clone();

    for (r_index, r_char) in rhs.chars().enumerate() {
        v1[0] = r_index + INSERTION_COST;

        for (l_index, l_char) in lhs.chars().enumerate() {
            let cost = if l_char == r_char {
                IDENTITY_COST
            } else {
                SUBSTITUTION_COST
            };

            v1[l_index + 1] = min3(
                v0[l_index]     + cost,
                v1[l_index]     + DELETION_COST,
                v0[l_index + 1] + INSERTION_COST,
            );
        }

        v0.clone_from_slice(&v1);
    }

    *v1.last().unwrap()
}

// Originally translated from the Wikipedia article on Levenshtein distance
/// Returns `None` early if distance is going to exceed `max`
pub fn distance_within(lhs: &str, rhs: &str, max: usize) -> Option<usize> {
    if lhs == rhs { return Some(0); }

    let lhs_len = lhs.chars().count();
    let rhs_len = rhs.chars().count();

    if lhs.is_empty() {
        return if rhs_len <= max { Some(rhs_len) } else { None };
    }

    if rhs.is_empty() {
        return if lhs_len <= max { Some(lhs_len) } else { None };
    }

    let mut v0 = (0..(lhs_len + 1)).collect::<Vec<usize>>();
    let mut v1 = v0.clone();

    for (r_index, r_char) in rhs.chars().enumerate() {
        v1[0] = r_index + INSERTION_COST;
        let mut v1_min = v1[0];

        for (l_index, l_char) in lhs.chars().enumerate() {
            let cost = if l_char == r_char {
                IDENTITY_COST
            } else {
                SUBSTITUTION_COST
            };

            v1[l_index + 1] = min3(
                v0[l_index]     + cost,
                v1[l_index]     + DELETION_COST,
                v0[l_index + 1] + INSERTION_COST,
            );

            v1_min = min(v1_min, v1[l_index + 1]);
        }

        if v1_min > max { return None };

        v0.clone_from_slice(&v1);
    }

    let out = *v1.last().unwrap();
    if out <= max {
        Some(out)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Note: specific levenshtein quickcheck invariants were suggested at
    // http://stackoverflow.com/a/36994442/808850
    use quickcheck::{Arbitrary, Gen};

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
    fn empty_empty() {
        assert_eq!(distance("", ""), 0);
    }

    #[test]
    fn blerg_blarg() {
        assert_eq!(distance("blërg", "blârg"), SUBSTITUTION_COST);
    }

    #[test]
    fn null_eighty() {
        assert_eq!("\u{0}".chars().count(), 1);
        assert_eq!("\u{80}".chars().count(), 1);

        assert_eq!("\u{0}".bytes().count(), 1);
        assert_eq!("\u{80}".bytes().count(), 2);

        assert_eq!(distance("\u{0}", "\u{80}"), SUBSTITUTION_COST);
        assert_eq!(distance("\u{80}", "\u{0}"), SUBSTITUTION_COST);
    }


    #[derive(Clone, Copy, Debug)]
    enum StrMod {
        Insert(usize, char),
        Delete(usize),
        Substitute(usize, char),
    }

    use self::StrMod::*;

    impl StrMod {
        pub fn cost(&self) -> usize {
            match *self {
                Insert(..)     => INSERTION_COST,
                Delete(..)     => DELETION_COST,
                Substitute(..) => SUBSTITUTION_COST,
            }
        }

        pub fn apply(self, items: &mut Vec<char>) {
            match self {
                Insert(idx, item)     => { items.insert(idx, item); },
                Delete(idx)           => { items.remove(idx); },
                Substitute(idx, item) => {
                    items.push(item);
                    items.swap_remove(idx);
                },
            }
        }

        pub fn apply_all(mut chars: &mut String, mods: &[StrMod]) {
            let mut char_vec: Vec<char> = chars.chars().collect();

            for modi in mods.into_iter() {
                modi.apply(&mut char_vec);
            }

            let char_string: String = char_vec.into_iter().collect();
            chars.clear();
            chars.clone_from(&char_string);
        }
    }

    #[test]
    fn apply_mods() {
        let kitten = String::from("kitten");

        let mut smitten_kitten = kitten.clone();
        StrMod::apply_all(&mut smitten_kitten, &vec!(Substitute(0, 's'), Insert(1, 'm')));

        assert_eq!(smitten_kitten, String::from("smitten"));

        let mut mittens_kitten = kitten.clone();
        StrMod::apply_all(&mut mittens_kitten, &vec!(Substitute(0, 'm'), Insert(6, 's')));

        assert_eq!(mittens_kitten, String::from("mittens"));
    }

    quickcheck! {
        // The edit distance between any string and itself is 0.
        fn one_string(string: String) -> bool {
            distance(&string, &string) == 0
        }

        // The edit distance is symmetric.
        // (No need to ensure non-negative since that's impossible with usize.)
        fn two_strings(string1: String, string2: String) -> bool {
            let d1 = distance(&string1, &string2);
            let d2 = distance(&string2, &string1);

            d1 == d2
        }
    }

    #[derive(Clone, Debug)]
    struct OneChange {
        pub original: String,
        pub change: StrMod,
    }

    fn gen_change_for_collection<G: Gen>(g: &mut G, collection_len: usize) -> StrMod {
        if collection_len == 0 {
            Insert(0, char::arbitrary(g))
        } else {
            match g.gen_range(0, 3) {
                0 => Insert(g.gen_range(0, collection_len + 1), char::arbitrary(g)),
                1 => Substitute(g.gen_range(0, collection_len), char::arbitrary(g)),
                _ => Delete(g.gen_range(0, collection_len)),
            }
        }
    }

    impl Arbitrary for OneChange {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let original = String::arbitrary(g);

            let change = gen_change_for_collection(g, original.chars().count());

            OneChange {
                original: original,
                change: change,
            }
        }
    }

    quickcheck! {
        // For an arbitrary string x, if you apply exactly one change to it, producing y, the edit distance between x and y should be 1.
        fn change_string_once(one_change: OneChange) -> bool {
            let mut changed_string: Vec<char> = one_change.original.chars().collect();
            one_change.change.apply(&mut changed_string);

            let changed_string: String = changed_string.into_iter().collect();
            let dist = distance(&one_change.original, &changed_string);

            match one_change.change {
                Insert(..) => dist == INSERTION_COST,
                Delete(..) => dist == DELETION_COST,
                Substitute(..) => {
                    (one_change.original == changed_string && dist == 0)
                        || dist == SUBSTITUTION_COST
                },
            }
        }
    }

    #[derive(Clone, Debug)]
    struct MultiChange {
        pub original: String,
        pub changes: Vec<StrMod>,
    }

    impl Arbitrary for MultiChange {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let original = String::arbitrary(g);
            let mut changes = vec!();
            let mut working_string: Vec<_> = original.chars().collect();

            for _ in 0..(g.gen_range(1, 25)) {
                let change = gen_change_for_collection(g, working_string.len());
                changes.push(change);
                change.apply(&mut working_string);
            }

            MultiChange {
                original: original,
                changes: changes,
            }
        }
    }

    quickcheck! {
        // Given two strings x and y, compute the distance d between them. Then, change y, yielding y', and compute its distance from x: it should differ from d by at most 1.
        // After applying n edits to a string x, the distance between the edited string and x should be at most n. Note this is a generalization of (1), so you could omit that one if you like.
        // The function should be symmetric: the edit distance from x to y should be the same as from y to x.
        fn change_string_multi(multi_change: MultiChange) -> bool {
            let MultiChange { original, changes } = multi_change;

            let mut changed_string = original.clone();
            StrMod::apply_all(&mut changed_string, &changes);

            let max_dist = changes.iter()
                .map(StrMod::cost)
                .fold(0usize, |acc, cost| acc + cost);

            let dist1 = distance(&original, &changed_string);
            let dist2 = distance(&changed_string, &original);

            dist1 <= max_dist && dist1 == dist2
        }
    }

    #[test]
    fn typo_1() {
        let group: Vec<String> = vec!(
            "user-agents".into(),
            "user-agent".into(),
            "User-Agents".into(),
            "loser-agent".into(),
            "language".into(),
            "session-length".into()
            );

        assert_eq!(all_within(&group, "usr-agent", 3),
                   vec!(String::from("user-agent"),
                        String::from("user-agents"),
                        String::from("loser-agent")));
    }
}
