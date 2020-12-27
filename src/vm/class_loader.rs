use crate::class::Class;
use crate::error::Result;
use crate::error::{Error, ErrorKind};
use crate::io::class::ClassReader;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

pub struct ClassLoader {
    classes: HashMap<String, Rc<Class>>,
    paths: Vec<String>,
}

impl ClassLoader {
    pub fn new() -> Self {
        ClassLoader {
            classes: HashMap::new(),
            paths: Vec::new(),
        }
    }

    pub fn set_paths(&mut self, paths: Vec<&str>) {
        self.paths = paths.iter().map(|s| String::from(*s)).collect();
    }

    pub fn load_class_file(&mut self, filename: &str) -> Result<Rc<Class>> {
        let class = ClassReader::from_file(filename)?;

        let c = Rc::new(class);
        let r = c.clone();
        self.classes = self.classes.clone();
        self.classes.insert(c.this_class.clone(), c);

        Ok(r)
    }

    pub fn resolve(&mut self, class_name: &str) -> Result<Rc<Class>> {
        if let Some(class) = self.classes.get(class_name) {
            Ok(class.clone())
        } else {
            self.find_class_file(class_name)
                .map(|filename| self.load_class_file(&filename))
                .unwrap_or_else(|| {
                    Err(Error::new(
                        ErrorKind::RuntimeError,
                        Some(format!(
                            "Could not resolve class {} in [{}]",
                            class_name,
                            self.paths.join(", ")
                        )),
                    ))
                })
        }
    }

    fn find_class_file(&self, class_name: &str) -> Option<String> {
        let mut filename = class_name.to_string();
        filename.push_str(".class");

        for path in &self.paths {
            let mut path = path.to_owned();
            path.push_str(&filename);

            if Path::new(&path).exists() {
                return Some(path);
            }
        }

        None
    }
}
