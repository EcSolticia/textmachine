use super::{tree::{Node, NodeType}, ui::note::{self, Issuer}};
use std::{io, fs, path};
use pandoc::{self, Pandoc, PandocOutput, PandocOption};
use tempfile::NamedTempFile;
use super::{errors, ui};

fn md_path_to_html(path: &str) -> Result<String, errors::Error> {
    if let Some(name) = path.strip_suffix(".md") {
        return Ok(String::from(name) + ".html")
    } else {
        return Err(
            errors::Error::SoftwareIllogic(
                String::from("non-markdown input passed to the '.md' to '.html' path converter\n")
            )
        )
    }
}

fn add_lua_filters(pandoc: &mut pandoc::Pandoc) -> io::Result<NamedTempFile> {
    let filter: &str = include_str!("./../../resources/filters.lua");

    let filter_tmp_file: NamedTempFile = NamedTempFile::new()?;
    
    let filter_path: path::PathBuf = filter_tmp_file.path().to_path_buf();

    fs::write(&filter_path, filter)?;
    
    pandoc.add_option(PandocOption::LuaFilter(filter_path));
    Ok(filter_tmp_file)
}

pub fn generate_page(from: &str, to: &str) -> Result<PandocOutput, errors::Error> {

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

    let output: PandocOutput = pandoc.execute()?;
    Ok(output)
}

#[derive(Debug, Clone)]
pub struct GenPandocOutput {
    path: String
}
impl GenPandocOutput {
    pub fn path(&self) -> String {
        self.path.clone()
    }

    fn make_string(pathb: path::PathBuf) -> Result<String, errors::Error> {
        if let Some(path_str) = pathb.to_str() {
            return Ok(String::from(path_str))
        } else {
            return Err(
                errors::Error::InvalidUnicode(
                    String::from("cannot word with pandoc output path")
                )
            )
        }
    }
    
    fn from(pandoc_output: PandocOutput) -> Result<GenPandocOutput, errors::Error> {
        match pandoc_output {
            PandocOutput::ToFile(pathb) => {
                let path: String = Self::make_string(pathb)?;
                Ok(GenPandocOutput{path: path})
            },
            _ => {
                Err(errors::Error::UnknownError(
                    String::from("generated HTML is not a file")
                ))
            }
        }
    }
}
impl ui::note::Issuer for GenPandocOutput {
    fn issue(&self) -> ui::note::Note {
        ui::note::Note::from("gen-pandoc-output", self.path())
    }
}

#[derive(Debug)]
pub struct GenOutput {
    pandoc_outputs: Vec<GenPandocOutput>
}
impl GenOutput {
    pub fn present_gen_outputs(&self) {
        let working_outputs = self.pandoc_outputs.clone();
        for po in working_outputs {
            let note: note::Note = po.issue();
            note.present();
        }
    }

    fn extend_pandoc_outputs(&mut self, pos: Vec<GenPandocOutput>) {
        self.pandoc_outputs.extend_from_slice(&pos);
    }

    fn add_pandoc_output(&mut self, po: GenPandocOutput) {
        self.pandoc_outputs.push(po);
    }

    fn new() -> GenOutput {
        GenOutput {pandoc_outputs: vec![]}
    }
}

pub fn generate(tree: Node, root_dir: &str, input_root_dir: &str) -> Result<GenOutput, errors::Error> {

    fs::create_dir(root_dir)?;

    let mut gen_output: GenOutput = GenOutput::new();
    
    for child in tree.get_children() {
        let rel_path: String = root_dir.to_string() + "/" + child.get_name().as_str();
        let input_rel_path: String = input_root_dir.to_string() + "/" + child.get_name().as_str();
        match child.get_nodetype() {
            NodeType::Dir => {
                println!("[textmachine-generator] dir: {}; from: {}", rel_path, input_rel_path);
                let new_genpandoc_outputs: Vec<GenPandocOutput> = generate(child, rel_path.as_str(), &input_rel_path)?.pandoc_outputs;
                gen_output.extend_pandoc_outputs(new_genpandoc_outputs);
            },
            NodeType::NormalFile => {
                fs::File::create(&rel_path)?;
                fs::copy(&input_rel_path, &rel_path)?;
                println!("[textmachine-generator] normal file: {}; from: {}", rel_path, input_rel_path)
            },
            NodeType::Page => {
                let page_path: String = md_path_to_html(&rel_path)?;
                let new_pandoc_output: PandocOutput = generate_page(&input_rel_path, &page_path)?;
                let new_genpandoc_output: GenPandocOutput = GenPandocOutput::from(new_pandoc_output)?;
                gen_output.add_pandoc_output(new_genpandoc_output);
                println!("[textmachine-generator] page: {}; from: {}", page_path, input_rel_path)
            }
        }
    }

    Ok(gen_output)
}
