use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash as StdHash, Hasher as StdHasher};
use std::io::{self, Write};

#[derive(Clone, Debug, PartialEq)]
pub struct HashTuning {
    pub growth_threshold: f32,
    pub growth_factor: f32,
    pub shrink_threshold: f32,
    pub shrink_factor: f32,
    pub is_n_buckets: bool,
}

impl Default for HashTuning {
    fn default() -> Self {
        Self {
            growth_threshold: 0.8,
            growth_factor: 1.414,
            shrink_threshold: 0.0,
            shrink_factor: 1.0,
            is_n_buckets: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct HashEntry<T> {
    pub data: T,
}

type Hasher<T> = Box<dyn Fn(&T, usize) -> usize>;
type Comparator<T> = Box<dyn Fn(&T, &T) -> bool>;
type DataFreer<T> = Box<dyn FnMut(T)>;

struct EntryNode<T> {
    data: T,
    next: Option<Box<EntryNode<T>>>,
}

pub struct Hash<T> {
    buckets: Vec<Option<Box<EntryNode<T>>>>,
    n_buckets_used: usize,
    n_entries: usize,
    tuning: HashTuning,
    hasher: Hasher<T>,
    comparator: Comparator<T>,
    data_freer: Option<DataFreer<T>>,
}

impl<T> Hash<T> {
    pub fn initialize(
        candidate: usize,
        tuning: Option<HashTuning>,
        hasher: Option<impl Fn(&T, usize) -> usize + 'static>,
        comparator: Option<impl Fn(&T, &T) -> bool + 'static>,
        data_freer: Option<impl FnMut(T) + 'static>,
    ) -> io::Result<Self>
    where
        T: 'static + PartialEq + StdHash,
    {
        let mut table = Self {
            buckets: Vec::new(),
            n_buckets_used: 0,
            n_entries: 0,
            tuning: tuning.unwrap_or_default(),
            hasher: match hasher {
                Some(f) => Box::new(f),
                None => Box::new(Self::raw_hasher),
            },
            comparator: match comparator {
                Some(f) => Box::new(f),
                None => Box::new(Self::raw_comparator),
            },
            data_freer: data_freer.map(|f| Box::new(f) as DataFreer<T>),
        };

        if !table.check_tuning() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid hash tuning",
            ));
        }

        let n_buckets = table.compute_bucket_size(candidate, &table.tuning)?;
        table.buckets = (0..n_buckets).map(|_| None).collect();
        Ok(table)
    }

    pub fn get_n_buckets(&self) -> usize {
        self.buckets.len()
    }

    pub fn get_n_buckets_used(&self) -> usize {
        self.n_buckets_used
    }

    pub fn get_n_entries(&self) -> usize {
        self.n_entries
    }

    pub fn get_max_bucket_length(&self) -> usize {
        self.buckets
            .iter()
            .map(|bucket| {
                let mut len = 0;
                let mut cursor = bucket.as_deref();
                while let Some(node) = cursor {
                    len += 1;
                    cursor = node.next.as_deref();
                }
                len
            })
            .max()
            .unwrap_or(0)
    }

    pub fn table_ok(&self) -> bool {
        let mut n_buckets_used = 0;
        let mut n_entries = 0;

        for bucket in &self.buckets {
            if bucket.is_some() {
                n_buckets_used += 1;
                let mut cursor = bucket.as_deref();
                while let Some(node) = cursor {
                    n_entries += 1;
                    cursor = node.next.as_deref();
                }
            }
        }

        n_buckets_used == self.n_buckets_used && n_entries == self.n_entries
    }

    pub fn print_statistics<W: Write>(&self, stream: &mut W) -> io::Result<()> {
        let n_entries = self.get_n_entries();
        let n_buckets = self.get_n_buckets();
        let n_buckets_used = self.get_n_buckets_used();
        let max_bucket_length = self.get_max_bucket_length();
        let percent = if n_buckets == 0 {
            0.0
        } else {
            (100.0 * n_buckets_used as f64) / n_buckets as f64
        };

        writeln!(stream, "# entries:         {}", n_entries)?;
        writeln!(stream, "# buckets:         {}", n_buckets)?;
        writeln!(stream, "# buckets used:    {} ({percent:.2}%)", n_buckets_used)?;
        writeln!(stream, "max bucket length: {}", max_bucket_length)?;
        Ok(())
    }

    pub fn safe_hasher(&self, key: &T) -> usize {
        let n = (self.hasher)(key, self.buckets.len());
        assert!(n < self.buckets.len(), "hasher returned out-of-range bucket");
        n
    }

