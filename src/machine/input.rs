use std::{fs, io, path::PathBuf};

type PotentialPage = io::Result<Option<Page>>;

#[derive(Debug, Clone)]
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

    pub fn new(containing_dir: PathBuf) -> PotentialPage {
        let dir_read: fs::ReadDir = fs::read_dir(containing_dir)?;
        
        let mut page: Page = Page::create_empty();

        for entry in dir_read {
            let unwrapped_entry: fs::DirEntry = entry?;

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
            return Ok(None)
        }        

        Ok(Some(page))
    }

}

type PageList = Vec<Page>;

#[derive(Debug)]
pub struct TracedPages {
    list: PageList
}
impl TracedPages {
    fn is_dir(entry: &fs::DirEntry) -> io::Result<bool> {
        let metadata: fs::Metadata = entry.path().metadata()?;

        Ok(metadata.is_dir())
    }

    pub fn trace_pages(root_dir: &PathBuf) -> io::Result<TracedPages> {
        let dir_read: fs::ReadDir = fs::read_dir(root_dir)?;

        let mut page_list: PageList = vec![];

        let new_page: Option<Page> = Page::new(root_dir.clone())?;
        if new_page.is_some() {
            page_list.push(new_page.unwrap());
        }

        for entry in dir_read {
            let unwrapped_entry: fs::DirEntry = entry?;

            if TracedPages::is_dir(&unwrapped_entry)? {
                let subdir_trace_results: TracedPages = TracedPages::trace_pages(&unwrapped_entry.path())?;
                page_list.extend_from_slice(&subdir_trace_results.list);
            }
        }

        Ok(TracedPages {list: page_list})
    }
}