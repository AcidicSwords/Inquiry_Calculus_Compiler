use std::{collections::BTreeSet, fmt, str::FromStr};

use thiserror::Error;

use crate::{ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef};

/// Canonical artifact kind for Phase 1 type definitions.
pub const TYPE_ARTIFACT_KIND: &str = "ic.type";

/// Canonical artifact kind for Phase 1 typed-form declarations.
pub const TYPED_FORM_ARTIFACT_KIND: &str = "ic.typed-form";

/// Payload schema version for Phase 1 type definitions.
pub const TYPE_SCHEMA_VERSION: u32 = 1;

/// Payload schema version for Phase 1 typed-form declarations.
pub const TYPED_FORM_SCHEMA_VERSION: u32 = 1;

/// A reference known to identify a canonical type artifact.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypeRef(ArtifactRef);

impl TypeRef {
    /// Wraps an artifact reference after the surrounding route has established its kind.
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }

    /// Returns the underlying content-addressed artifact reference.
    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
}

impl fmt::Display for TypeRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for TypeRef {
    type Err = ArtifactError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// A reference to the immutable binding version that scopes a typed form or type.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BindingVersionRef(ArtifactRef);

impl BindingVersionRef {
    /// Wraps an artifact reference after the surrounding route has established its kind.
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }

    /// Returns the underlying content-addressed artifact reference.
    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
}

impl fmt::Display for BindingVersionRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for BindingVersionRef {
    type Err = ArtifactError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// A reference to an admitted dependent type-family artifact.
///
/// Phase 1 preserves the family position in `Sigma` and `Pi` exactly, but does not
/// invent a generic type-level expression language. A catalog supplies the checked
/// domain of a family; its binding-native implementation is admitted in a later phase.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypeFamilyRef(ArtifactRef);

impl TypeFamilyRef {
    /// Wraps an artifact reference after the surrounding route has established its kind.
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }

    /// Returns the underlying content-addressed artifact reference.
    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
}

impl fmt::Display for TypeFamilyRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for TypeFamilyRef {
    type Err = ArtifactError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// A reference known to identify a canonical typed-form declaration artifact.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypedFormRef(ArtifactRef);

impl TypedFormRef {
    /// Wraps an artifact reference after the surrounding route has established its kind.
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }

    /// Returns the underlying content-addressed artifact reference.
    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
}

impl fmt::Display for TypedFormRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for TypedFormRef {
    type Err = ArtifactError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// A canonical ASCII identifier for a binding-local named type.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypeSymbol(String);

impl TypeSymbol {
    /// Validates and constructs a binding-local type symbol.
    pub fn new(value: impl Into<String>) -> Result<Self, TypeError> {
        let value = value.into();
        let mut bytes = value.bytes();
        let Some(first) = bytes.next() else {
            return Err(TypeError::EmptySymbol);
        };

        if !(first.is_ascii_alphabetic() || first == b'_')
            || !bytes.all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b'-'))
        {
            return Err(TypeError::InvalidSymbol(value));
        }

        u32::try_from(value.len()).map_err(|_| TypeError::SymbolTooLong(value.len()))?;
        Ok(Self(value))
    }

    /// Returns the exact canonical symbol text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TypeSymbol {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for TypeSymbol {
    type Err = TypeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// The accepted canonical v1.1 Phase 1 type grammar.
///
/// The variants deliberately follow the canonical grammar: binary `Product` and `Sum`,
/// `Prog(A)`, and unary `Code(A)`. Plan-only candidates such as `Int`, `Text`, `Bytes`,
/// n-ary products/sums, and input/output `Code` are not semantic constructors here.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TyIR {
    Unit,
    Bool,
    Nat,
    Named {
        binding: BindingVersionRef,
        name: TypeSymbol,
        version: ArtifactRef,
    },
    Product {
        left: TypeRef,
        right: TypeRef,
    },
    Sum {
        left: TypeRef,
        right: TypeRef,
    },
    Sigma {
        domain: TypeRef,
        family: TypeFamilyRef,
    },
    Pi {
        domain: TypeRef,
        family: TypeFamilyRef,
    },
    Finite(TypeRef),
    List(TypeRef),
    Raw(TypeRef),
    Result(TypeRef),
    Prog(TypeRef),
    Code(TypeRef),
}

