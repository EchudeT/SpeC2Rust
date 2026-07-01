use std::collections::VecDeque;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LinkedList<T> {
    items: VecDeque<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: VecDeque::with_capacity(capacity),
        }
    }

    pub fn append(&mut self, value: T) {
        self.items.push_back(value);
    }

    pub fn prepend(&mut self, value: T) {
        self.items.push_front(value);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn size(&self) -> usize {
        self.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.items.contains(value)
    }

    pub fn front(&self) -> Option<&T> {
        self.items.front()
    }

    pub fn back(&self) -> Option<&T> {
        self.items.back()
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.items.front_mut()
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.items.back_mut()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.items.pop_back()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.items.iter_mut()
    }

    pub fn unlink_first(&mut self, predicate: impl FnMut(&T) -> bool) -> Option<T> {
        let index = self.items.iter().position(predicate)?;
        self.items.remove(index)
    }

    pub fn any(&self, predicate: impl FnMut(&T) -> bool) -> bool {
        self.items.iter().any(predicate)
    }

    pub fn iterate<F>(&mut self, mut callback: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut index = 0;
        while index < self.items.len() {
            let remove = {
                let item = &self.items[index];
                callback(item)
            };

            if remove {
                self.items.remove(index);
            } else {
                index += 1;
            }
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        self.items.into_iter().collect()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = std::collections::vec_deque::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = std::collections::vec_deque::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = std::collections::vec_deque::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.items.extend(iter);
    }
}
