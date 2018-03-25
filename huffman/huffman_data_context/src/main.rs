use std::collections::HashMap;

// TODO: handle not found tokens
//       decode with traversal
//       turn into a library
// 			 split out into multiple mods

/// A node for the DataContext.
#[derive(Clone)]
enum Node {
    Interior {
        left: Box<Node>,
        right: Box<Node>,
        benefit: u64,
    },
    Leaf { data: String, benefit: u64 },
}


impl Node {
    pub fn benefit(&self) -> u64 {
        return match self {
            &Node::Interior { benefit, .. } => benefit,
            &Node::Leaf { benefit, .. } => benefit,
        };
    }

    pub fn traverse(&self, token: &str) -> Option<String> {
        match self {
            &Node::Leaf { ref data, .. } => {
                if data == token {
                    Some("".to_string())
                } else {
                    None
                }
            }
            &Node::Interior { .. } => self.handle_interior_traversal(token),
        }
    }

    fn children(&self) -> String {
        return match self {
            &Node::Interior {
                ref left,
                ref right,
                ..
            } => format!("Node: [l: {} || r: {} ]", left.children(), right.children()),
            &Node::Leaf { ref data, .. } => format!("data: {}", data),

        };

    }

    fn handle_interior_traversal(&self, token: &str) -> Option<String> {
        let mut to_return = "".to_string();
        match self {
            &Node::Interior {
                ref left,
                ref right,
                ..
            } => {
                if let Some(result) = left.clone().traverse(token) {
                    to_return = format!("0{}", result)
                };
                if let Some(result) = right.clone().traverse(token) {
                    to_return = format!("1{}", result)
                };
            }
            &Node::Leaf { .. } => {
                println!("This function shouldn't be called on a leaf");
            }
        }
        if to_return == "".to_string() {
            return None;
        } else {
            return Some(to_return);
        }
    }

    pub fn make_table(&self, tokens: Vec<&str>) -> HashMap<String, String> {
        let mut to_return = HashMap::new();
        for token in tokens {
            to_return.insert(
                token.to_string(),
                self.traverse(token).expect("token not found"),
            );

        }
        return to_return;

    }

    /// Finds a token in the encoding table and returns a binary string.
    pub fn decode(&self, to_decode: &str) -> String {
        let mut tree_position = self;
        let mut to_return_string = "".to_string();
        for direction in to_decode.chars() {
            match direction {
                '0' => {
                    match tree_position {
                        &Node::Interior { ref left, .. } => {
                            tree_position = &*left;
                        }

                        &Node::Leaf { ref data, .. } => {
                            tree_position = self;
                            to_return_string = format!("{} {}", to_return_string, data);
                        }
                    }
                }
                '1' => {
                    match self {
                        &Node::Interior { ref right, .. } => {
                            tree_position = &*right;
                        }

                        &Node::Leaf { ref data, .. } => {
                            tree_position = self;
                            to_return_string = format!("{} {}", to_return_string, data);
                        }
                    }
                }
                _ => (),
            }
        }
        return to_return_string;
    }

    pub fn visualize_tree(&self) {
        println!("{}", self.children());


    }
}

/// Modified Huffman code-type tree.
struct DataContext {
    _context_id: String,
    root: Node,
    encoding_table: HashMap<String, String>,
}

impl DataContext {
    pub fn encode(&self, to_encode: &str) -> String {
        let tokens: Vec<&str> = to_encode.split(" ").collect();
        let mut encoded_string = "".to_string();
        for token in tokens {
            let pattern = self.lookup_token(token);
            if let Some(x) = pattern {
                encoded_string = format!("{}{}", encoded_string, x);
            } else {
                println!("Something went wrong in encoding");
            }
        }
        return encoded_string;

    }

    fn lookup_token(&self, token: &str) -> Option<&str> {
        //							return	self.encoding_table.get(token);
        for (encoded_token, encoding) in &self.encoding_table {
            if encoded_token == token {
                return Some(encoding);
            }
        }
        return None;
    }

    fn lookup_encoding(&self, binary_string: &str) -> Option<&str> {
        for (token, encoding) in &self.encoding_table {
            if encoding == binary_string {
                return Some(token);
            }
        }
        return None;
    }
    pub fn decode(&self, to_decode: &str) -> String {
        let mut output = "".to_string();
        let mut currently_checking = "".to_string();
        for x in to_decode.chars() {
            currently_checking = format!("{}{}", currently_checking, x);
            if let Some(result) = self.lookup_encoding(&currently_checking) {
                output = format!("{} {}", output, result);
                currently_checking = "".to_string();
            }
        }
        output.remove(0);
        return output;

    }

