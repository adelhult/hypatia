use std::collections::HashMap;
use std::hash::Hash;
use std::{iter, mem};

/// A data structure for storing values keyed by arrays of K which allows for retrieval of all
/// values whose keys are prefix of the searched term. If you want to search using `[u64]`, your
/// K should be u64. If you want String/Char, see StringTrie.
#[derive(Debug, Clone, PartialEq)]
struct Trie<K, V>(Node<K, V>)
where
    K: Hash + Clone + PartialEq + Eq,
    V: Clone + PartialEq;

impl<K, V> Default for Trie<K, V>
where
    K: Hash + Clone + PartialEq + Eq,
    V: Clone + PartialEq,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Trie<K, V>
where
    K: Hash + Clone + PartialEq + Eq,
    V: Clone + PartialEq,
{
    /// Creates an empty Trie
    pub fn new() -> Self {
        Self(Node::new())
    }

    /// Gets the amount of values in this Trie
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Creates an iterator over all the key-value pairs for which values are stored in this Trie
    pub fn entries(&self) -> Box<dyn Iterator<Item = (&[K], &V)> + '_> {
        self.0.entries()
    }

    /// Creates an iterator over all the values which are stored in this Trie
    pub fn values(&self) -> Box<dyn Iterator<Item = &V> + '_> {
        self.0.values()
    }

    /// Creates an iterator over all the keys for which values are stored in this Trie
    pub fn keys(&self) -> Box<dyn Iterator<Item = &[K]> + '_> {
        self.0.keys()
    }

    /// Checks if this Trie contains the given key
    pub fn contains_key(&self, key: &[K]) -> bool {
        self.keys().any(|k| k == key)
    }

    /// Searches this Trie for all values for which their keys is prefixes of the given key.
    /// If the key `[1,2,3]` is searched, the values for `[1]`, `[1,2]`, and `[1,2,3]` (if they
    /// exist) is returned. Note that `[1,2]` may exist even though neither `[1]` nor `[1,2,3]`
    /// exists. The resulting Vec contains key-value-pairs sorted with the shortest keys first.
    pub fn search(&self, arr: &[K]) -> Vec<(Vec<K>, V)> {
        self.0.search(&[], arr)
    }

    /// Inserts the given value at the specified path, returning the previous value as an Option
    pub fn insert(&mut self, path: &[K], val: V) -> Option<V> {
        self.0.insert(0, path, val)
    }

    /// Removes the value at the given path, returning the previous value as an Option
    pub fn remove(&mut self, path: &[K]) -> Option<V> {
        self.0.remove(0, path)
    }

    /// Removes all nodes without values and/or children with values in this Trie. This may reduce
    /// the size of this Trie
    pub fn purge(&mut self) {
        self.0.purge();
    }
}

/// A wrapper around the data structure Trie for storing values keyed by strings, which allows for
/// retrieval of all values whose keys are prefix of the searched term.
#[derive(Debug, Clone, PartialEq)]
pub struct StringTrie<V>(Trie<u8, V>)
where
    V: Clone + PartialEq;

impl<V> Default for StringTrie<V>
where
    V: Clone + PartialEq,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<V> StringTrie<V>
