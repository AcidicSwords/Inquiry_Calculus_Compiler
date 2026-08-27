//! Derived, nonexecuting source-`Ask` to runtime-program lowering checks.

use std::collections::{BTreeMap, BTreeSet};

use ic_core::{
    ArtifactRef, AskOccurrence, AskOccurrenceCheckError, BindingVersionRef, DischargeMode,
    OpenQueryCheckError, ProvenanceRef, QuestionSuccessionCatalog, ResolutionPathCheckError,
    ResolutionPathRef, RouteRef, RuntimeProgramRef, TypeRef, TypeSymbol, TypedFormRef,
};
use thiserror::Error;

use crate::{
    ActualitySeparationCatalog, FiniteProbeDischargeBundle, ProbeDischargeBundleError,
    RuntimeCatalog, RuntimeProgramArtifact, RuntimeProgramCheckError,
    admit_finite_probe_discharge_bundle, admit_probe_ports_of_mixed_discharge,
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

/// What a non-Probe port supplies, in the two carriers the plan's payload IR keeps apart.
///
/// A `Generate` port may only ever hold a `Proposal`. Canonical states that generation evidence
/// carries no actuality authority, so a proposal must not share a carrier with a result a
/// registered computation, an independent checker, or the standing policy actually established.
/// The distinction is structural rather than a flag, so a consumer cannot read a proposal as a
/// determined result without naming the constructor that says otherwise.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NonProbePortOutput {
    /// A `Pure`, `Check`, or `Warrant` port's exact typed result.
    Determined(TypedFormRef),
    /// A `Generate` port's provisional filling, carrying no actuality authority.
    Proposal(TypedFormRef),
}

impl NonProbePortOutput {
    /// The typed form either carrier holds. Reading this discards the authority distinction, so
    /// use it only where the distinction has already been accounted for.
    #[must_use]
    pub const fn typed_form(self) -> TypedFormRef {
        match self {
            Self::Determined(form) | Self::Proposal(form) => form,
        }
    }

    #[must_use]
    pub const fn is_proposal(self) -> bool {
        matches!(self, Self::Proposal(_))
    }

    /// The one discharge mode family this carrier belongs to.
    const fn admits(self, mode: DischargeMode) -> bool {
        match self {
            Self::Proposal(_) => matches!(mode, DischargeMode::Generate),
            Self::Determined(_) => matches!(
                mode,
                DischargeMode::Pure | DischargeMode::Check | DischargeMode::Warrant
            ),
        }
    }
}

/// One non-Probe open-port evidence record within a source occurrence.
///
/// Canonical `PortEvidence` gives Pure, Generate, Check, and Warrant ports their exact typed
/// output, route, resolution path, versions, and provenance. None of them carries an
/// `ActualEvent`: only a Probe port enters the ordinary event spine, so this record has no event
/// field to fill and cannot be made to carry actuality.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonProbePortDischargeEvidence {
    port: TypeSymbol,
    mode: DischargeMode,
    output: NonProbePortOutput,
    route: RouteRef,
    resolution_path: ResolutionPathRef,
    binding: BindingVersionRef,
    compiler_version: ArtifactRef,
    provenance: ProvenanceRef,
}

impl NonProbePortDischargeEvidence {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        port: TypeSymbol,
        mode: DischargeMode,
        output: NonProbePortOutput,
        route: RouteRef,
        resolution_path: ResolutionPathRef,
        binding: BindingVersionRef,
        compiler_version: ArtifactRef,
        provenance: ProvenanceRef,
    ) -> Self {
        Self {
            port,
            mode,
            output,
            route,
            resolution_path,
            binding,
            compiler_version,
            provenance,
        }
    }

    #[must_use]
    pub const fn port(&self) -> &TypeSymbol {
        &self.port
    }

    #[must_use]
    pub const fn mode(&self) -> DischargeMode {
        self.mode
    }

    #[must_use]
    pub const fn output(&self) -> NonProbePortOutput {
        self.output
    }

    /// The typed form this port supplies, whether determined or merely proposed.
    #[must_use]
    pub const fn result(&self) -> TypedFormRef {
        self.output.typed_form()
    }

    #[must_use]
    pub const fn route(&self) -> RouteRef {
        self.route
    }

    #[must_use]
    pub const fn resolution_path(&self) -> ResolutionPathRef {
        self.resolution_path
    }
}

