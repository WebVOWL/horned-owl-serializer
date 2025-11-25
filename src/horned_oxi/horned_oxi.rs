use crate::horned_oxi::horned_oxi_visitor::{ForVisit, Visit, Walk};
use horned_owl::{
    model::{ClassExpression, DeclareClass, EquivalentClasses, ForIRI, IRI, Individual},
    ontology::set::SetOntology,
};
use std::{collections::HashMap, fmt, ops::Deref};

#[derive(Clone)]
pub struct Kind<T>(pub Thing<T>);

#[derive(Debug, Clone)]
pub enum Thing<T> {
    Node(Node<T>),
    Edge(Edge<T>),
}

#[derive(Debug, Clone)]
pub enum Node<T> {
    Class(T),
    ExternalClass(T),
    Thing(T),
    EquivalentClass(Vec<T>),
    Union(T),
    DisjointUnion(T),
    Intersection(T),
    Complement(T),
    DeprecatedClass(T),
    AnonymousClass(T),
    Literal(T),
    RdfsClass(T),
    RdfsResource(T),
}

impl From<Node<u32>> for Thing<u32> {
    fn from(node: Node<u32>) -> Self {
        Thing::Node(node)
    }
}

impl From<Edge<u32>> for Thing<u32> {
    fn from(edge: Edge<u32>) -> Self {
        Thing::Edge(edge)
    }
}



#[derive(Debug, Clone)]
#[repr(C)]
pub enum Edge<T> {
    Datatype(T, T),
    ObjectProperty(T,T, T),
    DatatypeProperty(T, T),
    SubclassOf(T, T),
    InverseProperty(T, T),
    DisjointWith(T, T),
    RdfProperty(T, T),
    DeprecatedProperty(T, T),
    ExternalProperty(T, T),
    ValuesFrom(T, T),
    NoDraw,
}

impl<T: Clone> ForVisit<Thing<T>> for Kind<T> {
    fn inner(&self) -> Thing<T> {
        self.0.clone()
    }
}
#[derive(Debug)]
pub struct HornedVOWLExtract<A> {
    //ontology: ComponentMappedOntology<A, Rc<AnnotatedComponent<A>>>,
    nodes: Vec<Node<u32>>,
    // [from, edge_type, to]
    edges: Vec<Edge<u32>>,
    iricache: HashMap<A, (u32, Option<u32>)>,
    domain: HashMap<A, Vec<A>>,
    range: HashMap<A, Vec<A>>,
}

impl<A> Default for HornedVOWLExtract<A> {
    fn default() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
            iricache: HashMap::new(),
            domain: HashMap::new(),
            range: HashMap::new(),
        }
    }
}

impl<A: ForIRI> HornedVOWLExtract<A> {
    pub fn insert(&mut self, x: A) -> (bool, u32) {
        let present = self.iricache.contains_key(&x);
        if !present {
            self.iricache
                .insert(x.clone(), (self.iricache.len() as u32, None));
        }
        (present, self.iricache[&x].0)
    }
}

impl<A: ForIRI> From<SetOntology<A>> for HornedVOWLExtract<A> {
    fn from(ontology: SetOntology<A>) -> Self {
        let mut walk = Walk::<A, u32, Self>::new(Self::default());
        walk.set_ontology(None, &ontology);
        walk.into_visit()
    }
}

