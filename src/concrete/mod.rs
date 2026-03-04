pub use crate::concrete::property::{
    ConcreteClassForAxialTension, ConcreteClassForCompression, ConcreteClassForFlexuralTension,
    ConcreteGradeForFrostResistanceByFirstMethod, ConcreteGradeForFrostResistanceBySecondMethod,
    ConcreteGradeForWaterResistance, ConcreteType,
};

pub mod property;

/// Concrete with its classification characteristics.
///
/// Compression class is mandatory. All other characteristics are optional and
/// are specified when required by design conditions.
#[derive(Clone, bon::Builder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Concrete {
    concrete_type: ConcreteType,
    class_for_compression: ConcreteClassForCompression,
    class_for_axial_tension: Option<ConcreteClassForAxialTension>,
    class_for_flexural_tension: Option<ConcreteClassForFlexuralTension>,
    grade_for_frost_resistance_by_first_method:
        Option<ConcreteGradeForFrostResistanceByFirstMethod>,
    grade_for_frost_resistance_by_second_method:
        Option<ConcreteGradeForFrostResistanceBySecondMethod>,
    grade_for_water_resistance: Option<ConcreteGradeForWaterResistance>,
}

impl Concrete {
    /// Returns the concrete type [`ConcreteType`].
    pub fn concrete_type(&self) -> ConcreteType {
        self.concrete_type
    }

    /// Returns the compression class [`ConcreteClassForCompression`].
    pub fn class_for_compression(&self) -> ConcreteClassForCompression {
        self.class_for_compression
    }

    /// Returns the axial tension class [`Some<ConcreteClassForAxialTension>`]
    /// if specified, else [`None`].
    pub fn class_for_axial_tension(&self) -> Option<ConcreteClassForAxialTension> {
        self.class_for_axial_tension
    }

    /// Returns the flexural tension class [`Some<ConcreteClassForFlexuralTension>`]
    /// if specified, else [`None`].
    pub fn class_for_flexural_tension(&self) -> Option<ConcreteClassForFlexuralTension> {
        self.class_for_flexural_tension
    }

    /// Returns the frost resistance grade by first method
    /// [`Some<ConcreteGradeForFrostResistanceByFirstMethod>`] if specified, else [`None`].
    pub fn grade_for_frost_resistance_by_first_method(
        &self,
    ) -> Option<ConcreteGradeForFrostResistanceByFirstMethod> {
        self.grade_for_frost_resistance_by_first_method
    }

    /// Returns the frost resistance grade by second method
    /// [`Some<ConcreteGradeForFrostResistanceBySecondMethod>`] if specified, else [`None`].
    pub fn grade_for_frost_resistance_by_second_method(
        &self,
    ) -> Option<ConcreteGradeForFrostResistanceBySecondMethod> {
        self.grade_for_frost_resistance_by_second_method
    }

    /// Returns the water resistance grade [`Some<ConcreteGradeForWaterResistance>`]
    /// if specified, else [`None`].
    pub fn grade_for_water_resistance(&self) -> Option<ConcreteGradeForWaterResistance> {
        self.grade_for_water_resistance
    }
}
