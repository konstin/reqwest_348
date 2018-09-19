extern crate pretty_env_logger;
extern crate reqwest;

use reqwest::{multipart::Form, Client};

fn main() {
    pretty_env_logger::init();
    // This can be any bigger file; This one's 4mb
    let filename = "hello_world-4-mb.whl";

    let form = Form::new().file("content", filename).unwrap();

    Client::new()
        .post("https://test.pypi.org/legacy/")
        .header("content-type", "application/json; charset=utf-8")
        .multipart(form)
        .send()
        .unwrap();
}
