//! Derived, nonexecuting source-`Ask` to runtime-program lowering checks.

use std::collections::BTreeSet;

use ic_core::{
    ArtifactRef, AskOccurrence, AskOccurrenceCheckError, DischargeMode, OpenQueryCheckError,
    QuestionSuccessionCatalog, RuntimeProgramRef, TypeSymbol,
};
use thiserror::Error;

use crate::{
    ActualitySeparationCatalog, FiniteProbeDischargeBundle, ProbeDischargeBundleError,
    RuntimeCatalog, RuntimeProgramArtifact, RuntimeProgramCheckError,
    admit_finite_probe_discharge_bundle,
};

/// One declared lowering of exactly one checked open source port.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PortLowering {
    port: TypeSymbol,
    mode: DischargeMode,
}

impl PortLowering {
    #[must_use]
    pub const fn new(port: TypeSymbol, mode: DischargeMode) -> Self {
        Self { port, mode }
    }
    #[must_use]
    pub const fn port(&self) -> &TypeSymbol {
        &self.port
    }
    #[must_use]
    pub const fn mode(&self) -> DischargeMode {
        self.mode
    }
}

/// The catalog needed to independently rewalk the source and verify its runtime program.
pub trait SourceAskLoweringCatalog: QuestionSuccessionCatalog + RuntimeCatalog {}
impl<T> SourceAskLoweringCatalog for T where T: QuestionSuccessionCatalog + RuntimeCatalog {}

/// The catalog needed to recheck an execution-conditioned pairing without dispatching.
pub trait SourceAskProbeDischargeCatalog:
    SourceAskLoweringCatalog + ActualitySeparationCatalog
{
}
impl<T> SourceAskProbeDischargeCatalog for T where
    T: SourceAskLoweringCatalog + ActualitySeparationCatalog
{
}

/// A derived source-to-runtime pairing, not a compiler, dispatch plan, or event record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceAskLowering {
    occurrence: AskOccurrence,
    port_lowerings: Vec<PortLowering>,
    runtime: RuntimeProgramArtifact,
}

impl SourceAskLowering {
    pub fn new(
        occurrence: AskOccurrence,
        port_lowerings: Vec<PortLowering>,
        runtime: RuntimeProgramArtifact,
    ) -> Result<Self, SourceAskLoweringCheckError> {
        if port_lowerings.is_empty() {
            return Err(SourceAskLoweringCheckError::EmptyPortLowerings);
        }
        Ok(Self {
            occurrence,
            port_lowerings,
            runtime,
        })
    }
    #[must_use]
    pub const fn occurrence(&self) -> &AskOccurrence {
        &self.occurrence
    }
    #[must_use]
    pub fn port_lowerings(&self) -> &[PortLowering] {
        &self.port_lowerings
    }
    #[must_use]
    pub const fn runtime(&self) -> &RuntimeProgramArtifact {
        &self.runtime
    }
    pub fn runtime_ref(&self) -> Result<RuntimeProgramRef, SourceAskLoweringCheckError> {
        Ok(self.runtime.runtime_program_ref()?)
    }

    /// Verifies this lowering against the exact source occurrence and runtime identity expected by
    /// a consuming reconstruction.  A structurally valid lowering for another occurrence or
    /// runtime program is not interchangeable.
    pub fn check_expected<C: SourceAskLoweringCatalog>(
        &self,
        expected_occurrence: &AskOccurrence,
        expected_runtime: RuntimeProgramRef,
        catalog: &C,
    ) -> Result<(), SourceAskLoweringCheckError> {
        self.check(catalog)?;
        if self.occurrence != *expected_occurrence {
            return Err(SourceAskLoweringCheckError::ExpectedOccurrenceMismatch);
        }
        let actual_runtime = self.runtime_ref()?;
        if actual_runtime != expected_runtime {
            return Err(SourceAskLoweringCheckError::ExpectedRuntimeMismatch {
                expected: expected_runtime,
                actual: actual_runtime,
            });
        }
        Ok(())
    }