    /// Creates a DataContext based on a corpus.
    fn new(corpus: String) -> DataContext {
        // Gather vector of pointers to individual words in the corpus.
        let mut tokens: Vec<&str> = corpus.split(" ").collect();

        // Create unique tuples of tokens and their benefit.
        let mut tokens_with_benefit: Vec<(&str, u64)> = tokens
            .iter()
            .map(|x| {
                (
                    *x,
                    (tokens.iter().filter(|&y| y == x).count() * x.len()) as u64,
                )
            })
            .collect();
        tokens_with_benefit.push(("token not contained", 0u64));
        tokens_with_benefit.sort();
        tokens_with_benefit.dedup();
        tokens_with_benefit.sort_by_key(|x| x.1);

        // Create the Huffman tree. At this point, tokens_with_benefit is sorted
        // by lowest benefit to highest benefit.
        let mut forest: Vec<Node> = tokens_with_benefit
            .iter()
            .map(|x| {
                Node::Leaf {
                    data: x.0.to_string(),
                    benefit: x.1,
                }
            })
            .collect();
        forest.sort_by_key(|x| x.benefit());

        while forest.len() > 1 {
            let new_tree = Node::Interior {
                left: Box::new(forest[0].clone()),
                right: Box::new(forest[1].clone()),
                benefit: forest[0].benefit() + forest[1].benefit(),
            };
            forest.remove(1);
            forest.remove(0);
            forest.push(new_tree);
            forest.sort_by_key(|x| x.benefit());
        }

        tokens.sort();
        tokens.dedup();
        assert!(forest.len() == 1);
        return DataContext {
            _context_id: "Test".to_string(),
            root: forest[0].clone(),
            encoding_table: forest[0].clone().make_table(tokens),
        };
    }
		
		/// Creates a DataContext that has no benefit calculation,
		/// each token is placed on the tree based only on its
		/// frequency of occurrence. 
		pub fn new_standard_huffman(corpus: String) -> DataContext {

        // Gather vector of pointers to individual words in the corpus.
        let mut tokens: Vec<&str> = corpus.split(" ").collect();

        // Create unique tuples of tokens and their frequency.
        let mut tokens_with_frequency: Vec<(&str, u64)> = tokens
            .iter()
            .map(|x| {
                (
                    *x,
                    tokens.iter().filter(|&y| y == x).count() as u64,
                )
            })
            .collect();
        tokens_with_frequency.push(("token not contained", 0u64));
        tokens_with_frequency.sort();
        tokens_with_frequency.dedup();
        tokens_with_frequency.sort_by_key(|x| x.1);

        // Create the Huffman tree. At this point, tokens_with_frequency is sorted
        // by lowest frequency to highest frequency.
        let mut forest: Vec<Node> = tokens_with_frequency
            .iter()
            .map(|x| {
                Node::Leaf {
                    data: x.0.to_string(),
                    benefit: x.1,
                }
            })
            .collect();
        forest.sort_by_key(|x| x.benefit());

        while forest.len() > 1 {
            let new_tree = Node::Interior {
                left: Box::new(forest[0].clone()),
                right: Box::new(forest[1].clone()),
                benefit: forest[0].benefit() + forest[1].benefit(),
            };
            forest.remove(1);
            forest.remove(0);
            forest.push(new_tree);
            forest.sort_by_key(|x| x.benefit());
        }

        tokens.sort();
        tokens.dedup();
        assert!(forest.len() == 1);
        return DataContext {
            _context_id: "Test".to_string(),
            root: forest[0].clone(),
            encoding_table: forest[0].clone().make_table(tokens),
        };

		}
}


fn main() {

    let corpus = String::from(
        "this this this this test this is just a big old test I can't believe this is just a test",
    );

    let data_context = DataContext::new(corpus.clone());
		let standard_huff = DataContext::new_standard_huffman(corpus);
    let encoded = data_context.encode("test test this is a test");
		let standard_encoded = standard_huff.encode("test test this is a test");

		println!("length of standard: {}, length with benefits: {}", standard_encoded.len(), encoded.len());
    let decoded = data_context.decode(&encoded);
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
