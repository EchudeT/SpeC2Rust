use std::fmt;
use std::io::{self, Write};

#[derive(Clone, Debug, PartialEq)]
pub struct Tuning {
    pub shrink_threshold: f32,
    pub shrink_factor: f32,
    pub growth_threshold: f32,
    pub growth_factor: f32,
    pub is_n_buckets: bool,
}

impl Default for Tuning {
    fn default() -> Self {
        Self {
            shrink_threshold: 0.0,
            shrink_factor: 1.0,
            growth_threshold: 0.8,
            growth_factor: 1.414,
            is_n_buckets: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry<T> {
    pub data: T,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry01<T> {
    pub data: T,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry02<T> {
    pub data: T,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Table {
    pub n_buckets: usize,
    pub n_buckets_used: usize,
    pub n_entries: usize,
}

#[derive(Clone, Debug)]
struct EntryNode<T> {
    data: T,
    next: Option<Box<EntryNode<T>>>,
}

#[derive(Clone)]
struct Bucket<T> {
    head: Option<T>,
    overflow: Option<Box<EntryNode<T>>>,
}

impl<T> Default for Bucket<T> {
    fn default() -> Self {
        Self {
            head: None,
            overflow: None,
        }
    }
}

type Hasher<T> = Box<dyn Fn(&T, usize) -> usize>;
type Comparator<T> = Box<dyn Fn(&T, &T) -> bool>;
type DataFreer<T> = Box<dyn FnMut(T)>;

pub struct Hash<T> {
    buckets: Vec<Bucket<T>>,
    tuning: Tuning,
    hasher: Hasher<T>,
    comparator: Comparator<T>,
    data_freer: Option<DataFreer<T>>,
    n_buckets_used: usize,
    n_entries: usize,
}

impl<T> Hash<T> {
    pub fn initialize<FH, FC>(
        candidate: usize,
        tuning: Option<Tuning>,
        hasher: Option<FH>,
        comparator: Option<FC>,
        data_freer: Option<DataFreer<T>>,
    ) -> Result<Self, HashError>
    where
        FH: Fn(&T, usize) -> usize + 'static,
        FC: Fn(&T, &T) -> bool + 'static,
    {
        let mut effective_tuning = tuning.unwrap_or_default();
        if !Self::check_tuning(&mut effective_tuning) {
            return Err(HashError::InvalidTuning);
        }

        let n_buckets =
            Self::compute_bucket_size(candidate, &effective_tuning).ok_or(HashError::OutOfMemory)?;
        let mut buckets = Vec::with_capacity(n_buckets);
        buckets.resize_with(n_buckets, Bucket::default);

        let hasher: Hasher<T> = match hasher {
            Some(f) => Box::new(f),
            None => Box::new(|_, _| 0),
        };

        let comparator: Comparator<T> = match comparator {
            Some(f) => Box::new(f),
            None => Box::new(|_, _| false),
        };

        Ok(Self {
            buckets,
            tuning: effective_tuning,
            hasher,
            comparator,
            data_freer,
            n_buckets_used: 0,
            n_entries: 0,
        })
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
        let mut max_bucket_length = 0;

        for bucket in &self.buckets {
            if bucket.head.is_some() {
                let mut bucket_length = 1;
                let mut cursor = bucket.overflow.as_deref();
                while let Some(node) = cursor {
                    bucket_length += 1;
                    cursor = node.next.as_deref();
                }
                max_bucket_length = max_bucket_length.max(bucket_length);
            }
        }

        max_bucket_length
    }

    pub fn table_ok(&self) -> bool {
        let mut n_buckets_used = 0;
        let mut n_entries = 0;

        for bucket in &self.buckets {
            if bucket.head.is_some() {
                n_buckets_used += 1;
                n_entries += 1;
                let mut cursor = bucket.overflow.as_deref();
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
        let used_pct = if n_buckets == 0 {
            0.0
        } else {
            (100.0 * n_buckets_used as f64) / n_buckets as f64
        };

        writeln!(stream, "# entries:         {}", n_entries)?;
        writeln!(stream, "# buckets:         {}", n_buckets)?;
        writeln!(
            stream,
            "# buckets used:    {} ({:.2}%)",
            n_buckets_used, used_pct
        )?;
        writeln!(stream, "max bucket length: {}", max_bucket_length)?;
        Ok(())
    }

    pub fn safe_hasher(&self, key: &T) -> usize {
        let len = self.buckets.len();
        if len == 0 {
            return 0;
        }
        (self.hasher)(key, len) % len
    }

    pub fn lookup(&self, entry: &T) -> Option<&T> {
        let bucket = &self.buckets[self.safe_hasher(entry)];
        let head = bucket.head.as_ref()?;
        if (self.comparator)(entry, head) {
            return Some(head);
        }
        let mut cursor = bucket.overflow.as_deref();
        while let Some(node) = cursor {
            if (self.comparator)(entry, &node.data) {
                return Some(&node.data);
            }
            cursor = node.next.as_deref();
        }
        None
    }

    pub fn get_first(&self) -> Option<&T> {
        for bucket in &self.buckets {
            if let Some(data) = bucket.head.as_ref() {
                return Some(data);
            }
        }
        None
    }

    pub fn get_next(&self, entry: &T) -> Option<&T> {
        let index = self.safe_hasher(entry);
        let bucket = &self.buckets[index];

        if let Some(head) = bucket.head.as_ref() {
            if (self.comparator)(entry, head) {
                return bucket.overflow.as_deref().map(|node| &node.data);
            }
        }

        let mut cursor = bucket.overflow.as_deref();
        while let Some(node) = cursor {
            if (self.comparator)(entry, &node.data) {
                return node.next.as_deref().map(|next| &next.data);
            }
            cursor = node.next.as_deref();
        }

        for bucket in self.buckets.iter().skip(index + 1) {
            if let Some(data) = bucket.head.as_ref() {
                return Some(data);
            }
        }

        None
    }

    pub fn get_entries(&self, buffer_size: usize) -> Vec<&T> {
        let mut out = Vec::with_capacity(buffer_size.min(self.n_entries));
        for bucket in &self.buckets {
            if let Some(head) = bucket.head.as_ref() {
                if out.len() >= buffer_size {
                    break;
                }
                out.push(head);

                let mut cursor = bucket.overflow.as_deref();
                while let Some(node) = cursor {
                    if out.len() >= buffer_size {
                        return out;
                    }
                    out.push(&node.data);
                    cursor = node.next.as_deref();
                }
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
            if let Some(head) = bucket.head.as_ref() {
                if !processor(head) {
                    return counter;
                }
                counter += 1;

                let mut cursor = bucket.overflow.as_deref();
                while let Some(node) = cursor {
                    if !processor(&node.data) {
                        return counter;
                    }
                    counter += 1;
                    cursor = node.next.as_deref();
                }
            }
        }
        counter
    }

    pub fn string(string: &str, n_buckets: usize) -> usize {
        if n_buckets == 0 {
            return 0;
        }

        let mut value = 0usize;
        for ch in string.bytes() {
            value = value.wrapping_mul(31).wrapping_add(ch as usize) % n_buckets;
        }
        value
    }

    pub fn gl_attribute_const() {}

    pub fn reset_tuning(tuning: &mut Tuning) {
        *tuning = Tuning::default();
    }

    pub fn raw_hasher(value: usize, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            value.rotate_right(3) % n
        }
    }

    pub fn raw_comparator<A: PartialEq>(a: &A, b: &A) -> bool {
        a == b
    }

    pub fn check_tuning(tuning: &mut Tuning) -> bool {
        let default = Tuning::default();
        if *tuning == default {
            return true;
        }

        let epsilon = 0.1f32;
        let ok = epsilon < tuning.growth_threshold
            && tuning.growth_threshold < 1.0 - epsilon
            && 1.0 + epsilon < tuning.growth_factor
            && 0.0 <= tuning.shrink_threshold
            && tuning.shrink_threshold + epsilon < tuning.shrink_factor
            && tuning.shrink_factor <= 1.0
            && tuning.shrink_threshold + epsilon < tuning.growth_threshold;

        if ok {
            true
        } else {
            *tuning = default;
            false
        }
    }

    pub fn compute_bucket_size(candidate: usize, tuning: &Tuning) -> Option<usize> {
        let mut candidate = candidate;
        if !tuning.is_n_buckets {
            let new_candidate = candidate as f64 / tuning.growth_threshold as f64;
            if new_candidate >= usize::MAX as f64 {
                return None;
            }
            candidate = new_candidate as usize;
        }
        let candidate = Self::next_prime(candidate);
        let elem = std::mem::size_of::<Bucket<T>>();
        candidate.checked_mul(elem).map(|_| candidate)
    }

    pub fn clear(&mut self) {
        for bucket in &mut self.buckets {
            let mut overflow = bucket.overflow.take();
            while let Some(mut node) = overflow {
                if let Some(f) = self.data_freer.as_mut() {
                    f(node.data);
                }
                overflow = node.next.take();
            }

            if let Some(data) = bucket.head.take() {
                if let Some(f) = self.data_freer.as_mut() {
                    f(data);
                }
            }
        }

        self.n_buckets_used = 0;
        self.n_entries = 0;
    }

    pub fn allocate_entry(&mut self, data: T) -> Entry<T> {
        Entry { data }
    }

    pub fn free_entry(&mut self, entry: Entry<T>) {
        if let Some(f) = self.data_freer.as_mut() {
            f(entry.data);
        }
    }

    pub fn find_entry(&mut self, entry: &T, delete: bool) -> Option<T>
    where
        T: Clone,
    {
        if delete {
            self.remove(entry)
        } else {
            self.lookup(entry).cloned()
        }
    }

    pub fn transfer_entries(&mut self, source: &mut Self, safe: bool) -> bool
    where
        T: Clone,
    {
        let entries: Vec<T> = source
            .get_entries(source.n_entries)
            .into_iter()
            .cloned()
            .collect();

        for entry in entries {
            let bucket_index = self.safe_hasher(&entry);
            let bucket = &mut self.buckets[bucket_index];

            if bucket.head.is_some() {
                bucket.overflow = Some(Box::new(EntryNode {
                    data: entry,
                    next: bucket.overflow.take(),
                }));
            } else {
                bucket.head = Some(entry);
                self.n_buckets_used += 1;
            }
            self.n_entries += 1;
        }

        if !safe {
            source.clear();
        }

        true
    }

    pub fn rehash(&mut self, candidate: usize) -> bool
    where
        T: Clone,
    {
        let new_size = match Self::compute_bucket_size(candidate, &self.tuning) {
            Some(size) => size,
            None => return false,
        };

        if new_size == self.buckets.len() {
            return true;
        }

        let entries: Vec<T> = self
            .get_entries(self.n_entries)
            .into_iter()
            .cloned()
            .collect();

        let mut new_buckets = Vec::with_capacity(new_size);
        new_buckets.resize_with(new_size, Bucket::default);
        self.buckets = new_buckets;
        self.n_buckets_used = 0;
        self.n_entries = 0;

        for entry in entries {
            let bucket_index = self.safe_hasher(&entry);
            let bucket = &mut self.buckets[bucket_index];
            if bucket.head.is_some() {
                bucket.overflow = Some(Box::new(EntryNode {
                    data: entry,
                    next: bucket.overflow.take(),
                }));
            } else {
                bucket.head = Some(entry);
                self.n_buckets_used += 1;
            }
            self.n_entries += 1;
        }

        true
    }

    pub fn insert_if_absent(&mut self, entry: T) -> Result<Option<T>, HashError>
    where
        T: Clone,
    {
        if let Some(existing) = self.lookup(&entry).cloned() {
            return Ok(Some(existing));
        }

        if (self.n_buckets_used as f32) > self.tuning.growth_threshold * self.buckets.len() as f32 {
            let mut tuning = self.tuning.clone();
            let _ = Self::check_tuning(&mut tuning);
            self.tuning = tuning;

            if (self.n_buckets_used as f32)
                > self.tuning.growth_threshold * self.buckets.len() as f32
            {
                let candidate = if self.tuning.is_n_buckets {
                    self.buckets.len() as f32 * self.tuning.growth_factor
                } else {
                    self.buckets.len() as f32
                        * self.tuning.growth_factor
                        * self.tuning.growth_threshold
                };

                if candidate >= usize::MAX as f32 {
                    return Err(HashError::OutOfMemory);
                }

                if !self.rehash(candidate as usize) {
                    return Err(HashError::OutOfMemory);
                }
            }
        }

        let bucket_index = self.safe_hasher(&entry);
        let bucket = &mut self.buckets[bucket_index];

        if bucket.head.is_some() {
            bucket.overflow = Some(Box::new(EntryNode {
                data: entry,
                next: bucket.overflow.take(),
            }));
            self.n_entries += 1;
        } else {
            bucket.head = Some(entry);
            self.n_entries += 1;
            self.n_buckets_used += 1;
        }

        Ok(None)
    }

    pub fn insert(&mut self, entry: T) -> Option<T>
    where
        T: Clone,
    {
        match self.insert_if_absent(entry.clone()) {
            Err(_) => None,
            Ok(Some(existing)) => Some(existing),
            Ok(None) => Some(entry),
        }
    }

    pub fn remove(&mut self, entry: &T) -> Option<T>
    where
        T: Clone,
    {
        let bucket_index = self.safe_hasher(entry);
        let bucket = &mut self.buckets[bucket_index];

        let head_matches = bucket
            .head
            .as_ref()
            .map(|head| (self.comparator)(entry, head))
            .unwrap_or(false);

        if head_matches {
            let data = bucket.head.as_ref().cloned()?;
            if let Some(mut next) = bucket.overflow.take() {
                bucket.head = Some(next.data);
                bucket.overflow = next.next.take();
            } else {
                bucket.head = None;
                self.n_buckets_used -= 1;
            }
            self.n_entries -= 1;
            return Some(data);
        }

        let mut link = &mut bucket.overflow;
        loop {
            let remove_here = match link.as_ref() {
                Some(node) => (self.comparator)(entry, &node.data),
                None => return None,
            };

            if remove_here {
                let mut removed = link.take().expect("node exists");
                let data = removed.data;
                *link = removed.next.take();
                self.n_entries -= 1;
                break Some(data);
            }

            link = &mut link.as_mut().expect("node exists").next;
        }
    }

    pub fn print<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
        T: fmt::Display,
    {
        for (i, bucket) in self.buckets.iter().enumerate() {
            writeln!(writer, "{}:", i)?;
            if let Some(head) = bucket.head.as_ref() {
                writeln!(writer, "  {}", head)?;
            }
            let mut cursor = bucket.overflow.as_deref();
            while let Some(node) = cursor {
                writeln!(writer, "  {}", node.data)?;
                cursor = node.next.as_deref();
            }
        }
        Ok(())
    }

    pub fn is_prime(candidate: usize) -> bool {
        if candidate < 2 {
            return false;
        }
        if candidate == 2 {
            return true;
        }
        if candidate.is_multiple_of(2) {
            return false;
        }

        let mut divisor = 3usize;
        while divisor <= candidate / divisor {
            if candidate.is_multiple_of(divisor) {
                return false;
            }
            divisor += 2;
        }
        true
    }

    pub fn next_prime(mut candidate: usize) -> usize {
        if candidate <= 2 {
            return 2;
        }
        if candidate < 10 {
            candidate = 10;
        }
        if candidate.is_multiple_of(2) {
            candidate += 1;
        }

        while candidate != usize::MAX && !Self::is_prime(candidate) {
            candidate = candidate.saturating_add(2);
        }

        candidate
    }

    pub fn get_13(&self) -> usize {
        13
    }

    pub fn table(&self) -> Table {
        Table {
            n_buckets: self.get_n_buckets(),
            n_buckets_used: self.get_n_buckets_used(),
            n_entries: self.get_n_entries(),
        }
    }

    pub fn entry(data: T) -> Entry<T> {
        Entry { data }
    }

    pub fn entry_01(data: T) -> Entry01<T> {
        Entry01 { data }
    }

    pub fn entry_02(data: T) -> Entry02<T> {
        Entry02 { data }
    }

    pub fn as_mut(&mut self) -> &mut Self {
        self
    }

    pub fn string_17(&self) -> &'static str {
        "17"
    }
}

impl<T> Drop for Hash<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HashError {
    InvalidTuning,
    OutOfMemory,
}
