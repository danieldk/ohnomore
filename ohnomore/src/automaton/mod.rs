use fst::raw::{Fst, Node};
use fst::Set;

/// Search prefixes of a string in a set.
pub trait Prefixes {
    /// Get an iterator over the prefixes of a string that are in a set.
    fn prefixes<'a, 'b>(&'a self, word: &'b str) -> PrefixIter<'a, 'b>;
}

impl Prefixes for Set {
    fn prefixes<'a, 'b>(&'a self, word: &'b str) -> PrefixIter<'a, 'b> {
        PrefixIter {
            fst: self.as_fst(),
            node: self.as_fst().root(),
            prefix_len: 0,
            word,
        }
    }
}

/// Prefix iterator.
pub struct PrefixIter<'a, 'b> {
    fst: &'a Fst,
    node: Node<'a>,
    prefix_len: usize,
    word: &'b str,
}

impl<'a, 'b> Iterator for PrefixIter<'a, 'b> {
    type Item = &'b str;

    fn next(&mut self) -> Option<Self::Item> {
        while self.prefix_len < self.word.len() {
            match self.node.find_input(self.word.as_bytes()[self.prefix_len]) {
                Some(trans_idx) => {
                    let trans = self.node.transition(trans_idx);
                    self.node = self.fst.node(trans.addr);
                    self.prefix_len += 1;
                }
                None => return None,
            };

            if self.node.is_final() {
                return Some(&self.word[..self.prefix_len]);
            }
        }

        None
    }
}

/// Search the longest prefix of a string in a set.
pub trait LongestPrefix {
    /// Search the longest prefix of a string in a set.
    fn longest_prefix<'a>(&self, word: &'a str) -> Option<&'a str>;
}

impl LongestPrefix for fst::Set {
    fn longest_prefix<'a>(&self, word: &'a str) -> Option<&'a str> {
        self.prefixes(word).last()
    }
}

#[cfg(test)]
mod tests {
    use fst::{Set, SetBuilder};

    use super::Prefixes;

    fn test_set() -> Set {
        let mut builder = SetBuilder::memory();
        builder
            .extend_iter(&["p", "pre", "pref", "prefix"])
            .unwrap();
        let bytes = builder.into_inner().unwrap();
        Set::from_bytes(bytes).unwrap()
    }

    #[test]
    fn finds_prefixes() {
        let set = test_set();

        let mut iter = set.prefixes("prefixes");
        assert_eq!(iter.next(), Some("p"));
        assert_eq!(iter.next(), Some("pre"));
        assert_eq!(iter.next(), Some("pref"));
        assert_eq!(iter.next(), Some("prefix"));
        assert!(iter.next().is_none());

        let mut iter = set.prefixes("pre");
        assert_eq!(iter.next(), Some("p"));
        assert_eq!(iter.next(), Some("pre"));
        assert!(iter.next().is_none());

        assert!(set.prefixes("fix").next().is_none());
    }
}
