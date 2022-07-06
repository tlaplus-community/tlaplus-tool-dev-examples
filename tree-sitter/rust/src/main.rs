use tree_sitter::{Parser, InputEdit, Point};

fn main() {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_tlaplus::language()).expect("Error loading TLA+ grammar");

    let source_code = "---- MODULE Test ----f(x) == x====";
    let mut tree = parser.parse(source_code, None).unwrap();
    println!("First tree: {}", tree.root_node().to_sexp());
    
    let source_code = "---- MODULE Test ----f(x, y) == x + y====";
    let edit = InputEdit {
        start_byte: 24,
        old_end_byte: 24,
        new_end_byte: 27,
        start_position: Point {row: 0, column: 24},
        old_end_position: Point {row: 0, column: 24},
        new_end_position: Point {row: 0, column: 27}
    };
    tree.edit(&edit);
    let edit = InputEdit {
        start_byte: 33,
        old_end_byte: 33,
        new_end_byte: 37,
        start_position: Point {row: 0, column: 33},
        old_end_position: Point {row: 0, column: 33},
        new_end_position: Point {row: 0, column: 37}
    };
    tree.edit(&edit);
    tree = parser.parse(&source_code, Some(&tree)).unwrap();
    println!("Second tree: {}", tree.root_node().to_sexp());
}

