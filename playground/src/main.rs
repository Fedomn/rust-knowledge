#![feature(allocator_api)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(test)]
#![feature(trait_alias)]
#![feature(type_alias_impl_trait)]
#![feature(trace_macros)]
#![feature(portable_simd)]
#![feature(proc_macro_hygiene, stmt_expr_attributes)]
#![feature(generators)]
#![feature(poll_ready)]

mod algorithm;
mod asyncs;
mod basic;
mod concurrency;
mod error;
mod iter;
mod lifetimes;
mod macros;
mod ownership;
mod pin;
mod simd;
mod smart_pointer;
mod tokio;
mod traits;
mod wrapper_type;

fn main() {}
