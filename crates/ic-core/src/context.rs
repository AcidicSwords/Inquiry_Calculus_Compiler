use std::{fmt, str::FromStr};

use crate::{ArtifactError, ArtifactRef};

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

artifact_reference!(ScopeRef);
artifact_reference!(ApplicabilityRef);
artifact_reference!(GrainRef);
artifact_reference!(HorizonRef);
artifact_reference!(SupportRef);
artifact_reference!(WarrantRef);

/// The required evidence route for a relation use or each open query port.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DischargeMode {
    Pure,
    Generate,
    Probe,
    Check,
    Warrant,
}

impl DischargeMode {
    pub(crate) const fn tag(self) -> u8 {
        match self {
            Self::Pure => 0,
            Self::Generate => 1,
            Self::Probe => 2,
            Self::Check => 3,
            Self::Warrant => 4,
        }
    }

    pub(crate) const fn from_tag(tag: u8) -> Option<Self> {
        match tag {
            0 => Some(Self::Pure),
            1 => Some(Self::Generate),
            2 => Some(Self::Probe),
            3 => Some(Self::Check),
            4 => Some(Self::Warrant),
            _ => None,
        }
    }
}
