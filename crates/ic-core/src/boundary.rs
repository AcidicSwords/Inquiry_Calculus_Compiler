//! Derived reciprocal-boundary chart identity.
//!
//! A [`BoundaryChart`] preserves the declared local chart inputs from the implementation plan.
//! It is not a global partition, an exterior generator, a negation admission, a return-fiber
//! constructor, or a completed sixfold occurrence.  Missing later roles remain missing.

use thiserror::Error;

use crate::{
    ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, BoundaryRef,
    DeterminationPresentationRef, FormulaRef, GrainRef, HorizonRef, NegationUseRef, QueryRef,
    RelationRef, RelationUseRef, TypeRef,
};

/// Canonical artifact kind for a derived local reciprocal-boundary chart.
pub const BOUNDARY_CHART_ARTIFACT_KIND: &str = "ic.boundary-chart";
/// Payload schema version for derived local reciprocal-boundary charts.
pub const BOUNDARY_CHART_SCHEMA_VERSION: u32 = 1;

/// A derived local chart whose missing roles are deliberately not filled by inference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundaryChart {
    query: QueryRef,
    x_type: TypeRef,
    y_type: TypeRef,
    boundary_type: TypeRef,
    pi_x: RelationRef,
    pi_y: RelationRef,
    x_determination: DeterminationPresentationRef,
    y_determination: Option<DeterminationPresentationRef>,
    negation_frontier_x: Vec<NegationUseRef>,
    negation_frontier_y: Vec<NegationUseRef>,
    seed_y: RelationUseRef,
    compatibility: FormulaRef,
    traversal: Option<RelationRef>,
    grain: GrainRef,
    horizon: HorizonRef,
}

