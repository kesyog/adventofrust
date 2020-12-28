use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::From;
use std::hash::Hash;
use std::iter::{FromIterator, IntoIterator};
use std::ops::{Add, AddAssign, Index};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Counter<T>
where
    T: Hash + Eq,
{
    map: HashMap<T, usize>,
}

impl<T> Counter<T>
where
    T: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&usize>
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.get(key)
    }

    /// Increment count for `key` by 1
    pub fn add(&mut self, key: T) {
        *self.map.entry(key).or_insert(0) += 1;
    }

    /// Increment count for `key` by n
    pub fn add_n(&mut self, key: T, n: usize) {
        *self.map.entry(key).or_insert(0) += n;
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, T, usize> {
        self.map.iter()
    }

    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, T, usize> {
        self.map.keys()
    }

    pub fn find_max_count(&self) -> Option<(&T, &usize)> {
        self.iter().max_by_key(|(_, &v)| v)
    }
}

impl<T> FromIterator<T> for Counter<T>
where
    T: Hash + Eq,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut counter = Self::new();
        for item in iter {
            counter.add(item);
        }
        counter
    }
}

impl<T> From<Counter<T>> for HashMap<T, usize>
where
    T: Hash + Eq,
{
    fn from(counter: Counter<T>) -> Self {
        counter.map
    }
}

impl<T> Index<T> for Counter<T>
where
    T: Hash + Eq,
{
    type Output = usize;

    fn index(&self, index: T) -> &Self::Output {
        self.map.index(&index)
    }
}

impl<T> Add<Counter<T>> for Counter<T>
where
    T: Hash + Eq + Clone + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut sum = self;
        sum += &other;
        sum
    }
}

impl<T> Add<&Counter<T>> for Counter<T>
where
    T: Hash + Eq + Clone + Copy,
{
    type Output = Self;

    fn add(self, other: &Self) -> Self::Output {
        let mut sum = self;
        sum += other;
        sum
    }
}

impl<T> AddAssign<Counter<T>> for Counter<T>
where
    T: Hash + Eq + Clone + Copy,
{
    fn add_assign(&mut self, other: Self) {
        for (k, v) in other.iter() {
            // TODO: remove dereference and Copy trait bound
            self.add_n(*k, *v);
        }
    }
}

impl<T> AddAssign<&Counter<T>> for Counter<T>
where
    T: Hash + Eq + Clone + Copy,
{
    fn add_assign(&mut self, other: &Self) {
        for (k, v) in other.iter() {
            // TODO: remove dereference and Copy trait bound
            self.add_n(*k, *v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all_integers() {
        let elements: Vec<u32> = vec![0, 1, 3, 1, 4, 4, 4];
        let counter = Counter::from_iter(elements);
        assert_eq!(counter.get(&0_u32), Some(&1));
        assert_eq!(counter.get(&1_u32), Some(&2));
        assert_eq!(counter.get(&3_u32), Some(&1));
        assert_eq!(counter.get(&4_u32), Some(&3));
        assert_eq!(counter.get(&5_u32), None);
    }
}
