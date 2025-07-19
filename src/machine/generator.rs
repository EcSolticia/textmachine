use crate::machine::{output};
use std::{path::PathBuf, io, fs};

fn page_dir_exists(page_path: &PathBuf) -> io::Result<bool> {
    page_path.parent().unwrap().try_exists()
}

pub fn generate(output_pages: output::OutputPages) -> io::Result<()> {
    for page in output_pages.list {
        let page_dir_exists: bool = page_dir_exists(&page.path)?;

        if !page_dir_exists {
            let _ = fs::create_dir(&page.path.parent().unwrap());
        }

        let mut pandoc: pandoc::Pandoc = pandoc::new();
        
        let prefix_path: Option<PathBuf> = (&page).input_page.prefix_path();
        if prefix_path.is_some() {
            pandoc.add_input(&prefix_path.unwrap());
        }
        pandoc.add_input(&page.input_page.path());
        let postfix_path: Option<PathBuf> = (&page).input_page.postfix_path();
        if postfix_path.is_some() {
            pandoc.add_input(&postfix_path.unwrap());
        }

        pandoc.set_output(pandoc::OutputKind::File(
            page.path
        ));

        pandoc.execute().unwrap();
        
    }

    Ok(())
}