use super::tree::{Node, NodeType};
use std::{io, fs, path};
use pandoc::{self, Pandoc, PandocOutput, PandocOption};
use tempfile::NamedTempFile;

fn md_path_to_html(path: &str) -> String {
    let path: path::PathBuf = path::PathBuf::from(path).with_extension("html");
    path.to_str().unwrap().to_string() //error-handle here
}

fn add_lua_filters(pandoc: &mut pandoc::Pandoc) -> io::Result<NamedTempFile> {
    let filter: &str = include_str!("./../../resources/filters.lua");

    let filter_tmp_file: NamedTempFile = NamedTempFile::new()?;
    
    let filter_path: path::PathBuf = filter_tmp_file.path().to_path_buf();

    fs::write(&filter_path, filter)?;
    
    pandoc.add_option(PandocOption::LuaFilter(filter_path));
    Ok(filter_tmp_file)
}

pub fn generate_page(from: &str, to: &str) -> io::Result<()> {

    let mut pandoc: Pandoc = pandoc::new();
    pandoc.add_input(from);
    pandoc.set_output(pandoc::OutputKind::File(
        path::PathBuf::from(to)
    ));

    let _lua_filter_file = add_lua_filters(&mut pandoc);
    pandoc.add_options(&vec![
        PandocOption::Standalone,
        PandocOption::TableOfContents,
        PandocOption::NumberSections
    ]);

    pandoc.execute().unwrap(); //error-handle here
        
    Ok(())
}

pub fn generate(tree: Node, root_dir: &str, input_root_dir: &str) -> io::Result<()> {

    fs::create_dir(root_dir)?;
    
    for child in tree.get_children() {
        let rel_path: String = root_dir.to_string() + "/" + child.get_name().as_str();
        let input_rel_path: String = input_root_dir.to_string() + "/" + child.get_name().as_str();
        match child.get_nodetype() {
            NodeType::Dir => {
                println!("[textmachine-generator] dir: {}; from: {}", rel_path, input_rel_path);
                generate(child, rel_path.as_str(), &input_rel_path)?;
            },
            NodeType::NormalFile => {
                fs::File::create(&rel_path)?;
                fs::copy(&input_rel_path, &rel_path)?;
                println!("[textmachine-generator] normal file: {}; from: {}", rel_path, input_rel_path)
            },
            NodeType::Page => {
                let page_path: String = md_path_to_html(&rel_path);
                generate_page(&input_rel_path, &page_path);
                println!("[textmachine-generator] page: {}; from: {}", page_path, input_rel_path)
            }
        }
    }

    Ok(())
}
