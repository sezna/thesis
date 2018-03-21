
/// A node for the DataContext.
#[derive(Clone)]
enum Node {
    Interior {
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
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
                    Some("0".to_string())
                } else {
                    None
                }
            }
            &Node::Interior { .. } => self.handle_interior_traversal(token), 
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
                if let Some(result) = left.clone().expect("").traverse(token) {
                    to_return = format!("0{}", result)
                };
                if let Some(result) = right.clone().expect("").traverse(token) {
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
}

/// Modified Huffman code-type tree.
struct DataContext {
    _context_id: String,
    root: Node,
}

impl DataContext {
    pub fn encode(&self, to_encode: &str) {
        let tokens: Vec<&str> = to_encode.split(" ").collect();
        for token in tokens {
            let pattern = self.root.traverse(token);
            if let Some(x) = pattern {
                println!("pattern: {}", x);
            }
        }
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
            left: Some(Box::new(forest[0].clone())),
            right: Some(Box::new(forest[1].clone())),
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
    data_context.encode("test test this is a test");

}
