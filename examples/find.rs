use clap::{Parser, ValueEnum};
use iocore::{Error, Path, WalkProgressHandler, walk_dir};

fn main() -> Result<(), iocore::Error> {
    let opt = Opt::parse();
    let matcher = opt.matcher();

    walk_dir(&Path::from(&opt.origin), matcher, opt.max_depth)?;
    Ok(())
}

#[derive(Debug, Clone, ValueEnum, Copy)]
pub enum Type {
    File,
    Any,
    Directory,
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Opt {
    #[arg(required = false, default_value = ".", value_parser=path_to_directory )]
    pub origin: Path,

    #[arg(long)]
    pub name: String,

    #[arg(long = "type")]
    pub file_type: Option<Type>,

    #[arg(short, long)]
    pub suppress_errors: bool,

    #[arg(short, long, default_value = "3")]
    pub max_depth: Option<usize>,
}
impl Opt {
    pub fn matcher(&self) -> SimpleMatcher {
        SimpleMatcher {
            matches: Vec::new(),
            opt: self.clone(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct SimpleMatcher {
    matches: Vec<String>,
    opt: Opt,
}

impl WalkProgressHandler for SimpleMatcher {
    fn path_matching(&mut self, path: &Path) -> Result<bool, Error> {
        if self.matches.contains(&path.name()) {
            Ok(false)
        } else {
            if path.name() == self.opt.name.clone() {
                println!("{}", path);
                self.matches.push(path.name());
                Ok(true)
            } else if path.is_dir() {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }

    fn error(&mut self, _: &Path, y: Error) -> Option<Error> {
        Some(y)
        //(!self.opt.suppress_errors).then_some(y)
    }
}

pub fn path_to_directory(s: &str) -> ::std::result::Result<Path, String> {
    let path = Path::new(s).canonicalize().map_err(|y| y.to_string())?;
    if path.is_dir() {
        Ok(path)
    } else {
        Err(format!("{:#?} is not a directory", path))
    }
}
