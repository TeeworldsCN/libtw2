#![feature(test)]

extern crate huffman;
extern crate itertools;
extern crate num;
extern crate test;

use huffman::Huffman;
use itertools::Itertools;
use num::ToPrimitive;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::u8;

fn read_file<T, F: FnMut(String) -> T>(filename: &str, f: F) -> Vec<T> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|ml| ml.unwrap())
        .map(f)
        .collect()
}

fn huffman_default() -> Huffman {
    let frequencies = read_file("data/frequencies", |l| {
        u32::from_str_radix(&l, 10).unwrap()
    });

    Huffman::from_frequencies(&frequencies).unwrap()
}

fn test_cases() -> Vec<(Vec<u8>, Vec<u8>)> {
    read_file("data/test_cases", |l| {
        let mut v = l.split('#').map(|hex_bytes| {
            hex_bytes.split(' ').map(|hex| u8::from_str_radix(hex, 16).unwrap()).collect()
        }).collect_vec().into_iter();
        assert_eq!(v.len(), 2);
        let uncompressed = v.next().unwrap();
        let compressed = v.next().unwrap();
        (uncompressed, compressed)
    })
}

#[bench]
fn decompress(b: &mut test::Bencher) {
    let h = huffman_default();
    let test_cases = test_cases();
    let mut buffer = (0..10240).map(|_| 0).collect_vec();
    b.iter(|| {
        for &(_, ref compressed) in &test_cases {
            test::black_box(h.decompress(compressed, &mut buffer));
        }
    });
    b.bytes = test_cases.iter().map(|&(_, ref compressed)| compressed.len())
        .fold(0, |s, l| s + l.to_u64().unwrap());
}
