use crate::{
    ArtifactRef, DistinctionRef, ExactReturnFiber, ReturnFiberError, SeedReorientation,
    SeedReorientationError, SelectedReturn, TaggedExteriorCatalog, TaggedExteriorClaim,
    TaggedExteriorClaimError, TypedFormRef,
};

/// One dependent reciprocal occurrence, from which the sixfold role view is read.
///
/// The fields are the dependency chain, not six slots. Plan section 38 replaced the conception in
/// which `(S_X, O_X, R_X; S_Y, O_Y, R_Y)` are jointly open independent openings, and a flat record
/// of six roles would quietly reinstate it: nothing would stop a `Y` side unrelated to the `X`
/// side from being assembled into a well-formed occurrence. Here the seed carries `u_X`, `S_X`,
/// `O_X` and `S_Y` as one checked unit, and the `Y`-side claim must continue from that `S_Y`.
///
/// A one-sided inquiry is not a value of this type at all, which is the structural reading of
/// "one-way negation does not imply reciprocal negation".
///
/// Section 43 calls this view derived and explicitly not authoritative history, so it carries no
/// canonical identity. It records which roles were generated and how; it does not make any of them
/// actual.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReciprocalOccurrence {
    seed: SeedReorientation,
    x_fiber: ExactReturnFiber,
    x_return: Option<ArtifactRef>,
    y_exterior: TaggedExteriorClaim,
    y_fiber: ExactReturnFiber,
    y_return: Option<ArtifactRef>,
}

/// The result of one section-40 role comparison.
///
/// Section 40 compares roles under protected equivalence, and nothing in this phase evaluates a
/// protected horizon. Only one direction is therefore decidable: identical fillings are
/// protected-equivalent under every horizon. Differing fillings are undecided, never different --
/// treating a failed identity test as a protected difference is the substitution the calculus
/// refuses everywhere else.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoleComparison {
    /// The same form fills both roles, so no horizon can separate them.
    Coincident,
    /// The fillings differ. Whether a protected horizon distinguishes them is unknown here.
    Undecided,
}

fn compare(left: TypedFormRef, right: TypedFormRef) -> RoleComparison {
    if left == right {
        RoleComparison::Coincident
    } else {
        RoleComparison::Undecided
    }
}

impl ReciprocalOccurrence {
    /// Assembles the occurrence from its dependent parts, enforcing selection membership.
    ///
    /// The links between the parts are checked by [`Self::check`], which needs a catalog; this
    /// constructor rejects only what it can decide without one.
    pub fn new(
        seed: SeedReorientation,
        x_fiber: ExactReturnFiber,
        x_return: Option<ArtifactRef>,
        y_exterior: TaggedExteriorClaim,
        y_fiber: ExactReturnFiber,
        y_return: Option<ArtifactRef>,
    ) -> Result<Self, ReturnFiberError> {
        if let Some(selected) = x_return {
            SelectedReturn::select(x_fiber.clone(), selected)?;
        }
        if let Some(selected) = y_return {
            SelectedReturn::select(y_fiber.clone(), selected)?;
        }
        Ok(Self {
            seed,
            x_fiber,
            x_return,
            y_exterior,
            y_fiber,
            y_return,
        })
    }

    /// `S_X`, the source the `X`-oriented determination presented.
    #[must_use]
    pub const fn source_x(&self) -> TypedFormRef {
        self.seed.exterior().source()
    }

    /// `O_X`, the exterior reached under `u_X`.
    #[must_use]
    pub const fn exterior_x(&self) -> TypedFormRef {
        self.seed.exterior_form()
    }

    /// `R_X`, one source selected from the `X` return fiber, when one was selected.
    #[must_use]
    pub const fn selected_return_x(&self) -> Option<ArtifactRef> {
        self.x_return
    }

    /// `S_Y`, the reciprocal source the seed reoriented to.
    #[must_use]
    pub const fn source_y(&self) -> TypedFormRef {
        self.seed.reoriented_source()
    }

    /// `O_Y`, the exterior reached under `u_Y`.
    #[must_use]
    pub const fn exterior_y(&self) -> TypedFormRef {
        self.y_exterior.candidate()
    }

    /// `R_Y`, one source selected from the `Y` return fiber, when one was selected.
    #[must_use]
    pub const fn selected_return_y(&self) -> Option<ArtifactRef> {
        self.y_return
    }

    /// The whole `X` return fiber, which is not the selected return.
    #[must_use]
    pub const fn return_fiber_x(&self) -> &ExactReturnFiber {
        &self.x_fiber
    }

    /// The whole `Y` return fiber, which is not the selected return.
    #[must_use]
    pub const fn return_fiber_y(&self) -> &ExactReturnFiber {
        &self.y_fiber
    }

    /// The seed that carried `O_X` to `S_Y`.
    #[must_use]
    pub const fn seed(&self) -> &SeedReorientation {
        &self.seed
    }

    /// The `Y`-oriented exterior claim.
    #[must_use]
    pub const fn exterior_claim_y(&self) -> TaggedExteriorClaim {
        self.y_exterior
    }

