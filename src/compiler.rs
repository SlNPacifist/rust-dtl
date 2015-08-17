use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind, Read, Result};
use std::fs::File;
use std::fs;

use filter::{FilterFunction, DEFAULT_FILTERS};
use template::Template;
use context::Context;

pub type FilterStorage = HashMap<String, FilterFunction>;

fn read_file(path: &Path) -> Result<String> {
    let mut text = String::new();
    let mut file = try!(File::open(path));
    try!(file.read_to_string(&mut text));
    Ok(text)
}


pub struct TemplateCompiler {
	pub filters: FilterStorage,
	pub root: PathBuf,
}

impl TemplateCompiler {
	fn get_default_filters() -> FilterStorage {
		let mut filters: FilterStorage = HashMap::new();
		for i in (0..DEFAULT_FILTERS.len()) {
			let (name, func) = DEFAULT_FILTERS[i];
			filters.insert(name.to_string(), func);
		}
		filters
	}
	
	pub fn new(root: PathBuf) -> Result<TemplateCompiler> {
	    let metadata = try!(fs::metadata(root.clone()));
	    match metadata.is_dir() {
	    	true => Ok(
	    		TemplateCompiler {
					root: root,
					filters: Self::get_default_filters(),
				}
    		),
	    	false => Err(
	    		Error::new(
	    			ErrorKind::InvalidInput,
	    			format!("{} is not directory", root.as_path().display())
    			)
    		),
	    }
	}
	
	pub fn compile_file(&self, path: &Path) -> Result<Template> {
	    let mut filepath = self.root.clone();
	    filepath.push(path);
		let text = try!(read_file(filepath.as_path()));
		Template::compile(text, self)
	}
	
	pub fn render_file(root: PathBuf, path: &Path, context: &Context) -> Result<String> {
		let compiler = try!(Self::new(root));
		Ok(try!(compiler.compile_file(path)).render(context))
	}
}