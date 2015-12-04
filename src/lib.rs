// Copyright 2015 Dawid Ciężarkiewicz
// See LICENSE-MPL
//! Simple Map with default for missing values and compacting (removal of
//! elements with default value from underlying map).
//!
//! So you can just:
//!
//! ```
//! use simplemap::SimpleMap;
//!
//! let mut map = SimpleMap::new();
//!
//! assert_eq!(map[0u32], 0u32);
//! map[1] = 3;
//! assert_eq!(map[1], 3);
//! ```

#![feature(hashmap_hasher)]

#![cfg_attr(all(test, feature="bench"), feature(test))]

#[cfg(all(test, feature="bench"))]
extern crate test;
extern crate fnv;


#[cfg(test)]
extern crate rand;

use std::ops::{Index, IndexMut};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::hash_state::{DefaultState, HashState};
use std::iter::Chain;
use std::hash::Hash;
use fnv::FnvHasher;
use std::fmt;

/// SimpleMap
///
/// Simple Map with default for missing values and compacting (removal of
/// elements with default value from underlying map).
#[derive(Clone)]
pub struct SimpleMap<K, V, S = DefaultState<FnvHasher>> {
    map : HashMap<K, V, S>,
    default : V,
    pending : Option<(K, V)>
}

impl<K, V> SimpleMap<K, V>
where K : Ord+Clone+Hash,
      V : Clone+Eq+Default,
{
    /// Create a `SimpleMap`.
    ///
    /// `Default::default()` will be used as a default value.
    pub fn new() -> SimpleMap<K, V, DefaultState<FnvHasher>> {
        SimpleMap {
            map : Default::default(),
            default: Default::default(),
            pending: None,
        }
    }
}

impl<K, V, S> SimpleMap<K, V, S>
where K : Ord+Clone+Hash,
      V : Clone+Eq+Default,
      S : HashState+Default
{
    /// Create a `SimpleMap`.
    ///
    /// `Default::default()` will be used as a default value.
    pub fn with_hash_state(hash_state : S) -> SimpleMap<K, V, S> {
        SimpleMap {
            map : HashMap::with_hash_state(hash_state),
            default: Default::default(),
            pending: None,
        }
    }
}


impl<K, V> SimpleMap<K, V>
where K : Ord+Clone+Hash,
      V : Clone+Eq,
{
    /// Create a `SimpleMap` with custom default value.
    pub fn new_with_default(default : V) -> SimpleMap<K, V, DefaultState<FnvHasher>> {
        SimpleMap {
            map : Default::default(),
            default: default,
            pending: None,
        }
    }
}


impl<K, V, S> SimpleMap<K, V, S>
where K : Ord+Clone+Hash,
      S : HashState+Default,
      V : Clone+Eq,
{
    pub fn with_default_with_hash_state(default : V, hash_state: S) -> SimpleMap<K, V, S> {
        SimpleMap {
            map : HashMap::with_hash_state(hash_state),
            default: default,
            pending: None,
        }
    }
}

impl<K, V, S> SimpleMap<K, V, S>
where K : Ord+Clone+Hash,
      V : Clone+Eq,
      S: HashState,
{
    fn apply_pending(&mut self) {
       match self.pending {
           Some(ref pending) => {
               let &(ref idx , ref val) = pending;
               if *val == self.default {
                   self.map.remove(idx);
               } else {
                   self.map.insert(idx.clone(), val.clone());
               }
           },
           None => {}
       }
       self.pending = None;
    }

    /// Make sure to remove all elements with default value.
    pub fn compact(&mut self) {
        self.apply_pending();
    }

    /// Iterator over map elements with non-default values.
    ///
    /// Note: It might return elements with default value, unless `compact`
    /// is called before `iter()`.
    pub fn iter<'a>(&'a self) -> Chain<std::collections::hash_map::Iter<'a, K, V>, std::iter::Map<std::option::Iter<'a, (K, V)>, fn(&'a (K, V)) -> (&'a K, &'a V)>> {
        let SimpleMap {
            ref map,
            ref pending,
            ..
        } = *self;

        let f: fn(&'a (K, V)) -> (&'a K, &'a V) = ref_to_tuple_to_tuple_of_refs;

        map.iter().chain(pending.iter().map(f))
    }
}

