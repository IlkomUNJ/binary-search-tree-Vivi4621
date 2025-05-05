mod structure;
mod tool;

use crate::structure::bst::BstNode;
use crate::structure::tree::Node;
use crate::structure::tree::NodeLink;
use crate::structure::bst::BstNodeLink;
use crate::tool::generate_dotfile;
use crate::tool::generate_dotfile_bst;

fn main() {
    //turn on to test the old code
    // test_binary_tree();
    test_binary_search_tree();
}

fn in_order_traversal(node: &Option<BstNodeLink>) {
    if let Some(ref n) = node {
        in_order_traversal(&n.borrow().left);
        print!("{} ", n.borrow().key.unwrap());
        in_order_traversal(&n.borrow().right);
    }
}

fn test_binary_search_tree(){
    let mut root: Option<BstNodeLink> = None;
    let values = vec![15, 6, 18, 3, 7, 17, 20, 2, 4, 13, 9];
    for &val in &values {
        tree_insert(&mut root, val);
    }

println!("Tree contents (in-order):");
in_order_traversal(&root);
println!();

    //add right subtree
    let right_subtree: &Option<BstNodeLink> = &root.as_ref().unwrap().borrow().right;
    if let Some(right_tree_extract) = right_subtree {
        right_tree_extract
            .borrow_mut()
            .add_left_child(right_tree_extract, 17);
        right_tree_extract
            .borrow_mut()
            .add_right_child(right_tree_extract, 20);
    }

    //add left subtree
    let left_subtree: &Option<BstNodeLink> = &root.as_ref().unwrap().borrow().left;
    if let Some(left_tree_extract) = left_subtree {
        left_tree_extract
            .borrow_mut()
            .add_left_child(left_tree_extract, 3);
        left_tree_extract
            .borrow_mut()
            .add_right_child(left_tree_extract, 7);

        //add left subtree terminal
        let left_subtree_terminal = &left_tree_extract.borrow().left;
        if let Some(terminal_left_tree_link) = left_subtree_terminal{
            terminal_left_tree_link.borrow_mut().add_left_child(terminal_left_tree_link, 2);
            terminal_left_tree_link.borrow_mut().add_right_child(terminal_left_tree_link, 4);
        }
        //add 2nd level right subtree of node 7
        let second_right_subtree = &left_tree_extract.borrow().right;
        if let Some(second_right_subtree_link) = second_right_subtree{
            second_right_subtree_link.borrow_mut().add_right_child(second_right_subtree_link, 13);

            let third_left_subtree = &second_right_subtree_link.borrow().right;
            if let Some(third_left_subtree_link) = third_left_subtree{
                third_left_subtree_link.borrow_mut().add_left_child(third_left_subtree_link, 9);
            }
        }
    }

    //print the tree at this time
    let main_tree_path = "bst_graph.dot";
    generate_dotfile_bst(&root.as_ref().unwrap(), main_tree_path);

    //tree search test
    let search_keys = vec![15, 9, 22];

    for &key in search_keys.iter() {
        print!("tree search result of key {} is ", key);

        if let Some(node_result) = root.as_ref().unwrap().borrow().tree_search(&key) {
            println!("found -> {:?}", node_result.borrow().key);
        } else {
            println!("not found");
        }
    }

    //min test
    let min_node = root.as_ref().unwrap().borrow().minimum();
    println!("minimum result {:?}", min_node.borrow().key);

    //max test
    let max_node = root.as_ref().unwrap().borrow().maximum();
    println!("maximum result {:?}", max_node.borrow().key);

    //root node get test
    let root_node = BstNode::get_root(&max_node);
    println!("root node {:?}", root_node.borrow().key);

    //successor test
    let query_keys = vec![
        2, // min_node, should return its parent Some(3)
        20, // max_node, should return None
        15, // root_node, should return the minimum of its right tree
        // test case for node with empty right child
        // should return a parent of the node's ancestor if it's a left child of the parent
        13,
        9, 7, // other keys
        22 // non-existent key
    ];

    for &key in query_keys.iter() {
        if let Some(node) = root.as_ref().unwrap().borrow().tree_search(&key) {
            print!("successor of node ({}) is ", key);

            if let Some(successor) = BstNode::tree_successor_simpler(&node) {
                println!("{:?}", successor.borrow().key);
            } else {
                println!("not found");
            }
        } else {
            println!("node with key of {} does not exist, failed to get successor", key)
        }
    }
}

