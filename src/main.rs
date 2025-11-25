use std::path::Path;

mod store;
mod horned_oxi;

use horned_owl::model::RcStr;
use oxigraph::store::Store;

use crate::store::HornedOxiStore;

pub fn main() {
    //let session = Store::open("oxigraph.db").unwrap();
    let session = Store::new().unwrap();
    println!("Loaded {} quads", session.len().unwrap());
    let path = Path::new("data/owl-rdf/owl1-compatible.owl");
    let horned_oxi = HornedOxiStore::<RcStr>::new(session);
    horned_oxi
        .insert_file(&path, false)
        .expect("Error inserting file");
    println!("Loaded {} quads", horned_oxi.session.len().unwrap());
    let ontology = horned_oxi
        .get_ontology(format!("file:://{}", path.display().to_string()))
        .unwrap();
    let horned_oxi_extract = horned_oxi.convert_ontology(ontology).unwrap();
    println!("{:#?}", horned_oxi_extract);
    /*
    let results = horned_oxi.session.quads_for_pattern(
        None,
        None,
        None,
        None).collect::<Result<Vec<_>, _>>().unwrap();
    for quad in results {
        let triple: Triple = quad.into();
        println!("{}", triple.to_string());
    }
    */
}