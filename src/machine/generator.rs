use crate::machine::{output};
use std::{path::PathBuf, io, fs, fmt};
use tempfile::NamedTempFile;

fn path_exists(path: &PathBuf) -> io::Result<bool> {
    path.try_exists()
}

fn page_dir_exists(page_path: &PathBuf) -> io::Result<bool> {
    path_exists(&page_path.parent().unwrap().to_path_buf())
}

fn add_lua_filters(pandoc: &mut pandoc::Pandoc) -> io::Result<NamedTempFile> {
    let filter: &str = include_str!("./../../resources/filters.lua");

    let filter_tmp_file: NamedTempFile = NamedTempFile::new()?;
    
    let filter_path: PathBuf = filter_tmp_file.path().to_path_buf();

    fs::write(&filter_path, filter)?;
    
    pandoc.add_option(PandocOption::LuaFilter(filter_path));
    Ok(filter_tmp_file)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    IoError(#[from] io::Error),
    #[error("some error with pandoc occured")]
    PandocError(#[from] pandoc::PandocError)
}

pub struct PandocOutputWrapper {
    actual: pandoc::PandocOutput
}
impl PandocOutputWrapper {
    pub fn new(pandoc_output: pandoc::PandocOutput) -> PandocOutputWrapper {
        PandocOutputWrapper { actual: pandoc_output }
    }
}
impl fmt::Debug for PandocOutputWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self.actual {
            pandoc::PandocOutput::ToFile(pathb) => write!(f, "{:?}", pathb),
            _ => write!(f, "pandoc output is non-file")
        }
    }
}

pub type PandocOutputs = Vec<PandocOutputWrapper>;

use pandoc::PandocOption;
fn get_options() -> Vec<PandocOption> {
    vec![
        PandocOption::Standalone,
        PandocOption::TableOfContents,
        PandocOption::NumberSections
    ]
}

pub fn generate(output_pages: output::OutputPages) -> Result<PandocOutputs, Error> {
    let mut pandoc_outputs: PandocOutputs = vec![];
    
    for page in output_pages.list {
        let page_dir_exists: bool = page_dir_exists(&page.path)?;

        if !page_dir_exists {
            fs::create_dir(&page.path.parent().unwrap())?;
        }

        let mut pandoc: pandoc::Pandoc = pandoc::new();

        if let Some(prefix_path) = (&page).input_page.prefix_path() {
            pandoc.add_input(&prefix_path);
        }
        pandoc.add_input(&page.input_page.path());
        if let Some(postfix_path) = (&page).input_page.postfix_path() {
            pandoc.add_input(&postfix_path);
        }
        
        pandoc.add_options(&get_options());

        let _filter_tmp_file = add_lua_filters(&mut pandoc)?;

        pandoc.set_show_cmdline(true);
        
        pandoc.set_output(pandoc::OutputKind::File(
            page.path
        ));

        let new_output: pandoc::PandocOutput = pandoc.execute()?;
        pandoc_outputs.push(PandocOutputWrapper::new(new_output));
    }
    
    Ok(pandoc_outputs)
}