    pub fn lookup(&self, entry: &T) -> Option<&T> {
        if self.buckets.is_empty() {
            return None;
        }
        let bucket = self.safe_hasher(entry);
        let mut cursor = self.buckets[bucket].as_deref();

        while let Some(node) = cursor {
            if (self.comparator)(entry, &node.data) {
                return Some(&node.data);
            }
            cursor = node.next.as_deref();
        }

        None
    }

    pub fn get_first(&self) -> Option<&T> {
        if self.n_entries == 0 {
            return None;
        }

        for bucket in &self.buckets {
            if let Some(node) = bucket.as_deref() {
                return Some(&node.data);
            }
        }

        None
    }

    pub fn get_next(&self, entry: &T) -> Option<&T> {
        if self.buckets.is_empty() {
            return None;
        }

        let bucket_index = self.safe_hasher(entry);
        let mut cursor = self.buckets[bucket_index].as_deref();

        while let Some(node) = cursor {
            if (self.comparator)(entry, &node.data) {
                if let Some(next) = node.next.as_deref() {
                    return Some(&next.data);
                }
                break;
            }
            cursor = node.next.as_deref();
        }

        for bucket in self.buckets.iter().skip(bucket_index + 1) {
            if let Some(node) = bucket.as_deref() {
                return Some(&node.data);
            }
        }

        None
    }

    pub fn get_entries(&self, buffer_size: usize) -> Vec<&T> {
        let mut out = Vec::with_capacity(buffer_size.min(self.n_entries));
        for bucket in &self.buckets {
            let mut cursor = bucket.as_deref();
            while let Some(node) = cursor {
                if out.len() >= buffer_size {
                    return out;
                }
                out.push(&node.data);
                cursor = node.next.as_deref();
            }
        }
        out
    }

    pub fn do_for_each<F>(&self, mut processor: F) -> usize
    where
        F: FnMut(&T) -> bool,
    {
        let mut counter = 0;
        for bucket in &self.buckets {
            let mut cursor = bucket.as_deref();
            while let Some(node) = cursor {
                if !processor(&node.data) {
                    return counter;
                }
                counter += 1;
                cursor = node.next.as_deref();
            }
        }
        counter
    }

    pub fn string(string: &str, n_buckets: usize) -> usize {
        let mut value = 0usize;
        for ch in string.bytes() {
            value = ch as usize + value.rotate_left(7);
        }
        value % n_buckets
    }

    pub fn gl_attribute_const() -> bool {
        true
    }

    pub fn reset_tuning(tuning: &mut HashTuning) {
        *tuning = HashTuning::default();
    }

    pub fn raw_hasher(data: &T, n: usize) -> usize
    where
        T: StdHash,
    {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        (hasher.finish() as usize) % n
    }

    pub fn raw_comparator(a: &T, b: &T) -> bool
    where
        T: PartialEq,
    {
        a == b
    }

    pub fn check_tuning(&mut self) -> bool {
        let epsilon = 0.1f32;
        let t = &self.tuning;

        if *t == HashTuning::default() {
            return true;
        }

        if epsilon < t.growth_threshold
            && t.growth_threshold < 1.0 - epsilon
            && 1.0 + epsilon < t.growth_factor
            && 0.0 <= t.shrink_threshold
            && t.shrink_threshold + epsilon < t.shrink_factor
            && t.shrink_factor <= 1.0
            && t.shrink_threshold + epsilon < t.growth_threshold
        {
            true
        } else {
            self.tuning = HashTuning::default();
            false
        }
    }

    pub fn compute_bucket_size(
        &self,
        mut candidate: usize,
        tuning: &HashTuning,
    ) -> io::Result<usize> {
        if !tuning.is_n_buckets {
            let new_candidate = candidate as f32 / tuning.growth_threshold;
            if new_candidate >= usize::MAX as f32 {
                return Err(io::Error::new(io::ErrorKind::OutOfMemory, "bucket size overflow"));
            }
            candidate = new_candidate as usize;
        }

        let candidate = Self::next_prime(candidate);
        let _ = candidate
            .checked_mul(std::mem::size_of::<Option<Box<EntryNode<T>>>>())
            .ok_or_else(|| io::Error::new(io::ErrorKind::OutOfMemory, "allocation overflow"))?;
        Ok(candidate)
    }

