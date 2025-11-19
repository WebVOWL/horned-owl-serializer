use std::{fs::File, io::{BufReader, Cursor}, marker::PhantomData, path::Path};
use horned_owl::{io::{rdf::reader::ConcreteRDFOntology, *}, model::{ForIRI, RcAnnotatedComponent, RcStr}, ontology::{component_mapped::RcComponentMappedOntology, set::SetOntology}};
use oxigraph::{io::{JsonLdProfileSet, RdfFormat, RdfParser, RdfSerializer}, model::{NamedNode, NamedNodeRef}, store::Store};

use crate::horned_oxi::{errors::{HornedOxiError, HornedOxiErrorKind}, horned_oxi::HornedVOWLExtract};

mod horned_oxi;

pub fn main() {
    //let session = Store::open("oxigraph.db").unwrap();
    let session = Store::new().unwrap();
    println!("Loaded {} quads", session.len().unwrap());
    let path = Path::new("data/owl-rdf/wine.owl");
    let _horned_oxi = HornedOxi::<RcStr>::new(session);
    _horned_oxi.insert_file(&path, false).expect("Error inserting file");
    println!("Loaded {} quads", _horned_oxi.session.len().unwrap());
    let ontology = _horned_oxi.get_ontology(format!("file:://{}", path.display().to_string())).unwrap();
    let horned_oxi = _horned_oxi.convert_ontology(ontology).unwrap();
    println!("{}", horned_oxi);
}

struct HornedOxi<A> {
    session: Store,
    phantom: PhantomData<A>
}
impl<A: ForIRI> HornedOxi<A> {
    pub fn new(session: Store) -> Self {
        Self { session, phantom: PhantomData }
    }

    // TTL format -> (oxittl) RDF XML quads -> (horned_owl) Normalize OWL/RDF -> Quads -> Insert into Oxigraph
    pub fn insert_file(&self, fs: &Path, lenient: bool) -> Result<(), HornedOxiError> {
        let parser = parser_from_format(fs, lenient)?;  
        let mut b_loader = self.session.bulk_loader();
        b_loader.parallel_load_from_slice(parser.parser, parser.input.as_slice())?;
        b_loader.commit()?;  
        
        Ok(())
    }

    pub fn get_ontology(&self, ontology_id: String) -> Result<SetOntology<RcStr>, HornedOxiError> {
        let quads = self.session.quads_for_pattern(
            None, 
            None, 
            None, 
            Some(NamedNodeRef::new(&ontology_id)?.into()))
            .collect::<Result<Vec<_>, _>>()?;
        
        let mut buf = Vec::new();
        let mut serializer = RdfSerializer::from_format(RdfFormat::RdfXml).for_writer(&mut buf);
        for quad in quads {
            serializer.serialize_triple(quad.as_ref())?;
        }
        let ontology = rdf::reader::read::<BufReader<Cursor<Vec<u8>>>>(
            &mut BufReader::new(Cursor::new(buf)),
            ParserConfiguration::default(),
        )?;
        Ok(ontology.0.into())
    }

    pub fn convert_ontology(&self, ontology: SetOntology<RcStr>) -> Result<HornedVOWLExtract<RcStr>, HornedOxiError> {
        //let cmpontology = RcComponentMappedOntology::from(ontology);
        let horned_oxi: HornedVOWLExtract<RcStr> = ontology.into();
        //println!("{}", horned_oxi);
        Ok(horned_oxi)
    }
}
pub enum ResourceType {
    OFN,
    OWX,
    RDF,
    OWL,
    TTL,
    NTriples,
    NQuads,
    TriG,
    JsonLd,
    N3
}
pub enum ParserInput {
    File(Vec<u8>),
    Buffer(Cursor<Vec<u8>>),
}
impl ParserInput {
    fn from_path(path: &Path) -> Result<Self, HornedOxiError> {
        std::fs::read(path)
        .map(ParserInput::File)
        .map_err(HornedOxiError::from)
    }