/// A derived view of one source `Ask` occurrence whose open ports do not all share a mode.
///
/// The Probe ports keep the existing checked bundle as the owner of their event, route, decoder,
/// and resolution provenance. The non-Probe ports keep their own typed result and declared
/// authority route. The two sides partition the exact open-port field of one occurrence; neither
/// side may claim a port declared for the other, and verification never dispatches.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MixedModeSourceAskDischarge {
    lowering: SourceAskLowering,
    probe_bundle: FiniteProbeDischargeBundle,
    non_probe: Vec<NonProbePortDischargeEvidence>,
}

impl MixedModeSourceAskDischarge {
    #[must_use]
    pub const fn new(
        lowering: SourceAskLowering,
        probe_bundle: FiniteProbeDischargeBundle,
        non_probe: Vec<NonProbePortDischargeEvidence>,
    ) -> Self {
        Self {
            lowering,
            probe_bundle,
            non_probe,
        }
    }

    #[must_use]
    pub const fn lowering(&self) -> &SourceAskLowering {
        &self.lowering
    }

    #[must_use]
    pub const fn probe_bundle(&self) -> &FiniteProbeDischargeBundle {
        &self.probe_bundle
    }

    #[must_use]
    pub fn non_probe(&self) -> &[NonProbePortDischargeEvidence] {
        &self.non_probe
    }

