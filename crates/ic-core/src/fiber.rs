use std::collections::BTreeSet;

use crate::{
    ExactFiberRecovery, ExactFiberRecoveryError, ExactFiniteSignature, NegationUseRef,
    RecoverySeparator, TaggedExteriorCatalog, TaggedExteriorClaim, TypedFormRef,
};

/// A caller-declared finite extension of one oriented negation relation.
///
/// The pairs are `(source, candidate)` incidences of `N_u`. Declaring them establishes nothing:
/// the caller asserts this extension, and no admission, soundness, coverage, or departure
/// evidence follows from it. The use tag is retained because the reverse section is
/// use-specific -- two uses reaching the same exterior may return different sources.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteNegationExtension {
    use_ref: NegationUseRef,
    incidences: BTreeSet<(crate::ArtifactRef, crate::ArtifactRef)>,
}

impl FiniteNegationExtension {
    /// Creates one finite declared extension, rejecting a duplicate incidence.
    pub fn new(
        use_ref: NegationUseRef,
        incidences: Vec<(crate::ArtifactRef, crate::ArtifactRef)>,
    ) -> Result<Self, ReturnFiberError> {
        let mut unique = BTreeSet::new();
        for incidence in incidences {
            if !unique.insert(incidence) {
                return Err(ReturnFiberError::DuplicateIncidence(incidence));
            }
        }
        Ok(Self {
            use_ref,
            incidences: unique,
        })
    }

    /// Returns the negation-use tag this extension belongs to.
    #[must_use]
    pub const fn use_ref(&self) -> NegationUseRef {
        self.use_ref
    }

    /// Returns the declared `(source, candidate)` incidences.
    #[must_use]
    pub const fn incidences(&self) -> &BTreeSet<(crate::ArtifactRef, crate::ArtifactRef)> {
        &self.incidences
    }
}

/// The exact reverse section `N_u^{-1}[e]` for one exterior under one use.
///
/// This is the whole fiber, never a selected return. It carries its use tag so that a later
/// consumer cannot merge two uses' returns into an untagged union.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactReturnFiber {
    use_ref: NegationUseRef,
    exterior: crate::ArtifactRef,
    sources: BTreeSet<crate::ArtifactRef>,
}

impl ExactReturnFiber {
    /// Returns the use whose reverse section this is.
    #[must_use]
    pub const fn use_ref(&self) -> NegationUseRef {
        self.use_ref
    }

    /// Returns the exterior the section was taken at.
    #[must_use]
    pub const fn exterior(&self) -> crate::ArtifactRef {
        self.exterior
    }

    /// Returns every source still possible through this return.
    #[must_use]
    pub const fn sources(&self) -> &BTreeSet<crate::ArtifactRef> {
        &self.sources
    }

    /// Reports whether a source survives this return.
    #[must_use]
    pub fn contains(&self, source: crate::ArtifactRef) -> bool {
        self.sources.contains(&source)
    }
}

/// Computes the exact finite reverse section of one declared negation extension.
///
/// This is the preimage `{s : N_u(s, e)}`, not the image `N_u[s]`; the two are different
/// relations and are not interchangeable for an asymmetric extension.
///
/// An exterior with no declared incidence is refused rather than returned as an empty fiber:
/// a *return* fiber presupposes the incidence it returns through. The refusal says only that
/// this extension declares no incidence at that candidate. It does not say the candidate is
/// interior, unrelated, or unreachable under some other use -- that would turn a missing
/// declaration into a negative result.
pub fn exact_return_fiber(
    extension: &FiniteNegationExtension,
    exterior: crate::ArtifactRef,
) -> Result<ExactReturnFiber, ReturnFiberError> {
    let sources: BTreeSet<_> = extension
        .incidences()
        .iter()
        .filter(|(_, candidate)| *candidate == exterior)
        .map(|(source, _)| *source)
        .collect();

    if sources.is_empty() {
        return Err(ReturnFiberError::ExteriorHasNoDeclaredIncidence(exterior));
    }

    Ok(ExactReturnFiber {
        use_ref: extension.use_ref(),
        exterior,
        sources,
    })
}