impl TyIR {
    /// Returns canonical child type references, excluding dependent-family references.
    #[must_use]
    pub fn child_types(&self) -> Vec<TypeRef> {
        match self {
            Self::Unit | Self::Bool | Self::Nat | Self::Named { .. } => Vec::new(),
            Self::Product { left, right } | Self::Sum { left, right } => vec![*left, *right],
            Self::Sigma { domain, .. } | Self::Pi { domain, .. } => vec![*domain],
            Self::Finite(inner)
            | Self::List(inner)
            | Self::Raw(inner)
            | Self::Result(inner)
            | Self::Prog(inner)
            | Self::Code(inner) => vec![*inner],
        }
    }
}

/// A binding-scoped canonical type definition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeArtifact {
    binding: BindingVersionRef,
    ty: TyIR,
}

impl TypeArtifact {
    /// Constructs a binding-scoped type definition.
    #[must_use]
    pub const fn new(binding: BindingVersionRef, ty: TyIR) -> Self {
        Self { binding, ty }
    }

    /// Returns the binding version that scopes this type.
    #[must_use]
    pub const fn binding(&self) -> BindingVersionRef {
        self.binding
    }

    /// Returns the represented type grammar node.
    #[must_use]
    pub const fn ty(&self) -> &TyIR {
        &self.ty
    }

    /// Encodes the type definition's canonical payload.
    pub fn canonical_payload(&self) -> Result<Vec<u8>, TypeError> {
        let mut encoded = Vec::new();
        write_reference(&mut encoded, self.binding.as_artifact_ref());

        match &self.ty {
            TyIR::Unit => encoded.push(0),
            TyIR::Bool => encoded.push(1),
            TyIR::Nat => encoded.push(2),
            TyIR::Named {
                binding,
                name,
                version,
            } => {
                encoded.push(3);
                write_reference(&mut encoded, binding.as_artifact_ref());
                write_text(&mut encoded, name.as_str())?;
                write_reference(&mut encoded, *version);
            }
            TyIR::Product { left, right } => {
                encoded.push(4);
                write_type_reference(&mut encoded, *left);
                write_type_reference(&mut encoded, *right);
            }
            TyIR::Sum { left, right } => {
                encoded.push(5);
                write_type_reference(&mut encoded, *left);
                write_type_reference(&mut encoded, *right);
            }
            TyIR::Sigma { domain, family } => {
                encoded.push(6);
                write_type_reference(&mut encoded, *domain);
                write_reference(&mut encoded, family.as_artifact_ref());
            }
            TyIR::Pi { domain, family } => {
                encoded.push(7);
                write_type_reference(&mut encoded, *domain);
                write_reference(&mut encoded, family.as_artifact_ref());
            }
            TyIR::Finite(inner) => {
                encoded.push(8);
                write_type_reference(&mut encoded, *inner);
            }
            TyIR::List(inner) => {
                encoded.push(9);
                write_type_reference(&mut encoded, *inner);
            }
            TyIR::Raw(inner) => {
                encoded.push(10);
                write_type_reference(&mut encoded, *inner);
            }
            TyIR::Result(inner) => {
                encoded.push(11);
                write_type_reference(&mut encoded, *inner);
            }
            TyIR::Prog(inner) => {
                encoded.push(12);
                write_type_reference(&mut encoded, *inner);
            }
            TyIR::Code(inner) => {
                encoded.push(13);
                write_type_reference(&mut encoded, *inner);
            }
        }

        Ok(encoded)
    }

