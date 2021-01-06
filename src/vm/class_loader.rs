use crate::class::{Class, MethodInfo};
use crate::error::Result;
use crate::io::class::ClassReader;
use crate::vm::class_loader::ClassSource::{Folder, Jar};
use crate::vm::frame::Frame;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::rc::Rc;
use zip::ZipArchive;

trait FileSource {
    fn load_class(&self, filename: &str) -> Result<Option<Class>>;
}

#[derive(Debug)]
struct FolderSource {
    path: String,
}

impl FileSource for FolderSource {
    fn load_class(&self, filename: &str) -> Result<Option<Class>> {
        let mut path = self.path.clone();
        path.push_str(&filename);
        path.push_str(".class");

        if Path::new(&path).exists() {
            Ok(Some(ClassReader::from_file(path)?))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
struct JarSource {
    path: String,
}

impl FileSource for JarSource {
    fn load_class(&self, filename: &str) -> Result<Option<Class>> {
        let mut filename = filename.to_owned();
        filename.push_str(".class");

        let file = File::open(&self.path).unwrap();
        let mut zip = ZipArchive::new(file).unwrap(); // TODO don't unwrap

        let file = match zip.by_name(&filename) {
            Ok(file) => file,
            Err(_) => return Ok(None),
        };

        Ok(Some(ClassReader::new(BufReader::new(file)).read_class()?))
    }
}

#[derive(Debug)]
enum ClassSource {
    Folder(FolderSource),
    Jar(JarSource),
}

impl FileSource for ClassSource {
    fn load_class(&self, filename: &str) -> Result<Option<Class>> {
        match self {
            Folder(f) => f.load_class(filename),
            Jar(j) => j.load_class(filename),
        }
    }
}

impl From<&str> for ClassSource {
    fn from(str: &str) -> Self {
        if str.ends_with(".jar") {
            Jar(JarSource {
                path: str.to_owned(),
            })
        } else {
            Folder(FolderSource {
                path: str.to_owned(),
            })
        }
    }
}

pub struct ClassLoader {
    classes: HashMap<String, Rc<Class>>,
    sources: Vec<ClassSource>,
}

impl ClassLoader {
    pub fn new() -> Self {
        ClassLoader {
            classes: HashMap::new(),
            sources: Vec::new(),
        }
    }

    pub fn set_paths(&mut self, paths: Vec<&str>) {
        self.sources = paths.iter().map(|s| (*s).into()).collect();
    }

    pub fn resolve(&mut self, class_name: &str) -> Result<(Rc<Class>, Option<Frame>)> {
        if let Some(class) = self.classes.get(class_name) {
            Ok((class.clone(), None))
        } else {
            let class = self.load_class(class_name)?;
            let init_frame = class.find_static_method("<clinit>").map(|m| {
                let mut frame = Frame::new(class.clone(), m);
                frame.implicit = true;
                frame
            });

            Ok((class, init_frame))
        }
    }

    pub fn resolve_static_method(
        &mut self,
        class_name: &str,
        method_name: &str,
        descriptor: &str,
    ) -> Result<(Rc<Class>, Rc<MethodInfo>, Vec<Frame>)> {
        let mut current_class = class_name.to_owned();
        let mut init_frames = Vec::new();
        loop {
            let (class, init_frame) = self.resolve(&current_class)?;

            if let Some(frame) = init_frame {
                init_frames.push(frame);
            }

            if let Some(method) = class.resolve_static_method(method_name, descriptor) {
                return Ok((class, method, init_frames));
            }

            if class.super_class.is_empty() {
                panic!(
                    "Could not resolve method {}:{} on class {}",
                    method_name, descriptor, class_name
                )
            }

            current_class = class.super_class.to_owned();
        }
    }

    fn load_class(&mut self, class_name: &str) -> Result<Rc<Class>> {
        for source in &self.sources {
            if let Some(class) = source.load_class(class_name)? {
                let c = Rc::new(class);
                let r = c.clone();

                self.classes = self.classes.clone();
                self.classes.insert(c.this_class.clone(), c);

                return Ok(r);
            }
        }

        panic!("Could not resolve class {}", class_name)
    }
}