    pub fn clear(&mut self) {
        for bucket in &mut self.buckets {
            let mut head = bucket.take();
            while let Some(mut node) = head {
                let next = node.next.take();
                if let Some(freer) = self.data_freer.as_mut() {
                    freer(node.data);
                }
                head = next;
            }
        }
        self.n_buckets_used = 0;
        self.n_entries = 0;
    }

    pub fn allocate_entry(&mut self, data: T) -> HashEntry<T> {
        HashEntry { data }
    }

    pub fn free_entry(&mut self, entry: HashEntry<T>) {
        if let Some(freer) = self.data_freer.as_mut() {
            freer(entry.data);
        }
    }

    pub fn find_entry(&self, entry: &T) -> Option<(usize, usize)> {
        if self.buckets.is_empty() {
            return None;
        }

        let bucket_index = self.safe_hasher(entry);
        let mut cursor = self.buckets[bucket_index].as_deref();
        let mut depth = 0;

        while let Some(node) = cursor {
            if (self.comparator)(entry, &node.data) {
                return Some((bucket_index, depth));
            }
            depth += 1;
            cursor = node.next.as_deref();
        }

        None
    }

    pub fn transfer_entries(&mut self, src: &mut Self, _safe: bool) -> io::Result<()>
    where
        T: 'static + PartialEq + StdHash,
    {
        let mut values = Vec::with_capacity(src.n_entries);
        for bucket in &mut src.buckets {
            let mut head = bucket.take();
            while let Some(mut node) = head {
                let next = node.next.take();
                values.push(node.data);
                head = next;
            }
        }
        src.n_buckets_used = 0;
        src.n_entries = 0;

        for value in values {
            self.insert(value)
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "transfer failed"))?;
        }

        Ok(())
    }

    pub fn rehash(&mut self, candidate: usize) -> bool
    where
        T: 'static + PartialEq + StdHash,
    {
        let Ok(new_size) = self.compute_bucket_size(candidate, &self.tuning) else {
            return false;
        };

        if new_size == self.buckets.len() {
            return true;
        }

        let mut new_table = Hash {
            buckets: (0..new_size).map(|_| None).collect(),
            n_buckets_used: 0,
            n_entries: 0,
            tuning: self.tuning.clone(),
            hasher: std::mem::replace(&mut self.hasher, Box::new(Self::raw_hasher)),
            comparator: std::mem::replace(&mut self.comparator, Box::new(Self::raw_comparator)),
            data_freer: self.data_freer.take(),
        };

        let mut values = Vec::with_capacity(self.n_entries);
        for bucket in &mut self.buckets {
            let mut head = bucket.take();
            while let Some(mut node) = head {
                let next = node.next.take();
                values.push(node.data);
                head = next;
            }
        }

        let old_entries = self.n_entries;
        self.n_buckets_used = 0;
        self.n_entries = 0;

        for value in values {
            if new_table.insert(value).is_none() {
                self.hasher = std::mem::replace(&mut new_table.hasher, Box::new(Self::raw_hasher));
                self.comparator =
                    std::mem::replace(&mut new_table.comparator, Box::new(Self::raw_comparator));
                self.data_freer = new_table.data_freer.take();
                return false;
            }
        }

        new_table.n_entries = old_entries;
        *self = new_table;
        true
    }

    pub fn insert_if_absent(&mut self, entry: T) -> Result<bool, T>
    where
        T: 'static + PartialEq + StdHash,
    {
        assert!(self.buckets.len() > 0, "uninitialized hash table");

        if self.lookup(&entry).is_some() {
            return Ok(false);
        }

        if self.n_buckets_used as f32 > self.tuning.growth_threshold * self.buckets.len() as f32 {
            self.check_tuning();
            if self.n_buckets_used as f32 > self.tuning.growth_threshold * self.buckets.len() as f32
            {
                let candidate = if self.tuning.is_n_buckets {
                    self.buckets.len() as f32 * self.tuning.growth_factor
                } else {
                    self.buckets.len() as f32
                        * self.tuning.growth_factor
                        * self.tuning.growth_threshold
                };

                if candidate >= usize::MAX as f32 || !self.rehash(candidate as usize) {
                    return Err(entry);
                }
            }
        }

        let idx = self.safe_hasher(&entry);
        match self.buckets[idx].as_mut() {
            Some(head) => {
                let next = head.next.take();
                head.next = Some(Box::new(EntryNode { data: entry, next }));
                self.n_entries += 1;
            }
            None => {
                self.buckets[idx] = Some(Box::new(EntryNode {
                    data: entry,
                    next: None,
                }));
                self.n_entries += 1;
                self.n_buckets_used += 1;
            }
        }

        Ok(true)
    }

    pub fn insert(&mut self, entry: T) -> Option<&T>
    where
        T: 'static + PartialEq + StdHash,
    {
        let inserted = self.insert_if_absent(entry).ok()?;
        if !inserted {
            return self.get_first().and_then(|_| None);
        }
        let last_bucket = self
            .buckets
            .iter()
            .enumerate()
            .find_map(|(i, bucket)| bucket.as_ref().map(|_| i))?;
        let mut cursor = self.buckets[last_bucket].as_deref();
        let mut last = None;
        while let Some(node) = cursor {
            last = Some(&node.data);
            cursor = node.next.as_deref();
        }
        last
    }

    pub fn remove(&mut self, entry: &T) -> Option<T>
    where
        T: 'static + PartialEq + StdHash,
    {
        if self.buckets.is_empty() {
            return None;
        }

        let idx = self.safe_hasher(entry);
        let mut head = self.buckets[idx].take()?;
        if (self.comparator)(entry, &head.data) {
            self.n_entries -= 1;
            if let Some(next) = head.next.take() {
                self.buckets[idx] = Some(next);
            } else {
                self.buckets[idx] = None;
                self.n_buckets_used -= 1;

                if self.n_buckets_used > 0
                    && (self.n_buckets_used as f32)
                        < self.tuning.shrink_threshold * self.buckets.len() as f32
                {
                    self.check_tuning();
                    if (self.n_buckets_used as f32)
                        < self.tuning.shrink_threshold * self.buckets.len() as f32
                    {
                        let candidate = if self.tuning.is_n_buckets {
                            (self.buckets.len() as f32 * self.tuning.shrink_factor) as usize
                        } else {
                            (self.buckets.len() as f32
                                * self.tuning.shrink_factor
                                * self.tuning.growth_threshold)
                                as usize
                        };
                        let _ = self.rehash(candidate);
                    }
                }
            }
            return Some(head.data);
        }

        let mut current = &mut head;
        loop {
            let Some(mut next) = current.next.take() else {
                self.buckets[idx] = Some(head);
                return None;
            };

            if (self.comparator)(entry, &next.data) {
                current.next = next.next.take();
                self.buckets[idx] = Some(head);
                self.n_entries -= 1;
                return Some(next.data);
            }

            current.next = Some(next);
            current = current.next.as_mut().expect("relinked node");
        }
    }

    pub fn print(&self)
    where
        T: fmt::Display,
    {
        for (i, bucket) in self.buckets.iter().enumerate() {
            println!("{i}:");
            let mut cursor = bucket.as_deref();
            while let Some(node) = cursor {
                println!("  {}", node.data);
                cursor = node.next.as_deref();
            }
        }
    }

    pub fn is_prime(candidate: usize) -> bool {
        let mut divisor = 3usize;
        let mut square = divisor * divisor;

        while square < candidate && !candidate.is_multiple_of(divisor) {
            divisor += 1;
            square += 4 * divisor;
            divisor += 1;
        }

        !candidate.is_multiple_of(divisor)
    }

    pub fn next_prime(mut candidate: usize) -> usize {
        if candidate < 10 {
            candidate = 10;
        }
        candidate |= 1;
        while candidate != usize::MAX && !Self::is_prime(candidate) {
            candidate = candidate.saturating_add(2);
        }
        candidate
    }

    pub fn get_13() -> usize {
        13
    }

    pub fn entry(data: T) -> HashEntry<T> {
        HashEntry { data }
    }

    pub fn table(
        candidate: usize,
        tuning: Option<HashTuning>,
        hasher: Option<impl Fn(&T, usize) -> usize + 'static>,
        comparator: Option<impl Fn(&T, &T) -> bool + 'static>,
        data_freer: Option<impl FnMut(T) + 'static>,
    ) -> io::Result<Self>
    where
        T: 'static + PartialEq + StdHash,
    {
        Self::initialize(candidate, tuning, hasher, comparator, data_freer)
    }

    pub fn entry_01(data: T) -> HashEntry<T> {
        HashEntry { data }
    }

    pub fn entry_02(data: T) -> HashEntry<T> {
        HashEntry { data }
    }

    pub fn string_17(string: &str) -> usize {
        Self::string(string, 17)
    }
}

impl<T> Drop for Hash<T> {
    fn drop(&mut self) {
        self.clear();
    }
}