    /// Decodes one complete canonical type payload.
    pub fn decode_payload(payload: &[u8]) -> Result<Self, TypeError> {
        let mut cursor = TypeCursor::new(payload);
        let binding = BindingVersionRef::from_artifact_ref(cursor.read_reference()?);
        let tag = cursor.read_byte()?;
        let ty = match tag {
            0 => TyIR::Unit,
            1 => TyIR::Bool,
            2 => TyIR::Nat,
            3 => TyIR::Named {
                binding: BindingVersionRef::from_artifact_ref(cursor.read_reference()?),
                name: TypeSymbol::new(cursor.read_text()?)?,
                version: cursor.read_reference()?,
            },
            4 => TyIR::Product {
                left: TypeRef::from_artifact_ref(cursor.read_reference()?),
                right: TypeRef::from_artifact_ref(cursor.read_reference()?),
            },
            5 => TyIR::Sum {
                left: TypeRef::from_artifact_ref(cursor.read_reference()?),
                right: TypeRef::from_artifact_ref(cursor.read_reference()?),
            },
            6 => TyIR::Sigma {
                domain: TypeRef::from_artifact_ref(cursor.read_reference()?),
                family: TypeFamilyRef::from_artifact_ref(cursor.read_reference()?),
            },
            7 => TyIR::Pi {
                domain: TypeRef::from_artifact_ref(cursor.read_reference()?),
                family: TypeFamilyRef::from_artifact_ref(cursor.read_reference()?),
            },
            8 => TyIR::Finite(TypeRef::from_artifact_ref(cursor.read_reference()?)),
            9 => TyIR::List(TypeRef::from_artifact_ref(cursor.read_reference()?)),
            10 => TyIR::Raw(TypeRef::from_artifact_ref(cursor.read_reference()?)),
            11 => TyIR::Result(TypeRef::from_artifact_ref(cursor.read_reference()?)),
            12 => TyIR::Prog(TypeRef::from_artifact_ref(cursor.read_reference()?)),
            13 => TyIR::Code(TypeRef::from_artifact_ref(cursor.read_reference()?)),
            other => return Err(TypeError::UnknownTypeTag(other)),
        };

        if !cursor.is_finished() {
            return Err(TypeError::TrailingPayloadBytes(cursor.remaining()));
        }

        Ok(Self { binding, ty })
    }

