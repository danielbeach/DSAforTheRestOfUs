fn main() {
    // crreate some nodes
    let mut node_1: Node = Node { value: 1, next: None };
    let mut node_2: Node = Node { value: 2, next: None };
    let mut node_3: Node = Node { value: 3, next: None };

    // create links bewteen nodes
    node_2.next = Some(Box::new(node_3));
    node_1.next = Some(Box::new(node_2));

    let mut linked_list: LinkedList = LinkedList { head: None };
    linked_list.head = Some(Box::new(node_1));
    println!("{:?}", linked_list)

}

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
struct LinkedList {
    head: Option<Box<Node>>,
}
