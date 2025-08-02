use clap::Parser;
use ui::note::Issuer;

mod generator;
mod tree;
mod ui;
mod errors;

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

pub fn execute(args: CmdArgs) {
    let mut error: Option<errors::Error> = None;

    let utree = tree::Node::from(&args.input_path);
    match utree {
        Ok(tree) => {

            // todo: delete output path with prompt

            let ugen_outputs = generator::generate(
                tree, 
                &args.output_path, 
                &args.input_path
            );

            match ugen_outputs {
                Ok(gen_output) => {
                    gen_output.present_gen_outputs();
                },
                Err(e) => error = Some(e)
            }
        },
        Err(e) => error = Some(e)
    }

    if let Some(err) = error {
        err.issue().present_error();
        std::process::exit(1);
    }
    std::process::exit(0);
}

pub fn execute_cmd() {
    let args: CmdArgs = CmdArgs::parse();

    execute(args);
}
