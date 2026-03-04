/// Concrete type per GOST 26633-2015.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConcreteType {
    Heavy,
    Cellular,
    Finegrained,
    Prestressed,
    Lightweight,
}

/// Concrete class for compression per GOST 26633-2015.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConcreteClassForCompression {
    B1_5,
    B2,
    B2_5,
    B3_5,
    B5,
    B7_5,
    B10,
    B12_5,
    B15,
    B20,
    B25,
    B30,
    B35,
    B40,
    B45,
    B50,
    B55,
    B60,
    B70,
    B80,
    B90,
    B100,
    B110,
    B120,
}

/// Concrete class for axial tension per GOST 26633-2015.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConcreteClassForAxialTension {
    B0_8,
    B1_2,
    B1_6,
    B2_0,
    B2_4,
    B2_8,
    B3_2,
    B3_6,
    B4_0,
    B4_4,
    B4_8,
}

/// Concrete class for flexural tension per GOST 26633-2015.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConcreteClassForFlexuralTension {
    B1_2,
    B1_6,
    B2_0,
    B2_4,
    B2_8,
    B3_2,
    B3_6,
    B4_0,
    B4_4,
    B4_8,
    B5_2,
    B5_6,
    B6_0,
    B6_4,
    B6_8,
    B7_2,
    B7_6,
    B8_0,
    B8_4,
    B8_8,
    B9_2,
    B9_6,
    B10_0,
}

/// Concrete grade for frost resistance by first method per GOST 26633-2015.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConcreteGradeForFrostResistanceByFirstMethod {
    F50,
    F75,
    F100,
    F150,
    F200,
    F300,
    F400,
    F500,
    F600,
    F800,
    F1000,
}

/// Concrete grade for frost resistance by second method per GOST 26633-2015.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConcreteGradeForFrostResistanceBySecondMethod {
    F100,
    F150,
    F200,
    F300,
    F400,
    F500,
}

/// Concrete grade for water resistance per GOST 26633-2015.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConcreteGradeForWaterResistance {
    W2,
    W4,
    W6,
    W8,
    W10,
    W12,
    W14,
    W16,
    W18,
    W20,
}
