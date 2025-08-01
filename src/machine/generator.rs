use super::tree::{Node, NodeType};
use std::{io, fs, path};

fn md_path_to_html(path: &str) -> String {
    let path: path::PathBuf = path::PathBuf::from(path).with_extension("html");
    path.to_str().unwrap().to_string() //error-handle here
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
                println!("[textmachine-generator] page: {}; from: {}", page_path, input_rel_path)
            }
        }
    }

    Ok(())
}
