use sort::{insert_sort, merge_sort, max_heap_sort};

fn main() {
    let mut a = vec![];
    for i in (0..10).rev() {
        a.push(i);
    }
    println!("before insert sort: {:?}", a);
    insert_sort(&mut a);
    println!("after insert sort: {:?}", a);

    let mut a = vec![];
    for i in (0..10).rev() {
        a.push(i);
    }
    println!("before merge sort: {:?}", a);
    merge_sort(&mut a);
    println!("after merge sort: {:?}", a);

    let mut a = [4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
    max_heap_sort(&mut a);
    println!("{:?}", a);
}
