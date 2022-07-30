mod iterator;

fn main() {
    iterator::gen_random_files(1024*1024*2, 1000);
    println!("Hello, world!");
}
