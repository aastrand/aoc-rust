use std::collections::HashSet;

// split off an arbitrary element from a (non-empty) set
pub fn hashset_pop<T>(set: &mut HashSet<T>) -> Option<T>
where
    T: Eq + Copy + Clone + std::hash::Hash,
{
    let elt = set.iter().next().cloned();
    if elt.is_some() {
        set.remove(&elt.unwrap());
    }
    elt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashset_pop() {
        let mut s = HashSet::new();

        s.insert(1);
        assert_eq!(Some(1), hashset_pop(&mut s));

        s.insert(2);
        assert_eq!(Some(2), hashset_pop(&mut s));

        assert_eq!(None, hashset_pop(&mut s));
    }
}