impl<K, V, S> SimpleMap<K, V, S>
where K : Ord+Clone+Hash,
      S: HashState,
      V : Clone+Eq,
{
    /// Iterator yielding (K, V) instead of (&K, &V)
    pub fn iter_cloned<'a>(&'a self) ->
        Chain<
            std::iter::Map<std::collections::hash_map::Iter<'a, K, V>, fn((&'a K, &'a V)) -> (K, V)>,
            std::iter::Cloned<std::option::Iter<'a, (K, V)>>
        >
    {
        let SimpleMap {
            ref map,
            ref pending,
            ..
        } = *self;

        let f: fn((&'a K, &'a V)) -> (K, V) = tuple_of_refs_to_tuple;

        map.iter().map(f).chain(pending.iter().cloned())
    }

}

fn ref_to_tuple_to_tuple_of_refs<'a, K, V>(t : &'a(K, V)) -> (&'a K, &'a V) {
    let &(ref i, ref t) = t;
    (i, t)
}

fn tuple_of_refs_to_tuple<'a, K : Clone, V : Clone>(t : (&'a K, &'a V)) -> (K, V) {
    let (i, t) = t;
    (i.clone(), t.clone())
}

use std::iter::FromIterator;
use std::iter::IntoIterator;

impl<K, V, S> FromIterator<(K, V)> for SimpleMap<K, V, S>
where K: Ord+Hash, V: Default,
      S: HashState+Default {
    fn from_iter<I>(iterator: I) -> SimpleMap<K, V, S>
        where I: IntoIterator<Item=(K, V)> {
            SimpleMap {
                default: Default::default(),
                map: FromIterator::from_iter(iterator),
                pending: None,
            }
    }
}

/// ```
/// use simplemap::SimpleMap;
///
/// let mut map = SimpleMap::new();
///
/// let val : u32 = map[0u32];
/// assert_eq!(val, 0);
/// ```
impl<K, V, S> Index<K> for SimpleMap<K, V, S>
where K : Ord+Hash,
      S : HashState+Default,
{
    type Output = V;
    fn index<'a>(&'a self, index: K) -> &'a V {
        match self.pending {
            Some(ref pending) => {
               let &(ref i, ref val) = pending;
               if *i == index {
                   return val
               }
            }
            None => {},
        }

        match self.map.get(&index) {
            Some(entry) => entry,
            None => &self.default,
        }
    }
}

/// ```
/// use simplemap::SimpleMap;
///
/// let mut map = SimpleMap::new();
///
/// map[1u32] = 3i32;
/// assert_eq!(map[1], 3);
/// ```
impl<K, V, S> IndexMut<K> for SimpleMap<K, V, S>
where
K : Ord+Clone+Hash,
V : Clone+Eq,
      S : HashState+Default,
{
    fn index_mut<'a>(&'a mut self, index: K) -> &'a mut V {
        self.apply_pending();

        match self.map.entry(index.clone()) {
            Entry::Occupied(entry) => {
                self.pending = Some((index, entry.get().clone()));
                let &mut (_, ref mut val) = self.pending.as_mut().unwrap();
                val
            },
            Entry::Vacant(_) => {
                self.pending = Some((index, self.default.clone()));
                let &mut (_, ref mut val) = self.pending.as_mut().unwrap();
                val
            }
        }
    }
}

impl<K, V, S> fmt::Debug for SimpleMap<K, V, S>
where K : Ord+Eq+Clone+Hash+fmt::Debug,
      V : Clone+Eq+fmt::Debug,
      S: HashState,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, V> Default for SimpleMap<K, V>
