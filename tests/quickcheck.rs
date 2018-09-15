extern crate yaml_rust;
#[macro_use]
extern crate quickcheck;
#[macro_use]
extern crate quickcheck_derive;

use quickcheck::TestResult;
use std::error::Error;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

#[derive(Arbitrary, Clone, Debug)]
enum ArbitraryYaml {
    Array(Vec<NonRecursive>),
    Hash(Vec<(NonRecursive, NonRecursive)>),
    NonRecursive(NonRecursive),
}

#[derive(Arbitrary, Clone, Debug)]
enum NonRecursive {
    Integer(i64),
    String(String),
    Boolean(bool),
    Null,
}

impl ArbitraryYaml {
    fn to_yaml(self) -> Yaml {
        match self {
            ArbitraryYaml::Array(v) => {
                Yaml::Array(v.into_iter().map(NonRecursive::to_yaml).collect())
            }
            ArbitraryYaml::Hash(v) => Yaml::Hash(
                v.into_iter()
                    .map(|(k, v)| (k.to_yaml(), v.to_yaml()))
                    .collect(),
            ),
            ArbitraryYaml::NonRecursive(v) => v.to_yaml(),
        }
    }
}

impl NonRecursive {
    fn to_yaml(self) -> Yaml {
        match self {
            NonRecursive::Integer(v) => Yaml::Integer(v),
            NonRecursive::String(v) => Yaml::String(v),
            NonRecursive::Boolean(v) => Yaml::Boolean(v),
            NonRecursive::Null => Yaml::Null,
        }
    }
}

quickcheck! {
    fn test_check_weird_keys(xs: Vec<String>) -> TestResult {
        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(&Yaml::Array(xs.into_iter().map(Yaml::String).collect())).unwrap();
        }
        if let Err(err) = YamlLoader::load_from_str(&out_str) {
            return TestResult::error(err.description());
        }
        TestResult::passed()
    }

    fn test_identity(arbitrary_yaml: ArbitraryYaml) -> TestResult {
        let mut out_str = String::new();
        let input = arbitrary_yaml.to_yaml();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(&input).unwrap();
        }
        match YamlLoader::load_from_str(&out_str) {
            Ok(output) => TestResult::from_bool(output.len() == 1 && input == output[0]),
            Err(err) => TestResult::error(err.description()),
        }
    }
}
