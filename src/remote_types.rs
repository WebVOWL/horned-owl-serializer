#![allow(dead_code, unused)]
use std::collections::HashSet;
use std::{borrow::Borrow, marker::PhantomData, ops::Deref, rc::Rc};
use std::fmt::Debug;
use std::hash::Hash;
use horned_owl::{model::{AnnotatedComponent, ForIRI, IRI}, ontology::{indexed::OneIndexedOntology, set::{SetIndex, SetOntology}}};
use rkyv::{Archive, Deserialize, Serialize};


#[derive(Archive, Serialize, Deserialize)]
#[rkyv(remote = SetOntology<A>)]
#[rkyv(archived = ArchivedSetOntology)]
struct SetOntologyDef<A: ForIRI>(
    OneIndexedOntology<A, AnnotatedComponent<A>, SetIndex<A, AnnotatedComponent<A>>>
);


impl <A: ForIRI> From<SetOntologyDef<A>> for horned_owl::ontology::set::SetOntology<A> {
    fn from(value: SetOntologyDef<A>) -> Self {
        horned_owl::ontology::set::SetOntology(value.0)
    }
}

struct SetIndexDef<A: ForIRI, AA: Hash + Eq>(
    HashSet<AA>, 
    PhantomData<A>
);

impl <A: ForIRI, AA: Hash + Eq> From<SetIndexDef<A, AA>> for horned_owl::ontology::set::SetIndex<A, AA> {
    fn from(value: SetIndexDef<A, AA>) -> Self {
        horned_owl::ontology::set::SetIndex(value.0, value.1)
    }
}