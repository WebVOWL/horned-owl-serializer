mod remote_types;

use horned_owl::{
    error::HornedError,
    io::{ParserConfiguration, ParserOutput, RDFParserConfiguration, ResourceType},
    model::{Build, RcAnnotatedComponent, RcStr},
    ontology::set::SetOntology,
};
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

fn main() {
    println!("Hello, world!");
    let parser_config = ParserConfiguration {
        rdf: RDFParserConfiguration::default(),
        ..Default::default()
    };
    parse(Path::new("envo.owl"), parser_config);
}

pub fn parse(path: &Path, parser_config: ParserConfiguration) {
    let r = parse_path(path, parser_config).unwrap();
    match r {
        horned_owl::io::ParserOutput::OFNParser(ont, map) => {
            let hash_map: HashMap<&String, &String> = map.mappings().collect();
            println!("Ontology:\n{ont:#?}\n\nMapping:\n{hash_map:#?}");
        }
        horned_owl::io::ParserOutput::OWXParser(ont, map) => {
            let hash_map: HashMap<&String, &String> = map.mappings().collect();
            println!("Ontology:\n{ont:#?}\n\nMapping:\n{hash_map:#?}");
        }
        horned_owl::io::ParserOutput::RDFParser(ont, inc) => {
            let so: SetOntology<_> = ont.into();
            println!("Ontology:\n{so:#?}");
            println!("Incomplete Parse:\n{inc:#?}");
        }
    }
}

pub fn parse_path(
    path: &Path,
    config: ParserConfiguration,
) -> Result<ParserOutput<RcStr, RcAnnotatedComponent>, HornedError> {
    Ok(match path_type(path) {
        Some(ResourceType::OFN) => {
            let file = File::open(path)?;
            let mut bufreader = BufReader::new(file);
            ParserOutput::ofn(horned_owl::io::ofn::reader::read(&mut bufreader, config)?)
        }
        Some(ResourceType::OWX) => {
            let file = File::open(path)?;
            let mut bufreader = BufReader::new(file);
            ParserOutput::owx(horned_owl::io::owx::reader::read(&mut bufreader, config)?)
        }
        Some(ResourceType::RDF) => {
            let b = Build::new();
            let iri = horned_owl::resolve::path_to_file_iri(&b, path);
            ParserOutput::rdf(horned_owl::io::rdf::closure_reader::read(&iri, config)?)
        }
        None => {
            return Err(HornedError::CommandError(format!(
                "Cannot parse a file of this format: {path:?}"
            )));
        }
    })
}

pub fn path_type(path: &Path) -> Option<ResourceType> {
    match path.extension().and_then(|s| s.to_str()) {
        Some("ofn") => Some(ResourceType::OFN),
        Some("owx") => Some(ResourceType::OWX),
        Some("owl") => Some(ResourceType::RDF),
        _ => None,
    }
}



