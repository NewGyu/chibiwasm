#![allow(dead_code)]
#![allow(unused)]

use anyhow::Result;
use anyhow::{bail, Context};
use clap::Parser;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read};
use std::{env, result};

#[derive(Debug, Parser)]
#[clap(author, about, version)]
struct Args {
    file: String,
    func: String,
    func_args: Vec<i32>,
}

impl Args {
    fn wasm_file(&self) -> io::Result<File> {
        File::open(&self.file)
    }
    // fn func_args(&self) -> Vec<Value> {
    //     self.func_args.iter().map(|x| Value::I32(*x)).collect()
    // }
}

fn main() -> Result<()> {
    let args = Args::parse();

    //Load module with decoder
    let mut file = args.wasm_file()?;
    let mut reader = BufReader::new(file);
    let mut module = chibiwasm::binary::module::decode(&mut reader)?;

    //Execute with runtime
    //    let mut runtime = Runtime::new(&mut module)?;
    //    let result = runtime.invoke(&args.func, &args.func_args());

    //    println!("{}", result?.unwrap());
    Ok(())
}
