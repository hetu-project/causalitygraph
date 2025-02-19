//! Verifiable logical clock.
//!
//! This crate implements a verifiable logical clock construct. The clock
//! can be used in a peer-to-peer network to order events. Any node in the
//! network can verify the correctness of the clock.

use serde::{Deserialize, Serialize};
use std::cmp;
use prost::Message;


/// vlc_type = request / sync
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VlcMsg {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub vlc_meta: ::prost::alloc::vec::Vec<u8>,
}

/// type = vlc
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZMessage {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub msg_meta: ::prost::alloc::vec::Vec<u8>,
}

/// vlc_type_request_meta
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VlcMeta {
    #[prost(message, optional, tag = "1")]
    pub clock_state: ::core::option::Option<Clock>,
    #[prost(bytes = "vec", tag = "2")]
    pub event_meta: ::prost::alloc::vec::Vec<u8>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Clock {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub value: u64,
    #[prost(message, repeated, tag = "3")]
    pub ancestors: ::prost::alloc::vec::Vec<Clock>,
}
impl Into<Vec<u8>> for ZMessage {
    fn into(self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.encode(&mut buf).unwrap();
        buf
    }
}
impl Into<Vec<u8>> for Clock {
    fn into(self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.encode(&mut buf).unwrap();
        buf
    }
}

impl PartialOrd for Clock {
    fn partial_cmp(&self, other: &Clock) -> Option<cmp::Ordering> {
        println!("in compare");
        if self.id == other.id {
            return self.value.partial_cmp(&other.value);
        } else {
            // If current clock is <= to any of the other clock's ancestors,
            // the clock is ordered before the other clock.
            for anc in &other.ancestors {
                match self.partial_cmp(anc) {
                    Some(cmp::Ordering::Less) | Some(cmp::Ordering::Equal) => {
                        return Some(cmp::Ordering::Less);
                    }
                    _ => (),
                }
            }
            // Do the same check with the reverse direction
            for anc in &self.ancestors {
                match other.partial_cmp(anc) {
                    Some(cmp::Ordering::Less) | Some(cmp::Ordering::Equal) => {
                        return Some(cmp::Ordering::Greater);
                    }
                    _ => (),
                }
            }
        }
        None
    }
}

impl Clock {
    /// Create a new clock.
    pub fn new(id: String) -> Self {
        Self {
            id,
            value: 0,
            ancestors: Vec::new(),
        }
    }

    /// Create a new clock that extends other clocks.
    pub fn create(id: String, ancestors: &Vec<Clock>) -> Self {
        Self {
            id,
            value: 0,
            ancestors: ancestors.clone(),
        }
    }

    /// Increment the clock
    pub fn inc(&mut self) {
        // If clock value overflows, panic
        assert_ne!(self.value.checked_add(1), None);
        self.value += 1;
    }

    /// Reset the clock.
    pub fn clear(&mut self) {
        self.value = 0;
        self.ancestors.clear();
    }

    /// Merge the clock with other clocks.
    pub fn merge(&mut self, others: &Vec<&Clock>) {
        for &clock in others {
            match self.clone().partial_cmp(clock) {
                Some(cmp::Ordering::Less) => {
                    self.value = clock.value;
                    self.ancestors = clock.ancestors.clone();
                }
                Some(cmp::Ordering::Equal) => (),
                Some(cmp::Ordering::Greater) => (),
                None => {
                    self.ancestors.push(clock.clone());
                }
            }
        }
    }

    pub fn get_value(&self) -> u64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clock_inc() {
        let mut c = Clock::new(1);
        c.inc();
        c.inc();
        assert_eq!(c.value, 2);
    }

    #[test]
    fn clock_cmp() {
        let mut c1 = Clock::new(1);
        let c2 = c1.clone();
        let c3 = Clock::new(2);

        assert_eq!(c1, c2);
        assert_eq!(c1.partial_cmp(&c3), None);
        assert_eq!(c2.partial_cmp(&c3), None);

        c1.inc();
        assert_eq!(c2.partial_cmp(&c1), Some(cmp::Ordering::Less));
        assert_eq!(c3.partial_cmp(&c1), None);
    }

    #[test]
    fn clock_merge() {
        let mut c1 = Clock::new(1);
        let mut c2 = Clock::new(2);
        let mut c3 = Clock::new(3);

        c1.inc();
        c2.inc();
        c3.inc();

        assert_eq!(c1.partial_cmp(&c2), None);
        assert_eq!(c1.partial_cmp(&c3), None);
        assert_eq!(c2.partial_cmp(&c3), None);

        c1.merge(&vec![&c2, &c3]);
        assert_eq!(c2.partial_cmp(&c1), Some(cmp::Ordering::Less));
        assert_eq!(c1.partial_cmp(&c2), Some(cmp::Ordering::Greater));
        assert_eq!(c3.partial_cmp(&c1), Some(cmp::Ordering::Less));
        assert_eq!(c1.partial_cmp(&c3), Some(cmp::Ordering::Greater));
    }
}
