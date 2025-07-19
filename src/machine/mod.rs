use std::path::PathBuf;
use clap::Parser;

mod generator;
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
impl MachineConfig {
    fn mirror_input_path(&self, path: PathBuf) -> PathBuf {
        let input_path_comps= self.input_path.components();
        let n: usize = input_path_comps.clone().count();

        let mut path_components = path.components();

        for i in 0..n {
            path_components.next();
        }

        let mut new_path: PathBuf = self.output_path.clone();
        new_path.extend(path_components);

        new_path
    }
}

pub fn execute_cmd() {
    let args = MachineConfig::parse();
    
    let o = input::TracedPages::trace_pages(&args.input_path);
    println!("{:#?}", o);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirror_input_path() {
        let machine_config: MachineConfig = MachineConfig {
            input_path: PathBuf::from("test-inputs"),
            output_path: PathBuf::from("test-outputs"),
            theme_path: PathBuf::from("theme.css"),
            top_toc: true,
            numbered_headers: true
        };

        assert_eq!(
            machine_config.mirror_input_path(PathBuf::from("test-inputs/pages/main.md")),
            PathBuf::from("test-outputs/pages/main.md")
        );
    }
    #[test]
    fn test_mirror_input_path_when_it_is_not_its_root_component() {
        let machine_config: MachineConfig = MachineConfig {
            input_path: PathBuf::from("test-inputs/pages"),
            output_path: PathBuf::from("test-outputs"),
            theme_path: PathBuf::from("theme.css"),
            top_toc: true,
            numbered_headers: true
        };
        
        assert_eq!(
            machine_config.mirror_input_path(PathBuf::from("test-inputs/pages/main.md")),
            PathBuf::from("test-outputs/main.md")
        );
    }
    #[test]
    fn test_mirror_input_path_when_output_path_is_not_its_root_component() {
        let machine_config: MachineConfig = MachineConfig {
            input_path: PathBuf::from("test-inputs"),
            output_path: PathBuf::from("test-outputs/pages"),
            theme_path: PathBuf::from("theme.css"),
            top_toc: true,
            numbered_headers: true
        };
        
        assert_eq!(
            machine_config.mirror_input_path(PathBuf::from("test-inputs/main.md")),
            PathBuf::from("test-outputs/pages/main.md")
        );
    }
}