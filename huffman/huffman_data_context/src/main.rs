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
								&Node::Interior { ref left, ref right, .. }=> format!("Node: [l: {} || r: {} ]", left.children(), right.children()),
								&Node::Leaf { ref data, .. }  => format!("data: {}", data)

				}
				
		}
		

    fn handle_interior_traversal(&self, token: &str) -> Option<String> {
        let mut to_return = "".to_string();
        match self {
            &Node::Interior {
                ref left,
                ref right,
                ..
            } => {
                // TODO: These expects will cause a crash. Need to just return
                // none if there is no left. Ideally, they should all be balanced
                // in a Huffman though?
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
				    println!("token {}, encoded {}", token, to_return);
            return Some(to_return);
        }
    }

		pub fn decode(&self, to_decode: &str) -> String {
				let mut tree_position = self;
				let mut to_return_string = "".to_string();
				for direction in to_decode.chars() {
								match direction {
												'0' => { 
																println!("going left");
																match tree_position {
																				&Node::Interior { ref left, .. } => { 
																								tree_position = &*left;
																				},

																				&Node::Leaf { ref data, .. }=> { 
																								println!("found data: {}", data);
																								tree_position = self;
																								to_return_string = format!("{} {}", to_return_string, data);
																				}
																}

												},
												'1' => {
																println!("going right");
																match self {
																				&Node::Interior { ref right, .. } => { 
																								tree_position = &*right;
																				},

																				&Node::Leaf { ref data, .. }=> { 
																								println!("found data: {}", data);
																								tree_position = self;
																								to_return_string = format!("{} {}", to_return_string, data);
																				}
																}
												},
												_ => ()
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
}

impl DataContext {
    pub fn encode(&self, to_encode: &str) -> String {
        let tokens: Vec<&str> = to_encode.split(" ").collect();
				let mut encoded_string = "".to_string();
        for token in tokens {
            let pattern = self.root.traverse(token);
            if let Some(x) = pattern {
								encoded_string = format!("{}{}", encoded_string, x);
            }
						else {
								println!("Something went wrong in encoding"); 
						}
        }
				return encoded_string;

    }

		pub fn decode(&self, to_decode: &str) -> String {
				return self.root.decode(to_decode);

		}
}

/// Creates a DataContext based on a corpus.
fn make_context(corpus: String) -> DataContext {
    // Gather vector of pointers to individual words in the corpus.
    let tokens: Vec<&str> = corpus.split(" ").collect();

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

    assert!(forest.len() == 1);
    return DataContext {
        _context_id: "Test".to_string(),
        root: forest[0].clone(),
    };
}

fn main() {

    let corpus = String::from(
        "this this this this test this is just a big old test I can't believe this is just a test",
    );

    let data_context = make_context(corpus);
    let encoded = data_context.encode("test test this is a test");
		let decoded = data_context.decode(&encoded);
		println!("encoded: {}", encoded);
		println!("decoded: {}", decoded);

		println!("Tree Visualization\n");
		data_context.root.visualize_tree();
}


#[test]
fn test_encode() {
				
    let corpus = String::from(
        "this this this this test this is just a big old test I can't believe this is just a test",
    );

    let data_context = make_context(corpus);
    data_context.encode("test test this is a test");
}
