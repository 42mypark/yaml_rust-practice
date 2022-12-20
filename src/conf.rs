extern crate yaml_rust;

use std::error::Error;
use std::fs;

use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

#[derive(Debug)]
struct FileReadError;

impl std::fmt::Display for FileReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid file")
    }
}

impl Error for FileReadError {}

#[derive(PartialEq)]
struct Conf {
    dictionary: Yaml,
    programs: Yaml,
}

impl Conf {
    fn new(file_path: &str) -> Self {
        match Conf::parsefile(file_path) {
            Ok(yaml) => Conf {
                programs: yaml["programs"].clone(),
                dictionary: yaml,
            },
            Err(e) => panic!("invalid file"), //?
        }
    }

    fn program_list(&self) -> Vec<String> {
        let mut list = Vec::<String>::new();
        let programs = &self.programs;

        for (k, _) in programs.clone().into_hash().unwrap().into_iter() {
            if let Some(s) = k.into_string() {
                list.push(s);
            }
        }
        list
    }

    fn diff_program(&self, other: &Conf, program_name: &str) -> bool {
        self.programs[program_name] != other.programs[program_name]
    }

    fn readfile(file_path: &str) -> Result<String, Box<dyn Error>> {
        let contents = fs::read_to_string(file_path)?;
        Ok(contents)
    }

    fn file_to_hashmap(s: &str) -> Result<Yaml, Box<dyn Error>> {
        let mut docs = YamlLoader::load_from_str(s).unwrap();
        match docs.pop() {
            Some(y) => Ok(y),
            None => Err(Box::<FileReadError>::new(FileReadError)),
        }
    }

    fn parsefile(file_path: &str) -> Result<Yaml, Box<dyn Error>> {
        let contents = Conf::readfile(file_path)?;
        let yaml = Conf::file_to_hashmap(&contents)?;
        Ok(yaml)
    }
}

#[cfg(test)]
mod tests {

    use std::mem::size_of_val;

    use super::*;

    #[test]
    fn test_program_list() {
        let c = Conf::new("./test/tm.conf.yaml");
        let list = c.program_list();

        for e in list {
            println!("{e}");
        }
    }

    #[test]
    fn test_diff_program() {
        let c1 = Conf::new("./test/tm.conf.yaml");
        let c2 = Conf::new("./test/tm.conf2.yaml");

        assert_eq!(c1.diff_program(&c2, "nginx"), true);
        assert_eq!(c1.diff_program(&c2, "vogsphere"), false);
    }

    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }

    #[test]
    fn test() {
        let c1 = Conf::new("./test/tm.conf.yaml");
        let y = Yaml::Integer(1);

        assert_eq!(c1.dictionary["programs"], c1.programs);

        println!(
            "{} {:p}",
            size_of_val(&c1.dictionary["programs"]),
            &c1.dictionary["programs"]
        );
        println!("{:#04x} {:p}", size_of_val(&c1.dictionary), &c1.dictionary);
        println!("{:#04x} {:p}", size_of_val(&c1.programs), &c1.programs);

        // println!("{:?}", c1.dictionary["programs"] as *const Yaml);
        // println!("{:?}", c1.programs as *const Yaml);
    }
}

// ($name:ident, $t:ty, $yt:ident)
// pub type Hash = LinkedHashMap<Yaml, Yaml>;
// pub fn as_hash(&self) -> Option<&Hash> {
//     match *self {
//         Yaml::Hash(ref v) => Some(v),
//         _ => None
//     }
// }
