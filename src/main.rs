mod iterator;

use reqwest::blocking::multipart;
use reqwest::blocking::Client;

fn main() {
    iterator::gen_random_files(1024*1024*100, 1);
    let part = multipart::Form::new().file("file", "/home/sky/data/test/1").unwrap();
    let client = Client::new();
    let res = client
        .post("http://127.0.0.1:5001/api/v0/add")
        .multipart(part)
        .send().unwrap();

    println!("{:?}", res);
}
