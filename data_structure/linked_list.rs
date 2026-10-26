
#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    // Create an empty linked list
    fn new() -> Self {
        Self { head: None }
    }

    // Insert a node at the beginning
    fn push_front(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    
    fn print(&self) {
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = node.next.as_ref();
        }

        println!("None");
    }


    fn search(&self, target: i32) -> Option<usize> {
        let mut current = self.head.as_ref();
        let mut index = 0;

        while let Some(node) = current {
            if node.value == target {
                return Some(index);
            }

            current = node.next.as_ref();
            index += 1;
        }

        None
    }
}

fn main() {
    // Create an empty linked list
    let mut list = LinkedList::new();

    // Insert elements
    list.push_front(30);
    list.push_front(20);
    list.push_front(10);

    // Print the linked list
    println!("Linked List:");
    list.print();

    // Search for elements
    println!("Position of 10: {:?}", list.search(10));
    println!("Position of 20: {:?}", list.search(20));
    println!("Position of 30: {:?}", list.search(30));
    println!("Position of 50: {:?}", list.search(50));
}
