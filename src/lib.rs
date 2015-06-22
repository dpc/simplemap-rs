// Copyright 2015 Dawid Ciężarkiewicz
// See LICENSE-MPL
//! Simple Map with default for missing values and compacting (removal of
//! default values from underlying map).
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

#![cfg_attr(all(test, feature="bench"), feature(test))]

#[cfg(all(test, feature="bench"))]
extern crate test;

#[cfg(test)]
extern crate rand;

use std::ops::{Index, IndexMut};
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

pub struct SimpleMap<Idx, T> {
    map : BTreeMap<Idx, T>,
    default : T,
    pending : Option<(Idx, T)>
}

impl<Idx, T> SimpleMap<Idx, T>
where Idx : Ord+Clone,
T : Clone+Eq+Default {
    pub fn new() -> SimpleMap<Idx, T> {
        SimpleMap {
            map : BTreeMap::new(),
            default: Default::default(),
            pending: None,
        }
    }
}

impl<Idx, T> SimpleMap<Idx, T>
where Idx : Ord+Clone,
T : Clone+Eq {
    pub fn new_with_default(default : T) -> SimpleMap<Idx, T> {
        SimpleMap {
            map : BTreeMap::new(),
            default: default,
            pending: None,
        }
    }

    fn apply_pending(&mut self) {
       match self.pending {
           Some(ref pending) => {
               let &(ref coord, ref val) = pending;
               if *val == self.default {
                   self.map.remove(coord);
               } else {
                   self.map.insert(coord.clone(), val.clone());
               }
           },
           None => {}
       }
       self.pending = None;
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
impl<Idx, T> Index<Idx> for SimpleMap<Idx, T>
where Idx : Ord {
    type Output = T;
    fn index<'a>(&'a self, index: Idx) -> &'a T {
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
impl<Idx, T> IndexMut<Idx> for SimpleMap<Idx, T>
where
Idx : Ord+Clone,
T : Clone+Eq {
    fn index_mut<'a>(&'a mut self, index: Idx) -> &'a mut T {
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

#[cfg(test)]
mod tests {
    pub use super::*;
    use rand;
    use std::collections::BTreeMap;
    use rand::Rng;

    #[test]
    fn default() {
        let map = SimpleMap::new_with_default(5u32);
        assert_eq!(map[1u32], 5u32);
    }

#[cfg(feature="bench")]
    mod bench {
        use std::collections::BTreeMap;
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

    #[test]
    fn random() {
        let mut bmap = BTreeMap::new();
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
}
