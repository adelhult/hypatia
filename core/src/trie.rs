use std::collections::HashMap;
use std::hash::Hash;
use std::{iter, mem};

/// A data structure for storing values keyed by arrays of K which allows for retrieval of all
/// values whose keys are prefix of the searched term. If you want to search using [u64], your
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
    fn new() -> Self {
        Self(Node::new())
    }

    fn entries(&self) -> Box<dyn Iterator<Item = (&[K], &V)> + '_> {
        self.0.entries()
    }

    fn values(&self) -> Box<dyn Iterator<Item = &V> + '_> {
        self.0.values()
    }

    fn keys(&self) -> Box<dyn Iterator<Item = &[K]> + '_> {
        self.0.keys()
    }

    /// Searches this Trie for all values for which their keys is prefixes of the given key.
    /// If the key [1,2,3] is searched, the values for [1], [1,2], and [1,2,3] (if they exist)
    /// is returned. Note that [1,2] may exist even though neither [1] nor [1,2,3] exists.
    /// The resulting Vec contains key-value-pairs sorted with the shortest keys first.
    fn search(&self, arr: &[K]) -> Vec<(Vec<K>, V)> {
        self.0.search(&[], arr)
    }

    /// Inserts the given value at the specified path, returning the previous value as an Option
    fn insert(&mut self, path: &[K], val: V) -> Option<V> {
        self.0.insert(0, path, val)
    }
}

/// A wrapper around the data structure Trie for storing values keyed by strings, which allows for
/// retrieval of all values whose keys are prefix of the searched term.
#[derive(Debug, Clone, PartialEq)]
pub struct StringTrie<V>(Trie<char, V>)
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

    pub fn entries(&self) -> Box<dyn Iterator<Item = (String, &V)> + '_> {
        let res = self
            .0
            .entries()
            .map(|(k, v): (&[char], &V)| (k.iter().collect(), v));
        Box::new(res)
    }

    pub fn values(&self) -> Box<dyn Iterator<Item = &V> + '_> {
        self.0.values()
    }

    pub fn keys(&self) -> Box<dyn Iterator<Item = String> + '_> {
        let res = self.0.keys().map(|k: &[char]| k.iter().collect());
        Box::new(res)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.keys().any(|k| k == key)
    }

    /// Searches this StringTrie for all values for which their keys is prefixes of the given key.
    /// If the key "abc" is searched, the values for "a", "b", and "c" (if they exist)
    /// is returned. Note that "ab" may exist even though neither "a" nor "abc exists.
    /// The resulting Vec contains key-value-pairs sorted with the shortest keys first.
    pub fn search(&self, key: &str) -> Vec<(String, V)> {
        let k: Vec<_> = key.chars().collect();
        self.0
            .search(&k)
            .into_iter()
            .map(|(cs, val)| (cs.into_iter().collect(), val))
            .collect()
    }

    /// Inserts the given value at the specified key, returning the previous value as an Option
    pub fn insert(&mut self, key: &str, val: V) -> Option<V> {
        let k: Vec<_> = key.chars().collect();
        self.0.insert(&k, val)
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
}
