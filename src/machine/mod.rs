use std::path::PathBuf;
use clap::Parser;

mod generator;
mod input;
mod output;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CmdArgs {
    #[arg(short, long)]    
    pub input_path: PathBuf,
    #[arg(short, long)]
    pub output_path: PathBuf,
    #[arg(short, long)]
    pub force: bool
}
impl CmdArgs {
    fn mirror_input_path(&self, path: PathBuf) -> PathBuf {
        let input_path_comps= self.input_path.components();
        let n: usize = input_path_comps.clone().count();

        let mut path_components = path.components();

        for _i in 0..n {
            path_components.next();
        }

        let mut new_path: PathBuf = self.output_path.clone();
        new_path.extend(path_components);
        new_path.set_extension("html");

        new_path
    }
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
            format!("input_path: {}", args.input_path.to_str().unwrap()).as_str()
        );
        print_line(
            format!("output_path: {}", args.output_path.to_str().unwrap()).as_str()  
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
    match input::TracedPages::trace_pages(&args.input_path) {
        Ok(traced_pages) => {
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
    }
}

pub fn execute_cmd() {
    let args: CmdArgs = CmdArgs::parse();

    execute(args);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirror_input_path() {
        let cmd_args: CmdArgs = CmdArgs {
            input_path: PathBuf::from("test-inputs"),
            output_path: PathBuf::from("test-outputs")
        };

        assert_eq!(
            cmd_args.mirror_input_path(PathBuf::from("test-inputs/pages/main.md")),
            PathBuf::from("test-outputs/pages/main.html")
        );
    }
    #[test]
    fn test_mirror_input_path_when_it_is_not_its_root_component() {
        let cmd_args: CmdArgs = CmdArgs {
            input_path: PathBuf::from("test-inputs/pages"),
            output_path: PathBuf::from("test-outputs")
        };
        
        assert_eq!(
            cmd_args.mirror_input_path(PathBuf::from("test-inputs/pages/main.md")),
            PathBuf::from("test-outputs/main.html")
        );
    }
    #[test]
    fn test_mirror_input_path_when_output_path_is_not_its_root_component() {
        let cmd_args: CmdArgs = CmdArgs {
            input_path: PathBuf::from("test-inputs"),
            output_path: PathBuf::from("test-outputs/pages")
        };
        
        assert_eq!(
            cmd_args.mirror_input_path(PathBuf::from("test-inputs/main.md")),
            PathBuf::from("test-outputs/pages/main.html")
        );
    }
}
