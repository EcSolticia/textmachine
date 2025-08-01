use super::tree::{Node, NodeType};

pub fn generate(tree: Node, root_dir: &str) {

    for child in tree.get_children() {
        let rel_path: String = root_dir.to_string() + "/" + child.get_name().as_str();
        match child.get_nodetype() {
            NodeType::Dir => {
                generate(child, rel_path.as_str())
            },
            NodeType::NormalFile => {
                println!("[textmachine-generator] normal file: {}", rel_path)
            },
            NodeType::Page => {
                println!("[textmachine-generator] page: {}", rel_path)
            }
        }
    }
}
