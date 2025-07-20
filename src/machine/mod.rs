use std::path::PathBuf;
use clap::Parser;

mod generator;
mod input;
mod output;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CmdArgs {
    #[arg(short, long)]    
    input_path: PathBuf,
    #[arg(short, long)]
    output_path: PathBuf
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

pub fn execute_cmd() {
    let args: CmdArgs = CmdArgs::parse();
    
    let traced_pages = input::TracedPages::trace_pages(&args.input_path);
    
    if traced_pages.is_err() {
        panic!();
    }
    let unwrapped_traced_pages: input::TracedPages = traced_pages.unwrap();

    let output_pages = output::OutputPages::new(unwrapped_traced_pages.clone(), &args);

    println!("--- Input Pages ---\n\n");
    println!("{:#?}\n\n", unwrapped_traced_pages.get_list());
    println!("--- Output Pages ---\n\n");
    println!("{:#?}\n\n", output_pages.list);

    println!("--- PandocOutput ---\n\n");
    let gen_output = generator::generate(output_pages);
    println!("{:#?}", gen_output);
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
