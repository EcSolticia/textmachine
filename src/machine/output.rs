use::std::path::PathBuf;
use crate::machine::input;
use super::CmdArgs;

#[derive(Debug)]
pub struct OutputPage {
    pub path: PathBuf,
    pub input_page: input::Page
}

pub type PageList = Vec<OutputPage>;

pub struct OutputPages {
    pub list: PageList
}
impl OutputPages {
    pub fn new(traced_pages: input::TracedPages, cmd_args: &CmdArgs) -> OutputPages {
        let mut working_list: PageList = vec![];

        let input_list: input::PageList = traced_pages.get_list().clone();
        for page in input_list {
            working_list.push(
                OutputPage { 
                    path: cmd_args.mirror_input_path(page.path()), 
                    input_page: page
                }
            )
        }

        OutputPages { list: working_list }
    }
}
