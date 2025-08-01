use clap::Parser;

extern crate exitcode;

mod generator;
mod tree;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CmdArgs {
    #[arg(short, long)]    
    pub input_path: String,
    #[arg(short, long)]
    pub output_path: String,
    #[arg(short, long)]
    pub force: bool
}

pub mod sure_prompt {
    fn print_line(line: &str) {
        println!("[textmachine-sure_prompt] {}", line);
    }
    
    pub fn handle(args: &super::CmdArgs) -> Option<bool> {        

        if args.force {
            return Some(true);
        }

        print_line("----");
        print_line("Called textmachine with the following arguments:");
        print_line(
            format!("input_path: {}", args.input_path).as_str()
        );
        print_line(
            format!("output_path: {}", args.output_path).as_str()  
        );
        print!("\n");
        print_line("!!!IMPORTANT!!!");
        print_line("textmachine will delete the output path directory prior to generating pages");
        print_line("Before continuing, ensure that the output path is really disposable.");
        print_line("Ideally, in a git repository, it should be gitignored.");
        print_line("Please confirm that you are certain, that outpath_path, if it exists, is disposable. [y/N]");
        print!("\n");

        let mut answer: String = String::new();
        
        match std::io::stdin().read_line(&mut answer) {
            Ok(_) => {
                let trimmed_answer = answer.trim();
                match trimmed_answer {
                    "n" | "N" | "no" | "No" => return Some(false),
                    "y" | "Y" | "yes" | "Yes" => return Some(true),
                    _ => {
                        print_line("Answer is not 'y' or 'N'. Assuming 'N'.");
                        return Some(false);
                    }
                }
            },
            Err(_) => return None
        };
    }
}

use std::{fs, io};
fn is_valid_input_dir(input_path: &str) -> io::Result<bool> {
    let meta: fs::Metadata = fs::metadata(input_path)?;
    Ok(meta.is_dir())
}

pub fn execute(args: CmdArgs) {
    match is_valid_input_dir(&args.input_path) {
        Ok(valid) => {
            if !valid {
                eprintln!("[textmachine-input-validation] input path {} is not a dir", &args.input_path);
                std::process::exit(exitcode::NOINPUT);
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(exitcode::IOERR);
        }
    }
    
    let wrapped_tree = tree::Node::from(&args.input_path);
    match wrapped_tree {
        Ok(node) => {
            sure_prompt::handle(&args);
            std::fs::remove_dir_all(&args.output_path);
            generator::generate(node, &args.output_path, &args.input_path);
        },

        Err(node_err) => {
            match node_err {
                tree::NodeError::NodeError(msg) => {
                    eprintln!("{}", msg);
                    std::process::exit(exitcode::DATAERR);
                },
                tree::NodeError::IoError(e) => {
                    eprintln!("{}", e);
                    std::process::exit(exitcode::IOERR);
                }
            }
        }
    }


}

pub fn execute_cmd() {
    let args: CmdArgs = CmdArgs::parse();

    execute(args);
}
