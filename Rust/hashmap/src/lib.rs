use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher, Hash};

const INITIAL_NBUCKETS:usize = 1;


pub struct HashhMap<K, V> {
    buckets: Vec<Vec<(K, V)>>
}

impl<K, V> HashhMap<K, V> {
    pub fn new() -> Self {
        HashhMap {
            buckets: Vec::new(),
        }
    }
}

impl<K, V> HashhMap<K, V>
where
    K: Hash + Eq
 {
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket = (hasher.finish() % self.buckets.len() as u64) as usize;
        let bucket = &mut self.buckets[bucket];

        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if ekey == &key {
                use std::mem;
                return Some(mem::replace(evalue, value));
            }
        }

        bucket.push((key, value));
        None
    }

    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_NBUCKETS,
            n => 2 * n
        };

        // TODO
    }


}