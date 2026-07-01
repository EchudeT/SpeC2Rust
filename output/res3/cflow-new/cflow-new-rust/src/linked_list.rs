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

    pub fn unlink_at(&mut self, index: usize) -> Option<T> {
        self.items.remove(index)
    }

    pub fn unlink_first<P>(&mut self, predicate: P) -> Option<T>
    where
        P: FnMut(&T) -> bool,
    {
        let index = self.items.iter().position(predicate)?;
        self.items.remove(index)
    }

    pub fn contains_data(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.items.contains(value)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn front(&self) -> Option<&T> {
        self.items.front()
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.items.front_mut()
    }

    pub fn back(&self) -> Option<&T> {
        self.items.back()
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.items.back_mut()
    }

    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, T> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> std::collections::vec_deque::IterMut<'_, T> {
        self.items.iter_mut()
    }

    pub fn iterate_while<F>(&mut self, mut visitor: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut index = 0;
        while index < self.items.len() {
            let keep_going = {
                let item = &self.items[index];
                visitor(item)
            };
            if !keep_going {
                break;
            }
            index += 1;
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
