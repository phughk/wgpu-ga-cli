mod cli;
mod parser;

use clap::Parser;
use wgpu::{Backends, DeviceDescriptor, InstanceDescriptor};

#[tokio::main]
async fn main() {
    let args = cli::CliArgs::parse();
    let input = std::fs::read_to_string(&args.input)
        .map_err(|e| {
            format!(
                "Error reading file {:?}: {:?}",
                args.input.canonicalize().unwrap(),
                e
            )
        })
        .unwrap();
    let ast = parser::parse_input(&input).unwrap();

    // https://webgpufundamentals.org/webgpu/lessons/webgpu-wgsl-function-reference.html
    let d = InstanceDescriptor::from_env_or_default();
    let i = wgpu::Instance::new(&d);
    let a = i.enumerate_adapters(Backends::all());
    let a = a.into_iter().next().unwrap();
    let (device, queue) = a
        .request_device(&DeviceDescriptor::default())
        .await
        .unwrap();
    println!("Adapters: {:#?}", a.features());
}
