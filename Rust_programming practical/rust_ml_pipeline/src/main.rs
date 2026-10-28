use polars::prelude::*;
use std::fs::File;

fn main() {
    let file = File::open("../datasets/KALI_LINUX_TOOLS_DATASET.jsonl")
        .expect("failed to open dataset");
    let df = JsonReader::new(file)
        .with_json_format(JsonFormat::JsonLines)
        .finish()
        .expect("failed to read dataset");



    let df = df.select(&["tool", "description", "category"]).unwrap();



    println!("{:?}", df.shape());
}