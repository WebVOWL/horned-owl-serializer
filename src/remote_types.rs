#![allow(dead_code, unused)]
use std::collections::HashSet;
use std::{borrow::Borrow, marker::PhantomData, ops::Deref, rc::Rc};
use std::fmt::Debug;
use std::hash::Hash;
use horned_owl::ontology::indexed::OntologyIndex;
use horned_owl::{model::{AnnotatedComponent, ForIRI, IRI}, ontology::{indexed::OneIndexedOntology, set::{SetIndex, SetOntology}}};
use rkyv::{Archive, Deserialize, Serialize};

// Remote derive wrapper for SetOntology<A>
#[derive(Archive, Serialize, Deserialize)]
#[rkyv(remote = SetOntology<A>)]
struct SetOntologyDef<A: ForIRI>(
    #[rkyv(with=OneIndexedOntologyDef<A, AnnotatedComponent<A>, SetIndexDef<A, AnnotatedComponent<A>>>)]
    OneIndexedOntology<A, AnnotatedComponent<A>, SetIndex<A, AnnotatedComponent<A>>>
);

// Convert from remote type to actual type
impl<A: ForIRI> From<SetOntologyDef<A>> for SetOntology<A> {
    fn from(value: SetOntologyDef<A>) -> Self {
        SetOntology(value.0)
    }
}

// Remote derive wrapper for OneIndexedOntology<A, AA, I>
#[derive(Archive, Serialize, Deserialize)]
#[rkyv(remote = OneIndexedOntology<A, AA, I>)]
struct OneIndexedOntologyDef<A: ForIRI, AA: AnnotatedComponent<A>, I: SetIndex<A, AA>>(
    #[rkyv(with=SetIndexDef<A, AA>)] I,
    Option<IRI<A>>, 
    PhantomData<AA>,
);

impl<A: ForIRI, AA: Hash + Eq, I: OntologyIndex<A, AA>> From<OneIndexedOntology<A, AA, I>> for OneIndexedOntologyDef<A, AA, I> {
    fn from(value: OneIndexedOntology<A, AA, I>) -> Self {
        OneIndexedOntologyDef(value.0, value.1, value.2)
    }
}

// Remote derive wrapper for SetIndex<A, AA>
#[derive(Archive, Serialize, Deserialize)]
#[rkyv(remote = horned_owl::ontology::set::SetIndex<A, AA>)]
struct SetIndexDef<A: ForIRI, AA: Hash + Eq>(
    HashSet<AA>,
    PhantomData<A>,
);

impl<A: ForIRI, AA: Hash + Eq> From<SetIndexDef<A, AA>> for SetIndex<A, AA> {
    fn from(value: SetIndexDef<A, AA>) -> Self {
        SetIndex(value.0, value.1)
    }
}

impl<A: ForIRI, AA: Hash + Eq> From<SetIndex<A, AA>> for SetIndexDef<A, AA> {
    fn from(value: SetIndex<A, AA>) -> Self {
        SetIndexDef(value.0, value.1)
    }
}

// Convert from remote type to actual type
impl<A: ForIRI> From<SetOntologyDef<A>> for SetOntology<A> {
    fn from(value: SetOntologyDef<A>) -> Self {
        SetOntology(value.0)
    }
}