impl BoundaryChart {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        query: QueryRef,
        x_type: TypeRef,
        y_type: TypeRef,
        boundary_type: TypeRef,
        pi_x: RelationRef,
        pi_y: RelationRef,
        x_determination: DeterminationPresentationRef,
        y_determination: Option<DeterminationPresentationRef>,
        negation_frontier_x: Vec<NegationUseRef>,
        negation_frontier_y: Vec<NegationUseRef>,
        seed_y: RelationUseRef,
        compatibility: FormulaRef,
        traversal: Option<RelationRef>,
        grain: GrainRef,
        horizon: HorizonRef,
    ) -> Self {
        Self {
            query,
            x_type,
            y_type,
            boundary_type,
            pi_x,
            pi_y,
            x_determination,
            y_determination,
            negation_frontier_x,
            negation_frontier_y,
            seed_y,
            compatibility,
            traversal,
            grain,
            horizon,
        }
    }

    #[must_use]
    pub const fn query(&self) -> QueryRef {
        self.query
    }
    #[must_use]
    pub const fn x_type(&self) -> TypeRef {
        self.x_type
    }
    #[must_use]
    pub const fn y_type(&self) -> TypeRef {
        self.y_type
    }
    #[must_use]
    pub const fn boundary_type(&self) -> TypeRef {
        self.boundary_type
    }
    #[must_use]
    pub const fn pi_x(&self) -> RelationRef {
        self.pi_x
    }
    #[must_use]
    pub const fn pi_y(&self) -> RelationRef {
        self.pi_y
    }
    #[must_use]
    pub const fn x_determination(&self) -> DeterminationPresentationRef {
        self.x_determination
    }
    #[must_use]
    pub const fn y_determination(&self) -> Option<DeterminationPresentationRef> {
        self.y_determination
    }
    #[must_use]
    pub fn negation_frontier_x(&self) -> &[NegationUseRef] {
        &self.negation_frontier_x
    }
    #[must_use]
    pub fn negation_frontier_y(&self) -> &[NegationUseRef] {
        &self.negation_frontier_y
    }
    #[must_use]
    pub const fn seed_y(&self) -> RelationUseRef {
        self.seed_y
    }
    #[must_use]
    pub const fn compatibility(&self) -> FormulaRef {
        self.compatibility
    }
    #[must_use]
    pub const fn traversal(&self) -> Option<RelationRef> {
        self.traversal
    }
    #[must_use]
    pub const fn grain(&self) -> GrainRef {
        self.grain
    }
    #[must_use]
    pub const fn horizon(&self) -> HorizonRef {
        self.horizon
    }

    pub fn canonical_payload(&self) -> Result<Vec<u8>, BoundaryChartError> {
        let mut encoded = Vec::new();
        for value in [
            self.query.as_artifact_ref(),
            self.x_type.as_artifact_ref(),
            self.y_type.as_artifact_ref(),
            self.boundary_type.as_artifact_ref(),
            self.pi_x.as_artifact_ref(),
            self.pi_y.as_artifact_ref(),
            self.x_determination.as_artifact_ref(),
        ] {
            reference(&mut encoded, value);
        }
        optional_reference(
            &mut encoded,
            self.y_determination
                .map(DeterminationPresentationRef::as_artifact_ref),
        );
        references(
            &mut encoded,
            self.negation_frontier_x
                .iter()
                .map(|value| value.as_artifact_ref()),
        )?;
        references(
            &mut encoded,
            self.negation_frontier_y
                .iter()
                .map(|value| value.as_artifact_ref()),
        )?;
        for value in [
            self.seed_y.as_artifact_ref(),
            self.compatibility.as_artifact_ref(),
        ] {
            reference(&mut encoded, value);
        }
        optional_reference(
            &mut encoded,
            self.traversal.map(RelationRef::as_artifact_ref),
        );
        reference(&mut encoded, self.grain.as_artifact_ref());
        reference(&mut encoded, self.horizon.as_artifact_ref());
        Ok(encoded)
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, BoundaryChartError> {
        let mut cursor = Cursor::new(payload);
        let query = QueryRef::from_artifact_ref(cursor.reference()?);
        let x_type = TypeRef::from_artifact_ref(cursor.reference()?);
        let y_type = TypeRef::from_artifact_ref(cursor.reference()?);
        let boundary_type = TypeRef::from_artifact_ref(cursor.reference()?);
        let pi_x = RelationRef::from_artifact_ref(cursor.reference()?);
        let pi_y = RelationRef::from_artifact_ref(cursor.reference()?);
        let x_determination = DeterminationPresentationRef::from_artifact_ref(cursor.reference()?);
        let y_determination = cursor
            .optional_reference()?
            .map(DeterminationPresentationRef::from_artifact_ref);
        let negation_frontier_x = cursor
            .references()?
            .into_iter()
            .map(NegationUseRef::from_artifact_ref)
            .collect();
        let negation_frontier_y = cursor
            .references()?
            .into_iter()
            .map(NegationUseRef::from_artifact_ref)
            .collect();
        let seed_y = RelationUseRef::from_artifact_ref(cursor.reference()?);
        let compatibility = FormulaRef::from_artifact_ref(cursor.reference()?);
        let traversal = cursor
            .optional_reference()?
            .map(RelationRef::from_artifact_ref);
        let grain = GrainRef::from_artifact_ref(cursor.reference()?);
        let horizon = HorizonRef::from_artifact_ref(cursor.reference()?);
        if !cursor.finished() {
            return Err(BoundaryChartError::TrailingPayloadBytes(cursor.remaining()));
        }
        Ok(Self::new(
            query,
            x_type,
            y_type,
            boundary_type,
            pi_x,
            pi_y,
            x_determination,
            y_determination,
            negation_frontier_x,
            negation_frontier_y,
            seed_y,
            compatibility,
            traversal,
            grain,
            horizon,
        ))
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, BoundaryChartError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(BOUNDARY_CHART_ARTIFACT_KIND)?,
            BOUNDARY_CHART_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }

    pub fn boundary_ref(&self) -> Result<BoundaryRef, BoundaryChartError> {
        Ok(BoundaryRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, BoundaryChartError> {
        if envelope.kind().as_str() != BOUNDARY_CHART_ARTIFACT_KIND {
            return Err(BoundaryChartError::UnexpectedArtifactKind {
                expected: BOUNDARY_CHART_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != BOUNDARY_CHART_SCHEMA_VERSION {
            return Err(BoundaryChartError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut values = vec![
            self.query.as_artifact_ref(),
            self.x_type.as_artifact_ref(),
            self.y_type.as_artifact_ref(),
            self.boundary_type.as_artifact_ref(),
            self.pi_x.as_artifact_ref(),
            self.pi_y.as_artifact_ref(),
            self.x_determination.as_artifact_ref(),
        ];
        if let Some(value) = self.y_determination {
            values.push(value.as_artifact_ref());
        }
        values.extend(
            self.negation_frontier_x
                .iter()
                .map(|value| value.as_artifact_ref()),
        );
        values.extend(
            self.negation_frontier_y
                .iter()
                .map(|value| value.as_artifact_ref()),
        );
        values.extend([
            self.seed_y.as_artifact_ref(),
            self.compatibility.as_artifact_ref(),
        ]);
        if let Some(value) = self.traversal {
            values.push(value.as_artifact_ref());
        }
        values.extend([self.grain.as_artifact_ref(), self.horizon.as_artifact_ref()]);
        values
    }
}

fn reference(encoded: &mut Vec<u8>, value: ArtifactRef) {
    encoded.extend_from_slice(value.as_bytes());
}
fn optional_reference(encoded: &mut Vec<u8>, value: Option<ArtifactRef>) {
    match value {
        Some(value) => {
            encoded.push(1);
            reference(encoded, value);
        }
        None => encoded.push(0),
    }
}
fn references<I: Iterator<Item = ArtifactRef>>(
    encoded: &mut Vec<u8>,
    values: I,
) -> Result<(), BoundaryChartError> {
    let values: Vec<_> = values.collect();
    let count = u32::try_from(values.len())
        .map_err(|_| BoundaryChartError::TooManyReferences(values.len()))?;
    encoded.extend_from_slice(&count.to_be_bytes());
    for value in values {
        reference(encoded, value);
    }
    Ok(())
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}
impl<'a> Cursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }
    fn take(&mut self, length: usize) -> Result<&'a [u8], BoundaryChartError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(BoundaryChartError::PayloadLengthOverflow)?;
        let value = self
            .bytes
            .get(self.position..end)
            .ok_or(BoundaryChartError::TruncatedPayload)?;
        self.position = end;
        Ok(value)
    }
    fn reference(&mut self) -> Result<ArtifactRef, BoundaryChartError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| BoundaryChartError::TruncatedPayload)?;
        Ok(ArtifactRef::from_bytes(bytes))
    }
    fn optional_reference(&mut self) -> Result<Option<ArtifactRef>, BoundaryChartError> {
        match self.take(1)?[0] {
            0 => Ok(None),
            1 => self.reference().map(Some),
            tag => Err(BoundaryChartError::UnknownOptionalTag(tag)),
        }
    }
    fn references(&mut self) -> Result<Vec<ArtifactRef>, BoundaryChartError> {
        let count: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| BoundaryChartError::TruncatedPayload)?;
        let count = usize::try_from(u32::from_be_bytes(count))
            .map_err(|_| BoundaryChartError::CountOverflow)?;
        (0..count).map(|_| self.reference()).collect()
    }
    const fn finished(&self) -> bool {
        self.position == self.bytes.len()
    }
    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

#[derive(Debug, Error)]
pub enum BoundaryChartError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("boundary-chart payload is truncated")]
    TruncatedPayload,
    #[error("boundary-chart payload length overflows this platform")]
    PayloadLengthOverflow,
    #[error("boundary-chart payload contains {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("boundary-chart payload has an unknown optional-reference tag {0}")]
    UnknownOptionalTag(u8),
    #[error("boundary-chart has {0} references, exceeding u32")]
    TooManyReferences(usize),
    #[error("boundary-chart reference count does not fit this platform")]
    CountOverflow,
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported boundary-chart schema version {0}")]
    UnsupportedSchemaVersion(u32),
}
