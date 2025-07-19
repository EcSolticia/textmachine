use crate::machine::{self, input};
use std::{path::PathBuf};

pub fn generate_traced_pages(traced_pages: input::TracedPages) {
    let list: input::PageList = traced_pages.get_list().clone();

    for page in list {
        
    }
}
