//! Benchmark for the memory functions
//!
//! Run with `cargo bench --features mem`

#![feature(compiler_builtins_lib)]

extern crate compiler_builtins;

#[macro_use]
extern crate criterion;

use compiler_builtins::mem;
use std::ptr;
use criterion::Criterion;

const SIZE: usize = 4 * 1024 * 1024;

fn memcpy(c: &mut Criterion) {
    let input = vec![0x1234_ABCDu32; SIZE];
    let mut output = vec![0u32; SIZE];

    let input_ptr = input.as_ptr() as *const u8;
    let output_ptr = output.as_mut_ptr() as *mut u8;
    let len = 4 * SIZE;

    c.bench_function("Rust memcpy 16 MiB", move |b| b.iter(|| unsafe {
        mem::memcpy(output_ptr, input_ptr, len);
    }));
    c.bench_function("libc memcpy 16 MiB", move |b| b.iter(|| unsafe {
        ptr::copy_nonoverlapping(input_ptr, output_ptr, len);
    }));
}

fn memset(c: &mut Criterion) {
    let mut buf = vec![0u32; SIZE];

    let ptr = buf.as_mut_ptr() as *mut u8;
    let pattern = 0xA;
    let len = 4 * SIZE;

    c.bench_function("Rust memset 16 MiB", move |b| b.iter(|| unsafe {
        mem::memset(ptr, pattern as i32, len);
    }));
    c.bench_function("libc memset 16 MiB", move |b| b.iter(|| unsafe {
        ptr::write_bytes(ptr, pattern, len);
    }));
}

criterion_group!(benches, memcpy, memset);
criterion_main!(benches);
