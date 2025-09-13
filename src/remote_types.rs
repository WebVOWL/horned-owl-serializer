use horned_owl::{model::{AnnotatedComponent, ForIRI}, ontology::{indexed::OneIndexedOntology, set::{SetIndex, SetOntology}}};
use rkyv::{Archive, Deserialize, Serialize};


#[derive(Archive, Serialize, Deserialize)]
#[rkyv(remote = horned_owl::ontology::set::SetOntology<A>)]
#[rkyv(archived = ArchivedSetOntology)]
struct SetOntologyDef<A: ForIRI>(

    OneIndexedOntology<A, AnnotatedComponent<A>, SetIndex<A, AnnotatedComponent<A>>>,
    

);

impl <A: ForIRI> From<SetOntologyDef<A>> for horned_owl::ontology::set::SetOntology<A> {
    fn from(value: SetOntologyDef<A>) -> Self {
        horned_owl::ontology::set::SetOntology::new(value)
    }

}
