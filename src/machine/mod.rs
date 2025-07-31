use clap::Parser;

//mod generator;
//mod input;
//mod output;
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

pub fn execute(args: CmdArgs) {
    let iputp: &str = &args.input_path;
    let tree = tree::Node::from(iputp);

    println!("{:#?}", tree);
    /*match input::TracedPages::trace_pages(&args.input_path) {
        Ok(traced_pages) => {
            println!("{:#?}", traced_pages.get_tree());
            let output_pages: output::OutputPages = output::OutputPages::new(traced_pages, &args);

            
            if let Some(sure) = sure_prompt::handle(&args) {
                if !sure {
                    return;
                }
            } else {
                println!("Could not read line from stdin, returning.");
                return;
            }
            
            match std::fs::remove_dir_all(args.output_path) {
                Ok(_) => (),
                Err(e) => println!("removing output dir: {:#?}", e)
            }

            let gen_output = generator::generate(output_pages);

            println!("gen_output: {:#?}", gen_output);
        },
        Err(e) => {
            println!("{:#?}", e);
        }
    }*/
}

pub fn execute_cmd() {
    let args: CmdArgs = CmdArgs::parse();

    execute(args);
}