where
    V: Clone + PartialEq,
{
    /// Creates an empty StringTrie
    pub fn new() -> Self {
        Self(Trie::new())
    }

    /// Gets the amount of values in this Trie
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Creates an iterator over all the key-value pairs for which values are stored in this Trie
    pub fn entries(&self) -> Box<dyn Iterator<Item = (String, &V)> + '_> {
        let res = self
            .0
            .entries()
            .map(|(k, v): (&[u8], &V)| (String::from_utf8(k.to_vec()).ok().unwrap(), v));
        Box::new(res)
    }

    /// Creates an iterator over all the values which are stored in this Trie
    pub fn values(&self) -> Box<dyn Iterator<Item = &V> + '_> {
        self.0.values()
    }

    /// Creates an iterator over all the keys for which values are stored in this Trie
    pub fn keys(&self) -> Box<dyn Iterator<Item = String> + '_> {
        let res = self
            .0
            .keys()
            .map(|k: &[u8]| String::from_utf8(k.to_vec()).ok().unwrap());
        Box::new(res)
    }

    /// Checks if this Trie contains the given key
    pub fn contains_key(&self, key: &str) -> bool {
        self.keys().any(|k| k == key)
    }

    /// Searches this StringTrie for all values for which their keys is prefixes of the given key.
    /// If the key "abc" is searched, the values for "a", "b", and "c" (if they exist)
    /// is returned. Note that "ab" may exist even though neither "a" nor "abc exists.
    /// The resulting Vec contains key-value-pairs sorted with the shortest keys first.
    pub fn search(&self, key: &str) -> Vec<(String, V)> {
        let k: &[u8] = key.as_bytes();
        self.0
            .search(&k)
            .into_iter()
            .map(|(cs, val)| (String::from_utf8(cs.to_vec()).ok().unwrap(), val))
            .collect()
    }

    /// Inserts the given value at the specified key, returning the previous value as an Option
    pub fn insert(&mut self, key: &str, val: V) -> Option<V> {
        let k: &[u8] = key.as_bytes();
        self.0.insert(k, val)
    }

    /// Removes the value with the given key, returning the previous value as an Option
    pub fn remove(&mut self, key: &str) -> Option<V> {
        let k: &[u8] = key.as_bytes();
        self.0.remove(k)
    }

    /// Removes all nodes without values and/or children with values in this Trie. This may reduce
    /// the size of this Trie
    pub fn purge(&mut self) {
        self.0.purge();
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Node<K, V>
where
    K: Hash + Clone + PartialEq + Eq,
    V: Clone + PartialEq,
{
    path: Vec<K>,
    val: Option<V>,
    edges: HashMap<K, Node<K, V>>,
}

impl<K, V> Default for Node<K, V>
where
    K: Hash + Clone + PartialEq + Eq,
    V: Clone + PartialEq,
{
    fn default() -> Self {
        Self::new()
    }
}

enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R, T> Iterator for Either<L, R>
where
    L: Iterator<Item = T>,
    R: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Either::Left(l) => l.next(),
            Either::Right(r) => r.next(),
        }
    }
}