    fn as_slice(&self) -> &[u8] {
        match self {
            ParserInput::Buffer(cursor) => cursor.get_ref().as_slice(),
            ParserInput::File(bytes) => bytes.as_slice(),
        }
    }
}

pub struct PreparedParser {
    pub parser: RdfParser,
    pub input: ParserInput,
}

pub fn path_type(path: &Path) -> Option<ResourceType> {
    match path.extension().and_then(|s| s.to_str()) {
        Some("ofn") => Some(ResourceType::OFN),
        Some("owx") => Some(ResourceType::OWX),
        Some("owl") => Some(ResourceType::OWL),
        Some("ttl") => Some(ResourceType::TTL),
        Some("nt") => Some(ResourceType::NTriples),
        Some("nq") => Some(ResourceType::NQuads),
        Some("trig") => Some(ResourceType::TriG),
        Some("jsonld") => Some(ResourceType::JsonLd),
        Some("n3") => Some(ResourceType::N3),
        _ => None,
    }
}
pub fn parser_from_format(path: &Path, lenient: bool) -> Result<PreparedParser, HornedOxiError> {
    let make_parser = |fmt| {
        let path_str = path.to_str().unwrap();
        let parser = RdfParser::from_format(fmt) 
            .with_default_graph(NamedNode::new(format!("file:://{}", path_str)).unwrap());
        if lenient { parser.lenient() } else { parser }
    };
    let t_pat = path_type(path);
    let prepared = match t_pat {
        Some(ResourceType::OFN) => {
            let file = File::open(path)?;
            let mut reader = BufReader::new(file);
            let (ont, _)
            : (RcComponentMappedOntology, _) = ofn::reader::read(
                &mut reader,
                ParserConfiguration::default(),
            )?;
            
            let mut buf = Vec::new();
            rdf::writer::write(&mut buf, &ont)?;
            Ok(PreparedParser {
                parser: make_parser(RdfFormat::RdfXml),
                input: ParserInput::Buffer(Cursor::new(buf)),
            })
        }
        Some(ResourceType::OWX) => {
            let file = File::open(path)?;
            let mut reader = BufReader::new(file);
            let ontology = owx::reader::read::<RcStr, ConcreteRDFOntology<RcStr, RcAnnotatedComponent>, _>(
                &mut reader,
                ParserConfiguration::default(),
            )?;

            let mut buf = Vec::new();
            rdf::writer::write(&mut buf, &ontology.0.into())?;

            Ok(PreparedParser {
                parser: make_parser(RdfFormat::RdfXml),
                input: ParserInput::Buffer(Cursor::new(buf)),
            })
        }
        Some(ResourceType::TTL) => {
            let input = ParserInput::from_path(path)?;
            Ok(PreparedParser {
                parser: make_parser(RdfFormat::Turtle),
                input,
            })
        },
        Some(ResourceType::NTriples) => {
            let input = ParserInput::from_path(path)?;
            Ok(PreparedParser {
                parser: make_parser(RdfFormat::NTriples),
                input,
            })
        },
        Some(ResourceType::NQuads) => {
            let input = ParserInput::from_path(path)?;
            Ok(PreparedParser {
                parser: make_parser(RdfFormat::NQuads),
                input,
            })
        },
        Some(ResourceType::TriG) => {
            let input = ParserInput::from_path(path)?;
            Ok(PreparedParser {
                parser: make_parser(RdfFormat::TriG),
                input,
            })
        },
        Some(ResourceType::JsonLd) => {
            let input = ParserInput::from_path(path)?;
            Ok( PreparedParser {
                parser: make_parser(RdfFormat::JsonLd { profile: JsonLdProfileSet::default() }),
                input,
            })
        },
        Some(ResourceType::N3) => {
            let input = ParserInput::from_path(path)?;
            Ok(PreparedParser {
                parser: make_parser(RdfFormat::N3),
                input,
            })
        },
        Some(ResourceType::OWL) => {
            let input = ParserInput::from_path(path)?;
            Ok(PreparedParser {
                parser: make_parser(RdfFormat::RdfXml),
                input,
            })
        },
        _ => {
            Err(HornedOxiErrorKind::InvalidInput(
                format!("Unsupported parser: {}", path.display()),
            ))
        }
    };
    Ok(prepared?)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_generator::test_resources;

    #[test_resources("data/owl-functional/*.ofn")]
    fn test_ofn_parser(resource: &str) {
        use oxigraph::store::Store;
        let session = Store::new().unwrap();
        let parser = parser_from_format(Path::new(&resource), false).unwrap();
        let _ = session.load_from_slice(parser.parser, parser.input.as_slice());
        assert_ne!(session.len().unwrap(), 0, "Expected non-zero quads for: {}", resource);
    }
    #[test_resources("data/owl-rdf/*.owl")]
    fn test_owl_parser(resource: &str) {
        use oxigraph::store::Store;
        let session = Store::new().unwrap();
        let parser = parser_from_format(Path::new(&resource), false).unwrap();
        let _ = session.load_from_slice(parser.parser, parser.input.as_slice());
        assert_ne!(session.len().unwrap(), 0, "Expected non-zero quads for: {}", resource);
    }
    #[test_resources("data/owl-ttl/*.ttl")]
    fn test_ttl_parser(resource: &str) {
        use oxigraph::store::Store;
        let session = Store::new().unwrap();
        let parser = parser_from_format(Path::new(&resource), false).unwrap();
        let _ = session.load_from_slice(parser.parser, parser.input.as_slice());
        assert_ne!(session.len().unwrap(), 0, "Expected non-zero quads for: {}", resource);
    }

    #[test_resources("data/owl-functional/*.ofn")]
    fn test_ofn_parser_roundtrip(resource: &str) {
        use oxigraph::store::Store;
        let session = Store::new().unwrap();
        let parser = parser_from_format(Path::new(&resource), false).unwrap();
        session.load_from_slice(parser.parser, parser.input.as_slice()).unwrap();
        let quads1size = session.len().unwrap();
        let quads1 = session.iter().collect::<Result<Vec<_>, _>>().unwrap();
        assert_ne!(quads1size, 0, "Expected non-zero quads for: {}", resource);
        session.clear().unwrap();

        for (ext, foldr) in [("owx", "owl-xml"), ("ttl", "owl-ttl")] {
            let repl = resource.replace("owl-functional", foldr);
            let path = Path::new(&repl).with_extension(ext);
            let parser = parser_from_format(&path, false).unwrap();
            session.load_from_slice(parser.parser, parser.input.as_slice()).unwrap();
        
            let quads2size = session.len().unwrap();
            let quads2 = session.iter().collect::<Result<Vec<_>, _>>().unwrap();
            assert_ne!(quads2size, 0, "Expected non-zero quads for: {}", path.display());
            assert_eq!(
                quads1size,
                quads2size,
                "Expected same number of quads for: {} and {} \n Got: \n{}",
                resource,
                path.display(),
                pretty_print_quads(&quads1, &quads2)
            );
            session.clear().unwrap();
        }
    }
    
    fn pretty_print_quads(
        quads1: &[oxigraph::model::Quad],
        quads2: &[oxigraph::model::Quad],
    ) -> String {
        use std::fmt::Write as FmtWrite;

        let mut left = quads1
            .iter()
            .map(|quad| quad.to_string())
            .collect::<Vec<_>>();
        let mut right = quads2
            .iter()
            .map(|quad| quad.to_string())
            .collect::<Vec<_>>();

        left.sort();
        right.sort();

        let rows = left.len().max(right.len());

        let mut out = String::new();
        for i in 0..rows {
            let l = left.get(i).map(String::as_str).unwrap_or("");
            let r = right.get(i).map(String::as_str).unwrap_or("");
            let _ = writeln!(out, "left[{}] : {}", i, l);
            let _ = writeln!(out, "right[{}]: {}", i, r);
            if i + 1 < rows {
                out.push('\n');
            }
        }

        out
    }
}
