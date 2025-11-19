#[allow(dead_code)]
use std::{collections::HashMap, fmt};
use horned_owl::{model::{ClassExpression, DeclareClass, EquivalentClasses, ForIRI}, 
    ontology::set::SetOntology};
use crate::horned_oxi::horned_oxi_visitor::{Visit, Walk};

#[derive(Debug)]
pub enum NodeType {
    Class(u32),
    ExternalClass(u32),
    Thing(u32),
    EquivalentClass(Vec<u32>),
    Union(u32),
    DisjointUnion(u32),
    Intersection(u32),
    Complement(u32),
    DeprecatedClass(u32),
    AnonymousClass(u32),
    Literal(u32),
    RdfsClass(u32),
    RdfsResource(u32),
}
pub enum EdgeTo {
    Node(u32),
    Edge(u32),
}
pub enum EdgeType {
    Datatype(EdgeTo, EdgeTo),
    ObjectProperty(EdgeTo, EdgeTo),
    DatatypeProperty(EdgeTo, EdgeTo),
    SubclassOf(EdgeTo, EdgeTo),
    InverseProperty(EdgeTo, EdgeTo),
    DisjointWith(EdgeTo, EdgeTo),
    RdfProperty(EdgeTo, EdgeTo),
    DeprecatedProperty(EdgeTo, EdgeTo),
    ExternalProperty(EdgeTo, EdgeTo),
    ValuesFrom(EdgeTo, EdgeTo),
    NoDraw,
}

pub struct HornedVOWLExtract<A> {
    //ontology: ComponentMappedOntology<A, Rc<AnnotatedComponent<A>>>,
    nodes: Vec<NodeType>,
    // [from, edge_type, to]
    edges: Vec<[u8; 3]>,
    index: HashMap<A, u32>,
}

impl<A> Default for HornedVOWLExtract<A> {
    fn default() -> Self {
        Self { nodes: vec![], edges: vec![], index: HashMap::new() }
    }
}

impl<A: ForIRI> HornedVOWLExtract<A> {   
    
    pub fn get_insert(&mut self, x: A)  -> u32 {
        if !self.index.contains_key(&x) {
            self.index.insert(x.clone(), self.index.len() as u32);
        }
        self.index[&x]
    }

}

impl<A: ForIRI> From<SetOntology<A>> for HornedVOWLExtract<A> {
    fn from(ontology: SetOntology<A>) -> Self {
        let mut walk = Walk::new(Self::default());
        walk.set_ontology(&ontology);
        walk.into_visit()
    }
}

impl<A: ForIRI, B> Visit<A, B> for HornedVOWLExtract<A> {
    fn visit_declare_class(&mut self, cmp: &DeclareClass<A>) -> Option<B> {
        let index = self.get_insert(cmp.0.underlying());
        self.nodes.push(NodeType::Class(index));
        None
    }
    fn visit_equivalent_classes(&mut self, cmp: &EquivalentClasses<A>) -> Option<B> {
        //let index = self.get_insert(cmp.0.underlying());
        for class in &cmp.0 {
            match class {
                ClassExpression::Class(name) => {
                    let index = self.get_insert(name.0.underlying());
                    self.nodes.push(NodeType::Class(index));
                }
                ClassExpression::<A>::ObjectIntersectionOf(classes) => {
                    for class in classes {
                        let index = self.get_insert(class);
                        self.nodes.push(NodeType::Class(index));
                        self.visit_class_expression(class);
                    }
                }
                _ => {}
            }
        }
        None
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
        write!(f, "HornedConvert {{ nodes: {:?}\nedges: {:?}\nindex: {:?} }}", self.nodes, self.edges, self.index)
    }
}


