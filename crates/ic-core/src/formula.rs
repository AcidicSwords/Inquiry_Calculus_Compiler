use std::{collections::BTreeSet, fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, BindingVersionRef, TypeArtifact,
    TypeCatalog, TypeCheckError, TypeError, TypeRef, TypedForm, TypedFormRef,
};

/// Canonical artifact kind for Phase 2 formula definitions.
pub const FORMULA_ARTIFACT_KIND: &str = "ic.formula";

/// Payload schema version for Phase 2 formula definitions.
pub const FORMULA_SCHEMA_VERSION: u32 = 1;

macro_rules! artifact_reference {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(ArtifactRef);

        impl $name {
            #[must_use]
            pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
                Self(reference)
            }

            #[must_use]
            pub const fn as_artifact_ref(self) -> ArtifactRef {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl FromStr for $name {
            type Err = ArtifactError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                ArtifactRef::from_str(value).map(Self)
            }
        }
    };
}

artifact_reference!(FormulaRef);
artifact_reference!(RelationRef);

/// A typed term usable in the first-order formula grammar.
///
/// De Bruijn indices make quantified formulas capture-safe. Index zero denotes the
/// innermost binder in a formula artifact's explicit typing context.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TermIR {
    Form(TypedFormRef),
    Bound { index: u32, ty: TypeRef },
}

/// The complete canonical v1.1 surface formula grammar.
///
/// `Not` is classical formula negation only. Contextual typed negation is represented
/// later by a supported `NegationUse`; it is intentionally not a FormulaIR variant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormulaIR {
    Top,
    Bottom,
    Atom {
        relation: RelationRef,
        arguments: Vec<TermIR>,
    },
    Equal {
        left: TermIR,
        right: TermIR,
    },
    And {
        left: FormulaRef,
        right: FormulaRef,
    },
    Or {
        left: FormulaRef,
        right: FormulaRef,
    },
    Implies {
        premise: FormulaRef,
        conclusion: FormulaRef,
    },
    Not(FormulaRef),
    Exists {
        binder: TypeRef,
        body: FormulaRef,
    },
    Forall {
        binder: TypeRef,
        body: FormulaRef,
    },
}

/// A binding-scoped formula together with the types of its free de Bruijn variables.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormulaArtifact {
    binding: BindingVersionRef,
    context: Vec<TypeRef>,
    formula: FormulaIR,
}

impl FormulaArtifact {
    /// Constructs a formula with an explicit outer-to-inner bound-variable context.
    #[must_use]
    pub fn new(binding: BindingVersionRef, context: Vec<TypeRef>, formula: FormulaIR) -> Self {
        Self {
            binding,
            context,
            formula,
        }
    }

    #[must_use]
    pub const fn binding(&self) -> BindingVersionRef {
        self.binding
    }

    #[must_use]
    pub fn context(&self) -> &[TypeRef] {
        &self.context
    }

    #[must_use]
    pub const fn formula(&self) -> &FormulaIR {
        &self.formula
    }

    /// Encodes the formula's canonical payload.
    pub fn canonical_payload(&self) -> Result<Vec<u8>, FormulaError> {
        let mut encoded = Vec::new();
        write_reference(&mut encoded, self.binding.as_artifact_ref());
        write_count(&mut encoded, self.context.len())?;
        for type_ref in &self.context {
            write_reference(&mut encoded, type_ref.as_artifact_ref());
        }
        write_formula(&mut encoded, &self.formula)?;
        Ok(encoded)
    }

    /// Decodes one complete canonical formula payload.
    pub fn decode_payload(payload: &[u8]) -> Result<Self, FormulaError> {
        let mut cursor = FormulaCursor::new(payload);
        let binding = BindingVersionRef::from_artifact_ref(cursor.read_reference()?);
        let context_count = cursor.read_count()?;
        let mut context = Vec::with_capacity(context_count);
        for _ in 0..context_count {
            context.push(TypeRef::from_artifact_ref(cursor.read_reference()?));
        }
        let formula = cursor.read_formula()?;
        if !cursor.is_finished() {
            return Err(FormulaError::TrailingPayloadBytes(cursor.remaining()));
        }
        Ok(Self {
            binding,
            context,
            formula,
        })
    }

