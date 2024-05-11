


// testing out formatting

use std::{marker::PhantomData, ops::Add};

use moveslice::{Error, Moveslice};
use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{effect::Corruption, prelude::Effect};

pub struct Reverse;

impl<V> Corruption<V> for Reverse where V: AsMut<[u8]> {
    fn corrupt(&self, mut item: V) {
       item.as_mut().reverse();
    }

    fn get_name(&self) -> String {
        "Reverse".into()
    }
}

pub struct Voidout;

impl<V> Corruption<V> for Voidout where V: AsMut<[u8]> {
    fn corrupt(&self, mut item: V) {
       for elem in item.as_mut().iter_mut() {
            *elem = 0
       }
    }

    fn get_name(&self) -> String {
        "Voidout".into()
    }
}

pub struct Increment(pub usize);

impl<V> Corruption<V> for Increment where V: AsMut<[u8]> {
    fn corrupt(&self, mut item: V) {
       for elem in item.as_mut().iter_mut() {
            *elem = ((*elem as usize + self.0) % 256) as u8
       }
    }

    fn get_name(&self) -> String {
        "Increment".into()
    }
}

pub struct Accelerate {
    pub start: usize,
    pub accel: usize,
}

impl<V> Corruption<V> for Accelerate where V: AsMut<[u8]> {
    fn corrupt(&self, mut item: V) {
        let mut increment = self.start;
       for elem in item.as_mut().iter_mut() {
            *elem = ((*elem as usize + increment) % 256) as u8;
            increment = increment + self.accel;
       }
    }

    fn get_name(&self) -> String {
        "Accelerate".into()
    }
}

pub struct Swap {
    start1: usize,
    start2: usize,
    chunksize: usize
}

impl<V> Corruption<V> for Swap where V: AsMut<[u8]> {
    fn corrupt(&self, mut item: V) {
        let (l, r) = item.as_mut().split_at_mut(self.start2-1);
        let l_slice = l.get_mut(self.start1..self.chunksize + self.start1);
        let r_slice = r.get_mut(0..self.chunksize);

        if l_slice.is_none() || r_slice.is_none() {
            panic!(
                "swap failed. couldn't get bytes at the following indices:\n
                start:[{}, {}]\n
                end:[{}, {}]\n
                \n
                bytes in slice: {}",
                self.start1,
                self.start1 + self.chunksize,
                self.start2,
                self.start2 + self.chunksize,
                item.as_mut().len(),
            )
        }

        l_slice.unwrap().as_mut().swap_with_slice(
            r_slice.unwrap().as_mut())
    }

    fn get_name(&self) -> String {
        "Swap".into()
    }
}

#[cfg(feature = "shift")]
pub struct Shift {
    pub subslice: (usize, usize),
    pub shift_by: isize,
}

#[cfg(feature = "shift")]
impl<V> Corruption<V> for Shift where V: AsMut<[u8]> {
    fn corrupt(&self, mut item: V) {
       let result = item.as_mut().try_moveslice(
        self.subslice.0..self.subslice.1,
        self.subslice.0.saturating_add_signed(self.shift_by));

        match result {
            Ok(()) => return,
            Err(Error::InvalidBounds { len, bounds }) => {
                panic!("
                    Bounds are invalid!\n
                    Length of bytes = {}\n
                    Bounds = ({}, {})\n
                ", len, bounds.0, bounds.1);
            },
            Err(Error::OutOfBoundsMove { len, dest }) => {
                panic!("
                    Shift would have gone out of bounds!\n
                    Length of bytes = {}\n
                    Specified Destination = (start={}, end={})
                ", len, dest.0, dest.1);
            }
        }
    }

    fn get_name(&self) -> String {
        "Shift".into()
    }
}

pub struct Loop {
    pub chunk: usize,
    pub times: usize,
}

impl<V> Corruption<V> for Loop where V: AsMut<[u8]> {
    fn corrupt(&self, mut item: V) {
        let to_modify = item.as_mut();

        let (template, to_overwrite) = to_modify.split_at_mut(self.chunk);

        let loop_from = self.chunk;
        let mut times = 0;

        while times < self.times && loop_from < to_overwrite.len() {
            times = times + 1;

            for (idx, byte) in to_overwrite.iter_mut().enumerate() {
                *byte = template[idx % template.len()];
            }
        }
    }

    fn get_name(&self) -> String {
        "Loop".into()
    }
}

#[cfg(feature = "random")]
pub struct Shuffle;

#[cfg(feature = "random")]
impl<V> Corruption<V> for Shuffle where V: AsMut<[u8]> {
    fn corrupt(&self, mut item: V) {
       item.as_mut().shuffle(&mut thread_rng())
    }

    fn get_name(&self) -> String {
        "Shuffle".into()
    }
}

#[cfg(feature = "random")]
pub struct Entropy;

#[cfg(feature = "random")]
impl<V> Corruption<V> for Entropy where V: AsMut<[u8]> {
    fn corrupt(&self, mut item: V) {
       for elem in item.as_mut().iter_mut() {
            *elem = thread_rng().gen_range(u8::MIN..u8::MAX)
       }
    }

    fn get_name(&self) -> String {
        "Entropy".into()
    }
}

pub struct PartialCorrupt<C: Corruption<V>, V: AsMut<[u8]>> {
    pub corruption: C,
    start_chunk: usize,
    end_chunk: usize,
    _phantom: PhantomData<V>,
}

impl<C: Corruption<V>, V: AsMut<[u8]>> PartialCorrupt<C, V> {
    pub fn from_corruption(corruption: C, start_chunk: usize, end_chunk: usize) -> PartialCorrupt<C, V> {
        PartialCorrupt {
            corruption,
            start_chunk,
            end_chunk,
            _phantom: PhantomData
        }
    }
}

impl<'a, C> Corruption<&'a mut [u8]> for PartialCorrupt<C, &'a mut [u8]> where
    C: Corruption<&'a mut [u8]>,
{
    fn corrupt(&self, item: &'a mut [u8]) {
        let item_len = item.len();
        let slice = item.as_mut().get_mut(self.start_chunk..self.end_chunk);

        match slice {
            Some(part) => {
                self.corruption.corrupt(part);
            },
            None => {
                panic!(
                    "\n
                     Failed to extract chunk from slice when corrupting... Effect: {}
                     Chunk[start={}, end={}]
                     Item Length: {}
                    ",
                     self.corruption.get_name(),
                    self.start_chunk, self.end_chunk,
                    item_len
                );
            }
        }
    }

    fn get_name(&self) -> String {
        format!("Partial[{}]", self.corruption.get_name())
    }
}