/// Checks protected recovery over a derived fiber rather than an unconnected table.
///
/// [`crate::check_exact_fiber_recovery`] accepts any signature table and rests on the caller's
/// word that it is the fiber of an admitted incidence. This entry point removes that word: the
/// signature domain must be exactly the derived fiber, so a table that omits the source or
/// describes some other set is rejected instead of reported as recovery.
///
/// It still establishes only signature constancy over the declared extension. It does not admit
/// the extension, evaluate the relation, or make the incidence actual.
pub fn check_fiber_recovery(
    fiber: &ExactReturnFiber,
    protected_signatures: &ExactFiniteSignature,
) -> Result<ExactFiberRecovery, FiberRecoveryError> {
    let signature_domain: BTreeSet<_> = protected_signatures.values().keys().copied().collect();
    if &signature_domain != fiber.sources() {
        return Err(FiberRecoveryError::SignatureDomainIsNotTheFiber {
            fiber: fiber.sources().clone(),
            signatures: signature_domain,
        });
    }
    Ok(crate::check_exact_fiber_recovery(protected_signatures)?)
}

/// `R_X`: one supported source selected *from* a return fiber.
///
/// Plan section 28 keeps the selected filling and the fiber apart. Holding a selection is holding
/// one source that survives the return; it is not the set of sources that survive it, and the
/// difference is the whole content of exact return stability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedReturn {
    fiber: ExactReturnFiber,
    selected: crate::ArtifactRef,
}

impl SelectedReturn {
    /// Selects one source from a derived fiber, refusing anything the return does not admit.
    ///
    /// A selection from outside the fiber is not a return role at all: the fiber is exactly the
    /// set of sources the incidence returns to.
    pub fn select(
        fiber: ExactReturnFiber,
        selected: crate::ArtifactRef,
    ) -> Result<Self, ReturnFiberError> {
        if !fiber.contains(selected) {
            return Err(ReturnFiberError::SelectionOutsideFiber {
                exterior: fiber.exterior(),
                selected,
            });
        }
        Ok(Self { fiber, selected })
    }

    /// Returns the whole fiber this selection was drawn from.
    #[must_use]
    pub const fn fiber(&self) -> &ExactReturnFiber {
        &self.fiber
    }

    /// Returns the selected source.
    #[must_use]
    pub const fn selected(&self) -> crate::ArtifactRef {
        self.selected
    }
}

/// Whether a return closes on one protected source class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReturnClosure {
    /// Every source the return admits shares this protected signature.
    Closed {
        protected_signature: crate::ArtifactRef,
    },
    /// Another protected class survives the return, named concretely.
    Open { separator: RecoverySeparator },
}

/// Decides exact return stability from the whole fiber, never from the selection.
///
/// The selection is present to name the occurrence, not to support the verdict. Plan section 28
/// is explicit that one observed `R_X` agreeing with `S_X` is insufficient while another
/// protected class remains in the fiber, so this reads every surviving source and reports a
/// concrete separator when one disagrees.
///
/// Establishes signature constancy over the declared extension and nothing further: it does not
/// admit the extension, evaluate the relation, or make the return actual.
pub fn check_return_closure(
    selection: &SelectedReturn,
    protected_signatures: &ExactFiniteSignature,
) -> Result<ReturnClosure, FiberRecoveryError> {
    Ok(
        match check_fiber_recovery(selection.fiber(), protected_signatures)? {
            ExactFiberRecovery::Recovered {
                protected_signature,
            } => ReturnClosure::Closed {
                protected_signature,
            },
            ExactFiberRecovery::NotRecovered { separator } => ReturnClosure::Open { separator },
        },
    )
}

/// Errors from declaring a finite negation extension or taking its reverse section.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReturnFiberError {
    /// The same `(source, candidate)` incidence was declared twice.
    DuplicateIncidence((crate::ArtifactRef, crate::ArtifactRef)),
    /// No declared incidence reaches this candidate, so it has no return to take.
    ExteriorHasNoDeclaredIncidence(crate::ArtifactRef),
    /// A selection was made from outside the set of sources the return admits.
    SelectionOutsideFiber {
        exterior: crate::ArtifactRef,
        selected: crate::ArtifactRef,
    },
}

