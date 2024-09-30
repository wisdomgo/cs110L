use linked_list::{CommputeNorm, LinkedList};
pub mod linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    // list.push_front(1);
    // list.push_front(1);
    // list.push_front(4);
    // list.push_front(5);
    // list.push_front(1);
    // list.push_front(4);
    // list.push_front(1919810);
    // list.pop_front();
    // println!("{}",list);
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    println!("测试一下clone");
    let list_test = list.clone();
    let mut list_test_test = list_test.clone();
    println!("{}",list_test_test);
    println!("{}",list_test == list_test_test);
    list_test_test.pop_front();
    println!("{}",list_test == list_test_test);

    let mut list_f64 = LinkedList::new();
    list_f64.push_front(3.0);
    list_f64.push_front(4.0);
    println!("{}",list_f64.compute_norm());
    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}
}