where K : Ord+Clone+Hash,
      V : Clone+Eq+Default,
{
     fn default() -> Self {
         SimpleMap::with_default_with_hash_state(Default::default(), Default::default())
     }
}


#[cfg(test)]
mod tests {
    pub use super::*;
    use std::collections::HashMap;
    use std::collections::hash_state::{DefaultState};
    use fnv::FnvHasher;
    use rand::Rng;
    use rand;

    #[test]
    fn default() {
        let map = SimpleMap::new_with_default(5u32);
        assert_eq!(map[1u32], 5u32);
    }

    #[test]
    fn iter() {
        let mut map = SimpleMap::new();

        map[0u32] = 3i32; // counts
        map[1u32] = 0i32; // default, doesn't count
        map[2u32] = 2i32; // counts
        map[0u32] = 2i32; // replaces the existing one
        let _ = map[0u32]; // shouldn't change anything

        map.compact();
        assert_eq!(map.iter().count(), 2);
    }

    #[test]
    fn random() {
        let mut bmap : HashMap<_, _, DefaultState<FnvHasher>> = Default::default();
        let mut smap = SimpleMap::new();

        let mut rng = rand::thread_rng();

        for val in 0u32..10000 {
            let idx = rng.gen_range(-5i32, 5);
            let bval = *bmap.get(&idx).unwrap_or(&Default::default());
            let sval = smap[idx];
            assert_eq!(bval, sval);
            bmap.insert(idx, val);
            smap[idx] = val;
        }
    }

    #[test]
    fn strings() {
        let mut smap = SimpleMap::new();

        smap["one"] = 1u32;
        smap["two"] = 2u32;

        assert_eq!(smap["zero"], 0u32);
    }

#[cfg(feature="bench")]
    mod bench {
        use std::collections::BTreeMap;
        use std::collections::HashMap;
        use std::collections::hash_state::{DefaultState};
        use fnv::FnvHasher;
        use super::*;
        use test::Bencher;
        use test;
#[bench]
        fn normal_btreemap_insert(b : &mut Bencher) {
            let mut map = BTreeMap::new();

            let mut i = 0u32;
            b.iter(|| {
                map.insert(i, i);
                i = i.wrapping_add(i);
            });
        }

#[bench]
        fn normal_btreemap_get(b : &mut Bencher) {
            let mut map = BTreeMap::new();

            for i in 0u32..10000 {
                map.insert(i, i);
            }

            let mut i = 0u32;
            b.iter(|| {
                test::black_box(map.get(&i));
                i = i.wrapping_add(i);
            });
        }

#[bench]
        fn normal_hashmap_insert(b : &mut Bencher) {
            let mut map : HashMap<_, _, DefaultState<FnvHasher>> = Default::default();

            let mut i = 0u32;
            b.iter(|| {
                map.insert(i, i);
                i = i.wrapping_add(i);
            });
        }

#[bench]
        fn normal_hashmap_get(b : &mut Bencher) {
            let mut map : HashMap<_, _, DefaultState<FnvHasher>> = Default::default();

            for i in 0u32..10000 {
                map.insert(i, i);
            }

            let mut i = 0u32;
            b.iter(|| {
                test::black_box(map.get(&i));
                i = i.wrapping_add(i);
            });
        }


#[bench]
        fn compact_map_idx_assign(b : &mut Bencher) {
            let mut map = SimpleMap::new();

            let mut i = 0u32;
            b.iter(|| {
                map[i] = i;
                i = i.wrapping_add(i);
            });
        }

#[bench]
        fn compact_map_idx_get(b : &mut Bencher) {
            let mut map = SimpleMap::new();

            for i in 0u32..10000 {
                map[i] = i;
            }

            let mut i = 0u32;
            b.iter(|| {
                test::black_box(map[i]);
                i = i.wrapping_add(i);
            });
        }
    }
}
