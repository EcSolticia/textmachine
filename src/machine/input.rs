use std::{path::PathBuf, fs, io};

#[derive(Debug)]
pub enum PossiblyNewPage {
    NoError(Option<Page>),
    Error(io::Error)
}

#[derive(Debug)]
pub struct Page {
    path: PathBuf,
    prefix_path: PathBuf,
    postfix_path: PathBuf
}
impl Page {

    fn get_path_if_entry_ends_with(dir_entry: &fs::DirEntry, pat: &str) -> Option<PathBuf> {
        let condition: bool = dir_entry.file_name().to_str()?.ends_with(pat);
        if condition {
            Some(dir_entry.path())
        } else {
            None
        }
    }

    fn get_entry_path_if_prefix(dir_entry: &fs::DirEntry) -> Option<PathBuf> {
        Page::get_path_if_entry_ends_with(dir_entry, ".textmachine.prefix.md")
    }
    fn get_entry_path_if_postfix(dir_entry: &fs::DirEntry) -> Option<PathBuf> {
        Page::get_path_if_entry_ends_with(dir_entry, ".textmachine.postfix.md")
    }
    fn get_entry_path_if_page(dir_entry: &fs::DirEntry) -> Option<PathBuf> {
        Page::get_path_if_entry_ends_with(dir_entry, ".textmachine.page.md")
    }

    pub fn create_empty() -> Page {
        return Page {
            path: PathBuf::new(),
            prefix_path: PathBuf::new(),
            postfix_path: PathBuf::new()
        }
    }

    fn missing_page_path(&self) -> bool {
        let s: Option<&str> = self.path.to_str();
        if s.is_none() {
            return true
        } else {
            return s.unwrap().is_empty()
        }
    }

    pub fn new(containing_dir: PathBuf) -> PossiblyNewPage {
        let dir_read: io::Result<fs::ReadDir> = fs::read_dir(containing_dir);
        if dir_read.is_err() {
            return PossiblyNewPage::Error(dir_read.err().unwrap())
        }

        let mut page: Page = Page::create_empty();

        for entry in dir_read.unwrap() {
            if entry.is_err() {
                return PossiblyNewPage::Error(entry.err().unwrap())
            }
            let unwrapped_entry: fs::DirEntry = entry.unwrap();

            {
                let potential_page_path: Option<PathBuf> = Page::get_entry_path_if_page(&unwrapped_entry);
                if potential_page_path.is_some() {
                    page.path = potential_page_path.unwrap();
                    continue;
                }
            }
            {
                let potential_prefix_path: Option<PathBuf> = Page::get_entry_path_if_prefix(&unwrapped_entry);
                if potential_prefix_path.is_some() {
                    page.prefix_path = potential_prefix_path.unwrap();
                    continue;
                }
            }
            {
                let potential_postfix_path: Option<PathBuf> = Page::get_entry_path_if_postfix(&unwrapped_entry);
                if potential_postfix_path.is_some() {
                    page.postfix_path = potential_postfix_path.unwrap();
                    continue;
                }
            }

        }

        if page.missing_page_path() {
            return PossiblyNewPage::NoError(None)
        }        

        PossiblyNewPage::NoError(Some(page))
    }

}

type PageList = Vec<Pages>;

pub struct Pages {
    list: PageList
}