    /// Rewalks source and runtime independently, without executing either.
    pub fn check<C: SourceAskLoweringCatalog>(
        &self,
        catalog: &C,
    ) -> Result<(), SourceAskLoweringCheckError> {
        self.occurrence.check(catalog)?;
        self.runtime.check(catalog)?;
        if self.runtime.binding() != self.occurrence.binding_version() {
            return Err(SourceAskLoweringCheckError::BindingMismatch {
                occurrence: self.occurrence.binding_version(),
                runtime: self.runtime.binding(),
            });
        }
        if self.runtime.compiler_version() != self.occurrence.compiler_version() {
            return Err(SourceAskLoweringCheckError::CompilerMismatch {
                occurrence: self.occurrence.compiler_version(),
                runtime: self.runtime.compiler_version(),
            });
        }
        let source = catalog
            .resolve_source_config(self.occurrence.source_config())
            .ok_or(SourceAskLoweringCheckError::UnresolvedSourceConfig(
                self.occurrence.source_config(),
            ))?;
        if source.source_config_ref()? != self.occurrence.source_config() {
            return Err(SourceAskLoweringCheckError::SourceConfigIdentityMismatch);
        }
        if source.result_type() != self.runtime.result() {
            return Err(SourceAskLoweringCheckError::ResultTypeMismatch {
                source_type: source.result_type(),
                runtime: self.runtime.result(),
            });
        }
        let query = catalog
            .resolve_open_query(self.occurrence.question())
            .ok_or(SourceAskLoweringCheckError::UnresolvedQuery(
                self.occurrence.question(),
            ))?;
        if query.query_ref()? != self.occurrence.question() {
            return Err(SourceAskLoweringCheckError::QueryIdentityMismatch);
        }
        query.check(catalog)?;
        let mut seen = BTreeSet::new();
        for lowering in &self.port_lowerings {
            if !seen.insert(lowering.port().as_str()) {
                return Err(SourceAskLoweringCheckError::DuplicatePort(
                    lowering.port().as_str().to_owned(),
                ));
            }
        }
        if query.open_ports().len() != self.port_lowerings.len() {
            return Err(SourceAskLoweringCheckError::PortCountMismatch {
                source_count: query.open_ports().len(),
                lowering: self.port_lowerings.len(),
            });
        }
        for lowering in &self.port_lowerings {
            let expected = query
                .open_ports()
                .iter()
                .find(|open| open.port() == lowering.port())
                .ok_or_else(|| {
                    SourceAskLoweringCheckError::ForeignPort(lowering.port().as_str().to_owned())
                })?;
            if expected.mode() != lowering.mode() {
                return Err(SourceAskLoweringCheckError::ModeMismatch {
                    port: lowering.port().as_str().to_owned(),
                    expected_mode: expected.mode(),
                    lowering: lowering.mode(),
                });
            }
        }
        Ok(())
    }
}

/// A derived pairing between one source lowering and the existing all-Probe discharge bundle.
///
/// The bundle remains the owner of event, route, decoder, and resolution provenance. This record
/// only proves that it discharges this exact source lowering.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceAskProbeDischarge {
    lowering: SourceAskLowering,
    bundle: FiniteProbeDischargeBundle,
}

impl SourceAskProbeDischarge {
    #[must_use]
    pub const fn new(lowering: SourceAskLowering, bundle: FiniteProbeDischargeBundle) -> Self {
        Self { lowering, bundle }
    }

    #[must_use]
    pub const fn lowering(&self) -> &SourceAskLowering {
        &self.lowering
    }

    #[must_use]
    pub const fn bundle(&self) -> &FiniteProbeDischargeBundle {
        &self.bundle
    }

