use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash as StdHash, Hasher};
use std::io::{self, Write};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HashEntry<T> {
    pub data: Option<T>,
    pub next: Option<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HashTable<T> {
    pub n_buckets: usize,
    pub n_buckets_used: usize,
    pub n_entries: usize,
    pub buckets: Vec<HashEntry<T>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HashEntry01<T> {
    pub data: Option<T>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HashEntry02 {
    pub next: Option<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HashString17 {
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
struct Tuning {
    shrink_threshold: f32,
    shrink_factor: f32,
    growth_threshold: f32,
    growth_factor: f32,
    is_n_buckets: bool,
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
pub enum InsertOutcome<T> {
    Inserted(T),
    Present(T),
}

type HashFn<T> = Box<dyn Fn(&T, usize) -> usize>;
type CompareFn<T> = Box<dyn Fn(&T, &T) -> bool>;
type FreeFn<T> = Box<dyn FnMut(T)>;

pub struct Hash<T>
where
    T: Clone + PartialEq + Debug + StdHash,
{
    buckets: Vec<HashEntry<T>>,
    overflow: Vec<Option<HashEntry<T>>>,
    free_entry_list: Vec<usize>,
    n_buckets_used: usize,
    n_entries: usize,
    tuning: Tuning,
    hasher: HashFn<T>,
    comparator: CompareFn<T>,
    data_freer: Option<FreeFn<T>>,
}

impl<T> Hash<T>
where
    T: Clone + PartialEq + Debug + StdHash,
{
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
            if bucket.data.is_some() {
                let mut bucket_length = 1usize;
                let mut cursor = bucket.next;
                while let Some(i) = cursor {
                    match self.overflow.get(i).and_then(|e| e.as_ref()) {
                        Some(entry) => {
                            bucket_length += 1;
                            cursor = entry.next;
                        }
                        None => break,
                    }
                }
                max_bucket_length = max_bucket_length.max(bucket_length);
            }
        }
        max_bucket_length
    }

    pub fn table_ok(&self) -> bool {
        let mut n_buckets_used = 0usize;
        let mut n_entries = 0usize;

        for bucket in &self.buckets {
            if bucket.data.is_some() {
                n_buckets_used += 1;
                n_entries += 1;
                let mut cursor = bucket.next;
                while let Some(i) = cursor {
                    match self.overflow.get(i).and_then(|e| e.as_ref()) {
                        Some(entry) => {
                            n_entries += 1;
                            cursor = entry.next;
                        }
                        None => return false,
                    }
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

        writeln!(stream, "# entries:         {}", n_entries)?;
        writeln!(stream, "# buckets:         {}", n_buckets)?;
        let pct = if n_buckets == 0 {
            0.0
        } else {
            (100.0 * n_buckets_used as f64) / n_buckets as f64
        };
        writeln!(stream, "# buckets used:    {} ({pct:.2}%)", n_buckets_used)?;
        writeln!(stream, "max bucket length: {}", max_bucket_length)?;
        Ok(())
    }

    pub fn safe_hasher(&self, key: &T) -> usize {
        if self.buckets.is_empty() {
            return 0;
        }
        let n = (self.hasher)(key, self.buckets.len());
        assert!(n < self.buckets.len(), "hash out of bounds");
        n
    }

    pub fn lookup(&self, entry: &T) -> Option<&T> {
        if self.buckets.is_empty() {
            return None;
        }

        let bucket_index = self.safe_hasher(entry);
        let bucket = &self.buckets[bucket_index];

        if bucket.data.is_none() {
            return None;
        }

        if let Some(data) = bucket.data.as_ref()
            && (data == entry || (self.comparator)(entry, data))
        {
            return Some(data);
        }

        let mut cursor = bucket.next;
        while let Some(i) = cursor {
            let node = self.overflow.get(i).and_then(|e| e.as_ref())?;
            if let Some(data) = node.data.as_ref()
                && (data == entry || (self.comparator)(entry, data))
            {
                return Some(data);
            }
            cursor = node.next;
        }

        None
    }

    pub fn get_first(&self) -> Option<&T> {
        if self.n_entries == 0 {
            return None;
        }

        for bucket in &self.buckets {
            if let Some(data) = bucket.data.as_ref() {
                return Some(data);
            }
        }

        None
    }

    pub fn get_next(&self, entry: &T) -> Option<&T> {
        if self.buckets.is_empty() {
            return None;
        }

        let bucket_index = self.safe_hasher(entry);
        let bucket = &self.buckets[bucket_index];

        if let Some(data) = bucket.data.as_ref()
            && data == entry
            && let Some(next_index) = bucket.next
            && let Some(next_entry) = self.overflow.get(next_index).and_then(|e| e.as_ref())
        {
            return next_entry.data.as_ref();
        }

        let mut cursor_index = bucket.next;
        while let Some(i) = cursor_index {
            let cursor = self.overflow.get(i).and_then(|e| e.as_ref())?;
            if let Some(data) = cursor.data.as_ref()
                && data == entry
                && let Some(next_index) = cursor.next
                && let Some(next_entry) = self.overflow.get(next_index).and_then(|e| e.as_ref())
            {
                return next_entry.data.as_ref();
            }
            cursor_index = cursor.next;
        }

        for next_bucket in self.buckets.iter().skip(bucket_index + 1) {
            if let Some(data) = next_bucket.data.as_ref() {
                return Some(data);
            }
        }

        None
    }

    pub fn get_entries(&self, buffer: &mut Vec<T>, buffer_size: usize) -> usize {
        buffer.clear();
        let mut counter = 0usize;

        for bucket in &self.buckets {
            if let Some(data) = bucket.data.as_ref() {
                if counter >= buffer_size {
                    return counter;
                }
                buffer.push(data.clone());
                counter += 1;

                let mut cursor = bucket.next;
                while let Some(i) = cursor {
                    let node = match self.overflow.get(i).and_then(|e| e.as_ref()) {
                        Some(node) => node,
                        None => return counter,
                    };
                    if counter >= buffer_size {
                        return counter;
                    }
                    if let Some(data) = node.data.as_ref() {
                        buffer.push(data.clone());
                        counter += 1;
                    }
                    cursor = node.next;
                }
            }
        }

        counter
    }

    pub fn do_for_each<F>(&self, mut processor: F) -> usize
    where
        F: FnMut(&T) -> bool,
    {
        let mut counter = 0usize;

        for bucket in &self.buckets {
            if let Some(data) = bucket.data.as_ref() {
                if !processor(data) {
                    return counter;
                }
                counter += 1;

                let mut cursor = bucket.next;
                while let Some(i) = cursor {
                    let node = match self.overflow.get(i).and_then(|e| e.as_ref()) {
                        Some(node) => node,
                        None => return counter,
                    };
                    if let Some(data) = node.data.as_ref() {
                        if !processor(data) {
                            return counter;
                        }
                        counter += 1;
                    }
                    cursor = node.next;
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
            value = (value * 31 + ch as usize) % n_buckets;
        }
        value
    }

    pub fn gl_attribute_const() -> bool {
        true
    }

    pub fn reset_tuning() -> (f32, f32, f32, f32, bool) {
        let tuning = Tuning::default();
        (
            tuning.shrink_threshold,
            tuning.shrink_factor,
            tuning.growth_threshold,
            tuning.growth_factor,
            tuning.is_n_buckets,
        )
    }

    pub fn raw_hasher(data: &T, n: usize) -> usize {
        if n == 0 {
            return 0;
        }

        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        (hasher.finish() as usize) % n
    }

    pub fn raw_comparator(a: &T, b: &T) -> bool {
        a == b
    }

    fn make_hasher(hasher: Option<HashFn<T>>) -> HashFn<T> {
        hasher.unwrap_or_else(|| Box::new(Self::raw_hasher))
    }

    fn make_comparator(comparator: Option<CompareFn<T>>) -> CompareFn<T> {
        comparator.unwrap_or_else(|| Box::new(Self::raw_comparator))
    }

    pub fn check_tuning(&mut self) -> bool {
        let tuning = &self.tuning;
        let epsilon = 0.1f32;

        if self.tuning == Tuning::default() {
            return true;
        }

        let ok = epsilon < tuning.growth_threshold
            && tuning.growth_threshold < 1.0 - epsilon
            && 1.0 + epsilon < tuning.growth_factor
            && 0.0 <= tuning.shrink_threshold
            && tuning.shrink_threshold + epsilon < tuning.shrink_factor
            && tuning.shrink_factor <= 1.0
            && tuning.shrink_threshold + epsilon < tuning.growth_threshold;

        if !ok {
            self.tuning = Tuning::default();
        }
        ok
    }

    pub fn compute_bucket_size(
        candidate: usize,
        tuning: Option<(f32, f32, f32, f32, bool)>,
    ) -> Option<usize> {
        let tuning = tuning
            .map(
                |(
                    shrink_threshold,
                    shrink_factor,
                    growth_threshold,
                    growth_factor,
                    is_n_buckets,
                )| Tuning {
                    shrink_threshold,
                    shrink_factor,
                    growth_threshold,
                    growth_factor,
                    is_n_buckets,
                },
            )
            .unwrap_or_default();

        let mut candidate = candidate.max(1);
        if !tuning.is_n_buckets {
            let new_candidate = candidate as f32 / tuning.growth_threshold;
            if new_candidate >= usize::MAX as f32 {
                return None;
            }
            candidate = new_candidate as usize;
        }

        Some(Self::next_prime(candidate.max(2)))
    }

    pub fn initialize(
        candidate: usize,
        tuning: Option<(f32, f32, f32, f32, bool)>,
        hasher: Option<HashFn<T>>,
        comparator: Option<CompareFn<T>>,
        data_freer: Option<FreeFn<T>>,
    ) -> Result<Self, &'static str> {
        let tuning = tuning
            .map(
                |(
                    shrink_threshold,
                    shrink_factor,
                    growth_threshold,
                    growth_factor,
                    is_n_buckets,
                )| Tuning {
                    shrink_threshold,
                    shrink_factor,
                    growth_threshold,
                    growth_factor,
                    is_n_buckets,
                },
            )
            .unwrap_or_default();

        let mut table = Self {
            buckets: Vec::new(),
            overflow: Vec::new(),
            free_entry_list: Vec::new(),
            n_buckets_used: 0,
            n_entries: 0,
            tuning,
            hasher: Self::make_hasher(hasher),
            comparator: Self::make_comparator(comparator),
            data_freer,
        };

        if !table.check_tuning() {
            return Err("invalid tuning");
        }

        let n_buckets = Self::compute_bucket_size(
            candidate,
            Some((
                table.tuning.shrink_threshold,
                table.tuning.shrink_factor,
                table.tuning.growth_threshold,
                table.tuning.growth_factor,
                table.tuning.is_n_buckets,
            )),
        )
        .ok_or("allocation size overflow")?;

        table.buckets = vec![HashEntry::default(); n_buckets];
        Ok(table)
    }

    pub fn clear(&mut self) {
        for bucket_index in 0..self.buckets.len() {
            if self.buckets[bucket_index].data.is_some() {
                let mut cursor = self.buckets[bucket_index].next;

                while let Some(i) = cursor {
                    let next = self
                        .overflow
                        .get(i)
                        .and_then(|e| e.as_ref())
                        .and_then(|e| e.next);

                    if let Some(node) = self.overflow.get_mut(i).and_then(|e| e.as_mut()) {
                        if let Some(freer) = self.data_freer.as_mut()
                            && let Some(data) = node.data.take()
                        {
                            freer(data);
                        } else {
                            node.data = None;
                        }
                        node.next = None;
                    }

                    self.free_entry_list.push(i);
                    cursor = next;
                }

                if let Some(freer) = self.data_freer.as_mut()
                    && let Some(data) = self.buckets[bucket_index].data.take()
                {
                    freer(data);
                } else {
                    self.buckets[bucket_index].data = None;
                }
                self.buckets[bucket_index].next = None;
            }
        }

        self.n_buckets_used = 0;
        self.n_entries = 0;
    }

    pub fn allocate_entry(&mut self) -> usize {
        if let Some(index) = self.free_entry_list.pop() {
            self.overflow[index] = Some(HashEntry::default());
            index
        } else {
            self.overflow.push(Some(HashEntry::default()));
            self.overflow.len() - 1
        }
    }

    pub fn free_entry(&mut self, entry_index: usize) {
        if let Some(slot) = self.overflow.get_mut(entry_index) {
            *slot = None;
        }
        self.free_entry_list.push(entry_index);
    }

    pub fn find_entry(&mut self, entry: &T, delete: bool) -> (Option<T>, usize) {
        let bucket_index = self.safe_hasher(entry);

        if self.buckets[bucket_index].data.is_none() {
            return (None, bucket_index);
        }

        let bucket_matches = self.buckets[bucket_index]
            .data
            .as_ref()
            .map(|data| data == entry || (self.comparator)(entry, data))
            .unwrap_or(false);

        if bucket_matches {
            let data = self.buckets[bucket_index].data.as_ref().cloned();

            if delete {
                if let Some(next_index) = self.buckets[bucket_index].next {
                    let next = self.overflow[next_index].take().unwrap_or_default();
                    self.buckets[bucket_index] = next;
                    self.free_entry_list.push(next_index);
                } else {
                    self.buckets[bucket_index].data = None;
                    self.buckets[bucket_index].next = None;
                }
            }

            return (data, bucket_index);
        }

        let mut cursor_index = bucket_index;
        loop {
            let next_index = if cursor_index == bucket_index {
                self.buckets[bucket_index].next
            } else {
                self.overflow
                    .get(cursor_index)
                    .and_then(|e| e.as_ref())
                    .and_then(|e| e.next)
            };

            let Some(next_i) = next_index else {
                break;
            };

            let matches = self
                .overflow
                .get(next_i)
                .and_then(|e| e.as_ref())
                .and_then(|e| e.data.as_ref())
                .map(|data| data == entry || (self.comparator)(entry, data))
                .unwrap_or(false);

            if matches {
                let data = self.overflow[next_i]
                    .as_ref()
                    .and_then(|e| e.data.as_ref())
                    .cloned();

                if delete {
                    let after = self.overflow[next_i].as_ref().and_then(|e| e.next);
                    if cursor_index == bucket_index {
                        self.buckets[bucket_index].next = after;
                    } else if let Some(cursor) =
                        self.overflow.get_mut(cursor_index).and_then(|e| e.as_mut())
                    {
                        cursor.next = after;
                    }
                    self.free_entry(next_i);
                }

                return (data, bucket_index);
            }

            cursor_index = next_i;
        }

        (None, bucket_index)
    }

    pub fn transfer_entries(&mut self, dst: &mut Self, safe: bool) -> bool {
        for bucket_index in 0..self.buckets.len() {
            if self.buckets[bucket_index].data.is_some() {
                let mut cursor = self.buckets[bucket_index].next;

                while let Some(i) = cursor {
                    let next = self
                        .overflow
                        .get(i)
                        .and_then(|e| e.as_ref())
                        .and_then(|e| e.next);

                    let data = match self
                        .overflow
                        .get(i)
                        .and_then(|e| e.as_ref())
                        .and_then(|e| e.data.as_ref())
                        .cloned()
                    {
                        Some(data) => data,
                        None => {
                            cursor = next;
                            continue;
                        }
                    };

                    let new_bucket_index = dst.safe_hasher(&data);
                    if dst.buckets[new_bucket_index].data.is_some() {
                        if i >= dst.overflow.len() {
                            dst.overflow.resize_with(i + 1, || None);
                        }
                        let moved = self.overflow[i].take();
                        if let Some(node) = moved {
                            let mut node = node;
                            node.next = dst.buckets[new_bucket_index].next;
                            dst.overflow[i] = Some(node);
                            dst.buckets[new_bucket_index].next = Some(i);
                        }
                    } else {
                        dst.buckets[new_bucket_index].data = Some(data);
                        dst.n_buckets_used += 1;
                        self.free_entry(i);
                    }

                    cursor = next;
                }

                let data = self.buckets[bucket_index].data.clone();
                self.buckets[bucket_index].next = None;

                if safe {
                    continue;
                }

                let Some(data) = data else {
                    continue;
                };

                let new_bucket_index = dst.safe_hasher(&data);
                if dst.buckets[new_bucket_index].data.is_some() {
                    let new_entry = dst.allocate_entry();
                    if let Some(entry) = dst.overflow.get_mut(new_entry).and_then(|e| e.as_mut()) {
                        entry.data = Some(data);
                        entry.next = dst.buckets[new_bucket_index].next;
                    }
                    dst.buckets[new_bucket_index].next = Some(new_entry);
                } else {
                    dst.buckets[new_bucket_index].data = Some(data);
                    dst.n_buckets_used += 1;
                }

                self.buckets[bucket_index].data = None;
                self.n_buckets_used = self.n_buckets_used.saturating_sub(1);
            }
        }

        true
    }

    pub fn rehash(&mut self, candidate: usize) -> bool {
        let new_size = match Self::compute_bucket_size(
            candidate,
            Some((
                self.tuning.shrink_threshold,
                self.tuning.shrink_factor,
                self.tuning.growth_threshold,
                self.tuning.growth_factor,
                self.tuning.is_n_buckets,
            )),
        ) {
            Some(size) => size,
            None => return false,
        };

        if new_size == self.get_n_buckets() {
            return true;
        }

        let mut new_table = Self {
            buckets: vec![HashEntry::default(); new_size],
            overflow: std::mem::take(&mut self.overflow),
            free_entry_list: std::mem::take(&mut self.free_entry_list),
            n_buckets_used: 0,
            n_entries: self.n_entries,
            tuning: self.tuning.clone(),
            hasher: std::mem::replace(&mut self.hasher, Box::new(Self::raw_hasher)),
            comparator: std::mem::replace(&mut self.comparator, Box::new(Self::raw_comparator)),
            data_freer: self.data_freer.take(),
        };

        if self.transfer_entries(&mut new_table, false) {
            self.buckets = new_table.buckets;
            self.overflow = new_table.overflow;
            self.free_entry_list = new_table.free_entry_list;
            self.n_buckets_used = new_table.n_buckets_used;
            self.hasher = new_table.hasher;
            self.comparator = new_table.comparator;
            self.data_freer = new_table.data_freer;
            true
        } else {
            false
        }
    }

    pub fn insert_if_absent(&mut self, entry: T) -> Result<InsertOutcome<T>, &'static str> {
        let (found, mut bucket_index) = self.find_entry(&entry, false);
        if let Some(existing) = found {
            return Ok(InsertOutcome::Present(existing));
        }

        if self.get_n_buckets() > 0
            && self.n_buckets_used as f32 > self.tuning.growth_threshold * self.get_n_buckets() as f32
        {
            self.check_tuning();
            if self.n_buckets_used as f32
                > self.tuning.growth_threshold * self.get_n_buckets() as f32
            {
                let candidate = if self.tuning.is_n_buckets {
                    self.get_n_buckets() as f32 * self.tuning.growth_factor
                } else {
                    self.get_n_buckets() as f32
                        * self.tuning.growth_factor
                        * self.tuning.growth_threshold
                };

                if candidate >= usize::MAX as f32 {
                    return Err("allocation size overflow");
                }

                if !self.rehash(candidate as usize) {
                    return Err("rehash failed");
                }

                let (found_again, new_bucket_index) = self.find_entry(&entry, false);
                if found_again.is_some() {
                    return Err("entry unexpectedly present after rehash");
                }
                bucket_index = new_bucket_index;
            }
        }

        if self.buckets[bucket_index].data.is_some() {
            let new_entry = self.allocate_entry();
            if let Some(node) = self.overflow.get_mut(new_entry).and_then(|e| e.as_mut()) {
                node.data = Some(entry.clone());
                node.next = self.buckets[bucket_index].next;
            }
            self.buckets[bucket_index].next = Some(new_entry);
            self.n_entries += 1;
            Ok(InsertOutcome::Inserted(entry))
        } else {
            self.buckets[bucket_index].data = Some(entry.clone());
            self.n_entries += 1;
            self.n_buckets_used += 1;
            Ok(InsertOutcome::Inserted(entry))
        }
    }

    pub fn insert(&mut self, entry: T) -> Option<T> {
        match self.insert_if_absent(entry.clone()) {
            Ok(InsertOutcome::Inserted(inserted)) => Some(inserted),
            Ok(InsertOutcome::Present(existing)) => Some(existing),
            Err(_) => None,
        }
    }

    pub fn remove(&mut self, entry: &T) -> Option<T> {
        let (data, bucket_index) = self.find_entry(entry, true);
        let data = data?;

        self.n_entries = self.n_entries.saturating_sub(1);
        if self.buckets[bucket_index].data.is_none() {
            self.n_buckets_used = self.n_buckets_used.saturating_sub(1);

            if self.get_n_buckets() > 0
                && self.n_buckets_used as f32
                    < self.tuning.shrink_threshold * self.get_n_buckets() as f32
            {
                self.check_tuning();
                if self.n_buckets_used as f32
                    < self.tuning.shrink_threshold * self.get_n_buckets() as f32
                {
                    let candidate = if self.tuning.is_n_buckets {
                        (self.get_n_buckets() as f32 * self.tuning.shrink_factor) as usize
                    } else {
                        (self.get_n_buckets() as f32
                            * self.tuning.shrink_factor
                            * self.tuning.growth_threshold) as usize
                    };
                    let _ = self.rehash(candidate);
                }
            }
        }

        Some(data)
    }

    pub fn print(&self) {
        for (i, bucket) in self.buckets.iter().enumerate() {
            println!("{i}:");
            if let Some(data) = bucket.data.as_ref() {
                println!("  {:?}", data);
            }
            let mut cursor = bucket.next;
            while let Some(idx) = cursor {
                let Some(node) = self.overflow.get(idx).and_then(|e| e.as_ref()) else {
                    break;
                };
                if let Some(data) = node.data.as_ref() {
                    println!("  {:?}", data);
                }
                cursor = node.next;
            }
        }
    }

    pub fn is_prime(candidate: usize) -> bool {
        if candidate <= 1 {
            return false;
        }
        if candidate == 2 {
            return true;
        }
        if candidate.is_multiple_of(2) {
            return false;
        }
        let mut divisor = 3usize;
        while divisor.saturating_mul(divisor) <= candidate {
            if candidate.is_multiple_of(divisor) {
                return false;
            }
            divisor += 2;
        }
        true
    }

    pub fn next_prime(candidate: usize) -> usize {
        if candidate <= 2 {
            return 2;
        }

        let mut n = if candidate.is_multiple_of(2) {
            candidate + 1
        } else {
            candidate
        };

        while !Self::is_prime(n) {
            n = n.saturating_add(2);
        }
        n
    }

    pub fn get_13() -> usize {
        13
    }

    pub fn entry(data: Option<T>, next: Option<usize>) -> HashEntry<T> {
        HashEntry { data, next }
    }

    pub fn table() -> HashTable<T> {
        HashTable {
            n_buckets: 0,
            n_buckets_used: 0,
            n_entries: 0,
            buckets: Vec::new(),
        }
    }

    pub fn entry_01(data: Option<T>) -> HashEntry01<T> {
        HashEntry01 { data }
    }

    pub fn entry_02(next: Option<usize>) -> HashEntry02 {
        HashEntry02 { next }
    }

    pub fn string_17(value: impl Into<String>) -> HashString17 {
        HashString17 {
            value: value.into(),
        }
    }
}

impl<T> Drop for Hash<T>
where
    T: Clone + PartialEq + Debug + StdHash,
{
    fn drop(&mut self) {
        self.clear();
    }
}