impl<A: ForIRI> Visit<A, u32> for HornedVOWLExtract<A> {
    fn visit_declare_class(
        &mut self,
        _: Option<Kind<u32>>,
        cmp: &DeclareClass<A>,
    ) -> Option<Kind<u32>> {
        let index = self.insert(cmp.0.underlying());
        if !index.0 {
            self.nodes.push(Node::Class(index.1));
        }
        Some(Kind(Thing::Node(Node::Class(index.1))))
    }
    fn visit_equivalent_classes(
        &mut self,
        _: Option<Kind<u32>>,
        cmp: &EquivalentClasses<A>,
    ) -> Option<Kind<u32>> {
        let mut equivalent_classes = vec![];
        //let index = self.get_insert(cmp.0.underlying());
        for class in &cmp.0 {
            match class {
                ClassExpression::Class(name) => {
                    let index = self.insert(name.0.underlying());
                    if !index.0 {
                        equivalent_classes.push(index.1);
                    }
                }
                _ => {}
            }
        }
        if !equivalent_classes.is_empty() {
            self.nodes.push(Node::EquivalentClass(equivalent_classes));
            self.nodes
                .last()
                .cloned()
                .map(|node| Kind(node.into()))
        } else {
            None
        }
    }
    fn visit_named_individual(
            &mut self,
            _: Option<Kind<u32>>,
            cmp: &horned_owl::model::NamedIndividual<A>,
        ) -> Option<Kind<u32>> {
            let index = self.insert(cmp.0.underlying());
            if !index.0 {
                self.nodes.push(Node::Thing(index.1));
            }
            Some(Kind(Thing::Node(Node::Thing(index.1))))
    }
    fn visit_object_property_assertion(
        &mut self,
        _: Option<Kind<u32>>,
        cmp: &horned_owl::model::ObjectPropertyAssertion<A>,
    ) -> Option<Kind<u32>> {
        let index_ope = self.insert(cmp.ope.as_property().unwrap().0.underlying());
        
        let index_from = match cmp.from.clone() {
            Individual::Named(name) => {
                self.insert(name.0.underlying())
            }
            Individual::Anonymous(anonymous) => {
                self.insert(anonymous.0)
            }
        };
        let index_to = match cmp.to.clone() {
            Individual::Named(name) => {
                self.insert(name.0.underlying())
            }
            Individual::Anonymous(anonymous) => {
                self.insert(anonymous.0)
            }
        };
        self.edges.push(Edge::ObjectProperty(index_from.1, index_ope.1, index_to.1));
        Some(Kind(Thing::Edge(Edge::ObjectProperty(index_from.1, index_ope.1, index_to.1))))
    }
    fn visit_sub_class_of(&mut self, _: Option<Kind<u32>>, cmp: &horned_owl::model::SubClassOf<A>) -> Option<Kind<u32>> {
        
        let index_sub = match &cmp.sub {
            ClassExpression::Class(name) => {
                Some(self.insert(name.0.underlying()))
            }
            _ => None
        };
        let index_sup = match &cmp.sup {
            ClassExpression::Class(name) => {
                Some(self.insert(name.0.underlying()))
            }
            _ => None
        };
        if !index_sub.is_some_and(|x| x.0) {
            self.nodes.push(Node::Class(index_sub.unwrap().1));
        }
        if !index_sup.is_some_and(|x| x.0) {
            self.nodes.push(Node::Class(index_sup.unwrap().1));
        }
        
        self.edges.push(Edge::SubclassOf(index_sub.unwrap().1, index_sup.unwrap().1));
        Some(Kind(Thing::Edge(Edge::SubclassOf(index_sub.unwrap().1, index_sup.unwrap().1))))
        
    }
}

/*
impl<A: ForIRI> From<ComponentMappedOntology<A, Rc<AnnotatedComponent<A>>>> for HornedConvert<A> {
    fn from(ontology: ComponentMappedOntology<A, Rc<AnnotatedComponent<A>>>) -> Self {
        let mut i: u32 = 0;
        let mut index = HashMap::<A, u32>::new();
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        nodes.append(ontology
            .declare_class()
            .into_iter()
            .map(|cmp| {
                let ul = cmp.0.underlying();
                if !index.contains_key(&ul) {
                    index.insert(ul.clone(), i.clone());
                    i += 1;
                }
                NodeType::Class(index[&ul.clone()])
            })
            .collect::<Vec<NodeType>>().as_mut()
        );

        edges.append(ontology
            .object_property_assertion()
            .into_iter()
            .map(|cmp| {
                let ul = cmp.ope.as_property().unwrap().0.underlying();
                EdgeType::ObjectProperty(index[&cmp.to.clone()], index[&cmp.from.clone()])
            })
            .collect::<Vec<EdgeType>>().as_mut());

        nodes.append(ontology
            .equivalent_class()
            .into_iter()
            .map(|cmp| {
                let ul = cmp.0;
                for class in ul {
                    match class {
                        ClassExpression::Class(name) => {
                            if !index.contains_key(&name) {
                                index.insert(name.0.underlying().clone(), i.clone());
                                i += 1;
                            }
                        }
                    }
                    if !index.contains_key(&class) {
                        index.insert(class.clone(), i.clone());
                        i += 1;
                    }
                }
                NodeType::EquivalentClass(ul.iter().map(|class| index[&class.clone()]).collect())
            }).collect::<Vec<NodeType>>().as_mut());
        //let nodes = ontology.index().iter().map(|cmp| cmp.kind()).collect();
        Self {
            nodes,
            edges: vec![],
            index,
            phantom: PhantomData
        }
    }
}
*/

impl<A: ForIRI> fmt::Display for HornedVOWLExtract<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HornedConvert {{ nodes: {:#?}\nedges: {:#?}\nindex: {:#?} }}",
            self.nodes, self.edges, self.iricache
        )
    }
}
