use std::path::PathBuf;
use clap::Parser;

mod input;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct MachineConfig {
    #[arg(short, long)]    
    input_path: PathBuf,
    #[arg(short, long)]
    output_path: PathBuf,
    #[arg(short, long)]
    theme_path: PathBuf,
    #[arg(long)]
    top_toc: bool,
    #[arg(long)]
    numbered_headers: bool
}

pub fn execute_cmd() {
    let args = MachineConfig::parse();
    
    let o = input::Page::new(args.input_path);
    println!("{:?}", o);
}
