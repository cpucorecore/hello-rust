fn main() {
    let strs = vec!["a", "bb", "c"];
    let _x: Vec<()> = strs.iter().map(|str| {
        println!("{}", str)
    }).collect();

    println!("{:?}", strs);
}