/// Errors from checking recovery against a derived fiber.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FiberRecoveryError {
    /// The protected-signature table does not cover exactly the derived fiber.
    SignatureDomainIsNotTheFiber {
        fiber: BTreeSet<crate::ArtifactRef>,
        signatures: BTreeSet<crate::ArtifactRef>,
    },
    /// The underlying exact recovery check refused the table.
    Recovery(ExactFiberRecoveryError),
}

impl From<ExactFiberRecoveryError> for FiberRecoveryError {
    fn from(error: ExactFiberRecoveryError) -> Self {
        Self::Recovery(error)
    }
}

/// A finite negation extension whose incidences are typed forms, checked against the relation.
///
/// [`FiniteNegationExtension`] takes bare references and a use tag nothing resolves, so it rests
/// entirely on the caller's word. This companion removes that word the way the typed
/// incompatibility table removed it from its untyped predecessor: the use is resolved and
/// rehashed, its relation is read, and every declared pair is type-checked against the ports it
/// claims to fill.
///
/// Declaring an extension still declares. Plan section 77 makes a deterministic finite list the
/// first lawful realisation of a negation frontier, and this is that list: it is not an admitted
/// relation, not evidence of soundness or coverage, and not an evaluation of anything.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedFiniteNegationExtension {
    negation_use: NegationUseRef,
    incidences: BTreeSet<(TypedFormRef, TypedFormRef)>,
}

impl TypedFiniteNegationExtension {
    /// Declares one finite typed extension, rejecting a duplicated incidence.
    pub fn declare(
        negation_use: NegationUseRef,
        incidences: Vec<(TypedFormRef, TypedFormRef)>,
    ) -> Result<Self, ReturnFiberError> {
        let mut unique = BTreeSet::new();
        for (source, candidate) in incidences {
            if !unique.insert((source, candidate)) {
                return Err(ReturnFiberError::DuplicateIncidence((
                    source.as_artifact_ref(),
                    candidate.as_artifact_ref(),
                )));
            }
        }
        Ok(Self {
            negation_use,
            incidences: unique,
        })
    }

    /// Returns the use this extension claims to extend.
    #[must_use]
    pub const fn negation_use(&self) -> NegationUseRef {
        self.negation_use
    }

    /// Returns the declared typed incidences.
    #[must_use]
    pub const fn incidences(&self) -> &BTreeSet<(TypedFormRef, TypedFormRef)> {
        &self.incidences
    }

