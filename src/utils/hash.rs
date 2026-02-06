use std::hash::{DefaultHasher, Hash, Hasher};

pub(crate) fn default_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}