    /// The four section-40 role comparisons, in order:
    /// `S_X ? R_X`, `O_X ? S_Y`, `O_Y ? S_X`, `S_Y ? R_Y`.
    ///
    /// A comparison against an unselected return is `Undecided`: no selection was made, so
    /// nothing has been compared. That is distinct from a selection that failed to coincide, and
    /// both are distinct from a protected difference, which this phase cannot establish.
    #[must_use]
    pub fn residuals(&self) -> [RoleComparison; 4] {
        let against_selection = |role: TypedFormRef, selected: Option<ArtifactRef>| match selected {
            Some(value) if value == role.as_artifact_ref() => RoleComparison::Coincident,
            _ => RoleComparison::Undecided,
        };
        [
            against_selection(self.source_x(), self.x_return),
            compare(self.exterior_x(), self.source_y()),
            compare(self.exterior_y(), self.source_x()),
            against_selection(self.source_y(), self.y_return),
        ]
    }

    /// Reports whether the downstream compatibility check may be reached at all.
    ///
    /// Plan section 41 puts `Gamma_D` last and forbids it from manufacturing role fillings, so
    /// this refuses while a role is missing rather than supplying one. Reaching `Gamma_D` is not
    /// passing it: no compatibility relation is evaluated here, and none is accepted.
    pub fn gamma_reachable(&self) -> Result<(), GammaError> {
        if self.x_return.is_none() {
            return Err(GammaError::RoleMissing("R_X"));
        }
        if self.y_return.is_none() {
            return Err(GammaError::RoleMissing("R_Y"));
        }
        Ok(())
    }

    /// Rechecks the whole dependent chain and the links between its parts.
    ///
    /// Establishes that the parts were generated in order and refer to one another: the `Y` side
    /// continues from the seed's reoriented source, each return fiber is the reverse section of
    /// its own use taken at its own exterior, and both orientations belong to one distinction. It
    /// admits nothing, evaluates no relation, and makes no role actual.
    pub fn check<C: TaggedExteriorCatalog>(
        &self,
        catalog: &C,
    ) -> Result<(), ReciprocalOccurrenceError> {
        self.seed
            .check(catalog)
            .map_err(|error| ReciprocalOccurrenceError::Seed(Box::new(error)))?;
        self.y_exterior
            .check(catalog)
            .map_err(|error| ReciprocalOccurrenceError::Exterior(Box::new(error)))?;

        // The joint that makes the occurrence dependent rather than six openings.
        if self.y_exterior.source() != self.seed.reoriented_source() {
            return Err(
                ReciprocalOccurrenceError::ReciprocalSourceIsNotTheSeededSource {
                    seeded: self.seed.reoriented_source(),
                    reciprocal: self.y_exterior.source(),
                },
            );
        }

        // Each return is the reverse section of the use that produced its exterior, taken at that
        // exterior. A fiber from another use or another exterior is a different return entirely.
        let x_use = self.seed.exterior().negation_use();
        if self.x_fiber.use_ref() != x_use {
            return Err(ReciprocalOccurrenceError::ReturnFiberUseMismatch("X"));
        }
        if self.x_fiber.exterior() != self.exterior_x().as_artifact_ref() {
            return Err(ReciprocalOccurrenceError::ReturnFiberExteriorMismatch("X"));
        }
        if self.y_fiber.use_ref() != self.y_exterior.negation_use() {
            return Err(ReciprocalOccurrenceError::ReturnFiberUseMismatch("Y"));
        }
        if self.y_fiber.exterior() != self.exterior_y().as_artifact_ref() {
            return Err(ReciprocalOccurrenceError::ReturnFiberExteriorMismatch("Y"));
        }

        // Two orientations of one distinction, not two unrelated inquiries.
        let x_negation = catalog
            .resolve_negation_use(x_use)
            .ok_or(ReciprocalOccurrenceError::UnresolvedNegationUse("X"))?;
        let y_negation = catalog
            .resolve_negation_use(self.y_exterior.negation_use())
            .ok_or(ReciprocalOccurrenceError::UnresolvedNegationUse("Y"))?;
        if x_negation.distinction() != y_negation.distinction() {
            return Err(ReciprocalOccurrenceError::DistinctionMismatch {
                x: x_negation.distinction(),
                y: y_negation.distinction(),
            });
        }

        Ok(())
    }
}

/// Why the downstream compatibility check cannot be reached.
#[derive(Debug, thiserror::Error)]
pub enum GammaError {
    #[error("role {0} has no filling; Gamma is downstream and may not supply one")]
    RoleMissing(&'static str),
}

/// Errors from checking a dependent reciprocal occurrence.
///
/// The two delegated errors are boxed: both carry the whole chain they came from, and inlining
/// them would widen every `Result` in this module to the size of its largest failure.
#[derive(Debug, thiserror::Error)]
pub enum ReciprocalOccurrenceError {
    #[error(transparent)]
    Seed(Box<SeedReorientationError>),
    #[error(transparent)]
    Exterior(Box<TaggedExteriorClaimError>),
    #[error("the reciprocal source {reciprocal} does not continue from the seeded source {seeded}")]
    ReciprocalSourceIsNotTheSeededSource {
        seeded: TypedFormRef,
        reciprocal: TypedFormRef,
    },
    #[error("the {0}-side return fiber belongs to a different negation use")]
    ReturnFiberUseMismatch(&'static str),
    #[error("the {0}-side return fiber was taken at a different exterior")]
    ReturnFiberExteriorMismatch(&'static str),
    #[error("the {0}-side negation use is not available from the declared catalog")]
    UnresolvedNegationUse(&'static str),
    #[error("the two orientations belong to different distinctions: {x} and {y}")]
    DistinctionMismatch {
        x: DistinctionRef,
        y: DistinctionRef,
    },
}
