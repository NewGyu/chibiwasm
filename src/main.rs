#![allow(dead_code)]
#![allow(unused)]

use anyhow::Result;
use anyhow::{bail, Context};
use chibiwasm::{
    grammer::{module::Module, section::*, types::FuncType, value::Value},
    runtime::{Decoder, Runtime},
};
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
    fn func_args(&self) -> Vec<Value> {
        self.func_args.iter().map(|x| Value::from(*x)).collect()
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    //Load module with decoder
    let mut decoder = Decoder::new(args.wasm_file()?);
    let mut module = decoder.decode()?;

    //Execute with runtime
    let mut runtime = Runtime::new(&mut module)?;
    let result = runtime.invoke(&args.func, &mut args.func_args());

    println!("{}", result?.unwrap());
    Ok(())
}