    /// Independently rechecks the lowering, the Probe side, and every non-Probe port without
    /// executing or dispatching anything.
    pub fn check<C: SourceAskProbeDischargeCatalog>(
        &self,
        catalog: &C,
    ) -> Result<(), MixedModeSourceAskDischargeError> {
        self.lowering.check(catalog)?;
        let occurrence = self.lowering.occurrence();

        let mut probe_ports = BTreeSet::new();
        let mut non_probe_modes = BTreeMap::new();
        for lowering in self.lowering.port_lowerings() {
            if lowering.mode() == DischargeMode::Probe {
                probe_ports.insert(lowering.port().clone());
            } else {
                non_probe_modes.insert(lowering.port().clone(), lowering.mode());
            }
        }
        if probe_ports.is_empty() {
            return Err(MixedModeSourceAskDischargeError::NoProbePort);
        }
        if non_probe_modes.is_empty() {
            return Err(MixedModeSourceAskDischargeError::NoNonProbePort);
        }

        let rechecked_bundle = admit_probe_ports_of_mixed_discharge(
            self.probe_bundle.occurrence().clone(),
            self.probe_bundle.components().to_vec(),
            self.probe_bundle.shared_events().to_vec(),
            catalog,
        )?;
        if rechecked_bundle.occurrence() != occurrence {
            return Err(MixedModeSourceAskDischargeError::OccurrenceMismatch);
        }
        let bundle_ports = rechecked_bundle
            .components()
            .iter()
            .map(|component| component.port().clone())
            .collect::<BTreeSet<_>>();
        if bundle_ports != probe_ports {
            return Err(MixedModeSourceAskDischargeError::ProbePortCoverageMismatch);
        }

        let query = ic_core::OpenQueryCatalog::resolve_open_query(catalog, occurrence.question())
            .ok_or(MixedModeSourceAskDischargeError::UnresolvedQuery(
            occurrence.question(),
        ))?;
        let schema = ic_core::RelationCatalog::resolve_relation_schema(catalog, query.relation())
            .ok_or(MixedModeSourceAskDischargeError::UnresolvedRelation(
            query.relation(),
        ))?;

        let mut covered = BTreeSet::new();
        for evidence in &self.non_probe {
            if evidence.mode == DischargeMode::Probe {
                return Err(MixedModeSourceAskDischargeError::ProbeModeOnNonProbeSide(
                    evidence.port.as_str().to_owned(),
                ));
            }
            let declared = non_probe_modes.get(&evidence.port).ok_or_else(|| {
                MixedModeSourceAskDischargeError::ForeignNonProbePort(
                    evidence.port.as_str().to_owned(),
                )
            })?;
            if *declared != evidence.mode {
                return Err(MixedModeSourceAskDischargeError::NonProbeModeMismatch {
                    port: evidence.port.as_str().to_owned(),
                    declared: *declared,
                    evidence: evidence.mode,
                });
            }
            // Only a Generate port may hold a proposal, and a Generate port may hold nothing else.
            if !evidence.output.admits(evidence.mode) {
                return Err(MixedModeSourceAskDischargeError::OutputAuthorityMismatch {
                    port: evidence.port.as_str().to_owned(),
                    mode: evidence.mode,
                    proposed: evidence.output.is_proposal(),
                });
            }
            if !covered.insert(evidence.port.clone()) {
                return Err(MixedModeSourceAskDischargeError::DuplicateNonProbePort(
                    evidence.port.as_str().to_owned(),
                ));
            }
            if evidence.binding != occurrence.binding_version() {
                return Err(MixedModeSourceAskDischargeError::NonProbeBindingMismatch(
                    evidence.port.as_str().to_owned(),
                ));
            }
            if evidence.compiler_version != occurrence.compiler_version() {
                return Err(
                    MixedModeSourceAskDischargeError::NonProbeCompilerVersionMismatch(
                        evidence.port.as_str().to_owned(),
                    ),
                );
            }
            if evidence.provenance != occurrence.provenance() {
                return Err(
                    MixedModeSourceAskDischargeError::NonProbeProvenanceMismatch(
                        evidence.port.as_str().to_owned(),
                    ),
                );
            }

            let typed_form =
                ic_core::FormulaCatalog::resolve_typed_form(catalog, evidence.result()).ok_or(
                    MixedModeSourceAskDischargeError::UnresolvedNonProbeResult(evidence.result()),
                )?;
            let calculated = typed_form.typed_form_ref()?;
            if calculated != evidence.result() {
                return Err(
                    MixedModeSourceAskDischargeError::NonProbeResultIdentityMismatch {
                        expected: evidence.result(),
                        actual: calculated,
                    },
                );
            }
            if typed_form.binding() != occurrence.binding_version() {
                return Err(
                    MixedModeSourceAskDischargeError::NonProbeResultBindingMismatch(
                        evidence.port.as_str().to_owned(),
                    ),
                );
            }

            let path = catalog
                .resolve_resolution_path(evidence.resolution_path)
                .ok_or(MixedModeSourceAskDischargeError::UnresolvedResolutionPath(
                    evidence.resolution_path,
                ))?;
            let calculated = path.resolution_path_ref()?;
            if calculated != evidence.resolution_path {
                return Err(
                    MixedModeSourceAskDischargeError::ResolutionPathIdentityMismatch {
                        expected: evidence.resolution_path,
                        actual: calculated,
                    },
                );
            }
            path.check(catalog).map_err(|error| {
                MixedModeSourceAskDischargeError::ResolutionPathCheck(Box::new(error))
            })?;
            let carrier = schema
                .ports()
                .iter()
                .find(|port| port.name() == &evidence.port)
                .map(ic_core::RelationPort::ty)
                .ok_or_else(|| {
                    MixedModeSourceAskDischargeError::PortMissingFromSchema(
                        evidence.port.as_str().to_owned(),
                    )
                })?;
            if path.input() != typed_form.ty() || path.output() != carrier {
                return Err(
                    MixedModeSourceAskDischargeError::NonProbeResolutionTypeMismatch(Box::new(
                        NonProbeResolutionTypeMismatch {
                            port: evidence.port.clone(),
                            expected_input: typed_form.ty(),
                            actual_input: path.input(),
                            expected_output: carrier,
                            actual_output: path.output(),
                        },
                    )),
                );
            }
        }

        let declared_non_probe = non_probe_modes.keys().cloned().collect::<BTreeSet<_>>();
        if covered != declared_non_probe {
            return Err(MixedModeSourceAskDischargeError::NonProbePortCoverageMismatch);
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

#[derive(Debug, Error)]
pub enum MixedModeSourceAskDischargeError {
    #[error(transparent)]
    Lowering(#[from] SourceAskLoweringCheckError),
    #[error(transparent)]
    Bundle(#[from] ProbeDischargeBundleError),
    #[error(transparent)]
    TypedFormEncoding(#[from] ic_core::TypeError),
    #[error(transparent)]
    ResolutionPathEncoding(#[from] ic_core::ResolutionPathError),
    #[error("resolution path failed recheck: {0}")]
    ResolutionPathCheck(Box<ResolutionPathCheckError>),
    #[error("a mixed-mode source Ask view requires at least one Probe port")]
    NoProbePort,
    #[error("a mixed-mode source Ask view requires at least one non-Probe port")]
    NoNonProbePort,
    #[error("Probe discharge bundle belongs to a different source Ask occurrence")]
    OccurrenceMismatch,
    #[error("Probe bundle components do not exactly cover the Probe-mode source ports")]
    ProbePortCoverageMismatch,
    #[error("non-Probe evidence does not exactly cover the non-Probe source ports")]
    NonProbePortCoverageMismatch,
    #[error("non-Probe evidence for port {0:?} declares Probe mode and would claim an event")]
    ProbeModeOnNonProbeSide(String),
    #[error(
        "non-Probe evidence names port {0:?}, which this source Ask does not declare non-Probe"
    )]
    ForeignNonProbePort(String),
    #[error("non-Probe evidence repeats port {0:?}")]
    DuplicateNonProbePort(String),
    #[error(
        "non-Probe evidence mode {evidence:?} differs from source mode {declared:?} for port {port:?}"
    )]
    NonProbeModeMismatch {
        port: String,
        declared: DischargeMode,
        evidence: DischargeMode,
    },
    #[error(
        "port {port:?} declares mode {mode:?}, which does not admit a {} carrier",
        if *proposed { "proposal" } else { "determined-result" }
    )]
    OutputAuthorityMismatch {
        port: String,
        mode: DischargeMode,
        proposed: bool,
    },
    #[error("non-Probe evidence for port {0:?} has a binding other than its source occurrence")]
    NonProbeBindingMismatch(String),
    #[error(
        "non-Probe evidence for port {0:?} has a compiler version other than its source occurrence"
    )]
    NonProbeCompilerVersionMismatch(String),
    #[error("non-Probe evidence for port {0:?} has provenance other than its source occurrence")]
    NonProbeProvenanceMismatch(String),
    #[error("question {0} is unavailable")]
    UnresolvedQuery(ic_core::QueryRef),
    #[error("relation {0} is unavailable")]
    UnresolvedRelation(ic_core::RelationRef),
    #[error("open port {0:?} is absent from its relation schema")]
    PortMissingFromSchema(String),
    #[error("non-Probe typed result {0} is unavailable")]
    UnresolvedNonProbeResult(TypedFormRef),
    #[error("non-Probe typed result identity is {actual}, expected {expected}")]
    NonProbeResultIdentityMismatch {
        expected: TypedFormRef,
        actual: TypedFormRef,
    },
    #[error("non-Probe typed result for port {0:?} is scoped to another binding")]
    NonProbeResultBindingMismatch(String),
    #[error("resolution path {0} is unavailable")]
    UnresolvedResolutionPath(ResolutionPathRef),
    #[error("resolution path identity is {actual}, expected {expected}")]
    ResolutionPathIdentityMismatch {
        expected: ResolutionPathRef,
        actual: ResolutionPathRef,
    },
    #[error(transparent)]
    NonProbeResolutionTypeMismatch(Box<NonProbeResolutionTypeMismatch>),
}

#[derive(Debug, Error)]
#[error(
    "non-Probe port {port} resolution path has {actual_input}->{actual_output}, expected {expected_input}->{expected_output}"
)]
pub struct NonProbeResolutionTypeMismatch {
    port: TypeSymbol,
    expected_input: TypeRef,
    actual_input: TypeRef,
    expected_output: TypeRef,
    actual_output: TypeRef,
}