    /// Returns the canonical artifact envelope for this type definition.
    pub fn envelope(&self) -> Result<ArtifactEnvelope, TypeError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(TYPE_ARTIFACT_KIND)?,
            TYPE_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }

    /// Calculates this type definition's stable content-addressed reference.
    pub fn type_ref(&self) -> Result<TypeRef, TypeError> {
        Ok(TypeRef::from_artifact_ref(self.envelope()?.artifact_ref()?))
    }

    /// Returns the explicit artifact dependencies required before this type is stored.
    ///
    /// The list follows the represented syntax and is never inferred by inspecting the
    /// canonical payload after construction. Repeated references are retained because
    /// they are distinct syntactic positions, although a store may check their presence
    /// only once.
    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut references = vec![self.binding.as_artifact_ref()];
        match &self.ty {
            TyIR::Unit | TyIR::Bool | TyIR::Nat => {}
            TyIR::Named {
                binding, version, ..
            } => {
                references.push(binding.as_artifact_ref());
                references.push(*version);
            }
            TyIR::Product { left, right } | TyIR::Sum { left, right } => {
                references.push(left.as_artifact_ref());
                references.push(right.as_artifact_ref());
            }
            TyIR::Sigma { domain, family } | TyIR::Pi { domain, family } => {
                references.push(domain.as_artifact_ref());
                references.push(family.as_artifact_ref());
            }
            TyIR::Finite(inner)
            | TyIR::List(inner)
            | TyIR::Raw(inner)
            | TyIR::Result(inner)
            | TyIR::Prog(inner)
            | TyIR::Code(inner) => references.push(inner.as_artifact_ref()),
        }
        references
    }

    /// Decodes a type definition from its canonical artifact envelope.
    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, TypeError> {
        if envelope.kind().as_str() != TYPE_ARTIFACT_KIND {
            return Err(TypeError::UnexpectedArtifactKind {
                expected: TYPE_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != TYPE_SCHEMA_VERSION {
            return Err(TypeError::UnsupportedTypeSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    /// Checks this type and every reachable child against a binding-aware catalog.
    pub fn check<C: TypeCatalog>(&self, catalog: &C) -> Result<(), TypeCheckError> {
        let mut visiting = BTreeSet::new();
        let mut completed = BTreeSet::new();
        check_type_node(
            &self.ty,
            self.binding,
            catalog,
            &mut visiting,
            &mut completed,
        )
    }
}

/// A binding-scoped declaration that an immutable form artifact has a type artifact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypedForm {
    binding: BindingVersionRef,
    ty: TypeRef,
    form: ArtifactRef,
}

impl TypedForm {
    /// Constructs a typed-form declaration.
    #[must_use]
    pub const fn new(binding: BindingVersionRef, ty: TypeRef, form: ArtifactRef) -> Self {
        Self { binding, ty, form }
    }

    /// Returns the binding version that scopes the declaration.
    #[must_use]
    pub const fn binding(&self) -> BindingVersionRef {
        self.binding
    }

    /// Returns the declared type artifact.
    #[must_use]
    pub const fn ty(&self) -> TypeRef {
        self.ty
    }

    /// Returns the opaque represented-form artifact.
    #[must_use]
    pub const fn form(&self) -> ArtifactRef {
        self.form
    }

    /// Encodes the typed-form declaration's canonical payload.
    #[must_use]
    pub fn canonical_payload(&self) -> Vec<u8> {
        let mut encoded = Vec::with_capacity(96);
        write_reference(&mut encoded, self.binding.as_artifact_ref());
        write_type_reference(&mut encoded, self.ty);
        write_reference(&mut encoded, self.form);
        encoded
    }

    /// Decodes one complete canonical typed-form payload.
    pub fn decode_payload(payload: &[u8]) -> Result<Self, TypeError> {
        let mut cursor = TypeCursor::new(payload);
        let typed_form = Self {
            binding: BindingVersionRef::from_artifact_ref(cursor.read_reference()?),
            ty: TypeRef::from_artifact_ref(cursor.read_reference()?),
            form: cursor.read_reference()?,
        };
        if !cursor.is_finished() {
            return Err(TypeError::TrailingPayloadBytes(cursor.remaining()));
        }
        Ok(typed_form)
    }

    /// Returns the canonical artifact envelope for this typed-form declaration.
    pub fn envelope(&self) -> Result<ArtifactEnvelope, TypeError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(TYPED_FORM_ARTIFACT_KIND)?,
            TYPED_FORM_SCHEMA_VERSION,
            self.canonical_payload(),
        ))
    }

    /// Calculates this typed-form declaration's stable content-addressed reference.
    pub fn typed_form_ref(&self) -> Result<TypedFormRef, TypeError> {
        Ok(TypedFormRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    /// Returns the explicit artifact dependencies required before this declaration is stored.
    #[must_use]
    pub fn referenced_artifacts(&self) -> [ArtifactRef; 3] {
        [
            self.binding.as_artifact_ref(),
            self.ty.as_artifact_ref(),
            self.form,
        ]
    }

    /// Decodes a typed-form declaration from its canonical artifact envelope.
    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, TypeError> {
        if envelope.kind().as_str() != TYPED_FORM_ARTIFACT_KIND {
            return Err(TypeError::UnexpectedArtifactKind {
                expected: TYPED_FORM_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != TYPED_FORM_SCHEMA_VERSION {
            return Err(TypeError::UnsupportedTypedFormSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    /// Checks that the declared type is canonical, resolved, and scoped by this binding.
    ///
    /// Validation of the form's binding-native value syntax begins with the relation and
    /// term kernels; this Phase 1 declaration deliberately does not infer a type from
    /// opaque artifact bytes.
    pub fn check<C: TypeCatalog>(&self, catalog: &C) -> Result<(), TypeCheckError> {
        let type_artifact = resolve_type(self.ty, catalog)?;
        if type_artifact.binding() != self.binding {
            return Err(TypeCheckError::TypedFormBindingMismatch {
                form_binding: self.binding,
                type_binding: type_artifact.binding(),
            });
        }
        type_artifact.check(catalog)
    }
}

/// The checked source for Phase 1 type and dependent-family references.
pub trait TypeCatalog {
    /// Resolves a type artifact by its claimed stable type reference.
    fn resolve_type(&self, reference: TypeRef) -> Option<TypeArtifact>;

    /// Resolves the binding and domain of an admitted dependent type family.
    fn resolve_family_domain(
        &self,
        reference: TypeFamilyRef,
    ) -> Option<(BindingVersionRef, TypeRef)>;
}

fn check_type_node<C: TypeCatalog>(
    ty: &TyIR,
    expected_binding: BindingVersionRef,
    catalog: &C,
    visiting: &mut BTreeSet<TypeRef>,
    completed: &mut BTreeSet<TypeRef>,
) -> Result<(), TypeCheckError> {
    match ty {
        TyIR::Unit | TyIR::Bool | TyIR::Nat => Ok(()),
        TyIR::Named { binding, .. } => {
            if *binding == expected_binding {
                Ok(())
            } else {
                Err(TypeCheckError::NamedBindingMismatch {
                    expected: expected_binding,
                    actual: *binding,
                })
            }
        }
        TyIR::Product { left, right } | TyIR::Sum { left, right } => {
            check_type_ref(*left, expected_binding, catalog, visiting, completed)?;
            check_type_ref(*right, expected_binding, catalog, visiting, completed)
        }
        TyIR::Sigma { domain, family } | TyIR::Pi { domain, family } => {
            check_type_ref(*domain, expected_binding, catalog, visiting, completed)?;
            let Some((family_binding, family_domain)) = catalog.resolve_family_domain(*family)
            else {
                return Err(TypeCheckError::UnresolvedTypeFamily(*family));
            };
            if family_binding != expected_binding {
                return Err(TypeCheckError::FamilyBindingMismatch {
                    expected: expected_binding,
                    actual: family_binding,
                });
            }
            if family_domain != *domain {
                return Err(TypeCheckError::FamilyDomainMismatch {
                    expected: *domain,
                    actual: family_domain,
                });
            }
            Ok(())
        }
        TyIR::Finite(inner)
        | TyIR::List(inner)
        | TyIR::Raw(inner)
        | TyIR::Result(inner)
        | TyIR::Prog(inner)
        | TyIR::Code(inner) => {
            check_type_ref(*inner, expected_binding, catalog, visiting, completed)
        }
    }
}

fn check_type_ref<C: TypeCatalog>(
    reference: TypeRef,
    expected_binding: BindingVersionRef,
    catalog: &C,
    visiting: &mut BTreeSet<TypeRef>,
    completed: &mut BTreeSet<TypeRef>,
) -> Result<(), TypeCheckError> {
    if completed.contains(&reference) {
        return Ok(());
    }
    if !visiting.insert(reference) {
        return Err(TypeCheckError::RecursiveTypeReference(reference));
    }

    let type_artifact = resolve_type(reference, catalog)?;
    if type_artifact.binding() != expected_binding {
        return Err(TypeCheckError::ChildBindingMismatch {
            expected: expected_binding,
            actual: type_artifact.binding(),
            reference,
        });
    }
    check_type_node(
        type_artifact.ty(),
        expected_binding,
        catalog,
        visiting,
        completed,
    )?;
    visiting.remove(&reference);
    completed.insert(reference);
    Ok(())
}

fn resolve_type<C: TypeCatalog>(
    reference: TypeRef,
    catalog: &C,
) -> Result<TypeArtifact, TypeCheckError> {
    let Some(type_artifact) = catalog.resolve_type(reference) else {
        return Err(TypeCheckError::UnresolvedType(reference));
    };
    let calculated = type_artifact.type_ref()?;
    if calculated != reference {
        return Err(TypeCheckError::TypeReferenceIdentityMismatch {
            reference,
            calculated,
        });
    }
    Ok(type_artifact)
}

fn write_reference(encoded: &mut Vec<u8>, reference: ArtifactRef) {
    encoded.extend_from_slice(reference.as_bytes());
}

fn write_type_reference(encoded: &mut Vec<u8>, reference: TypeRef) {
    write_reference(encoded, reference.as_artifact_ref());
}

fn write_text(encoded: &mut Vec<u8>, value: &str) -> Result<(), TypeError> {
    let length = u32::try_from(value.len()).map_err(|_| TypeError::SymbolTooLong(value.len()))?;
    encoded.extend_from_slice(&length.to_be_bytes());
    encoded.extend_from_slice(value.as_bytes());
    Ok(())
}

struct TypeCursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> TypeCursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], TypeError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(TypeError::PayloadLengthOverflow)?;
        let value = self
            .bytes
            .get(self.position..end)
            .ok_or(TypeError::TruncatedPayload)?;
        self.position = end;
        Ok(value)
    }

    fn read_byte(&mut self) -> Result<u8, TypeError> {
        Ok(self.take(1)?[0])
    }

    fn read_u32(&mut self) -> Result<u32, TypeError> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| TypeError::TruncatedPayload)?;
        Ok(u32::from_be_bytes(bytes))
    }

    fn read_reference(&mut self) -> Result<ArtifactRef, TypeError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| TypeError::TruncatedPayload)?;
        Ok(ArtifactRef::from_bytes(bytes))
    }

    fn read_text(&mut self) -> Result<String, TypeError> {
        let length =
            usize::try_from(self.read_u32()?).map_err(|_| TypeError::PayloadLengthOverflow)?;
        let bytes = self.take(length)?;
        String::from_utf8(bytes.to_vec()).map_err(TypeError::InvalidSymbolUtf8)
    }

    const fn is_finished(&self) -> bool {
        self.position == self.bytes.len()
    }

    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

/// Errors from canonical type and typed-form encoding.
#[derive(Debug, Error)]
pub enum TypeError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),

    #[error("type symbol cannot be empty")]
    EmptySymbol,

    #[error("invalid type symbol {0:?}; expected [A-Za-z_][A-Za-z0-9_.-]*")]
    InvalidSymbol(String),

    #[error("type symbol is too long: {0} bytes")]
    SymbolTooLong(usize),

    #[error("type payload is truncated")]
    TruncatedPayload,

    #[error("type payload length overflows this platform")]
    PayloadLengthOverflow,

    #[error("type payload contains {0} trailing bytes")]
    TrailingPayloadBytes(usize),

    #[error("type payload contains an unknown tag {0}")]
    UnknownTypeTag(u8),

    #[error("type symbol bytes are not valid UTF-8")]
    InvalidSymbolUtf8(#[source] std::string::FromUtf8Error),

    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },

    #[error("unsupported type schema version {0}")]
    UnsupportedTypeSchemaVersion(u32),

    #[error("unsupported typed-form schema version {0}")]
    UnsupportedTypedFormSchemaVersion(u32),
}

/// Errors from Phase 1 structural type checking.
#[derive(Debug, Error)]
pub enum TypeCheckError {
    #[error(transparent)]
    Type(#[from] TypeError),

    #[error("type {0} is not available from the declared catalog")]
    UnresolvedType(TypeRef),

    #[error("dependent type family {0} is not available from the declared catalog")]
    UnresolvedTypeFamily(TypeFamilyRef),

    #[error("catalog entry for {reference} hashes to {calculated}, not its claimed identity")]
    TypeReferenceIdentityMismatch {
        reference: TypeRef,
        calculated: TypeRef,
    },

    #[error("recursive type reference encountered at {0}")]
    RecursiveTypeReference(TypeRef),

    #[error("named type belongs to binding {actual}, expected {expected}")]
    NamedBindingMismatch {
        expected: BindingVersionRef,
        actual: BindingVersionRef,
    },

    #[error("child type {reference} belongs to binding {actual}, expected {expected}")]
    ChildBindingMismatch {
        expected: BindingVersionRef,
        actual: BindingVersionRef,
        reference: TypeRef,
    },

    #[error("dependent type family belongs to binding {actual}, expected {expected}")]
    FamilyBindingMismatch {
        expected: BindingVersionRef,
        actual: BindingVersionRef,
    },

    #[error("dependent type family domain is {actual}, expected {expected}")]
    FamilyDomainMismatch { expected: TypeRef, actual: TypeRef },

    #[error("typed form binding {form_binding} does not match type binding {type_binding}")]
    TypedFormBindingMismatch {
        form_binding: BindingVersionRef,
        type_binding: BindingVersionRef,
    },
}
