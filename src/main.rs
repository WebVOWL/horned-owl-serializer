mod remote_types;

use anyhow::Result;
use horned_owl::{
    error::HornedError,
    io::{ParserConfiguration, ParserOutput, RDFParserConfiguration, ResourceType},
    model::{Build, ForIRI, RcAnnotatedComponent, RcStr},
    ontology::{component_mapped::{ComponentMappedOntology, RcComponentMappedOntology}, indexed::ForIndex, set::SetOntology},
};
use datafusion::{prelude::SessionConfig};
use rdf_fusion::{io::{RdfFormat, RdfParser, RdfSyntaxError}, model::Quad, storage::memory::{MemObjectIdMapping, MemQuadStorage}};
use std::{fs::File, io::{BufReader, BufWriter, Write}, path::Path, rc::Rc, sync::Arc};
// use rkyv::{rancor::Error}; // Commented out since not currently used
use rdf_fusion::store::Store;
use futures::StreamExt;

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error>{
    println!("Hello, world!");
    let storage = Store::default();
    let parser_config = ParserConfiguration {
        rdf: RDFParserConfiguration::default(),
        ..Default::default()
    };
    let quads = parse(Path::new("and-complex.owl"), parser_config);
    //println!("Quads: {:?}", quads);
    let clean = quads.unwrap_or_else(|_| Vec::new());
    storage.extend(clean).await?;
    println!("Storage: {}", storage.len().await?);
    let mut stream: rdf_fusion::execution::results::QuadStream = storage.stream().await?;
    println!("Quads:");
    while let Some(quad) = stream.next().await {
        println!("{}", quad?);
    }
    Ok(())
}

pub fn parse(path: &Path, parser_config: ParserConfiguration) -> Result<Vec<Quad>, RdfSyntaxError> {
    let r = parse_path(path, parser_config).unwrap();
    match r {
        horned_owl::io::ParserOutput::OFNParser(ont, _map) => {
            //let hash_map: HashMap<&String, &String> = map.mappings().collect();
            print!("OFNParse");
            let ontology_debug = format!("Ontology:\n{ont:#?}");
            let reader = RdfParser::from_format(RdfFormat::RdfXml);
            let quads = reader
                .rename_blank_nodes()
                .for_slice(ontology_debug.as_bytes())
                .collect::<Result<Vec<Quad>, _>>();
            return quads;
            //println!("Ontology:\n{ont:#?}\n\nMapping:\n{hash_map:#?}");
        }
        horned_owl::io::ParserOutput::OWXParser(ont, map) => {
            //let hash_map: HashMap<&String, &String> = map.mappings().collect();
            print!("OWXParser");
            let cmtont: RcComponentMappedOntology = ont.into();
            let mut buff = BufWriter::new(Vec::new());
            horned_owl::io::rdf::writer::write_raw(buff.get_mut(), &cmtont);
            let reader = RdfParser::from_format(RdfFormat::RdfXml);
            let quads = reader
                .rename_blank_nodes()
                .for_slice(&buff.get_ref())
                .collect::<Result<Vec<Quad>, _>>();
            return quads;
            //println!("Ontology:\n{ont:#?}\n\nMapping:\n{hash_map:#?}");
        }
        horned_owl::io::ParserOutput::RDFParser(ont, _inc) => {
            //let so: SetOntology<_> = ont.into();
            print!("RDFParser");
            let cmtont: RcComponentMappedOntology = ont.into();
            let mut buff = BufWriter::new(Vec::new());
            let clean = horned_owl::io::rdf::writer::write_raw(&mut buff, &cmtont);
            //let mut read_buff = BufReader::new(buff);
            let reader = RdfParser::from_format(RdfFormat::RdfXml);
            let out = buff.into_inner().inspect(|x: &Vec<u8>| print!("{:?}", String::from_utf8(x.to_vec())));
            let new_buff = BufWriter::new(out.unwrap());
            
            let quads = reader
                .rename_blank_nodes()
                .for_slice(&new_buff.get_ref())
                .collect::<Result<Vec<Quad>, _>>();
            return quads;
            //let _bytes = rkyv::to_bytes::<Error>(&so).unwrap();
            //let archived = rkyv::access::<ArchivedTest, Error>(&bytes[..]).unwrap();
            //assert_eq!(archived, &value);
            //let deserialized = deserialize::<Test, Error>(archived).unwrap();
            //assert_eq!(deserialized, value);

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
fn create_storage(config: &SessionConfig) -> Arc<MemQuadStorage> {
    let mapping = Arc::new(MemObjectIdMapping::default());
    Arc::new(MemQuadStorage::new(mapping, config.batch_size()))
}




