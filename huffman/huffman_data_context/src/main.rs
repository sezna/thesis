mod data_context;
extern crate rayon;
use std::io::prelude::*;
use std::fs::File;
use data_context::DataContext;

// TODO: handle not found tokens
//       decode with traversal
//       turn into a library
// 			 split out into multiple mods
//			 serialize context for reuse
//       parallelize

fn main() {
    let mut file = File::open("data/big.txt").expect("Unable to open the file");
    let mut corpus = String::new();
    file.read_to_string(&mut corpus).expect(
        "Unable to read the file",
    );
    println!("Creating DataContext with Benefits");
    let data_context = DataContext::new(corpus.clone());
    println!("Creating standard Huffman");
    let standard_huff = DataContext::new_standard_huffman(corpus);
    println!("Encoding");
    let encoded = data_context.encode("test test this is a test");
		let huff_encoded = standard_huff.encode("test test this is a test");
    //		let standard_encoded = standard_huff.encode("test test this is a test");

    //		println!("length of standard: {}, length with benefits: {}", standard_encoded.len(), encoded.len());
    let decoded = data_context.decode(&encoded);

    println!("encoded len: {}, huff_encoded len: {}, decoded: {}", encoded.len(), huff_encoded.len(), decoded.clone());
}


#[test]
fn test_encode() {

    let corpus = String::from(
        "this this this this test this is just a big old test I can't believe this is just a test",
    );

    let data_context = DataContext::new(corpus);
    let encoded = data_context.encode("test test this is a test");
    assert!(encoded == "00001101110110100");
}

#[test]
fn test_encode_decode() {

    let corpus = String::from(
        "this this this this test this is just a big old test I can't believe this is just a test",
    );
    let message = "test test this is a test";
    let data_context = DataContext::new(corpus);
    let encoded = data_context.encode(message);
    let decoded = data_context.decode(&encoded);
    println!("decoded: {}", decoded);
    println!("message: {}", message.to_string());
    assert!(message.to_string() == decoded)

}
