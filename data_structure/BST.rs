
#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

struct BinarySearchTree {
    root: Option<Box<Node>>,
}

impl BinarySearchTree {
    // Create an empty BST
    fn new() -> Self {
        Self { root: None }
    }

    // INSERTION
    fn insert(&mut self, value: i32) {
        Self::insert_node(&mut self.root, value);
    }

    fn insert_node(node: &mut Option<Box<Node>>, value: i32) {
        match node {
            Some(current) => {
                if value < current.value {
                    Self::insert_node(&mut current.left, value);
                } else if value > current.value {
                    Self::insert_node(&mut current.right, value);
                }
            }

            None => {
                *node = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                }));
            }
        }
    }

    // SEARCHING
    fn search(&self, value: i32) -> bool {
        Self::search_node(&self.root, value)
    }

    fn search_node(node: &Option<Box<Node>>, value: i32) -> bool {
        match node {
            Some(current) => {
                if value == current.value {
                    true
                } else if value < current.value {
                    Self::search_node(&current.left, value)
                } else {
                    Self::search_node(&current.right, value)
                }
            }

            None => false,
        }
    }

    // INORDER TRAVERSAL
    fn inorder(&self) {
        Self::inorder_node(&self.root);
        println!();
    }

    fn inorder_node(node: &Option<Box<Node>>) {
        if let Some(current) = node {
            Self::inorder_node(&current.left);
            print!("{} ", current.value);
            Self::inorder_node(&current.right);
        }
    }
}

fn main() {
    let mut tree = BinarySearchTree::new();

    // Insert nodes
    tree.insert(50);
    tree.insert(30);
    tree.insert(70);
    tree.insert(20);
    tree.insert(40);
    tree.insert(60);
    tree.insert(80);

    // Inorder traversal
    print!("Inorder Traversal: ");
    tree.inorder();

    // Search nodes
    println!("Search 40: {}", tree.search(40));
    println!("Search 90: {}", tree.search(90));
}