    /// Resolves the use and type-checks every declared incidence against its relation.
    ///
    /// The source port is the one the use's own relation use binds, and the candidate port is the
    /// one it leaves open, matching how the positive-negation question is built. Requiring exactly
    /// one open port keeps a source-and-candidate relation from being confused with a relation of
    /// some other arity.
    ///
    /// This checks that the declared pairs could fill the relation. It does not evaluate the
    /// relation, admit the extension as its denotation, or establish soundness or coverage.
    pub fn check<C: TaggedExteriorCatalog>(
        &self,
        catalog: &C,
    ) -> Result<(), TypedNegationExtensionError> {
        let negation_use = catalog.resolve_negation_use(self.negation_use).ok_or(
            TypedNegationExtensionError::UnresolvedNegationUse(self.negation_use),
        )?;
        let calculated = negation_use
            .negation_use_ref()
            .map_err(|error| TypedNegationExtensionError::NegationUseEncoding(Box::new(error)))?;
        if calculated != self.negation_use {
            return Err(TypedNegationExtensionError::NegationUseIdentityMismatch {
                reference: self.negation_use,
                calculated,
            });
        }

        let relation_use = catalog
            .resolve_relation_use(negation_use.relation_use())
            .ok_or(TypedNegationExtensionError::UnresolvedRelationUse)?;
        let schema = catalog
            .resolve_relation_schema(relation_use.relation())
            .ok_or(TypedNegationExtensionError::UnresolvedRelationSchema)?;

        let mut bound = Vec::new();
        let mut open = Vec::new();
        for port in schema.ports() {
            if relation_use
                .bindings()
                .iter()
                .any(|binding| binding.port() == port.name())
            {
                bound.push(port);
            } else {
                open.push(port);
            }
        }
        let (Some(source_port), 1, 1) = (bound.first().copied(), bound.len(), open.len()) else {
            return Err(
                TypedNegationExtensionError::NotOneSourceAndOneCandidatePort {
                    bound: bound.len(),
                    open: open.len(),
                },
            );
        };
        let candidate_port = open[0];

        for (source, candidate) in &self.incidences {
            for (form, port, role) in [
                (*source, source_port, "source"),
                (*candidate, candidate_port, "candidate"),
            ] {
                let resolved = catalog
                    .resolve_typed_form(form)
                    .ok_or(TypedNegationExtensionError::UnresolvedForm(form))?;
                let calculated = resolved
                    .typed_form_ref()
                    .map_err(|error| TypedNegationExtensionError::FormEncoding(Box::new(error)))?;
                if calculated != form {
                    return Err(TypedNegationExtensionError::FormIdentityMismatch {
                        reference: form,
                        calculated,
                    });
                }
                if resolved.ty() != port.ty() {
                    return Err(TypedNegationExtensionError::IncidenceTypeMismatch {
                        role,
                        form,
                        expected: port.ty(),
                        actual: resolved.ty(),
                    });
                }
            }
        }
        Ok(())
    }

    /// Drops the typing so the existing untyped section machinery applies unchanged.
    pub fn erase(&self) -> Result<FiniteNegationExtension, ReturnFiberError> {
        FiniteNegationExtension::new(
            self.negation_use,
            self.incidences
                .iter()
                .map(|(source, candidate)| (source.as_artifact_ref(), candidate.as_artifact_ref()))
                .collect(),
        )
    }

    /// The forward section `N_u[s]`: every candidate the extension relates to this source.
    ///
    /// Plan section 26 makes this the candidate field a positive-negation question ranges over.
    /// Membership is candidacy under a declared extension and nothing more: a candidate here is
    /// not an exterior, and becomes one only through its own departure witness. A generated `O_X`
    /// is not an actualised `O_X`.
    ///
    /// This is the image, not the preimage. For an asymmetric extension the two differ, and
    /// `exact_return_fiber` on the erased extension supplies the other direction.
    #[must_use]
    pub fn negation_field(&self, source: TypedFormRef) -> ExactNegationField {
        ExactNegationField {
            negation_use: self.negation_use,
            source,
            candidates: self
                .incidences
                .iter()
                .filter(|(declared, _)| *declared == source)
                .map(|(_, candidate)| *candidate)
                .collect(),
        }
    }
}

/// The candidate field of one source under one use.
///
/// An empty field says the declared extension relates no candidate to this source. It does not
/// say the source has no exterior: an unsearched field and an exactly exhausted one are different
/// claims, and this carries neither.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactNegationField {
    negation_use: NegationUseRef,
    source: TypedFormRef,
    candidates: BTreeSet<TypedFormRef>,
}

impl ExactNegationField {
    /// Returns the use whose field this is.
    #[must_use]
    pub const fn negation_use(&self) -> NegationUseRef {
        self.negation_use
    }

    /// Returns the source the field was taken at.
    #[must_use]
    pub const fn source(&self) -> TypedFormRef {
        self.source
    }

    /// Returns every candidate the declared extension relates to the source.
    #[must_use]
    pub const fn candidates(&self) -> &BTreeSet<TypedFormRef> {
        &self.candidates
    }

    /// Reports whether the extension relates this candidate to the source.
    #[must_use]
    pub fn contains(&self, candidate: TypedFormRef) -> bool {
        self.candidates.contains(&candidate)
    }
}

