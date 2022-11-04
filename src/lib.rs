use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            items: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.items
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let index = self.bucket_index(&key);
        let mut bucket = &mut self.buckets[index];

        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if *ekey == key {
                return Some(mem::replace(evalue, value));
            }
        }

        bucket.push((key, value));
        self.items += 1;
        None
    }

    pub fn bucket_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let index = (hasher.finish() % self.buckets.len() as u64) as usize;
        index
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.bucket_index(key);
        /* self.buckets[index]
        .iter()
        .find(|&(ref ekey, _)| ekey == key)
        .map(|(_, ref value)| value) */

        self.buckets[index]
            .iter()
            .find_map(|(ekey, evalue)| if (ekey == key) { Some(evalue) } else { None })
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let bucket_index = self.bucket_index(key);
        let bucket = &mut self.buckets[bucket_index];
        let key_index = bucket.iter().position(|(ekey, evalue)| ekey == key)?;

        self.items -= 1;
        let value = bucket.swap_remove(key_index).1;
        Some(value)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => 1,
            n => n * 2,
        };

        let mut new_bucket = Vec::with_capacity(target_size);
        new_bucket.extend((0..target_size).map(|_| Vec::new()));

        for (key, value) in self.buckets.iter_mut().flat_map(|b| b.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let index = (hasher.finish() % new_bucket.len() as u64) as usize;
            new_bucket[index].push((key, value))
        }

        // new_bucket[..self.buckets.len()] = self.buckets[..];
        mem::replace(&mut self.buckets, new_bucket);
    }
}

struct Iter<'a, K: 'a, V: 'a> {
    map: &'a HashMap<K, V>,
    bucket_index: usize,
    key_index: usize,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.buckets.get(self.bucket_index) {
                Some(bucket) => match bucket.get(self.key_index) {
                    Some((key, value)) => {
                        self.bucket_index += 1;
                        break Some((key, value));
                    }
                    None => {
                        self.bucket_index += 1;
                        self.key_index = 0;
                        continue;
                    }
                },
                None => break None,
            }
        }
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
    type Item = (&'a K, &'a V);

    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            map: self,
            bucket_index: 0,
            key_index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let mut hmap = HashMap::new();
        hmap.insert("foo", 42);
        hmap.insert("bar", 43);
        assert_eq!(hmap.buckets.len(), 2);
        assert_eq!(hmap.get(&"foo"), Some(42).as_ref());
        assert_eq!(hmap.get(&"bar"), Some(43).as_ref());
        assert_eq!(hmap.len(), 2);

        hmap.remove(&"foo");
        assert_eq!(hmap.len(), 1);
        assert_eq!(hmap.get(&"foo"), None);
        assert_eq!(hmap.get(&"bar"), Some(43).as_ref());

        assert_eq!(hmap.contains_key(&"foo"), false);
        assert_eq!(hmap.contains_key(&"bar"), true);
    }

    #[test]
    fn test_iter() {
        let mut hmap = HashMap::new();
        hmap.insert("foo", 42);
        hmap.insert("bar1", 43);
        hmap.insert("bar2", 44);
        hmap.insert("bar3", 45);

        //let expected = [("foo", 42), ("bar1", 43), ("bar2", 44), ("bar3", 45)];

        for (key, value) in &hmap {
            println!("{:?} : {:?}", key, value);
        }

        assert_eq!(hmap.len(), 4)
    }
}
