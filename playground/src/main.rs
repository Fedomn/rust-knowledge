#![feature(allocator_api)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(test)]
#![feature(trait_alias)]
#![feature(type_alias_impl_trait)]
#![feature(trace_macros)]
#![feature(portable_simd)]

mod basic;
mod bitvec;
mod closure;
mod concurrency;
mod constant;
mod enums;
mod error;
mod generic;
mod hashmap;
mod iter;
mod lifetimes;
mod macros;
mod modules;
mod ownership;
mod pin;
mod simd;
mod smart_pointer;
mod structs;
mod tokio;
mod traits;
mod wrapper_type;

fn main() {}