/// Errors from checking a typed finite negation extension.
#[derive(Debug, thiserror::Error)]
pub enum TypedNegationExtensionError {
    #[error(transparent)]
    NegationUseEncoding(Box<crate::NegationUseError>),
    #[error(transparent)]
    FormEncoding(Box<crate::TypeError>),
    #[error("negation use {0} is not available from the declared catalog")]
    UnresolvedNegationUse(NegationUseRef),
    #[error("catalog negation use {reference} hashes to {calculated}, not its claimed identity")]
    NegationUseIdentityMismatch {
        reference: NegationUseRef,
        calculated: NegationUseRef,
    },
    #[error("the negation use's relation use is not available from the declared catalog")]
    UnresolvedRelationUse,
    #[error("the negation relation schema is not available from the declared catalog")]
    UnresolvedRelationSchema,
    #[error(
        "a negation relation needs one bound source port and one open candidate port, found {bound} bound and {open} open"
    )]
    NotOneSourceAndOneCandidatePort { bound: usize, open: usize },
    #[error("declared form {0} is not available from the declared catalog")]
    UnresolvedForm(TypedFormRef),
    #[error("catalog form {reference} hashes to {calculated}, not its claimed identity")]
    FormIdentityMismatch {
        reference: TypedFormRef,
        calculated: TypedFormRef,
    },
    #[error("declared {role} {form} has type {actual}, but its port has type {expected}")]
    IncidenceTypeMismatch {
        role: &'static str,
        form: TypedFormRef,
        expected: crate::TypeRef,
        actual: crate::TypeRef,
    },
}

/// Checks that a tagged exterior claim names an incidence its use actually declares.
///
/// The specification defines the role directly: `PosNeg(u; s, e, w)` records `e` in
/// `NegField_u(s)`. Until a field could be derived, [`TaggedExteriorClaim`] had to omit that
/// membership and said so; this supplies it against a declared finite extension. A claim whose
/// candidate the relation never relates to its source is now refused rather than merely
/// well-formed.
///
/// What this establishes is exactly one thing: the extension declares the incidence. It does not
/// admit the extension as the relation's denotation, admit the use, or establish that the
/// candidate is genuinely exterior -- the departure witness carries that burden, and this does
/// not relieve it. Both the claim and the extension are still checked in full on the way through.
pub fn check_declared_incidence<C: TaggedExteriorCatalog>(
    claim: &TaggedExteriorClaim,
    extension: &TypedFiniteNegationExtension,
    catalog: &C,
) -> Result<(), DeclaredIncidenceError> {
    if extension.negation_use() != claim.negation_use() {
        return Err(DeclaredIncidenceError::ExtensionIsForAnotherUse {
            claim: claim.negation_use(),
            extension: extension.negation_use(),
        });
    }
    claim
        .check(catalog)
        .map_err(|error| DeclaredIncidenceError::Claim(Box::new(error)))?;
    extension
        .check(catalog)
        .map_err(|error| DeclaredIncidenceError::Extension(Box::new(error)))?;

    // The pair, not the candidate alone. A candidate the relation relates to some other source
    // says nothing about this one.
    if !extension
        .negation_field(claim.source())
        .contains(claim.candidate())
    {
        return Err(DeclaredIncidenceError::IncidenceNotDeclared {
            declared_source: claim.source(),
            candidate: claim.candidate(),
        });
    }
    Ok(())
}

/// Errors from checking a claimed incidence against a declared extension.
#[derive(Debug, thiserror::Error)]
pub enum DeclaredIncidenceError {
    #[error(transparent)]
    Claim(Box<crate::TaggedExteriorClaimError>),
    #[error(transparent)]
    Extension(Box<TypedNegationExtensionError>),
    #[error("the extension declares use {extension}, but the claim is tagged {claim}")]
    ExtensionIsForAnotherUse {
        claim: NegationUseRef,
        extension: NegationUseRef,
    },
    #[error("the extension does not relate candidate {candidate} to source {declared_source}")]
    IncidenceNotDeclared {
        declared_source: TypedFormRef,
        candidate: TypedFormRef,
    },
}
