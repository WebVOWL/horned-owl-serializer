/// An immutable `Visit`/`Walk` for Horned-OWL Ontologies.
use horned_owl::model::*;
use horned_owl::ontology::set::SetOntology;
use horned_owl::vocab::Facet;
use std::collections::BTreeSet;
use std::marker::PhantomData;

use crate::horned_oxi::horned_oxi::Kind;

pub trait ForVisit<T> {
    fn inner(&self) -> T;
}

pub trait Visit<A: ForIRI, B> {
    fn visit_string(&mut self, _: Option<Kind<B>>, _: &String) -> Option<Kind<B>> {
        None
    }
    fn visit_u32(&mut self, _: Option<Kind<B>>, _: &u32) -> Option<Kind<B>> {
        None
    }
    fn visit_iri(&mut self, _: Option<Kind<B>>, _: &IRI<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_anonymous_individual(
        &mut self,
        _: Option<Kind<B>>,
        _: &AnonymousIndividual<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_individual(&mut self, _: Option<Kind<B>>, _: &Individual<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_annotation_subject(
        &mut self,
        _: Option<Kind<B>>,
        _: &AnnotationSubject<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_dociri(&mut self, _: Option<Kind<B>>, _: &DocIRI<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_class(&mut self, _: Option<Kind<B>>, _: &Class<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_datatype(&mut self, _: Option<Kind<B>>, _: &Datatype<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_object_property(
        &mut self,
        _: Option<Kind<B>>,
        _: &ObjectProperty<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_data_property(&mut self, _: Option<Kind<B>>, _: &DataProperty<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_annotation_property(
        &mut self,
        _: Option<Kind<B>>,
        _: &AnnotationProperty<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_named_individual(
        &mut self,
        _: Option<Kind<B>>,
        _: &NamedIndividual<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_annotated_component(
        &mut self,
        _: Option<Kind<B>>,
        _: &AnnotatedComponent<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_component(&mut self, _: Option<Kind<B>>, _: &Component<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_import(&mut self, _: Option<Kind<B>>, _: &Import<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_ontology_annotation(
        &mut self,
        _: Option<Kind<B>>,
        _: &OntologyAnnotation<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_declare_class(&mut self, _: Option<Kind<B>>, _: &DeclareClass<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_declare_object_property(
        &mut self,
        _: Option<Kind<B>>,
        _: &DeclareObjectProperty<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_declare_annotation_property(
        &mut self,
        _: Option<Kind<B>>,
        _: &DeclareAnnotationProperty<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_declare_data_property(
        &mut self,
        _: Option<Kind<B>>,
        _: &DeclareDataProperty<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_declare_named_individual(
        &mut self,
        _: Option<Kind<B>>,
        _: &DeclareNamedIndividual<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_declare_datatype(
        &mut self,
        _: Option<Kind<B>>,
        _: &DeclareDatatype<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_sub_class_of(&mut self, _: Option<Kind<B>>, _: &SubClassOf<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_equivalent_classes(
        &mut self,
        _: Option<Kind<B>>,
        _: &EquivalentClasses<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_disjoint_classes(
        &mut self,
        _: Option<Kind<B>>,
        _: &DisjointClasses<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_disjoint_union(
        &mut self,
        _: Option<Kind<B>>,
        _: &DisjointUnion<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_sub_object_property_of(
        &mut self,
        _: Option<Kind<B>>,
        _: &SubObjectPropertyOf<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_equivalent_object_properties(
        &mut self,
        _: Option<Kind<B>>,
        _: &EquivalentObjectProperties<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_disjoint_object_properties(
        &mut self,
        _: Option<Kind<B>>,
        _: &DisjointObjectProperties<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_inverse_object_properties(
        &mut self,
        _: Option<Kind<B>>,
        _: &InverseObjectProperties<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_object_property_domain(
        &mut self,
        _: Option<Kind<B>>,
        _: &ObjectPropertyDomain<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_object_property_range(
        &mut self,
        _: Option<Kind<B>>,
        _: &ObjectPropertyRange<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_functional_object_property(
        &mut self,
        _: Option<Kind<B>>,
        _: &FunctionalObjectProperty<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_inverse_functional_object_property(
        &mut self,
        _: Option<Kind<B>>,
        _: &InverseFunctionalObjectProperty<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_reflexive_object_property(
        &mut self,
        _: Option<Kind<B>>,
        _: &ReflexiveObjectProperty<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_irreflexive_object_property(
        &mut self,
        _: Option<Kind<B>>,
        _: &IrreflexiveObjectProperty<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_symmetric_object_property(
        &mut self,
        _: Option<Kind<B>>,
        _: &SymmetricObjectProperty<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_asymmetric_object_property(
        &mut self,
        _: Option<Kind<B>>,
        _: &AsymmetricObjectProperty<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_transitive_object_property(
        &mut self,
        _: Option<Kind<B>>,
        _: &TransitiveObjectProperty<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_sub_data_property_of(
        &mut self,
        _: Option<Kind<B>>,
        _: &SubDataPropertyOf<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_equivalent_data_properties(
        &mut self,
        _: Option<Kind<B>>,
        _: &EquivalentDataProperties<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_disjoint_data_properties(
        &mut self,
        _: Option<Kind<B>>,
        _: &DisjointDataProperties<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_data_property_domain(
        &mut self,
        _: Option<Kind<B>>,
        _: &DataPropertyDomain<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_data_property_range(
        &mut self,
        _: Option<Kind<B>>,
        _: &DataPropertyRange<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_functional_data_property(
        &mut self,
        _: Option<Kind<B>>,
        _: &FunctionalDataProperty<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_datatype_definition(
        &mut self,
        _: Option<Kind<B>>,
        _: &DatatypeDefinition<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_has_key(&mut self, _: Option<Kind<B>>, _: &HasKey<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_same_individual(
        &mut self,
        _: Option<Kind<B>>,
        _: &SameIndividual<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_different_individuals(
        &mut self,
        _: Option<Kind<B>>,
        _: &DifferentIndividuals<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_class_assertion(
        &mut self,
        _: Option<Kind<B>>,
        _: &ClassAssertion<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_object_property_assertion(
        &mut self,
        _: Option<Kind<B>>,
        _: &ObjectPropertyAssertion<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_negative_object_property_assertion(
        &mut self,
        _: Option<Kind<B>>,
        _: &NegativeObjectPropertyAssertion<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_data_property_assertion(
        &mut self,
        _: Option<Kind<B>>,
        _: &DataPropertyAssertion<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_negative_data_property_assertion(
        &mut self,
        _: Option<Kind<B>>,
        _: &NegativeDataPropertyAssertion<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_annotation_assertion(
        &mut self,
        _: Option<Kind<B>>,
        _: &AnnotationAssertion<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_sub_annotation_property_of(
        &mut self,
        _: Option<Kind<B>>,
        _: &SubAnnotationPropertyOf<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_annotation_property_domain(
        &mut self,
        _: Option<Kind<B>>,
        _: &AnnotationPropertyDomain<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_annotation_property_range(
        &mut self,
        _: Option<Kind<B>>,
        _: &AnnotationPropertyRange<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_rule(&mut self, _: Option<Kind<B>>, _: &Rule<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_atom(&mut self, _: Option<Kind<B>>, _: &Atom<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_variable(&mut self, _: Option<Kind<B>>, _: &Variable<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_iarg(&mut self, _: Option<Kind<B>>, _: &IArgument<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_darg(&mut self, _: Option<Kind<B>>, _: &DArgument<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_literal(&mut self, _: Option<Kind<B>>, _: &Literal<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_annotation(&mut self, _: Option<Kind<B>>, _: &Annotation<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_annotation_value(
        &mut self,
        _: Option<Kind<B>>,
        _: &AnnotationValue<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_object_property_expression(
        &mut self,
        _: Option<Kind<B>>,
        _: &ObjectPropertyExpression<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_sub_object_property_expression(
        &mut self,
        _: Option<Kind<B>>,
        _: &SubObjectPropertyExpression<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_property_expression(
        &mut self,
        _: Option<Kind<B>>,
        _: &PropertyExpression<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_facet_restriction(
        &mut self,
        _: Option<Kind<B>>,
        _: &FacetRestriction<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_facet(&mut self, _: Option<Kind<B>>, _: &Facet) -> Option<Kind<B>> {
        None
    }
    fn visit_data_range(&mut self, _: Option<Kind<B>>, _: &DataRange<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_class_expression(
        &mut self,
        _: Option<Kind<B>>,
        _: &ClassExpression<A>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_ontology_id(&mut self, _: Option<Kind<B>>, _: &OntologyID<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_set_ontology(&mut self, _: Option<Kind<B>>, _: &SetOntology<A>) -> Option<Kind<B>> {
        None
    }
    fn visit_option_iri(&mut self, _: Option<Kind<B>>, _: &Option<IRI<A>>) -> Option<Kind<B>> {
        None
    }
    fn visit_annotation_set(
        &mut self,
        _: Option<Kind<B>>,
        _: &BTreeSet<Annotation<A>>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_class_expression_vec(
        &mut self,
        _: Option<Kind<B>>,
        _: &Vec<ClassExpression<A>>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_object_property_expression_vec(
        &mut self,
        _: Option<Kind<B>>,
        _: &Vec<ObjectPropertyExpression<A>>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_data_property_vec(
        &mut self,
        _: Option<Kind<B>>,
        _: &Vec<DataProperty<A>>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_data_range_vec(
        &mut self,
        _: Option<Kind<B>>,
        _: &Vec<DataRange<A>>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_individual_vec(
        &mut self,
        _: Option<Kind<B>>,
        _: &Vec<Individual<A>>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_literal_vec(&mut self, _: Option<Kind<B>>, _: &Vec<Literal<A>>) -> Option<Kind<B>> {
        None
    }
    fn visit_facet_restriction_vec(
        &mut self,
        _: Option<Kind<B>>,
        _: &Vec<FacetRestriction<A>>,
    ) -> Option<Kind<B>> {
        None
    }
    fn visit_atom_vec(&mut self, _: Option<Kind<B>>, _: &Vec<Atom<A>>) -> Option<Kind<B>> {
        None
    }
    fn visit_darg_vec(&mut self, _: Option<Kind<B>>, _: &Vec<DArgument<A>>) -> Option<Kind<B>> {
        None
    }
}

pub struct Walk<A, B, V>(V, PhantomData<(A, B)>);

impl<A: ForIRI, B: Clone, V: Visit<A, B>> Walk<A, B, V> {
    pub fn new(v: V) -> Self {
        Walk(v, PhantomData)
    }

    pub fn as_mut_visit(&mut self) -> &mut V {
        &mut self.0
    }

    pub fn into_visit(self) -> V {
        self.0
    }

    pub fn iri(&mut self, parent: Option<Kind<B>>, e: &IRI<A>) {
        self.0.visit_iri(parent, e);
    }

    pub fn anonymous_individual(&mut self, parent: Option<Kind<B>>, e: &AnonymousIndividual<A>) {
        self.0.visit_anonymous_individual(parent, e);
    }

    pub fn individual(&mut self, parent: Option<Kind<B>>, e: &Individual<A>) {
        let new_parent = self.0.visit_individual(parent, e);
        match e {
            Individual::Anonymous(e) => self.anonymous_individual(new_parent, e),
            Individual::Named(e) => self.named_individual(new_parent, e),
        }
    }

    pub fn annotation_subject(&mut self, parent: Option<Kind<B>>, e: &AnnotationSubject<A>) {
        let new_parent = self.0.visit_annotation_subject(parent, e);
        match e {
            AnnotationSubject::IRI(iri) => self.iri(new_parent.clone(), iri),
            AnnotationSubject::AnonymousIndividual(anon) => {
                self.anonymous_individual(new_parent.clone(), anon)
            }
        };
    }

    pub fn dociri(&mut self, parent: Option<Kind<B>>, e: &DocIRI<A>) {
        let new_parent = self.0.visit_dociri(parent, e);
        self.iri(new_parent, &e.0);
    }

    pub fn class(&mut self, parent: Option<Kind<B>>, e: &Class<A>) {
        let new_parent = self.0.visit_class(parent, e);
        self.iri(new_parent, &e.0);
    }

    pub fn datatype(&mut self, parent: Option<Kind<B>>, e: &Datatype<A>) {
        let new_parent = self.0.visit_datatype(parent, e);
        self.iri(new_parent, &e.0);
    }

    pub fn object_property(&mut self, parent: Option<Kind<B>>, e: &ObjectProperty<A>) {
        let new_parent = self.0.visit_object_property(parent, e);
        self.iri(new_parent, &e.0);
    }

    pub fn data_property(&mut self, parent: Option<Kind<B>>, e: &DataProperty<A>) {
        let new_parent = self.0.visit_data_property(parent, e);
        self.iri(new_parent, &e.0);
    }

    pub fn annotation_property(&mut self, parent: Option<Kind<B>>, e: &AnnotationProperty<A>) {
        let new_parent = self.0.visit_annotation_property(parent, e);
        self.iri(new_parent, &e.0);
    }

    pub fn named_individual(&mut self, parent: Option<Kind<B>>, e: &NamedIndividual<A>) {
        let new_parent = self.0.visit_named_individual(parent, e);
        self.iri(new_parent, &e.0);
    }

    pub fn annotated_component(&mut self, parent: Option<Kind<B>>, e: &AnnotatedComponent<A>) {
        let new_parent = self.0.visit_annotated_component(parent, e);
        self.component(new_parent.clone(), &e.component);
        self.annotation_set(new_parent, &e.ann);
    }

    pub fn component(&mut self, parent: Option<Kind<B>>, e: &Component<A>) {
        let new_parent = self.0.visit_component(parent, e);
        match e {
            Component::OntologyID(ax) => self.ontology_id(new_parent.clone(), ax),
            Component::DocIRI(ax) => self.dociri(new_parent.clone(), ax),
            Component::Import(ax) => self.import(new_parent.clone(), ax),
            Component::OntologyAnnotation(ax) => self.ontology_annotation(new_parent.clone(), ax),
            Component::DeclareClass(ax) => self.declare_class(new_parent.clone(), ax),
            Component::DeclareObjectProperty(ax) => {
                self.declare_object_property(new_parent.clone(), ax)
            }
            Component::DeclareAnnotationProperty(ax) => {
                self.declare_annotation_property(new_parent.clone(), ax)
            }
            Component::DeclareDataProperty(ax) => {
                self.declare_data_property(new_parent.clone(), ax)
            }
            Component::DeclareNamedIndividual(ax) => {
                self.declare_named_individual(new_parent.clone(), ax)
            }
            Component::DeclareDatatype(ax) => self.declare_datatype(new_parent.clone(), ax),
            Component::SubClassOf(ax) => self.sub_class_of(new_parent.clone(), ax),
            Component::EquivalentClasses(ax) => self.equivalent_classes(new_parent.clone(), ax),
            Component::DisjointClasses(ax) => self.disjoint_classes(new_parent.clone(), ax),
            Component::DisjointUnion(ax) => self.disjoint_union(new_parent.clone(), ax),
            Component::SubObjectPropertyOf(ax) => {
                self.sub_object_property_of(new_parent.clone(), ax)
            }
            Component::EquivalentObjectProperties(ax) => {
                self.equivalent_object_properties(new_parent.clone(), ax)
            }
            Component::DisjointObjectProperties(ax) => {
                self.disjoint_object_properties(new_parent.clone(), ax)
            }
            Component::InverseObjectProperties(ax) => {
                self.inverse_object_properties(new_parent.clone(), ax)
            }
            Component::ObjectPropertyDomain(ax) => {
                self.object_property_domain(new_parent.clone(), ax)
            }
            Component::ObjectPropertyRange(ax) => {
                self.object_property_range(new_parent.clone(), ax)
            }
            Component::FunctionalObjectProperty(ax) => {
                self.functional_object_property(new_parent.clone(), ax)
            }
            Component::InverseFunctionalObjectProperty(ax) => {
                self.inverse_functional_object_property(new_parent.clone(), ax)
            }
            Component::ReflexiveObjectProperty(ax) => {
                self.reflexive_object_property(new_parent.clone(), ax)
            }
            Component::IrreflexiveObjectProperty(ax) => {
                self.irreflexive_object_property(new_parent.clone(), ax)
            }
            Component::SymmetricObjectProperty(ax) => {
                self.symmetric_object_property(new_parent.clone(), ax)
            }
            Component::AsymmetricObjectProperty(ax) => {
                self.asymmetric_object_property(new_parent.clone(), ax)
            }
            Component::TransitiveObjectProperty(ax) => {
                self.transitive_object_property(new_parent.clone(), ax)
            }
            Component::SubDataPropertyOf(ax) => self.sub_data_property_of(new_parent.clone(), ax),
            Component::EquivalentDataProperties(ax) => {
                self.equivalent_data_properties(new_parent.clone(), ax)
            }
            Component::DisjointDataProperties(ax) => {
                self.disjoint_data_properties(new_parent.clone(), ax)
            }
            Component::DataPropertyDomain(ax) => self.data_property_domain(new_parent.clone(), ax),
            Component::DataPropertyRange(ax) => self.data_property_range(new_parent.clone(), ax),
            Component::FunctionalDataProperty(ax) => {
                self.functional_data_property(new_parent.clone(), ax)
            }
            Component::DatatypeDefinition(ax) => self.datatype_definition(new_parent.clone(), ax),
            Component::HasKey(ax) => self.has_key(new_parent.clone(), ax),
            Component::SameIndividual(ax) => self.same_individual(new_parent.clone(), ax),
            Component::DifferentIndividuals(ax) => {
                self.different_individuals(new_parent.clone(), ax)
            }
            Component::ClassAssertion(ax) => self.class_assertion(new_parent.clone(), ax),
            Component::ObjectPropertyAssertion(ax) => {
                self.object_property_assertion(new_parent.clone(), ax)
            }
            Component::NegativeObjectPropertyAssertion(ax) => {
                self.negative_object_property_assertion(new_parent.clone(), ax)
            }
            Component::DataPropertyAssertion(ax) => {
                self.data_property_assertion(new_parent.clone(), ax)
            }
            Component::NegativeDataPropertyAssertion(ax) => {
                self.negative_data_property_assertion(new_parent.clone(), ax)
            }
            Component::AnnotationAssertion(ax) => self.annotation_assertion(new_parent.clone(), ax),
            Component::SubAnnotationPropertyOf(ax) => {
                self.sub_annotation_property_of(new_parent.clone(), ax)
            }
            Component::AnnotationPropertyDomain(ax) => {
                self.annotation_property_domain(new_parent.clone(), ax)
            }
            Component::AnnotationPropertyRange(ax) => {
                self.annotation_property_range(new_parent.clone(), ax)
            }
            Component::Rule(sr) => self.rule(new_parent.clone(), sr),
        }
    }

    pub fn import(&mut self, parent: Option<Kind<B>>, e: &Import<A>) {
        let new_parent = self.0.visit_import(parent, e);
        self.iri(new_parent, &e.0);
    }

    pub fn ontology_annotation(&mut self, parent: Option<Kind<B>>, e: &OntologyAnnotation<A>) {
        let new_parent = self.0.visit_ontology_annotation(parent, e);
        self.annotation(new_parent, &e.0);
    }

    pub fn declare_class(&mut self, parent: Option<Kind<B>>, e: &DeclareClass<A>) {
        let new_parent = self.0.visit_declare_class(parent, e);
        self.class(new_parent, &e.0);
    }

    pub fn declare_object_property(
        &mut self,
        parent: Option<Kind<B>>,
        e: &DeclareObjectProperty<A>,
    ) {
        let new_parent = self.0.visit_declare_object_property(parent, e);
        self.object_property(new_parent, &e.0);
    }

    pub fn declare_annotation_property(
        &mut self,
        parent: Option<Kind<B>>,
        e: &DeclareAnnotationProperty<A>,
    ) {
        let new_parent = self.0.visit_declare_annotation_property(parent, e);
        self.annotation_property(new_parent, &e.0);
    }

    pub fn declare_data_property(&mut self, parent: Option<Kind<B>>, e: &DeclareDataProperty<A>) {
        let new_parent = self.0.visit_declare_data_property(parent, e);
        self.data_property(new_parent, &e.0);
    }

    pub fn declare_named_individual(
        &mut self,
        parent: Option<Kind<B>>,
        e: &DeclareNamedIndividual<A>,
    ) {
        let new_parent = self.0.visit_declare_named_individual(parent, e);
        self.named_individual(new_parent, &e.0);
    }

    pub fn declare_datatype(&mut self, parent: Option<Kind<B>>, e: &DeclareDatatype<A>) {
        let new_parent = self.0.visit_declare_datatype(parent, e);
        self.datatype(new_parent, &e.0);
    }

    pub fn sub_class_of(&mut self, parent: Option<Kind<B>>, e: &SubClassOf<A>) {
        let new_parent = self.0.visit_sub_class_of(parent, e);
        self.class_expression(new_parent.clone(), &e.sup);
        self.class_expression(new_parent, &e.sub);
    }

    pub fn equivalent_classes(&mut self, parent: Option<Kind<B>>, e: &EquivalentClasses<A>) {
        let new_parent = self.0.visit_equivalent_classes(parent, e);
        self.class_expression_vec(new_parent, &e.0);
    }

    pub fn disjoint_classes(&mut self, parent: Option<Kind<B>>, e: &DisjointClasses<A>) {
        let new_parent = self.0.visit_disjoint_classes(parent, e);
        self.class_expression_vec(new_parent, &e.0);
    }

    pub fn disjoint_union(&mut self, parent: Option<Kind<B>>, e: &DisjointUnion<A>) {
        let new_parent = self.0.visit_disjoint_union(parent, e);
        self.class(new_parent.clone(), &e.0);
        self.class_expression_vec(new_parent, &e.1);
    }

    pub fn sub_object_property_of(&mut self, parent: Option<Kind<B>>, e: &SubObjectPropertyOf<A>) {
        let new_parent = self.0.visit_sub_object_property_of(parent, e);
        self.object_property_expression(new_parent.clone(), &e.sup);
        self.sub_object_property_expression(new_parent, &e.sub);
    }

    pub fn equivalent_object_properties(
        &mut self,
        parent: Option<Kind<B>>,
        e: &EquivalentObjectProperties<A>,
    ) {
        let new_parent = self.0.visit_equivalent_object_properties(parent, e);
        self.object_property_expression_vec(new_parent, &e.0);
    }

    pub fn disjoint_object_properties(
        &mut self,
        parent: Option<Kind<B>>,
        e: &DisjointObjectProperties<A>,
    ) {
        let new_parent = self.0.visit_disjoint_object_properties(parent, e);
        self.object_property_expression_vec(new_parent, &e.0);
    }

    pub fn inverse_object_properties(
        &mut self,
        parent: Option<Kind<B>>,
        e: &InverseObjectProperties<A>,
    ) {
        let new_parent = self.0.visit_inverse_object_properties(parent, e);
        self.object_property(new_parent.clone(), &e.0);
        self.object_property(new_parent, &e.1);
    }

    pub fn object_property_domain(&mut self, parent: Option<Kind<B>>, e: &ObjectPropertyDomain<A>) {
        let new_parent = self.0.visit_object_property_domain(parent, e);
        self.object_property_expression(new_parent.clone(), &e.ope);
        self.class_expression(new_parent, &e.ce);
    }

    pub fn object_property_range(&mut self, parent: Option<Kind<B>>, e: &ObjectPropertyRange<A>) {
        let new_parent = self.0.visit_object_property_range(parent, e);
        self.object_property_expression(new_parent.clone(), &e.ope);
        self.class_expression(new_parent, &e.ce);
    }

    pub fn functional_object_property(
        &mut self,
        parent: Option<Kind<B>>,
        e: &FunctionalObjectProperty<A>,
    ) {
        let new_parent = self.0.visit_functional_object_property(parent, e);
        self.object_property_expression(new_parent, &e.0);
    }

    pub fn inverse_functional_object_property(
        &mut self,
        parent: Option<Kind<B>>,
        e: &InverseFunctionalObjectProperty<A>,
    ) {
        let new_parent = self.0.visit_inverse_functional_object_property(parent, e);
        self.object_property_expression(new_parent, &e.0);
    }

    pub fn reflexive_object_property(
        &mut self,
        parent: Option<Kind<B>>,
        e: &ReflexiveObjectProperty<A>,
    ) {
        let new_parent = self.0.visit_reflexive_object_property(parent, e);
        self.object_property_expression(new_parent, &e.0);
    }

    pub fn irreflexive_object_property(
        &mut self,
        parent: Option<Kind<B>>,
        e: &IrreflexiveObjectProperty<A>,
    ) {
        let new_parent = self.0.visit_irreflexive_object_property(parent, e);
        self.object_property_expression(new_parent, &e.0);
    }

    pub fn symmetric_object_property(
        &mut self,
        parent: Option<Kind<B>>,
        e: &SymmetricObjectProperty<A>,
    ) {
        let new_parent = self.0.visit_symmetric_object_property(parent, e);
        self.object_property_expression(new_parent, &e.0);
    }

    pub fn asymmetric_object_property(
        &mut self,
        parent: Option<Kind<B>>,
        e: &AsymmetricObjectProperty<A>,
    ) {
        let new_parent = self.0.visit_asymmetric_object_property(parent, e);
        self.object_property_expression(new_parent, &e.0);
    }

    pub fn transitive_object_property(
        &mut self,
        parent: Option<Kind<B>>,
        e: &TransitiveObjectProperty<A>,
    ) {
        let new_parent = self.0.visit_transitive_object_property(parent, e);
        self.object_property_expression(new_parent, &e.0);
    }

    pub fn sub_data_property_of(&mut self, parent: Option<Kind<B>>, e: &SubDataPropertyOf<A>) {
        let new_parent = self.0.visit_sub_data_property_of(parent, e);
        self.data_property(new_parent.clone(), &e.sup);
        self.data_property(new_parent, &e.sub);
    }

    pub fn equivalent_data_properties(
        &mut self,
        parent: Option<Kind<B>>,
        e: &EquivalentDataProperties<A>,
    ) {
        let new_parent = self.0.visit_equivalent_data_properties(parent, e);
        self.data_property_vec(new_parent, &e.0);
    }

    pub fn disjoint_data_properties(
        &mut self,
        parent: Option<Kind<B>>,
        e: &DisjointDataProperties<A>,
    ) {
        let new_parent = self.0.visit_disjoint_data_properties(parent, e);
        self.data_property_vec(new_parent, &e.0);
    }

    pub fn data_property_domain(&mut self, parent: Option<Kind<B>>, e: &DataPropertyDomain<A>) {
        let new_parent = self.0.visit_data_property_domain(parent, e);
        self.data_property(new_parent.clone(), &e.dp);
        self.class_expression(new_parent, &e.ce);
    }

    pub fn data_property_range(&mut self, parent: Option<Kind<B>>, e: &DataPropertyRange<A>) {
        let new_parent = self.0.visit_data_property_range(parent, e);
        self.data_property(new_parent.clone(), &e.dp);
        self.data_range(new_parent, &e.dr);
    }

    pub fn functional_data_property(
        &mut self,
        parent: Option<Kind<B>>,
        e: &FunctionalDataProperty<A>,
    ) {
        let new_parent = self.0.visit_functional_data_property(parent, e);
        self.data_property(new_parent, &e.0);
    }

    pub fn datatype_definition(&mut self, parent: Option<Kind<B>>, e: &DatatypeDefinition<A>) {
        let new_parent = self.0.visit_datatype_definition(parent, e);
        self.datatype(new_parent.clone(), &e.kind);
        self.data_range(new_parent, &e.range);
    }

    pub fn has_key(&mut self, parent: Option<Kind<B>>, e: &HasKey<A>) {
        let new_parent = self.0.visit_has_key(parent, e);
        self.class_expression(new_parent.clone(), &e.ce);
        for i in e.vpe.iter() {
            self.property_expression(new_parent.clone(), i);
        }
    }

    pub fn same_individual(&mut self, parent: Option<Kind<B>>, e: &SameIndividual<A>) {
        let new_parent = self.0.visit_same_individual(parent, e);
        self.individual_vec(new_parent, &e.0);
    }

    pub fn different_individuals(&mut self, parent: Option<Kind<B>>, e: &DifferentIndividuals<A>) {
        let new_parent = self.0.visit_different_individuals(parent, e);
        self.individual_vec(new_parent, &e.0);
    }

    pub fn class_assertion(&mut self, parent: Option<Kind<B>>, e: &ClassAssertion<A>) {
        let new_parent = self.0.visit_class_assertion(parent, e);
        self.class_expression(new_parent.clone(), &e.ce);
        self.individual(new_parent, &e.i);
    }

    pub fn object_property_assertion(
        &mut self,
        parent: Option<Kind<B>>,
        e: &ObjectPropertyAssertion<A>,
    ) {
        let new_parent = self.0.visit_object_property_assertion(parent, e);
        self.object_property_expression(new_parent.clone(), &e.ope);
        self.individual(new_parent.clone(), &e.from);
        self.individual(new_parent, &e.to);
    }

    pub fn negative_object_property_assertion(
        &mut self,
        parent: Option<Kind<B>>,
        e: &NegativeObjectPropertyAssertion<A>,
    ) {
        let new_parent = self.0.visit_negative_object_property_assertion(parent, e);
        self.object_property_expression(new_parent.clone(), &e.ope);
        self.individual(new_parent.clone(), &e.from);
        self.individual(new_parent, &e.to);
    }

    pub fn data_property_assertion(
        &mut self,
        parent: Option<Kind<B>>,
        e: &DataPropertyAssertion<A>,
    ) {
        let new_parent = self.0.visit_data_property_assertion(parent, e);
        self.data_property(new_parent.clone(), &e.dp);
        self.individual(new_parent.clone(), &e.from);
        self.literal(new_parent, &e.to);
    }

    pub fn negative_data_property_assertion(
        &mut self,
        parent: Option<Kind<B>>,
        e: &NegativeDataPropertyAssertion<A>,
    ) {
        let new_parent = self.0.visit_negative_data_property_assertion(parent, e);
        self.data_property(new_parent.clone(), &e.dp);
        self.individual(new_parent.clone(), &e.from);
        self.literal(new_parent, &e.to);
    }

    pub fn annotation_assertion(&mut self, parent: Option<Kind<B>>, e: &AnnotationAssertion<A>) {
        let new_parent = self.0.visit_annotation_assertion(parent, e);
        self.annotation_subject(new_parent.clone(), &e.subject);
        self.annotation(new_parent, &e.ann);
    }

    pub fn sub_annotation_property_of(
        &mut self,
        parent: Option<Kind<B>>,
        e: &SubAnnotationPropertyOf<A>,
    ) {
        let new_parent = self.0.visit_sub_annotation_property_of(parent, e);
        self.annotation_property(new_parent.clone(), &e.sup);
        self.annotation_property(new_parent, &e.sub);
    }

    pub fn annotation_property_domain(
        &mut self,
        parent: Option<Kind<B>>,
        e: &AnnotationPropertyDomain<A>,
    ) {
        let new_parent = self.0.visit_annotation_property_domain(parent, e);
        self.annotation_property(new_parent.clone(), &e.ap);
        self.iri(new_parent, &e.iri);
    }

    pub fn annotation_property_range(
        &mut self,
        parent: Option<Kind<B>>,
        e: &AnnotationPropertyRange<A>,
    ) {
        let new_parent = self.0.visit_annotation_property_range(parent, e);
        self.annotation_property(new_parent.clone(), &e.ap);
        self.iri(new_parent, &e.iri);
    }

    pub fn rule(&mut self, parent: Option<Kind<B>>, r: &Rule<A>) {
        let new_parent = self.0.visit_rule(parent, r);
        self.atom_vec(new_parent.clone(), &r.head);
        self.atom_vec(new_parent, &r.body);
    }

    pub fn atom(&mut self, parent: Option<Kind<B>>, a: &Atom<A>) {
        let new_parent = self.0.visit_atom(parent, a);
        match a {
            Atom::BuiltInAtom { pred, args } => {
                let branch_parent = new_parent.clone();
                self.iri(branch_parent.clone(), pred);
                self.darg_vec(branch_parent, args);
            }
            Atom::ClassAtom { pred, arg } => {
                let branch_parent = new_parent.clone();
                self.class_expression(branch_parent.clone(), pred);
                self.iarg(branch_parent, arg);
            }
            Atom::DataPropertyAtom { pred, args } => {
                let branch_parent = new_parent.clone();
                self.data_property(branch_parent.clone(), pred);
                self.darg(branch_parent.clone(), &args.0);
                self.darg(branch_parent, &args.1);
            }
            Atom::DataRangeAtom { pred, arg } => {
                let branch_parent = new_parent.clone();
                self.data_range(branch_parent.clone(), pred);
                self.darg(branch_parent, arg);
            }
            Atom::DifferentIndividualsAtom(arg1, arg2) => {
                let branch_parent = new_parent.clone();
                self.iarg(branch_parent.clone(), arg1);
                self.iarg(branch_parent, arg2);
            }
            Atom::ObjectPropertyAtom { pred, args } => {
                let branch_parent = new_parent.clone();
                self.object_property_expression(branch_parent.clone(), pred);
                self.iarg(branch_parent.clone(), &args.0);
                self.iarg(branch_parent, &args.1);
            }
            Atom::SameIndividualAtom(arg1, arg2) => {
                let branch_parent = new_parent.clone();
                self.iarg(branch_parent.clone(), arg1);
                self.iarg(branch_parent, arg2);
            }
        }
    }

    pub fn variable(&mut self, parent: Option<Kind<B>>, v: &Variable<A>) {
        self.0.visit_variable(parent, v);
    }

    pub fn darg(&mut self, parent: Option<Kind<B>>, d: &DArgument<A>) {
        let new_parent = self.0.visit_darg(parent, d);
        match d {
            DArgument::Literal(l) => self.literal(new_parent.clone(), l),
            DArgument::Variable(v) => self.variable(new_parent.clone(), v),
        }
    }

    pub fn iarg(&mut self, parent: Option<Kind<B>>, i: &IArgument<A>) {
        let new_parent = self.0.visit_iarg(parent, i);
        match i {
            IArgument::Individual(ind) => self.individual(new_parent.clone(), ind),
            IArgument::Variable(v) => self.variable(new_parent.clone(), v),
        }
    }

    pub fn literal(&mut self, parent: Option<Kind<B>>, e: &Literal<A>) {
        let new_parent = self.0.visit_literal(parent, e);
        match e {
            Literal::Simple { literal } => {
                self.0.visit_string(new_parent.clone(), literal);
            }
            Literal::Language { literal, lang } => {
                self.0.visit_string(new_parent.clone(), literal);
                self.0.visit_string(new_parent.clone(), lang);
            }
            Literal::Datatype {
                literal: _,
                datatype_iri,
            } => self.iri(new_parent.clone(), datatype_iri),
        }
    }

    pub fn annotation(&mut self, parent: Option<Kind<B>>, e: &Annotation<A>) {
        let new_parent = self.0.visit_annotation(parent, e);
        self.annotation_property(new_parent.clone(), &e.ap);
        self.annotation_value(new_parent, &e.av);
    }

    pub fn annotation_value(&mut self, parent: Option<Kind<B>>, e: &AnnotationValue<A>) {
        let new_parent = self.0.visit_annotation_value(parent, e);
        match e {
            AnnotationValue::Literal(lit) => self.literal(new_parent.clone(), lit),
            AnnotationValue::IRI(iri) => self.iri(new_parent.clone(), iri),
            AnnotationValue::AnonymousIndividual(anon) => {
                self.anonymous_individual(new_parent.clone(), anon)
            }
        }
    }

    pub fn object_property_expression(
        &mut self,
        parent: Option<Kind<B>>,
        e: &ObjectPropertyExpression<A>,
    ) {
        let new_parent = self.0.visit_object_property_expression(parent, e);
        match e {
            ObjectPropertyExpression::ObjectProperty(prop) => {
                self.object_property(new_parent.clone(), prop)
            }
            ObjectPropertyExpression::InverseObjectProperty(prop) => {
                self.object_property(new_parent.clone(), prop)
            }
        }
    }

    pub fn sub_object_property_expression(
        &mut self,
        parent: Option<Kind<B>>,
        e: &SubObjectPropertyExpression<A>,
    ) {
        let new_parent = self.0.visit_sub_object_property_expression(parent, e);
        match e {
            SubObjectPropertyExpression::ObjectPropertyChain(e) => {
                self.object_property_expression_vec(new_parent.clone(), e)
            }
            SubObjectPropertyExpression::ObjectPropertyExpression(e) => {
                self.object_property_expression(new_parent.clone(), e)
            }
        }
    }

    pub fn property_expression(&mut self, parent: Option<Kind<B>>, e: &PropertyExpression<A>) {
        let new_parent = self.0.visit_property_expression(parent, e);
        match e {
            PropertyExpression::ObjectPropertyExpression(ope) => {
                self.object_property_expression(new_parent.clone(), ope)
            }
            PropertyExpression::DataProperty(dp) => self.data_property(new_parent.clone(), dp),
            PropertyExpression::AnnotationProperty(ap) => {
                self.annotation_property(new_parent.clone(), ap)
            }
        }
    }

    pub fn facet_restriction(&mut self, parent: Option<Kind<B>>, e: &FacetRestriction<A>) {
        let new_parent = self.0.visit_facet_restriction(parent, e);
        self.facet(new_parent.clone(), &e.f);
        self.literal(new_parent.clone(), &e.l);
    }

    pub fn facet(&mut self, parent: Option<Kind<B>>, e: &Facet) {
        self.0.visit_facet(parent, e);
    }

    pub fn data_range(&mut self, parent: Option<Kind<B>>, e: &DataRange<A>) {
        let new_parent = self.0.visit_data_range(parent, e);
        match e {
            DataRange::Datatype(dt) => self.datatype(new_parent.clone(), dt),
            DataRange::DataIntersectionOf(v) => self.data_range_vec(new_parent.clone(), v),
            DataRange::DataUnionOf(v) => self.data_range_vec(new_parent.clone(), v),
            DataRange::DataComplementOf(inner) => self.data_range(new_parent.clone(), inner),
            DataRange::DataOneOf(v) => self.literal_vec(new_parent.clone(), v),
            DataRange::DatatypeRestriction(dt, v) => {
                self.datatype(new_parent.clone(), dt);
                self.facet_restriction_vec(new_parent.clone(), v);
            }
        }
    }

    pub fn class_expression(&mut self, parent: Option<Kind<B>>, e: &ClassExpression<A>) {
        let new_parent = self.0.visit_class_expression(parent, e);
        match e {
            ClassExpression::Class(cls) => self.class(new_parent.clone(), cls),
            ClassExpression::ObjectIntersectionOf(v) => {
                self.class_expression_vec(new_parent.clone(), v)
            }
            ClassExpression::ObjectUnionOf(v) => self.class_expression_vec(new_parent.clone(), v),
            ClassExpression::ObjectComplementOf(inner) => {
                self.class_expression(new_parent.clone(), inner)
            }
            ClassExpression::ObjectOneOf(v) => self.individual_vec(new_parent.clone(), v),
            ClassExpression::ObjectSomeValuesFrom { ope, bce } => {
                self.object_property_expression(new_parent.clone(), ope);
                self.class_expression(new_parent.clone(), bce);
            }
            ClassExpression::ObjectAllValuesFrom { ope, bce } => {
                self.object_property_expression(new_parent.clone(), ope);
                self.class_expression(new_parent.clone(), bce);
            }
            ClassExpression::ObjectHasValue { ope, i } => {
                self.object_property_expression(new_parent.clone(), ope);
                self.individual(new_parent.clone(), i);
            }
            ClassExpression::ObjectHasSelf(ope) => {
                self.object_property_expression(new_parent.clone(), ope)
            }
            ClassExpression::ObjectMinCardinality { n, ope, bce } => {
                self.0.visit_u32(new_parent.clone(), n);
                self.object_property_expression(new_parent.clone(), ope);
                self.class_expression(new_parent.clone(), bce);
            }
            ClassExpression::ObjectMaxCardinality { n, ope, bce } => {
                self.0.visit_u32(new_parent.clone(), n);
                self.object_property_expression(new_parent.clone(), ope);
                self.class_expression(new_parent.clone(), bce);
            }
            ClassExpression::ObjectExactCardinality { n, ope, bce } => {
                self.0.visit_u32(new_parent.clone(), n);
                self.object_property_expression(new_parent.clone(), ope);
                self.class_expression(new_parent.clone(), bce);
            }
            ClassExpression::DataSomeValuesFrom { dp, dr } => {
                self.data_property(new_parent.clone(), dp);
                self.data_range(new_parent.clone(), dr);
            }
            ClassExpression::DataAllValuesFrom { dp, dr } => {
                self.data_property(new_parent.clone(), dp);
                self.data_range(new_parent.clone(), dr);
            }
            ClassExpression::DataHasValue { dp, l } => {
                self.data_property(new_parent.clone(), dp);
                self.literal(new_parent.clone(), l);
            }
            ClassExpression::DataMinCardinality { n, dp, dr } => {
                self.0.visit_u32(new_parent.clone(), n);
                self.data_property(new_parent.clone(), dp);
                self.data_range(new_parent.clone(), dr);
            }
            ClassExpression::DataMaxCardinality { n, dp, dr } => {
                self.0.visit_u32(new_parent.clone(), n);
                self.data_property(new_parent.clone(), dp);
                self.data_range(new_parent.clone(), dr);
            }
            ClassExpression::DataExactCardinality { n, dp, dr } => {
                self.0.visit_u32(new_parent.clone(), n);
                self.data_property(new_parent.clone(), dp);
                self.data_range(new_parent.clone(), dr);
            }
        }
    }

    pub fn ontology_id(&mut self, parent: Option<Kind<B>>, e: &OntologyID<A>) {
        let new_parent = self.0.visit_ontology_id(parent, e);
        self.option_iri(new_parent.clone(), &e.iri);
        self.option_iri(new_parent, &e.viri);
    }

    pub fn set_ontology(&mut self, parent: Option<Kind<B>>, e: &SetOntology<A>) {
        let new_parent = self.0.visit_set_ontology(parent, e);
        for i in e.iter() {
            self.annotated_component(new_parent.clone(), i);
        }
    }

    pub fn option_iri(&mut self, parent: Option<Kind<B>>, e: &Option<IRI<A>>) {
        let new_parent = self.0.visit_option_iri(parent, e);
        if let Some(e) = e {
            self.iri(new_parent, e)
        }
    }

    // Collections
    pub fn annotation_set(&mut self, parent: Option<Kind<B>>, e: &BTreeSet<Annotation<A>>) {
        let new_parent = self.0.visit_annotation_set(parent, e);
        for i in e.iter() {
            self.annotation(new_parent.clone(), i);
        }
    }

    pub fn class_expression_vec(&mut self, parent: Option<Kind<B>>, e: &Vec<ClassExpression<A>>) {
        let new_parent = self.0.visit_class_expression_vec(parent, e);
        for i in e.iter() {
            self.class_expression(new_parent.clone(), i);
        }
    }

    pub fn object_property_expression_vec(
        &mut self,
        parent: Option<Kind<B>>,
        e: &Vec<ObjectPropertyExpression<A>>,
    ) {
        let new_parent = self.0.visit_object_property_expression_vec(parent, e);
        for i in e.iter() {
            self.object_property_expression(new_parent.clone(), i);
        }
    }

    pub fn data_property_vec(&mut self, parent: Option<Kind<B>>, e: &Vec<DataProperty<A>>) {
        let new_parent = self.0.visit_data_property_vec(parent, e);
        for i in e.iter() {
            self.data_property(new_parent.clone(), i);
        }
    }

    pub fn individual_vec(&mut self, parent: Option<Kind<B>>, e: &Vec<Individual<A>>) {
        let new_parent = self.0.visit_individual_vec(parent, e);
        for i in e.iter() {
            self.individual(new_parent.clone(), i);
        }
    }

    pub fn literal_vec(&mut self, parent: Option<Kind<B>>, e: &Vec<Literal<A>>) {
        let new_parent = self.0.visit_literal_vec(parent, e);
        for i in e.iter() {
            self.literal(new_parent.clone(), i);
        }
    }

    pub fn facet_restriction_vec(&mut self, parent: Option<Kind<B>>, e: &Vec<FacetRestriction<A>>) {
        let new_parent = self.0.visit_facet_restriction_vec(parent, e);
        for i in e.iter() {
            self.facet_restriction(new_parent.clone(), i);
        }
    }

    pub fn data_range_vec(&mut self, parent: Option<Kind<B>>, e: &Vec<DataRange<A>>) {
        let new_parent = self.0.visit_data_range_vec(parent, e);
        for i in e.iter() {
            self.data_range(new_parent.clone(), i);
        }
    }

    pub fn atom_vec(&mut self, parent: Option<Kind<B>>, v: &Vec<Atom<A>>) {
        let new_parent = self.0.visit_atom_vec(parent, v);
        for i in v.iter() {
            self.atom(new_parent.clone(), i);
        }
    }

    pub fn darg_vec(&mut self, parent: Option<Kind<B>>, v: &Vec<DArgument<A>>) {
        let new_parent = self.0.visit_darg_vec(parent, v);
        for i in v.iter() {
            self.darg(new_parent.clone(), i);
        }
    }
}