    /// Returns the canonical envelope for this formula.
    pub fn envelope(&self) -> Result<ArtifactEnvelope, FormulaError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(FORMULA_ARTIFACT_KIND)?,
            FORMULA_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }

    /// Returns this formula's stable content-addressed reference.
    pub fn formula_ref(&self) -> Result<FormulaRef, FormulaError> {
        Ok(FormulaRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    /// Decodes a formula from its canonical artifact envelope.
    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, FormulaError> {
        if envelope.kind().as_str() != FORMULA_ARTIFACT_KIND {
            return Err(FormulaError::UnexpectedArtifactKind {
                expected: FORMULA_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != FORMULA_SCHEMA_VERSION {
            return Err(FormulaError::UnsupportedFormulaSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    /// Returns dependencies declared by the represented formula syntax.
    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut references = vec![self.binding.as_artifact_ref()];
        references.extend(
            self.context
                .iter()
                .map(|reference| reference.as_artifact_ref()),
        );
        self.formula_references(&mut references);
        references
    }

    fn formula_references(&self, references: &mut Vec<ArtifactRef>) {
        match &self.formula {
            FormulaIR::Top | FormulaIR::Bottom => {}
            FormulaIR::Atom {
                relation,
                arguments,
            } => {
                references.push(relation.as_artifact_ref());
                for argument in arguments {
                    term_references(argument, references);
                }
            }
            FormulaIR::Equal { left, right } => {
                term_references(left, references);
                term_references(right, references);
            }
            FormulaIR::And { left, right } | FormulaIR::Or { left, right } => {
                references.push(left.as_artifact_ref());
                references.push(right.as_artifact_ref());
            }
            FormulaIR::Implies {
                premise,
                conclusion,
            } => {
                references.push(premise.as_artifact_ref());
                references.push(conclusion.as_artifact_ref());
            }
            FormulaIR::Not(body) => references.push(body.as_artifact_ref()),
            FormulaIR::Exists { binder, body } | FormulaIR::Forall { binder, body } => {
                references.push(binder.as_artifact_ref());
                references.push(body.as_artifact_ref());
            }
        }
    }

    /// Checks context, terms, and nested formula contexts using an explicit catalog.
    ///
    /// Relation-atom signatures are checked by the relation-schema layer. This method
    /// deliberately does not reinterpret an atom's relation reference as opaque bytes.
    pub fn check_terms<C: FormulaCatalog>(&self, catalog: &C) -> Result<(), FormulaCheckError> {
        let mut visiting = BTreeSet::new();
        let mut completed = BTreeSet::new();
        check_context(&self.context, self.binding, catalog)?;
        check_formula_node(
            &self.formula,
            self.binding,
            &self.context,
            catalog,
            &mut visiting,
            &mut completed,
        )
    }
}

/// The catalog required to validate formula references before relation evaluation exists.
pub trait FormulaCatalog: TypeCatalog {
    /// Resolves a formula artifact by its claimed stable identity.
    fn resolve_formula(&self, reference: FormulaRef) -> Option<FormulaArtifact>;

    /// Resolves a typed-form declaration by its claimed stable identity.
    fn resolve_typed_form(&self, reference: TypedFormRef) -> Option<TypedForm>;
}

fn check_context<C: TypeCatalog>(
    context: &[TypeRef],
    binding: BindingVersionRef,
    catalog: &C,
) -> Result<(), FormulaCheckError> {
    for reference in context {
        let type_artifact = resolve_type(*reference, catalog)?;
        if type_artifact.binding() != binding {
            return Err(FormulaCheckError::ContextBindingMismatch {
                expected: binding,
                actual: type_artifact.binding(),
                reference: *reference,
            });
        }
        type_artifact.check(catalog)?;
    }
    Ok(())
}

fn check_formula_node<C: FormulaCatalog>(
    formula: &FormulaIR,
    binding: BindingVersionRef,
    context: &[TypeRef],
    catalog: &C,
    visiting: &mut BTreeSet<FormulaRef>,
    completed: &mut BTreeSet<FormulaRef>,
) -> Result<(), FormulaCheckError> {
    match formula {
        FormulaIR::Top | FormulaIR::Bottom => Ok(()),
        FormulaIR::Atom { arguments, .. } => {
            for argument in arguments {
                check_term(argument, binding, context, catalog)?;
            }
            Ok(())
        }
        FormulaIR::Equal { left, right } => {
            let left_type = check_term(left, binding, context, catalog)?;
            let right_type = check_term(right, binding, context, catalog)?;
            if left_type == right_type {
                Ok(())
            } else {
                Err(FormulaCheckError::EqualityTypeMismatch {
                    left: left_type,
                    right: right_type,
                })
            }
        }
        FormulaIR::And { left, right } | FormulaIR::Or { left, right } => {
            check_formula_ref(*left, binding, context, catalog, visiting, completed)?;
            check_formula_ref(*right, binding, context, catalog, visiting, completed)
        }
        FormulaIR::Implies {
            premise,
            conclusion,
        } => {
            check_formula_ref(*premise, binding, context, catalog, visiting, completed)?;
            check_formula_ref(*conclusion, binding, context, catalog, visiting, completed)
        }
        FormulaIR::Not(body) => {
            check_formula_ref(*body, binding, context, catalog, visiting, completed)
        }
        FormulaIR::Exists { binder, body } | FormulaIR::Forall { binder, body } => {
            let binder_type = resolve_type(*binder, catalog)?;
            if binder_type.binding() != binding {
                return Err(FormulaCheckError::ContextBindingMismatch {
                    expected: binding,
                    actual: binder_type.binding(),
                    reference: *binder,
                });
            }
            binder_type.check(catalog)?;
            let mut body_context = context.to_vec();
            body_context.push(*binder);
            check_formula_ref(*body, binding, &body_context, catalog, visiting, completed)
        }
    }
}

fn check_formula_ref<C: FormulaCatalog>(
    reference: FormulaRef,
    binding: BindingVersionRef,
    expected_context: &[TypeRef],
    catalog: &C,
    visiting: &mut BTreeSet<FormulaRef>,
    completed: &mut BTreeSet<FormulaRef>,
) -> Result<(), FormulaCheckError> {
    if completed.contains(&reference) {
        return Ok(());
    }
    if !visiting.insert(reference) {
        return Err(FormulaCheckError::RecursiveFormulaReference(reference));
    }
    let Some(formula_artifact) = catalog.resolve_formula(reference) else {
        return Err(FormulaCheckError::UnresolvedFormula(reference));
    };
    let calculated = formula_artifact.formula_ref()?;
    if calculated != reference {
        return Err(FormulaCheckError::FormulaReferenceIdentityMismatch {
            reference,
            calculated,
        });
    }
    if formula_artifact.binding() != binding {
        return Err(FormulaCheckError::FormulaBindingMismatch {
            expected: binding,
            actual: formula_artifact.binding(),
            reference,
        });
    }
    if formula_artifact.context() != expected_context {
        return Err(FormulaCheckError::FormulaContextMismatch {
            reference,
            expected: expected_context.to_vec(),
            actual: formula_artifact.context().to_vec(),
        });
    }
    check_formula_node(
        formula_artifact.formula(),
        binding,
        expected_context,
        catalog,
        visiting,
        completed,
    )?;
    visiting.remove(&reference);
    completed.insert(reference);
    Ok(())
}

fn check_term<C: FormulaCatalog>(
    term: &TermIR,
    binding: BindingVersionRef,
    context: &[TypeRef],
    catalog: &C,
) -> Result<TypeRef, FormulaCheckError> {
    match term {
        TermIR::Form(reference) => {
            let Some(typed_form) = catalog.resolve_typed_form(*reference) else {
                return Err(FormulaCheckError::UnresolvedTypedForm(*reference));
            };
            let calculated = typed_form.typed_form_ref()?;
            if calculated != *reference {
                return Err(FormulaCheckError::TypedFormReferenceIdentityMismatch {
                    reference: *reference,
                    calculated,
                });
            }
            if typed_form.binding() != binding {
                return Err(FormulaCheckError::TypedFormBindingMismatch {
                    expected: binding,
                    actual: typed_form.binding(),
                    reference: *reference,
                });
            }
            typed_form.check(catalog)?;
            Ok(typed_form.ty())
        }
        TermIR::Bound { index, ty } => {
            let index_usize =
                usize::try_from(*index).map_err(|_| FormulaCheckError::BoundIndexOutOfRange {
                    index: *index,
                    context_len: context.len(),
                })?;
            let Some(expected) = context.iter().rev().nth(index_usize) else {
                return Err(FormulaCheckError::BoundIndexOutOfRange {
                    index: *index,
                    context_len: context.len(),
                });
            };
            if *expected == *ty {
                Ok(*ty)
            } else {
                Err(FormulaCheckError::BoundTypeMismatch {
                    index: *index,
                    expected: *expected,
                    actual: *ty,
                })
            }
        }
    }
}

fn resolve_type<C: TypeCatalog>(
    reference: TypeRef,
    catalog: &C,
) -> Result<TypeArtifact, FormulaCheckError> {
    let Some(type_artifact) = catalog.resolve_type(reference) else {
        return Err(FormulaCheckError::UnresolvedType(reference));
    };
    let calculated = type_artifact.type_ref()?;
    if calculated != reference {
        return Err(FormulaCheckError::TypeReferenceIdentityMismatch {
            reference,
            calculated,
        });
    }
    Ok(type_artifact)
}

fn term_references(term: &TermIR, references: &mut Vec<ArtifactRef>) {
    if let TermIR::Form(reference) = term {
        references.push(reference.as_artifact_ref());
    }
}

fn write_formula(encoded: &mut Vec<u8>, formula: &FormulaIR) -> Result<(), FormulaError> {
    match formula {
        FormulaIR::Top => encoded.push(0),
        FormulaIR::Bottom => encoded.push(1),
        FormulaIR::Atom {
            relation,
            arguments,
        } => {
            encoded.push(2);
            write_reference(encoded, relation.as_artifact_ref());
            write_count(encoded, arguments.len())?;
            for argument in arguments {
                write_term(encoded, argument);
            }
        }
        FormulaIR::Equal { left, right } => {
            encoded.push(3);
            write_term(encoded, left);
            write_term(encoded, right);
        }
        FormulaIR::And { left, right } => {
            encoded.push(4);
            write_reference(encoded, left.as_artifact_ref());
            write_reference(encoded, right.as_artifact_ref());
        }
        FormulaIR::Or { left, right } => {
            encoded.push(5);
            write_reference(encoded, left.as_artifact_ref());
            write_reference(encoded, right.as_artifact_ref());
        }
        FormulaIR::Implies {
            premise,
            conclusion,
        } => {
            encoded.push(6);
            write_reference(encoded, premise.as_artifact_ref());
            write_reference(encoded, conclusion.as_artifact_ref());
        }
        FormulaIR::Not(body) => {
            encoded.push(7);
            write_reference(encoded, body.as_artifact_ref());
        }
        FormulaIR::Exists { binder, body } => {
            encoded.push(8);
            write_reference(encoded, binder.as_artifact_ref());
            write_reference(encoded, body.as_artifact_ref());
        }
        FormulaIR::Forall { binder, body } => {
            encoded.push(9);
            write_reference(encoded, binder.as_artifact_ref());
            write_reference(encoded, body.as_artifact_ref());
        }
    }
    Ok(())
}

fn write_term(encoded: &mut Vec<u8>, term: &TermIR) {
    match term {
        TermIR::Form(reference) => {
            encoded.push(0);
            write_reference(encoded, reference.as_artifact_ref());
        }
        TermIR::Bound { index, ty } => {
            encoded.push(1);
            encoded.extend_from_slice(&index.to_be_bytes());
            write_reference(encoded, ty.as_artifact_ref());
        }
    }
}

fn write_reference(encoded: &mut Vec<u8>, reference: ArtifactRef) {
    encoded.extend_from_slice(reference.as_bytes());
}

fn write_count(encoded: &mut Vec<u8>, count: usize) -> Result<(), FormulaError> {
    let count = u32::try_from(count).map_err(|_| FormulaError::CollectionTooLong(count))?;
    encoded.extend_from_slice(&count.to_be_bytes());
    Ok(())
}

struct FormulaCursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> FormulaCursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn read_formula(&mut self) -> Result<FormulaIR, FormulaError> {
        match self.read_byte()? {
            0 => Ok(FormulaIR::Top),
            1 => Ok(FormulaIR::Bottom),
            2 => {
                let relation = RelationRef::from_artifact_ref(self.read_reference()?);
                let count = self.read_count()?;
                let mut arguments = Vec::with_capacity(count);
                for _ in 0..count {
                    arguments.push(self.read_term()?);
                }
                Ok(FormulaIR::Atom {
                    relation,
                    arguments,
                })
            }
            3 => Ok(FormulaIR::Equal {
                left: self.read_term()?,
                right: self.read_term()?,
            }),
            4 => Ok(FormulaIR::And {
                left: FormulaRef::from_artifact_ref(self.read_reference()?),
                right: FormulaRef::from_artifact_ref(self.read_reference()?),
            }),
            5 => Ok(FormulaIR::Or {
                left: FormulaRef::from_artifact_ref(self.read_reference()?),
                right: FormulaRef::from_artifact_ref(self.read_reference()?),
            }),
            6 => Ok(FormulaIR::Implies {
                premise: FormulaRef::from_artifact_ref(self.read_reference()?),
                conclusion: FormulaRef::from_artifact_ref(self.read_reference()?),
            }),
            7 => Ok(FormulaIR::Not(FormulaRef::from_artifact_ref(
                self.read_reference()?,
            ))),
            8 => Ok(FormulaIR::Exists {
                binder: TypeRef::from_artifact_ref(self.read_reference()?),
                body: FormulaRef::from_artifact_ref(self.read_reference()?),
            }),
            9 => Ok(FormulaIR::Forall {
                binder: TypeRef::from_artifact_ref(self.read_reference()?),
                body: FormulaRef::from_artifact_ref(self.read_reference()?),
            }),
            tag => Err(FormulaError::UnknownFormulaTag(tag)),
        }
    }

    fn read_term(&mut self) -> Result<TermIR, FormulaError> {
        match self.read_byte()? {
            0 => Ok(TermIR::Form(TypedFormRef::from_artifact_ref(
                self.read_reference()?,
            ))),
            1 => Ok(TermIR::Bound {
                index: self.read_u32()?,
                ty: TypeRef::from_artifact_ref(self.read_reference()?),
            }),
            tag => Err(FormulaError::UnknownTermTag(tag)),
        }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], FormulaError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(FormulaError::PayloadLengthOverflow)?;
        let value = self
            .bytes
            .get(self.position..end)
            .ok_or(FormulaError::TruncatedPayload)?;
        self.position = end;
        Ok(value)
    }

    fn read_byte(&mut self) -> Result<u8, FormulaError> {
        Ok(self.take(1)?[0])
    }

    fn read_u32(&mut self) -> Result<u32, FormulaError> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| FormulaError::TruncatedPayload)?;
        Ok(u32::from_be_bytes(bytes))
    }

    fn read_count(&mut self) -> Result<usize, FormulaError> {
        usize::try_from(self.read_u32()?).map_err(|_| FormulaError::PayloadLengthOverflow)
    }

    fn read_reference(&mut self) -> Result<ArtifactRef, FormulaError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| FormulaError::TruncatedPayload)?;
        Ok(ArtifactRef::from_bytes(bytes))
    }

    const fn is_finished(&self) -> bool {
        self.position == self.bytes.len()
    }

    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

/// Formula encoding and decoding errors.
#[derive(Debug, Error)]
pub enum FormulaError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),

    #[error("formula collection is too long: {0} entries")]
    CollectionTooLong(usize),

    #[error("formula payload is truncated")]
    TruncatedPayload,

    #[error("formula payload length overflows this platform")]
    PayloadLengthOverflow,

    #[error("formula payload contains {0} trailing bytes")]
    TrailingPayloadBytes(usize),

    #[error("formula payload contains unknown formula tag {0}")]
    UnknownFormulaTag(u8),

    #[error("formula payload contains unknown term tag {0}")]
    UnknownTermTag(u8),

    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },

    #[error("unsupported formula schema version {0}")]
    UnsupportedFormulaSchemaVersion(u32),
}