#[allow(dead_code)]
fn test_binary_tree() {
    //create the nodelink of the root node
    let root.as_ref().unwrap(): NodeLink = Node::new_nodelink(5);

    //add a new left node value
    root.as_ref().unwrap().borrow_mut().add_left_child(&root.as_ref().unwrap(), 3);
    //add a new right node value
    root.as_ref().unwrap().borrow_mut().add_right_child(&root.as_ref().unwrap(), 7);

    //println!("{:?}", root.as_ref().unwrap());

    //print the tree at this time
    let mut main_tree_path = "prime.dot";
    generate_dotfile(&root.as_ref().unwrap(), main_tree_path);

    //add new child values to the left subtree
    let left_subtree = &root.as_ref().unwrap().borrow().left;
    if let Some(left_tree_extract) = left_subtree {
        left_tree_extract
            .borrow_mut()
            .add_left_child(left_tree_extract, 2);
        left_tree_extract
            .borrow_mut()
            .add_right_child(left_tree_extract, 4);
    }

    //add new child values to the right subtree
    let right_subtree = &root.as_ref().unwrap().borrow().right;
    if let Some(right_tree_extract) = right_subtree {
        right_tree_extract
            .borrow_mut()
            .add_right_child(right_tree_extract, 10);
    }

    //print the tree again, now been added with more values
    main_tree_path = "prime_t2.dot";
    generate_dotfile(&root.as_ref().unwrap(), main_tree_path);

    //Call tree depth function at this time
    let recorded_depth = root.as_ref().unwrap().borrow().tree_depth();
    println!("Current tree depth: {0}", recorded_depth);

    //Call count_nodes function
    let total_nodes = root.as_ref().unwrap().borrow().count_nodes();
    println!("Amount of nodes in current tree: {0}", total_nodes);

    //Call count_nodes_by_nodelink function, supplied right subtree as parameter
    //TODO
    let subtree_count = Node::count_nodes_by_nodelink(&right_subtree.clone().unwrap(), 0);
    println!("Amount of nodes in current subtree: {0}", subtree_count);

    //Get the sibling of the leftsubtree from parent
    let _left_subtree_sibling = Node::get_sibling(&left_subtree.as_ref().unwrap());
    //println!("sibling of left subtree {:?}", left_subtree_sibling);

    //get the left subtree by value
    let left_subtree = root.as_ref().unwrap().borrow().get_node_by_value(3);
    println!("left subtree seek by value {:?}", left_subtree);
    //get the left subtree by full properties
    let another_left_subtree = root.as_ref().unwrap()
        .borrow()
        .get_node_by_full_property(&left_subtree.as_ref().unwrap());
    println!(
        "left subtree seek by full property {:?}",
        another_left_subtree
    );

    //Discard the right subtree from parent
    let root.as_ref().unwrap()2 = root.as_ref().unwrap().borrow().get_nodelink_copy();

    let flag = root.as_ref().unwrap()2.borrow_mut().discard_node_by_value(3);
    println!("status of node deletion: {0}", flag);

    //print the tree again
    main_tree_path = "prime_t3.dot";
    generate_dotfile(&root.as_ref().unwrap()2, main_tree_path);

    //Call tree depth function at this time
    //TODO
    let depth_now = root.as_ref().unwrap()2.borrow().tree_depth();
    println!("Depth after discard {0}", depth_now);

    //Call count_nodes function
    let count_now = root.as_ref().unwrap()2.borrow().count_nodes();
    println!("Count nodes after discard {0}", count_now);

    //print the tree again
    main_tree_path = "prime_t4.dot";
    generate_dotfile(&root.as_ref().unwrap(), main_tree_path);
}