impl<K, V> Node<K, V>
where
    K: Hash + Clone + PartialEq + Eq,
    V: Clone + PartialEq,
{
    fn new() -> Self {
        Self {
            path: vec![],
            val: None,
            edges: HashMap::new(),
        }
    }

    fn new_with_path(path: Vec<K>) -> Self {
        Self {
            path,
            val: None,
            edges: HashMap::new(),
        }
    }

    fn len(&self) -> usize {
        let this: usize = self.val.is_some().into();
        let children: usize = self.edges.values().map(|node| node.len()).sum();
        this + children
    }

    fn entries(&self) -> Box<dyn Iterator<Item = (&[K], &V)> + '_> {
        let iter = if let Some(this_val) = &self.val {
            Either::Left(iter::once((self.path.as_slice(), this_val)))
        } else {
            Either::Right(iter::empty())
        };

        let rest = self.edges.values().flat_map(|node| node.entries());

        Box::new(iter.chain(rest))
    }

    fn values(&self) -> Box<dyn Iterator<Item = &V> + '_> {
        let iter = if let Some(this_val) = &self.val {
            Either::Left(iter::once(this_val))
        } else {
            Either::Right(iter::empty())
        };

        let rest = self.edges.values().flat_map(|node| node.values());

        Box::new(iter.chain(rest))
    }

    fn keys(&self) -> Box<dyn Iterator<Item = &[K]> + '_> {
        let iter = if self.val.is_some() {
            Either::Left(iter::once(self.path.as_slice()))
        } else {
            Either::Right(iter::empty())
        };

        let rest = self.edges.values().flat_map(|node| node.keys());

        Box::new(iter.chain(rest))
    }

    // locates the path and inserts the specified value there
    // if path is empty, this node is at the end of the path
    // if path isn't empty, try to find the next node (and create a new one if it doesn't exist)
    // and then recurse with tail of path into that node
    // val is the value to insert, return value is the previous value
    fn insert(&mut self, depth: usize, path: &[K], val: V) -> Option<V> {
        if depth == path.len() {
            // We have reached the path and set our value, returning the old value
            // as per the API of HashMap
            let old_val = mem::replace(&mut self.val, Some(val));
            return old_val;
        }

        // if path remains: extract head from list (which is the next key)
        let head = path[depth].clone();

        // if no next node, make a new one
        self.edges
            .entry(head.clone())
            .or_insert_with(|| Node::new_with_path(path[..=depth].to_owned()));

        // recurse into next node, save the result and return it
        let mut ret = None;
        self.edges
            .entry(head)
            .and_modify(|next_node| ret = next_node.insert(depth + 1, path, val));
        ret
    }

    // locates the path and inserts the specified value there
    // if path is empty, this node is at the end of the path
    // if path isn't empty, try to find the next node (and create a new one if it doesn't exist)
    // and then recurse with tail of path into that node
    // val is the value to insert, return value is the previous value
    fn remove(&mut self, depth: usize, path: &[K]) -> Option<V> {
        if depth == path.len() {
            // We have reached the path and set our value, returning the old value
            // as per the API of HashMap
            let old_val = mem::replace(&mut self.val, None);
            return old_val;
        }

        // if path remains: extract head from list (which is the next key)
        let head = path[depth].clone();

        // if no next node, return None immediately
        if !self.edges.contains_key(&head) {
            return None;
        }

        // recurse into next node, save the result and return it
        let mut ret = None;
        self.edges
            .entry(head)
            .and_modify(|next_node| ret = next_node.remove(depth + 1, path));
        ret
    }

    fn search(&self, acc: &[K], arr: &[K]) -> Vec<(Vec<K>, V)> {
        let mut res = Vec::new();

        if let Some(this_val) = &self.val {
            res.push((acc.to_owned(), this_val.clone()))
        }

        if arr.is_empty() {
            return res;
        }

        let head = arr[0].clone();
        if let Some(next_node) = self.edges.get(&head) {
            let mut next_acc = acc.to_owned();
            next_acc.push(head);
            res.append(&mut next_node.search(&next_acc, &arr[1..]));
        }

        res
    }

    // RETURNS TRUE IF THIS NODE CAN BE REMOVED
    fn purge(&mut self) -> bool {
        self.edges.retain(|_, node| !node.purge());
        self.edges.is_empty() && self.val.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::{StringTrie, Trie};
    use proptest::prelude::*;
    use std::collections::HashMap;

    proptest! {
        #[test]
        fn add_remove_len_purge(unfiltered_entries: Vec<(Vec<u8>, u8)>) {
            let max_key_len = 20;
            let max_entries = 20;

            let entries: Vec<(Vec<u8>, u8)> =
                unfiltered_entries.into_iter()
                    .map(|(k, v)| (k.into_iter().take(max_key_len).collect(), v))
                    .take(max_entries)
                    .collect();

            let mut hm : HashMap<Vec<u8>, u8> = HashMap::new();
            let mut tr : Trie<u8, u8> = Trie::new();

            for (key, val) in &entries {
                let a = hm.insert(key.clone(), *val);
                let b = tr.insert(key, *val);
                assert!(a == b);
                assert!(hm.len() == tr.len());
                tr.purge();
                assert!(hm.len() == tr.len());
            }

            for (key, _) in &entries {
                let hmvals: Vec<u8> = (0..=key.len()).flat_map(|i| hm.get(&key[0..i]).into_iter()).copied().collect();
                let trvals: Vec<u8> = tr.search(key).iter().map(|(_,x)| *x).collect();
                assert!(hmvals == trvals);
            }

            for (key, _) in &entries {
                let a = hm.remove(key);
                let b = tr.remove(key);
                assert!(a == b);
                assert!(hm.len() == tr.len());
                tr.purge();
                assert!(hm.len() == tr.len());
            }
            assert!(tr.len() == 0);
        }
    }

    proptest! {
        #[test]
        fn entry_iterator(unfiltered_entries: Vec<(Vec<u8>, u8)>) {
            let max_key_len = 20;
            let max_entries = 20;

            let entries: Vec<(Vec<u8>, u8)> =
                unfiltered_entries.into_iter()
                    .map(|(k, v)| (k.into_iter().take(max_key_len).collect(), v))
                    .take(max_entries)
                    .collect();

            let mut hm : HashMap<Vec<u8>, u8> = HashMap::new();
            let mut tr : Trie<u8, u8> = Trie::new();

            for (key, val) in &entries {
                hm.insert(key.clone(), *val);
                tr.insert(key, *val);

                let mut trv : Vec<(&[u8], &u8)> = tr
                    .entries()
                    .collect();
                let mut hmv : Vec<(&[u8], &u8)> = hm
                    .iter()
                    .map(|(vec, k)| (vec.as_slice(), k))
                    .collect();
                trv.sort();
                hmv.sort();
                assert!(trv == hmv);
            }
        }
    }

    proptest! {
        #[test]
        fn key_iterator(unfiltered_entries: Vec<(Vec<u8>, u8)>) {
            let max_key_len = 20;
            let max_entries = 20;

            let entries: Vec<(Vec<u8>, u8)> =
                unfiltered_entries.into_iter()
                    .map(|(k, v)| (k.into_iter().take(max_key_len).collect(), v))
                    .take(max_entries)
                    .collect();

            let mut hm : HashMap<Vec<u8>, u8> = HashMap::new();
            let mut tr : Trie<u8, u8> = Trie::new();

            for (key, val) in &entries {
                hm.insert(key.clone(), *val);
                tr.insert(key, *val);

                let mut trv : Vec<&[u8]> = tr
                    .keys()
                    .collect();
                let mut hmv : Vec<&[u8]> = hm
                    .keys()
                    .map(|u| u.as_slice())
                    .collect();
                trv.sort();
                hmv.sort();
                assert!(trv == hmv);
            }
        }
    }

    proptest! {
        #[test]
        fn val_iterator(unfiltered_entries: Vec<(Vec<u8>, u8)>) {
            let max_key_len = 20;
            let max_entries = 20;

            let entries: Vec<(Vec<u8>, u8)> =
                unfiltered_entries.into_iter()
                    .map(|(k, v)| (k.into_iter().take(max_key_len).collect(), v))
                    .take(max_entries)
                    .collect();

            let mut hm : HashMap<Vec<u8>, u8> = HashMap::new();
            let mut tr : Trie<u8, u8> = Trie::new();

            for (key, val) in &entries {
                hm.insert(key.clone(), *val);
                tr.insert(key, *val);

                let mut trv : Vec<&u8> = tr
                    .values()
                    .collect();
                let mut hmv : Vec<&u8> = hm
                    .values()
                    .collect();
                trv.sort();
                hmv.sort();
                assert!(trv == hmv);
            }
        }
    }

    proptest! {
        #[test]
        fn string_add_remove_len_purge(unfiltered_entries: Vec<(String, u8)>) {
            let max_key_len = 20;
            let max_entries = 20;

            let entries: Vec<(String, u8)> =
                unfiltered_entries.into_iter()
                    .map(|(mut k, v)| {
                        while k.len() > max_key_len {
                            k.pop();
                        }
                        (k,v)
                    })
                    .take(max_entries)
                    .collect();

            let mut hm : HashMap<Vec<u8>, u8> = HashMap::new();
            let mut tr : StringTrie<u8> = StringTrie::new();

            for (key, val) in &entries {
                let a = hm.insert(key.as_bytes().to_vec(), *val);
                let b = tr.insert(key, *val);
                assert!(a == b);
                assert!(hm.len() == tr.len());
                tr.purge();
                assert!(hm.len() == tr.len());
            }

            for (key, _) in &entries {
                let hmvals: Vec<u8> = (0..=key.as_bytes().len()).flat_map(|i| hm.get(&key.as_bytes()[0..i]).into_iter()).copied().collect();
                let trvals: Vec<u8> = tr.search(key).iter().map(|(_, x)| *x).collect();
                assert!(hmvals == trvals);
            }

            for (key, _) in &entries {
                let a = hm.remove(&key.as_bytes().to_vec());
                let b = tr.remove(key);
                assert!(a == b);
                assert!(hm.len() == tr.len());
                tr.purge();
                assert!(hm.len() == tr.len());
            }
            assert!(tr.len() == 0);
        }
    }

    proptest! {
        #[test]
        fn string_entry_iterator(unfiltered_entries: Vec<(String, u8)>) {
            let max_key_len = 20;
            let max_entries = 20;

            let entries: Vec<(String, u8)> =
                unfiltered_entries.into_iter()
                    .map(|(mut k, v)| {
                        while k.len() > max_key_len {
                            k.pop();
                        }
                        (k,v)
                    })
                    .take(max_entries)
                    .collect();

            let mut hm : HashMap<String, u8> = HashMap::new();
            let mut tr : StringTrie<u8> = StringTrie::new();

            for (key, val) in &entries {
                hm.insert(key.clone(), *val);
                tr.insert(key, *val);

                let mut trv : Vec<(String, &u8)> = tr
                    .entries()
                    .collect();
                let mut hmv : Vec<(String, &u8)> = hm
                    .iter()
                    .map(|(s,v)| (s.to_string(), v))
                    .collect();
                trv.sort();
                hmv.sort();
                assert!(trv == hmv);
            }
        }
    }

    proptest! {
        #[test]
        fn string_key_iterator(unfiltered_entries: Vec<(String, u8)>) {
            let max_key_len = 20;
            let max_entries = 20;

            let entries: Vec<(String, u8)> =
                unfiltered_entries.into_iter()
                    .map(|(mut k, v)| {
                        while k.len() > max_key_len {
                            k.pop();
                        }
                        (k,v)
                    })
                    .take(max_entries)
                    .collect();

            let mut hm : HashMap<String, u8> = HashMap::new();
            let mut tr : StringTrie<u8> = StringTrie::new();

            for (key, val) in &entries {
                hm.insert(key.clone(), *val);
                tr.insert(key, *val);

                let mut trv : Vec<String> = tr
                    .keys()
                    .collect();
                let mut hmv : Vec<String> = hm
                    .keys()
                    .map(|u| u.to_string())
                    .collect();
                trv.sort();
                hmv.sort();
                assert!(trv == hmv);
            }
        }
    }

    proptest! {
        #[test]
        fn string_val_iterator(unfiltered_entries: Vec<(String, u8)>) {
            let max_key_len = 20;
            let max_entries = 20;

            let entries: Vec<(String, u8)> =
                unfiltered_entries.into_iter()
                    .map(|(mut k, v)| {
                        while k.len() > max_key_len {
                            k.pop();
                        }
                        (k,v)
                    })
                    .take(max_entries)
                    .collect();

            let mut hm : HashMap<String, u8> = HashMap::new();
            let mut tr : StringTrie<u8> = StringTrie::new();

            for (key, val) in &entries {
                hm.insert(key.clone(), *val);
                tr.insert(key, *val);

                let mut trv : Vec<&u8> = tr
                    .values()
                    .collect();
                let mut hmv : Vec<&u8> = hm
                    .values()
                    .collect();
                trv.sort();
                hmv.sort();
                assert!(trv == hmv);
            }
        }
    }
}
