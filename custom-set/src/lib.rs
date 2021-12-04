use std::iter::FromIterator;

#[derive(Debug)]
pub struct CustomSet<T> {
    inner: Vec<T>,
}

impl<T: PartialEq + Clone> CustomSet<T> {
    pub fn new(input: &[T]) -> Self
    where
        T: Clone,
    {
        CustomSet {
            inner: input.to_vec(), 
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.inner.iter().find(|&x| x == element).is_some()
    }

    pub fn add(&mut self, element: T) {
        if !self.inner.contains(&element) {
            self.inner.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
       self.inner.iter().filter(|&x| other.contains(x)).count() == self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.inner.iter().filter(|&x| !other.contains(x)).count() == self.inner.len()
    }

    pub fn intersection(&self, other: &Self) -> Self {
        self.inner.iter().filter(|&x| other.contains(x)).cloned().collect()
    }

    pub fn difference(&self, other: &Self) -> Self {
        self.inner.iter().filter(|&x| !other.contains(x)).cloned().collect()
    }

    pub fn union(&self, other: &Self) -> Self {
        self.inner.iter().cloned().fold(Self::new(&other.inner), |mut acc, val| {
            acc.add(val);
            acc
        })
    }
}

impl<T: PartialEq + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.len() == other.inner.len() && self.inner.iter().find(|&x| !other.contains(x)).is_none()
    }
}

impl<T> FromIterator<T> for CustomSet<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item=T>,
    {
        CustomSet {
            inner: iter.into_iter().collect(),
        }
    }
}