    /// Independently rechecks both operands and their exact source-port pairing.
    pub fn check<C: SourceAskProbeDischargeCatalog>(
        &self,
        catalog: &C,
    ) -> Result<(), SourceAskProbeDischargeError> {
        self.lowering.check(catalog)?;
        for lowering in self.lowering.port_lowerings() {
            if lowering.mode() != DischargeMode::Probe {
                return Err(SourceAskProbeDischargeError::NonProbeLoweringPort(
                    lowering.port().as_str().to_owned(),
                ));
            }
        }
        let rechecked_bundle = admit_finite_probe_discharge_bundle(
            self.bundle.occurrence().clone(),
            self.bundle.components().to_vec(),
            self.bundle.shared_events().to_vec(),
            catalog,
        )?;
        if rechecked_bundle.occurrence() != self.lowering.occurrence() {
            return Err(SourceAskProbeDischargeError::OccurrenceMismatch);
        }
        let lowering_ports = self
            .lowering
            .port_lowerings()
            .iter()
            .map(|lowering| lowering.port().as_str())
            .collect::<BTreeSet<_>>();
        let bundle_ports = rechecked_bundle
            .components()
            .iter()
            .map(|component| component.port().as_str())
            .collect::<BTreeSet<_>>();
        if lowering_ports != bundle_ports {
            return Err(SourceAskProbeDischargeError::PortCoverageMismatch);
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum SourceAskLoweringCheckError {
    #[error(transparent)]
    Occurrence(#[from] AskOccurrenceCheckError),
    #[error(transparent)]
    Runtime(#[from] RuntimeProgramCheckError),
    #[error(transparent)]
    RuntimeEncoding(#[from] crate::RuntimeProgramError),
    #[error(transparent)]
    SourceConfigEncoding(#[from] ic_core::SourceConfigError),
    #[error(transparent)]
    Query(#[from] OpenQueryCheckError),
    #[error(transparent)]
    QueryEncoding(#[from] ic_core::OpenQueryError),
    #[error("source Ask lowering must name at least one port")]
    EmptyPortLowerings,
    #[error("source Ask lowering names a different source occurrence than the consumer expects")]
    ExpectedOccurrenceMismatch,
    #[error("source Ask lowering runtime {actual} differs from the expected runtime {expected}")]
    ExpectedRuntimeMismatch {
        expected: RuntimeProgramRef,
        actual: RuntimeProgramRef,
    },
    #[error("source configuration {0} is unavailable")]
    UnresolvedSourceConfig(ic_core::SourceConfigRef),
    #[error("source configuration identity differs from its claimed reference")]
    SourceConfigIdentityMismatch,
    #[error("source result type {source_type} differs from runtime result type {runtime}")]
    ResultTypeMismatch {
        source_type: ic_core::TypeRef,
        runtime: ic_core::TypeRef,
    },
    #[error("runtime binding {runtime} differs from source occurrence binding {occurrence}")]
    BindingMismatch {
        occurrence: ic_core::BindingVersionRef,
        runtime: ic_core::BindingVersionRef,
    },
    #[error("runtime compiler {runtime} differs from source occurrence compiler {occurrence}")]
    CompilerMismatch {
        occurrence: ArtifactRef,
        runtime: ArtifactRef,
    },
    #[error("source query {0} is unavailable")]
    UnresolvedQuery(ic_core::QueryRef),
    #[error("source query identity differs from its claimed reference")]
    QueryIdentityMismatch,
    #[error("source has {source_count} open ports but lowering has {lowering}")]
    PortCountMismatch {
        source_count: usize,
        lowering: usize,
    },
    #[error("lowering repeats source port {0:?}")]
    DuplicatePort(String),
    #[error("lowering names foreign source port {0:?}")]
    ForeignPort(String),
    #[error(
        "lowering mode {lowering:?} differs from source mode {expected_mode:?} for port {port:?}"
    )]
    ModeMismatch {
        port: String,
        expected_mode: DischargeMode,
        lowering: DischargeMode,
    },
}

#[derive(Debug, Error)]
pub enum SourceAskProbeDischargeError {
    #[error(transparent)]
    Lowering(#[from] SourceAskLoweringCheckError),
    #[error(transparent)]
    Bundle(#[from] ProbeDischargeBundleError),
    #[error("finite Probe discharge bundle belongs to a different source Ask occurrence")]
    OccurrenceMismatch,
    #[error("source lowering port {0:?} is not Probe-mode for a finite Probe discharge bundle")]
    NonProbeLoweringPort(String),
    #[error("source lowering ports do not exactly match finite Probe bundle component ports")]
    PortCoverageMismatch,
}
