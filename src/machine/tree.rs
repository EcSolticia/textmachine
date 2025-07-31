use std::{fs, io, path::{PathBuf, Component}};

mod helpers {
    use super::*;

    pub fn get_filename(path: &str) -> Result<String, NodeError> {
        let pathb: PathBuf = PathBuf::from(path);

        if let Some(last_component) = pathb.components().last() {
            match last_component {
                Component::Normal(ostr) => {
                    let ostring = ostr.to_os_string();
                    if let Ok(string) = ostring.into_string() {
                        return Ok(string)
                    } else {
                        return Err(NodeError::NodeError(
                            format!("path name in input dir has invalid unicode characters")
                        ))
                    }
                },
                Component::CurDir => {
                    return Err(NodeError::NodeError(
                        format!("cannot include current dir as an input")
                    ))
                },
                Component::ParentDir => {
                    return Err(NodeError::NodeError(
                        format!("cannot include parent dir as an input")
                    ))
                },
                Component::Prefix(_) => {
                    return Err(NodeError::NodeError(
                        format!("cannot include windows path prefix as an input")
                    ))
                },
                Component::RootDir => {
                    return Err(NodeError::NodeError(
                        format!("cannot include root dir as an input")
                    ))
                }
            }
        }

        return Err(NodeError::NodeError(
            String::from("unexpected error occured")
        ))
    }

    pub fn get_nodetype(path: &str) -> Result<NodeType, NodeError> {
        let meta: fs::Metadata = fs::metadata(path)?;

        if meta.is_dir() {
            return Ok(NodeType::Dir)
        } else if meta.is_file() {
            if path.ends_with(".md") {
                return Ok(NodeType::Page)
            } else {
                return Ok(NodeType::NormalFile)
            }
        } else if meta.is_symlink() {
            return Err(NodeError::NodeError(
                format!("symlinks in inputs not supported")
            ))
        } else {
            return Err(NodeError::NodeError(
                format!("found unexpected file type in inputs")
            ))
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NodeError {
    #[error("[textmachine-error] {0}")]
    NodeError(String),
    #[error("{0}")]
    IoError(#[from] io::Error)
}

#[derive(Debug)]
pub enum NodeType {
    NormalFile,
    Page,
    Dir
}

#[derive(Debug)]
pub struct Node {
    name: String,
    node_type: NodeType,
    children: Vec<Node>
}
impl Node {

    pub fn from(path: &str) -> Result<Node, NodeError> {
        let node_type: NodeType = helpers::get_nodetype(path)?;

        let name: String = helpers::get_filename(path)?;

        let mut working_children: Vec<Node> = vec![];

        match node_type {
            NodeType::Dir => {
                let dir_read: fs::ReadDir = fs::read_dir(PathBuf::from(path))?;

                for wrapped_entry in dir_read {
                    let entry: fs::DirEntry = wrapped_entry?;

                    let epathb: PathBuf = entry.path();

                    let new_path: &str;
                    if let Some(npath) = epathb.to_str() {
                        new_path = npath;
                    } else {
                        return Err(NodeError::NodeError(
                            format!("path name in input dir has invalid unicode characters")
                        ))
                    }

                    let new_node: Node = Node::from(new_path)?;
                    working_children.push(new_node);
                }
            },
            _ => ()
        }

        Ok(
            Node {
                name: name,
                node_type: node_type,
                children: working_children
            }
        )
    }
}
