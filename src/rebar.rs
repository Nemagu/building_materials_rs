///A reinforcing bar (rebar) as defined in GOST 34028-2016.
///
/// Combines a rebar class, surface configuration profile.
#[derive(Clone, bon::Builder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rebar {
    rebar_class: RebarClass,
    configuration_profile: Option<RebarConfigurationProfile>,
}

impl Rebar {
    /// Returns the rebar class [`RebarClass`].
    pub fn rebar_class(&self) -> RebarClass {
        self.rebar_class
    }

    /// Returns the surface configuration profile [`Some<RebarConfigurationProfile>`]
    /// if it was specified during creation else [`None`].
    pub fn configuration_profile(&self) -> Option<RebarConfigurationProfile> {
        self.configuration_profile
    }
}

/// Rebar class.
///
/// Represents the class of reinforcing bar.
/// If prefix is A and Ap, then source is GOST 34028-2016.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RebarClass {
    A240,
    A400,
    A500,
    A600,
    Ap600,
    A800,
    A1000,
    B500,
    Br500,
    Br1200,
    Br1300,
    Br1400,
    Br1500,
    Br1600,
    K1400,
    K1500,
    K1600,
    K1700,
    K1800,
    K1900,
}

/// Rebar configuration profile.
///
/// Represents the surface configuration profile of reinforcing bar.
/// Source is GOST 34028-2016.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RebarConfigurationProfile {
    F1,
    F2,
    F3,
    F4,
}
