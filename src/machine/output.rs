use::std::path::PathBuf;
use crate::machine::input;
use super::MachineConfig;

#[derive(Debug)]
pub struct OutputPage {
    pub path: PathBuf
}

pub type PageList = Vec<OutputPage>;

pub struct OutputPages {
    pub list: PageList
}
impl OutputPages {
    pub fn new(traced_pages: input::TracedPages, machine_config: &MachineConfig) -> OutputPages {
        let mut working_list: PageList = vec![];

        let input_list: input::PageList = traced_pages.get_list().clone();
        for page in input_list {
            working_list.push(
                OutputPage { path: machine_config.mirror_input_path(page.path()) }
            )
        }

        OutputPages { list: working_list }
    }
}