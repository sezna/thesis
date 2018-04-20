/// Creates a DataContext based on a corpus.
pub fn new(corpus: String) -> DataContext {
    // Gather vector of pointers to individual words in the corpus.
    let mut tokens: Vec<&str> = corpus.split(" ").collect();
    // Create unique tuples of tokens and their benefit.
    let mut tokens_with_benefit: Vec<(&str, u64)> = tokens
        .par_iter()
        .map(|x| {
        (*x,(tokens.iter().filter(|&y| y == x).count() * x.len()) as u64)})
        .collect();
    // Add the "not contained" token with a benefit of zero
    tokens_with_benefit.push(("token not contained", 0u64));
    tokens_with_benefit.sort();
    tokens_with_benefit.dedup();
    tokens_with_benefit.sort_by_key(|x| x.1);
    // Create the Huffman tree. At this point, tokens_with_benefit is sorted
    // by lowest benefit to highest benefit. Note that the compare
    // function for this node type is inverted to make this BinaryHeap a 
    // MinHeap, useful for creating a Huffman code.
    // Start with all singletons
    let mut forest: BinaryHeap<Node> = tokens_with_benefit
        .iter()
        .map(|x| {
            Node::Leaf {data: x.0.to_string(), benefit: x.1,}}).collect();
    // Merge the singletons one by one into a tree.
    while forest.len() > 1 {
	    let left = forest.pop().expect("heap pop didn't work");
		let right = forest.pop().expect("heap pop didn't work");
		let benefit = left.benefit() + right.benefit();
        let new_tree = Node::Interior {
            left: Box::new(left),
            right: Box::new(right),
            benefit: benefit,
        };
        forest.push(new_tree);
    }
    // There should be only one node in the forest left, the root.
    assert!(forest.len() == 1);
    return DataContext {
        _context_id: "Test".to_string(),
        root: forest.peek().unwrap().clone(),
        encoding_table: forest.peek().unwrap().make_table(tokens),
    };
}