/// Formula term and nested-context validation errors.
#[derive(Debug, Error)]
pub enum FormulaCheckError {
    #[error(transparent)]
    Formula(#[from] FormulaError),

    #[error(transparent)]
    Type(#[from] TypeCheckError),

    #[error(transparent)]
    TypeArtifact(#[from] TypeError),

    #[error("type {0} is not available from the declared catalog")]
    UnresolvedType(TypeRef),

    #[error("catalog entry for type {reference} hashes to {calculated}, not its claimed identity")]
    TypeReferenceIdentityMismatch {
        reference: TypeRef,
        calculated: TypeRef,
    },

    #[error("context type {reference} belongs to binding {actual}, expected {expected}")]
    ContextBindingMismatch {
        expected: BindingVersionRef,
        actual: BindingVersionRef,
        reference: TypeRef,
    },

    #[error("formula {0} is not available from the declared catalog")]
    UnresolvedFormula(FormulaRef),

    #[error(
        "catalog entry for formula {reference} hashes to {calculated}, not its claimed identity"
    )]
    FormulaReferenceIdentityMismatch {
        reference: FormulaRef,
        calculated: FormulaRef,
    },

    #[error("formula {reference} belongs to binding {actual}, expected {expected}")]
    FormulaBindingMismatch {
        expected: BindingVersionRef,
        actual: BindingVersionRef,
        reference: FormulaRef,
    },

    #[error("formula {reference} has a different bound-variable context")]
    FormulaContextMismatch {
        reference: FormulaRef,
        expected: Vec<TypeRef>,
        actual: Vec<TypeRef>,
    },

    #[error("recursive formula reference encountered at {0}")]
    RecursiveFormulaReference(FormulaRef),

    #[error("typed form {0} is not available from the declared catalog")]
    UnresolvedTypedForm(TypedFormRef),

    #[error(
        "catalog entry for typed form {reference} hashes to {calculated}, not its claimed identity"
    )]
    TypedFormReferenceIdentityMismatch {
        reference: TypedFormRef,
        calculated: TypedFormRef,
    },

    #[error("typed form {reference} belongs to binding {actual}, expected {expected}")]
    TypedFormBindingMismatch {
        expected: BindingVersionRef,
        actual: BindingVersionRef,
        reference: TypedFormRef,
    },

    #[error("bound variable index {index} exceeds context length {context_len}")]
    BoundIndexOutOfRange { index: u32, context_len: usize },

    #[error("bound variable {index} has type {actual}, expected {expected}")]
    BoundTypeMismatch {
        index: u32,
        expected: TypeRef,
        actual: TypeRef,
    },

    #[error("equality compares type {left} with incompatible type {right}")]
    EqualityTypeMismatch { left: TypeRef, right: TypeRef },
}
