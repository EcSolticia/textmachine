use std::{fs, io, path::PathBuf};

type PotentialPage = io::Result<Option<Page>>;

#[derive(Debug, Clone)]
pub struct Page {
    path: PathBuf,
    prefix_path: Option<PathBuf>,
    postfix_path: Option<PathBuf>
}
impl Page {
    pub fn path(&self) -> PathBuf {
        return self.path.clone();
    }
    pub fn prefix_path(&self) -> Option<PathBuf> {
        return self.prefix_path.clone();
    }
    pub fn postfix_path(&self) -> Option<PathBuf> {
        return self.postfix_path.clone();
    }

    fn get_path_if_entry_ends_with(dir_entry: &fs::DirEntry, pat: &str) -> Option<PathBuf> {
        let condition: bool = dir_entry.file_name().to_str()?.ends_with(pat);
        if condition {
            Some(dir_entry.path())
        } else {
            None
        }
    }

    fn get_entry_path_if_prefix(dir_entry: &fs::DirEntry) -> Option<PathBuf> {
        Page::get_path_if_entry_ends_with(dir_entry, ".textmachine.prefix")
    }
    fn get_entry_path_if_postfix(dir_entry: &fs::DirEntry) -> Option<PathBuf> {
        Page::get_path_if_entry_ends_with(dir_entry, ".textmachine.postfix")
    }
    fn get_entry_path_if_page(dir_entry: &fs::DirEntry) -> Option<PathBuf> {
        Page::get_path_if_entry_ends_with(dir_entry, ".md")
    }

    pub fn create_empty() -> Page {
        Page {
            path: PathBuf::new(),
            prefix_path: None,
            postfix_path: None
        }
    }

    pub fn new(containing_dir: PathBuf) -> PotentialPage {
        let dir_read: fs::ReadDir = fs::read_dir(containing_dir)?;
        
        let mut page: Page = Page::create_empty();
        let mut page_path: Option<PathBuf> = None;

        for entry in dir_read {
            let unwrapped_entry: fs::DirEntry = entry?;

            if page_path.is_none() {
                page_path = Page::get_entry_path_if_page(&unwrapped_entry);
            }
            if page.prefix_path.is_none() {
                page.prefix_path = Page::get_entry_path_if_prefix(&unwrapped_entry);
            }
            if page.postfix_path.is_none() {
                page.postfix_path = Page::get_entry_path_if_postfix(&unwrapped_entry);
            }

        }

        if page_path.is_none() {
            return Ok(None)
        } else {
            page.path = page_path.unwrap();
        }

        Ok(Some(page))
    }

}

pub type PageList = Vec<Page>;

#[derive(Debug, Clone)]
pub struct TracedPages {
    list: PageList
}
impl TracedPages {
    pub fn get_list(&self) -> &PageList {
        &self.list
    }

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
