use std::cell::RefCell;
use std::hash::Hash;

use super::Numberer;

/// Trait for feature/label index lookup.
pub trait Lookup<T> {
    /// Lookup a feature index.
    fn lookup(&self, feature: &T) -> Option<usize>;

    /// Null value.
    fn null(&self) -> usize;
}

pub struct MutableLookupTable<T>
where
    T: Eq + Hash,
{
    numberer: RefCell<Numberer<T>>,
}

impl<T> MutableLookupTable<T>
where
    T: Clone + Eq + Hash,
{
    pub fn new() -> Self {
        MutableLookupTable {
            numberer: RefCell::new(Numberer::new(1)),
        }
    }
}

impl<T> Lookup<T> for MutableLookupTable<T>
where
    T: Clone + Eq + Hash,
{
    fn lookup(&self, feature: &T) -> Option<usize> {
        let mut numberer = self.numberer.borrow_mut();
        Some(numberer.add(feature.to_owned()))
    }

    fn null(&self) -> usize {
        0
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub struct LookupTable<T>
where
    T: Eq + Hash,
{
    numberer: Numberer<T>,
}

impl<T> Lookup<T> for LookupTable<T>
where
    T: Clone + Eq + Hash,
{
    fn lookup(&self, feature: &T) -> Option<usize> {
        self.numberer.number(feature)
    }

    fn null(&self) -> usize {
        0
    }
}

impl<T> From<MutableLookupTable<T>> for LookupTable<T>
where
    T: Eq + Hash,
{
    fn from(t: MutableLookupTable<T>) -> Self {
        LookupTable {
            numberer: t.numberer.into_inner(),
        }
    }
}

pub struct BoxedLookup<T>(Option<Box<Lookup<T>>>);

impl<T> BoxedLookup<T> {
    /// Construct a boxed lookup from a lookup.
    pub fn new<L>(lookup: L) -> Self
    where
        L: Into<Box<Lookup<T>>>,
    {
        BoxedLookup(Some(lookup.into()))
    }

    /// Get the lookup as a reference.
    pub fn as_ref(&self) -> Option<&Lookup<T>> {
        self.0.as_ref().map(AsRef::as_ref)
    }
}

impl<T> Default for BoxedLookup<T> {
    fn default() -> Self {
        BoxedLookup(None)
    }
}

#[cfg(test)]
mod tests {
    use super::{Lookup, LookupTable, MutableLookupTable};

    #[test]
    fn mutable_lookup_table_test() {
        let table = MutableLookupTable::new();
        assert_eq!(table.lookup("a"), Some(1));
        assert_eq!(table.lookup("b"), Some(2));
        assert_eq!(table.lookup("a"), Some(1));
        assert_eq!(table.null(), 0);
    }

    #[test]
    fn lookup_table_test() {
        let table = MutableLookupTable::new();
        assert_eq!(table.lookup("a"), Some(1));
        assert_eq!(table.lookup("a"), Some(1));
        assert_eq!(table.lookup("b"), Some(2));

        let table: LookupTable = table.into();
        assert_eq!(table.lookup("a"), Some(1));
        assert_eq!(table.lookup("a"), Some(1));
        assert_eq!(table.lookup("b"), Some(2));
        assert_eq!(table.lookup("c"), None);
        assert_eq!(table.null(), 0);
    }
}
