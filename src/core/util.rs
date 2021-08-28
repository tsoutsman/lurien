use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// A simple wrapper type that ensures that the vector we are using is sorted.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct SortedVec<T: Ord>(Vec<T>);

impl<T: Ord> std::iter::IntoIterator for SortedVec<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Ord> SortedVec<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn try_push(&mut self, value: T) -> Result<()> {
        let last_item = match self.0.last() {
            Some(l) => l,
            // The vector is empty and so it is safe to push to.
            None => {
                self.0.push(value);
                return Ok(());
            }
        };
        if value >= *last_item {
            self.0.push(value);
            Ok(())
        } else {
            Err(Error::NotSorted)
        }
    }
}

impl<T: Ord> std::ops::Deref for SortedVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Ord> std::convert::From<SortedVec<T>> for Vec<T> {
    fn from(sv: SortedVec<T>) -> Self {
        sv.0
    }
}

impl<T: Ord> std::convert::From<Vec<T>> for SortedVec<T> {
    fn from(mut v: Vec<T>) -> Self {
        v.sort();
        Self(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_vec_try_push() {
        let mut sv = SortedVec::new();

        sv.try_push(0).unwrap();
        sv.try_push(1).unwrap();
        sv.try_push(5).unwrap();

        match sv.try_push(3) {
            Err(Error::NotSorted) => {}
            _ => panic!(),
        }
    }

    #[test]
    fn test_sorted_vec_traits() {
        // Tests `From<Vec<T>> for SortedVec<T>`.
        let sv = SortedVec::from(vec![1, 5, 3]);

        // Tests `From<Vec<T>> for SortedVec<T>` and `Deref for SortedVec<T>`.
        assert_eq!(*sv, vec![1, 3, 5]);

        // Tests `From<SortedVec<T>> for Vec<T>`.
        let v: Vec<i32> = sv.into();
        assert_eq!(v, vec![1, 3, 5]);
    }
}
