use sort::{insert_sort, merge_sort};

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
}
