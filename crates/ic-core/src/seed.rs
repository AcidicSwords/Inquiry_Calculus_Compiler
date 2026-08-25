use crate::{
    DischargeMode, NegationUseCheckError, RelationUseCheckError, RelationUseError, RelationUseRef,
    TaggedExteriorCatalog, TaggedExteriorClaim, TaggedExteriorClaimError, TypedFormRef,
    departure::relation_use_binds_pair,
};

/// The `Y`-oriented seed `Seed_Y(O_X, S_Y)` taken at one tagged exterior.
///
/// The exterior arrives as a [`TaggedExteriorClaim`] rather than a bare form, because plan
/// section 38 keeps both negation-use tags in the occurrence provenance: a reorientation that
/// forgets which use produced `O_X` cannot say which relation the reciprocal side must return
/// through.
///
/// `exterior_form` and `reoriented_source` are separate fields and stay separate when they hold
/// the same form. Plan section 37 permits `S_Y = O_X` when the representation is already
/// appropriate, and in the same breath forbids reading that as role collapse: reorientation is an
/// inquiry transformation, not proof of reciprocal symmetry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SeedReorientation {
    exterior: TaggedExteriorClaim,
    seed_use: RelationUseRef,
    reoriented_source: TypedFormRef,
}

impl SeedReorientation {
    /// Declares one seed reorientation. Declaring establishes nothing until it is checked.
    #[must_use]
    pub const fn new(
        exterior: TaggedExteriorClaim,
        seed_use: RelationUseRef,
        reoriented_source: TypedFormRef,
    ) -> Self {
        Self {
            exterior,
            seed_use,
            reoriented_source,
        }
    }

    /// Returns the tagged exterior claim this seed was taken at.
    #[must_use]
    pub const fn exterior(&self) -> TaggedExteriorClaim {
        self.exterior
    }

    /// Returns `O_X`, the exterior form occupying the `X`-side role.
    #[must_use]
    pub const fn exterior_form(&self) -> TypedFormRef {
        self.exterior.candidate()
    }

    /// Returns the seed relation use `sigma_{X->Y}`.
    #[must_use]
    pub const fn seed_use(&self) -> RelationUseRef {
        self.seed_use
    }

    /// Returns `S_Y`, the form occupying the reciprocal source role.
    #[must_use]
    pub const fn reoriented_source(&self) -> TypedFormRef {
        self.reoriented_source
    }

    /// Reports that the seed carried the same form across the reorientation.
    ///
    /// This reports a coincidence of fillings; it does not merge the roles. Both remain
    /// separately retrievable and the seed relation use remains required.
    #[must_use]
    pub fn is_identity_seed(&self) -> bool {
        self.exterior_form() == self.reoriented_source
    }

    /// Checks the declared reorientation without performing it.
    ///
    /// Establishes only that the exterior claim survives its own check, that the named seed use
    /// exists and relates exactly this `(O_X, S_Y)` pair, that its declared route is not merely
    /// generative, and that it shares the exterior's indexed context. It does not evaluate the
    /// seed relation, select `S_Y`, admit the reorientation, or make the reciprocal side actual.
    pub fn check<C: TaggedExteriorCatalog>(
        &self,
        catalog: &C,
    ) -> Result<(), SeedReorientationError> {
        self.exterior.check(catalog)?;

        let negation_use = catalog
            .resolve_negation_use(self.exterior.negation_use())
            .ok_or(SeedReorientationError::UnresolvedNegationUse)?;

        let seed_use = catalog
            .resolve_relation_use(self.seed_use)
            .ok_or(SeedReorientationError::UnresolvedSeedUse(self.seed_use))?;
        let calculated = seed_use.relation_use_ref()?;
        if calculated != self.seed_use {
            return Err(SeedReorientationError::SeedUseIdentityMismatch {
                reference: self.seed_use,
                calculated,
            });
        }
        seed_use.check(catalog)?;

        // A generator proposes a filling; the seed relation must be supported, so the same rule
        // that keeps generated evidence out of a departure witness keeps it out of a seed.
        if seed_use.mode() == DischargeMode::Generate {
            return Err(SeedReorientationError::GeneratedSeedRoute(self.seed_use));
        }

        // The seed must relate exactly this pair. When the same form fills both roles the helper
        // requires two bindings, so an identity seed still has to say which occurrence is the
        // exterior and which is the reciprocal source.
        if !relation_use_binds_pair(&seed_use, self.exterior_form(), self.reoriented_source) {
            return Err(SeedReorientationError::SeedDoesNotRelateThePair {
                seed_use: self.seed_use,
                exterior: self.exterior_form(),
                reoriented_source: self.reoriented_source,
            });
        }

        if seed_use.scope() != negation_use.scope()
            || seed_use.applicability() != negation_use.applicability()
            || seed_use.grain() != negation_use.grain()
            || seed_use.horizon() != negation_use.horizon()
        {
            return Err(SeedReorientationError::SeedContextMismatch(self.seed_use));
        }

        Ok(())
    }
}

/// Errors from checking a declared seed reorientation.
#[derive(Debug, thiserror::Error)]
pub enum SeedReorientationError {
    #[error(transparent)]
    Exterior(#[from] TaggedExteriorClaimError),
    #[error(transparent)]
    NegationUse(#[from] NegationUseCheckError),
    #[error(transparent)]
    RelationUse(#[from] RelationUseCheckError),
    #[error(transparent)]
    RelationUseEncoding(#[from] RelationUseError),
    #[error("the tagged exterior's negation use is not available from the declared catalog")]
    UnresolvedNegationUse,
    #[error("seed relation use {0} is not available from the declared catalog")]
    UnresolvedSeedUse(RelationUseRef),
    #[error("catalog seed use {reference} hashes to {calculated}, not its claimed identity")]
    SeedUseIdentityMismatch {
        reference: RelationUseRef,
        calculated: RelationUseRef,
    },
    #[error("seed relation use {0} declares Generate, which proposes rather than supports")]
    GeneratedSeedRoute(RelationUseRef),
    #[error(
        "seed relation use {seed_use} does not relate exterior {exterior} to reoriented source {reoriented_source}"
    )]
    SeedDoesNotRelateThePair {
        seed_use: RelationUseRef,
        exterior: TypedFormRef,
        reoriented_source: TypedFormRef,
    },
    #[error("seed relation use {0} does not share the negation use's indexed context")]
    SeedContextMismatch(RelationUseRef),
}
