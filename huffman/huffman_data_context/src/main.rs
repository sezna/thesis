mod data_context;
extern crate rayon;
use std::io::prelude::*;
use std::mem;
use std::io::BufReader;
use std::io::BufRead;
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
    println!("Creating DataContext with no benefits");
    let no_benefits_context = DataContext::new_no_benefits(corpus.clone());
		println!("Creating standard huffman code");
    let standard_huff = DataContext::new_standard_huffman(corpus.clone());
    let to_encode_file = File::open("data/to_encode.txt").unwrap();
		let to_encode = BufReader::new(&to_encode_file);
		let mut average_benefits_compression = 0f64;
		let mut average_no_benefits_compression = 0f64;
		let mut average_huffman_compression = 0f64;
		let mut average_huffman_plus_tree = 0f64;
		let mut count = 0f64;
		let char_size = std::mem::size_of::<char>() * 8;
		for line_result in to_encode.lines() {
				println!("Encoding line number: {}", count);
			  let line = line_result.unwrap();	
				let uncomp_size = (line.chars().count() * char_size) as f64;
				 average_benefits_compression += data_context.encode(&line).chars().count() as f64 / uncomp_size;
				 average_no_benefits_compression += no_benefits_context.encode(&line).chars().count() as f64 / uncomp_size;
				 average_huffman_compression += standard_huff.encode(&line).chars().count() as f64 / uncomp_size;
				 average_huffman_plus_tree += (standard_huff.encode(&line).chars().count() as f64 + 256f64) / uncomp_size;
				 count += 1.0;
		}
    println!("benefits avg ratio: {}\nno benefits avg ratio: {}\nstandard huff no tree: {}\nwith tree: {}:",
		average_benefits_compression, average_no_benefits_compression, average_huffman_compression, average_huffman_plus_tree);

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
