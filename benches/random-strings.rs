//! Symtern benchmarks using short random strings as inputs.
#![feature(test)]
extern crate test;
extern crate rand;
extern crate symtern;
#[macro_use] extern crate lazy_static;

use rand::Rng;
use test::Bencher;
use symtern::traits::*;
use symtern::basic;
use symtern::short;

const TEST_STRING_CHARS: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

lazy_static! {
    static ref TEST_STRINGS_4: Vec<String> = generate_strings(100_000, 4);
    static ref TEST_STRINGS_8: Vec<String> = generate_strings(100_000, 8);
    static ref TEST_STRINGS_16: Vec<String> = generate_strings(100_000, 16);
    static ref TEST_STRINGS_32: Vec<String> = generate_strings(100_000, 32);
}

fn generate_string(dest: &mut String, length: usize, chars: &[char]) {
    dest.clear();
    let mut rng = rand::thread_rng();
    for _ in 0..length {
        dest.push(chars[rng.gen::<usize>() % chars.len()]);
    }
}

fn generate_strings(n: usize, length: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut s = String::with_capacity(length);
    for _ in 0..n {
        generate_string(&mut s, length, &TEST_STRING_CHARS);
        out.push(s.clone());
    }
    out
}

macro_rules! bench_intern_fn {
    ($name: ident, $new: expr, $strings_set: ident, $len: expr) => {
        #[bench]
        #[allow(unused_mut)]
        fn $name(b: &mut Bencher) {
            let mut strings = $strings_set.clone();
            let mut pool = $new;
            b.iter(|| pool.intern(&strings.pop().expect("ran out of test strings")[..]));
            b.bytes = ($strings_set.len() - strings.len()) as u64 * $len ;
        }
    };
}

macro_rules! bench_resolve_fn {
    ($name: ident, $new: expr, $strings_set: ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut pool = $new;
            let strings = &$strings_set;
            let mut symbols = strings.iter().map(|s| pool.intern(&s[..]).expect("failed to intern string")).collect::<Vec<_>>();
            b.iter(|| pool.resolve(symbols.pop().expect("ran out of test symbols")).expect("resolution failure"));
        }
    };
    ($name: ident, $new: expr, $strings_set: ident, self_resolve) => {
        #[bench]
        #[allow(unused_mut)]
        fn $name(b: &mut Bencher) {
            let pool = $new;
            let strings = &$strings_set;
            let mut symbols = strings.iter().map(|s| pool.intern(&s[..]).expect("failed to intern string")).collect::<Vec<_>>();
            b.iter(|| { let sym = symbols.pop().expect("ran out of test symbols");
                        sym.resolve(); });
        }
    }
}

bench_intern_fn!(intern_basic_4 , basic::Pool::<str,u32>::new(), TEST_STRINGS_4, 4);
bench_intern_fn!(intern_basic_8 , basic::Pool::<str,u32>::new(), TEST_STRINGS_8, 8);
bench_intern_fn!(intern_basic_16, basic::Pool::<str,u32>::new(), TEST_STRINGS_16, 16);
bench_intern_fn!(intern_basic_32, basic::Pool::<str,u32>::new(), TEST_STRINGS_16, 32);

bench_intern_fn!(intern_short_4 , short::Pool::new()           , TEST_STRINGS_4, 4);
bench_intern_fn!(intern_short_8 , short::Pool::new()           , TEST_STRINGS_8, 8);
bench_intern_fn!(intern_short_16, short::Pool::new()           , TEST_STRINGS_16, 16);
bench_intern_fn!(intern_short_32, short::Pool::new()           , TEST_STRINGS_16, 32);

bench_resolve_fn!(resolve_basic_4 , basic::Pool::<str,u32>::new(), TEST_STRINGS_4);
bench_resolve_fn!(resolve_basic_8 , basic::Pool::<str,u32>::new(), TEST_STRINGS_8);
bench_resolve_fn!(resolve_basic_16, basic::Pool::<str,u32>::new(), TEST_STRINGS_16);
bench_resolve_fn!(resolve_basic_32, basic::Pool::<str,u32>::new(), TEST_STRINGS_32);

bench_resolve_fn!(resolve_short_4 , short::Pool::new()           , TEST_STRINGS_4, self_resolve);
bench_resolve_fn!(resolve_short_8 , short::Pool::new()           , TEST_STRINGS_8, self_resolve);
bench_resolve_fn!(resolve_short_16, short::Pool::new()           , TEST_STRINGS_16, self_resolve);
bench_resolve_fn!(resolve_short_32, short::Pool::new()           , TEST_STRINGS_32, self_resolve);
