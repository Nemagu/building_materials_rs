use crate::concrete::{Concrete, ConcreteClassForCompression, ConcreteType};

/// Concrete material properties per SP 63.13330.2018.
pub struct ConcreteBySp63_13330_2018 {
    material: Concrete,
}

impl ConcreteBySp63_13330_2018 {
    pub fn new(material: Concrete) -> Self {
        Self { material }
    }

    /// Returns the normative axial compressive resistance **Rbn, Rb,ser** (Pa)
    /// for second group of load.
    ///
    /// Source: SP 63.13330.2018, Table 6.7.
    ///
    /// Returns `None` if the combination of class and concrete type
    /// is not covered by the standard.
    pub fn normative_axial_compression_resistance(&self) -> Option<f64> {
        let (hfs, lw, cell) = match self.material.class_for_compression() {
            ConcreteClassForCompression::B1_5 => (None, None, Some(1_400_000.0)),
            ConcreteClassForCompression::B2 => (None, None, Some(1_900_000.0)),
            ConcreteClassForCompression::B2_5 => (None, Some(1_900_000.0), Some(2_400_000.0)),
            ConcreteClassForCompression::B3_5 => {
                (Some(2_700_000.0), Some(2_700_000.0), Some(3_300_000.0))
            }
            ConcreteClassForCompression::B5 => {
                (Some(3_500_000.0), Some(3_500_000.0), Some(4_600_000.0))
            }
            ConcreteClassForCompression::B7_5 => {
                (Some(5_500_000.0), Some(5_500_000.0), Some(6_900_000.0))
            }
            ConcreteClassForCompression::B10 => {
                (Some(7_500_000.0), Some(7_500_000.0), Some(9_000_000.0))
            }
            ConcreteClassForCompression::B12_5 => {
                (Some(9_500_000.0), Some(9_500_000.0), Some(10_500_000.0))
            }
            ConcreteClassForCompression::B15 => {
                (Some(11_000_000.0), Some(11_000_000.0), Some(11_500_000.0))
            }
            ConcreteClassForCompression::B20 => (Some(15_000_000.0), Some(15_000_000.0), None),
            ConcreteClassForCompression::B25 => (Some(18_500_000.0), Some(18_500_000.0), None),
            ConcreteClassForCompression::B30 => (Some(22_000_000.0), Some(22_000_000.0), None),
            ConcreteClassForCompression::B35 => (Some(25_500_000.0), Some(25_500_000.0), None),
            ConcreteClassForCompression::B40 => (Some(29_000_000.0), Some(29_000_000.0), None),
            ConcreteClassForCompression::B45 => (Some(32_000_000.0), None, None),
            ConcreteClassForCompression::B50 => (Some(36_000_000.0), None, None),
            ConcreteClassForCompression::B55 => (Some(39_500_000.0), None, None),
            ConcreteClassForCompression::B60 => (Some(43_000_000.0), None, None),
            ConcreteClassForCompression::B70 => (Some(50_000_000.0), None, None),
            ConcreteClassForCompression::B80 => (Some(57_000_000.0), None, None),
            ConcreteClassForCompression::B90 => (Some(64_000_000.0), None, None),
            ConcreteClassForCompression::B100 => (Some(71_000_000.0), None, None),
            _ => return None,
        };
        match self.material.concrete_type() {
            ConcreteType::Heavy | ConcreteType::Finegrained | ConcreteType::Prestressed => hfs,
            ConcreteType::Lightweight => lw,
            ConcreteType::Cellular => cell,
        }
    }

    /// Alias for [`Self::normative_axial_compression_resistance`].
    pub fn rbn(&self) -> Option<f64> {
        self.normative_axial_compression_resistance()
    }

    /// Returns the calculated axial compressive resistance for second group of load **Rb,ser** (Pa).
    ///
    /// Equals [`Self::normative_axial_compression_resistance`].
    pub fn calculated_axial_compression_by_second_group(&self) -> Option<f64> {
        self.normative_axial_tension_resistance()
    }

    /// Alias for [`Self::calculated_axial_compression_by_second_group`].
    pub fn rbser(&self) -> Option<f64> {
        self.calculated_axial_compression_by_second_group()
    }

    /// Returns the calculated axial compressive resistance for first group of load **Rb** (Pa).
    ///
    /// Derived from [`Self::normative_axial_compression_resistance`] divided by
    /// safety coefficient: 1.3 for heavy/fine-grained/lightweight, 1.5 for cellular concrete.
    pub fn calculated_axial_compression_by_first_group(&self) -> Option<f64> {
        let value = match self.normative_axial_compression_resistance() {
            Some(v) => v,
            None => return None,
        };
        match self.material.concrete_type() {
            ConcreteType::Cellular => Some(value / 1.5),
            _ => Some(value / 1.3),
        }
    }

    /// Alias for [`Self::calculated_axial_compression_by_first_group`].
    pub fn rb(&self) -> Option<f64> {
        self.calculated_axial_compression_by_first_group()
    }

    /// Returns the normative axial tensile resistance **Rbtn, Rbt,ser** (Pa)
    /// for second group of load.
    ///
    /// Source: SP 63.13330.2018, Table 6.7.
    ///
    /// Returns `None` if the combination of class and concrete type
    /// is not covered by the standard.
    pub fn normative_axial_tension_resistance(&self) -> Option<f64> {
        let (hfs, lw, cell) = match self.material.class_for_compression() {
            ConcreteClassForCompression::B1_5 => (None, None, Some(220_000.0)),
            ConcreteClassForCompression::B2 => (None, None, Some(260_000.0)),
            ConcreteClassForCompression::B2_5 => (None, Some(290_000.0), Some(310_000.0)),
            ConcreteClassForCompression::B3_5 => {
                (Some(390_000.0), Some(390_000.0), Some(410_000.0))
            }
            ConcreteClassForCompression::B5 => (Some(550_000.0), Some(550_000.0), Some(550_000.0)),
            ConcreteClassForCompression::B7_5 => {
                (Some(700_000.0), Some(700_000.0), Some(630_000.0))
            }
            ConcreteClassForCompression::B10 => (Some(850_000.0), Some(850_000.0), Some(890_000.0)),
            ConcreteClassForCompression::B12_5 => {
                (Some(1_000_000.0), Some(1_000_000.0), Some(1_000_000.0))
            }
            ConcreteClassForCompression::B15 => {
                (Some(1_100_000.0), Some(1_100_000.0), Some(1_050_000.0))
            }
            ConcreteClassForCompression::B20 => (Some(1_350_000.0), Some(1_350_000.0), None),
            ConcreteClassForCompression::B25 => (Some(1_550_000.0), Some(1_550_000.0), None),
            ConcreteClassForCompression::B30 => (Some(1_750_000.0), Some(1_750_000.0), None),
            ConcreteClassForCompression::B35 => (Some(1_950_000.0), Some(1_950_000.0), None),
            ConcreteClassForCompression::B40 => (Some(2_100_000.0), Some(2_100_000.0), None),
            ConcreteClassForCompression::B45 => (Some(2_250_000.0), None, None),
            ConcreteClassForCompression::B50 => (Some(2_450_000.0), None, None),
            ConcreteClassForCompression::B55 => (Some(2_600_000.0), None, None),
            ConcreteClassForCompression::B60 => (Some(2_750_000.0), None, None),
            ConcreteClassForCompression::B70 => (Some(3_000_000.0), None, None),
            ConcreteClassForCompression::B80 => (Some(3_300_000.0), None, None),
            ConcreteClassForCompression::B90 => (Some(3_600_000.0), None, None),
            ConcreteClassForCompression::B100 => (Some(3_800_000.0), None, None),
            _ => return None,
        };
        match self.material.concrete_type() {
            ConcreteType::Heavy | ConcreteType::Finegrained => hfs,
            ConcreteType::Prestressed => match hfs {
                Some(v) => Some(v * 1.2),
                None => hfs,
            },
            ConcreteType::Lightweight => lw,
            ConcreteType::Cellular => cell,
        }
    }

    /// Alias for [`Self::normative_axial_tension_resistance`].
    pub fn rbtn(&self) -> Option<f64> {
        self.normative_axial_tension_resistance()
    }

    /// Returns the calculated axial tensile resistance for second group of load **Rbt,ser** (Pa).
    ///
    /// Equals [`Self::normative_axial_tension_resistance`].
    pub fn calculated_axial_tension_by_second_group(&self) -> Option<f64> {
        self.normative_axial_tension_resistance()
    }

    /// Alias for [`Self::calculated_axial_tension_by_second_group`].
    pub fn rbtser(&self) -> Option<f64> {
        self.calculated_axial_tension_by_second_group()
    }

    /// Returns the calculated axial tensile resistance for first group of load **Rbt** (Pa).
    ///
    /// Derived from [`Self::normative_axial_tension_resistance`] divided by
    /// safety coefficient: 2.5 for cellular concrete,
    /// for heavy/fine-grained/lightweight/prestressed concrete the coefficient is 1.3
    /// when axial tension class is specified, and 1.5 otherwise.
    pub fn calculated_axial_tension_by_first_group(&self) -> Option<f64> {
        let value = match self.normative_axial_tension_resistance() {
            Some(v) => v,
            None => return None,
        };
        match self.material.concrete_type() {
            ConcreteType::Cellular => Some(value / 2.5),
            ConcreteType::Finegrained
            | ConcreteType::Heavy
            | ConcreteType::Lightweight
            | ConcreteType::Prestressed => match self.material.class_for_axial_tension() {
                Some(c) => Some(value / 1.3),
                None => Some(value / 1.5),
            },
        }
    }

    /// Alias for [`Self::calculated_axial_tension_by_first_group`].
    pub fn rbt(&self) -> Option<f64> {
        self.calculated_axial_tension_by_first_group()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concrete::{
        Concrete, ConcreteClassForAxialTension, ConcreteClassForCompression, ConcreteType,
    };

    fn concrete(
        concrete_type: ConcreteType,
        class: ConcreteClassForCompression,
    ) -> ConcreteBySp63_13330_2018 {
        ConcreteBySp63_13330_2018::new(
            Concrete::builder()
                .concrete_type(concrete_type)
                .class_for_compression(class)
                .build(),
        )
    }

    fn concrete_with_axial_tension(
        concrete_type: ConcreteType,
        class: ConcreteClassForCompression,
        axial_tension: ConcreteClassForAxialTension,
    ) -> ConcreteBySp63_13330_2018 {
        ConcreteBySp63_13330_2018::new(
            Concrete::builder()
                .concrete_type(concrete_type)
                .class_for_compression(class)
                .class_for_axial_tension(axial_tension)
                .build(),
        )
    }

    #[test]
    fn normative_axial_compression_resistance() {
        use ConcreteClassForCompression::*;
        use ConcreteType::*;

        let cases: &[(ConcreteType, ConcreteClassForCompression, Option<f64>)] = &[
            //
            (Heavy, B1_5, None),
            (Heavy, B2, None),
            (Heavy, B2_5, None),
            (Heavy, B3_5, Some(2_700_000.0)),
            (Heavy, B5, Some(3_500_000.0)),
            (Heavy, B7_5, Some(5_500_000.0)),
            (Heavy, B10, Some(7_500_000.0)),
            (Heavy, B12_5, Some(9_500_000.0)),
            (Heavy, B15, Some(11_000_000.0)),
            (Heavy, B20, Some(15_000_000.0)),
            (Heavy, B25, Some(18_500_000.0)),
            (Heavy, B30, Some(22_000_000.0)),
            (Heavy, B35, Some(25_500_000.0)),
            (Heavy, B40, Some(29_000_000.0)),
            (Heavy, B45, Some(32_000_000.0)),
            (Heavy, B50, Some(36_000_000.0)),
            (Heavy, B55, Some(39_500_000.0)),
            (Heavy, B60, Some(43_000_000.0)),
            (Heavy, B70, Some(50_000_000.0)),
            (Heavy, B80, Some(57_000_000.0)),
            (Heavy, B90, Some(64_000_000.0)),
            (Heavy, B100, Some(71_000_000.0)),
            (Heavy, B110, None),
            (Heavy, B120, None),
            //
            (Finegrained, B1_5, None),
            (Finegrained, B3_5, Some(2_700_000.0)),
            (Finegrained, B25, Some(18_500_000.0)),
            (Finegrained, B100, Some(71_000_000.0)),
            (Finegrained, B110, None),
            //
            (Prestressed, B1_5, None),
            (Prestressed, B3_5, Some(2_700_000.0)),
            (Prestressed, B25, Some(18_500_000.0)),
            (Prestressed, B100, Some(71_000_000.0)),
            (Prestressed, B110, None),
            //
            (Lightweight, B1_5, None),
            (Lightweight, B2, None),
            (Lightweight, B2_5, Some(1_900_000.0)),
            (Lightweight, B3_5, Some(2_700_000.0)),
            (Lightweight, B5, Some(3_500_000.0)),
            (Lightweight, B7_5, Some(5_500_000.0)),
            (Lightweight, B10, Some(7_500_000.0)),
            (Lightweight, B12_5, Some(9_500_000.0)),
            (Lightweight, B15, Some(11_000_000.0)),
            (Lightweight, B20, Some(15_000_000.0)),
            (Lightweight, B25, Some(18_500_000.0)),
            (Lightweight, B30, Some(22_000_000.0)),
            (Lightweight, B35, Some(25_500_000.0)),
            (Lightweight, B40, Some(29_000_000.0)),
            (Lightweight, B45, None),
            (Lightweight, B50, None),
            (Lightweight, B55, None),
            (Lightweight, B60, None),
            (Lightweight, B70, None),
            (Lightweight, B80, None),
            (Lightweight, B90, None),
            (Lightweight, B100, None),
            (Lightweight, B110, None),
            (Lightweight, B120, None),
            //
            (Cellular, B1_5, Some(1_400_000.0)),
            (Cellular, B2, Some(1_900_000.0)),
            (Cellular, B2_5, Some(2_400_000.0)),
            (Cellular, B3_5, Some(3_300_000.0)),
            (Cellular, B5, Some(4_600_000.0)),
            (Cellular, B7_5, Some(6_900_000.0)),
            (Cellular, B10, Some(9_000_000.0)),
            (Cellular, B12_5, Some(10_500_000.0)),
            (Cellular, B15, Some(11_500_000.0)),
            (Cellular, B20, None),
            (Cellular, B25, None),
            (Cellular, B30, None),
            (Cellular, B35, None),
            (Cellular, B40, None),
            (Cellular, B45, None),
            (Cellular, B50, None),
            (Cellular, B55, None),
            (Cellular, B60, None),
            (Cellular, B70, None),
            (Cellular, B80, None),
            (Cellular, B90, None),
            (Cellular, B100, None),
            (Cellular, B110, None),
            (Cellular, B120, None),
        ];

        for (concrete_type, class, expected) in cases {
            assert_eq!(
                concrete(*concrete_type, *class).normative_axial_compression_resistance(),
                *expected,
                "type={:?}, class={:?}",
                concrete_type,
                class,
            );
        }
    }

    #[test]
    fn calculated_axial_compression_by_first_group() {
        use ConcreteClassForCompression::*;
        use ConcreteType::*;

        let cases: &[(ConcreteType, ConcreteClassForCompression, Option<f64>)] = &[
            //
            (Heavy, B1_5, None),
            (Heavy, B2, None),
            (Heavy, B2_5, None),
            (Heavy, B3_5, Some(2_700_000.0 / 1.3)),
            (Heavy, B5, Some(3_500_000.0 / 1.3)),
            (Heavy, B7_5, Some(5_500_000.0 / 1.3)),
            (Heavy, B10, Some(7_500_000.0 / 1.3)),
            (Heavy, B12_5, Some(9_500_000.0 / 1.3)),
            (Heavy, B15, Some(11_000_000.0 / 1.3)),
            (Heavy, B20, Some(15_000_000.0 / 1.3)),
            (Heavy, B25, Some(18_500_000.0 / 1.3)),
            (Heavy, B30, Some(22_000_000.0 / 1.3)),
            (Heavy, B35, Some(25_500_000.0 / 1.3)),
            (Heavy, B40, Some(29_000_000.0 / 1.3)),
            (Heavy, B45, Some(32_000_000.0 / 1.3)),
            (Heavy, B50, Some(36_000_000.0 / 1.3)),
            (Heavy, B55, Some(39_500_000.0 / 1.3)),
            (Heavy, B60, Some(43_000_000.0 / 1.3)),
            (Heavy, B70, Some(50_000_000.0 / 1.3)),
            (Heavy, B80, Some(57_000_000.0 / 1.3)),
            (Heavy, B90, Some(64_000_000.0 / 1.3)),
            (Heavy, B100, Some(71_000_000.0 / 1.3)),
            (Heavy, B110, None),
            (Heavy, B120, None),
            //
            (Finegrained, B1_5, None),
            (Finegrained, B25, Some(18_500_000.0 / 1.3)),
            (Finegrained, B100, Some(71_000_000.0 / 1.3)),
            (Finegrained, B110, None),
            //
            (Prestressed, B1_5, None),
            (Prestressed, B25, Some(18_500_000.0 / 1.3)),
            (Prestressed, B100, Some(71_000_000.0 / 1.3)),
            (Prestressed, B110, None),
            //
            (Lightweight, B1_5, None),
            (Lightweight, B2, None),
            (Lightweight, B2_5, Some(1_900_000.0 / 1.3)),
            (Lightweight, B25, Some(18_500_000.0 / 1.3)),
            (Lightweight, B40, Some(29_000_000.0 / 1.3)),
            (Lightweight, B45, None),
            (Lightweight, B110, None),
            //
            (Cellular, B1_5, Some(1_400_000.0 / 1.5)),
            (Cellular, B2, Some(1_900_000.0 / 1.5)),
            (Cellular, B2_5, Some(2_400_000.0 / 1.5)),
            (Cellular, B3_5, Some(3_300_000.0 / 1.5)),
            (Cellular, B5, Some(4_600_000.0 / 1.5)),
            (Cellular, B7_5, Some(6_900_000.0 / 1.5)),
            (Cellular, B10, Some(9_000_000.0 / 1.5)),
            (Cellular, B12_5, Some(10_500_000.0 / 1.5)),
            (Cellular, B15, Some(11_500_000.0 / 1.5)),
            (Cellular, B20, None),
            (Cellular, B25, None),
            (Cellular, B30, None),
            (Cellular, B35, None),
            (Cellular, B40, None),
            (Cellular, B45, None),
            (Cellular, B50, None),
            (Cellular, B55, None),
            (Cellular, B60, None),
            (Cellular, B70, None),
            (Cellular, B80, None),
            (Cellular, B90, None),
            (Cellular, B100, None),
            (Cellular, B110, None),
            (Cellular, B120, None),
        ];

        for (concrete_type, class, expected) in cases {
            assert_eq!(
                concrete(*concrete_type, *class).calculated_axial_compression_by_first_group(),
                *expected,
                "type={:?}, class={:?}",
                concrete_type,
                class,
            );
        }
    }

    #[test]
    fn normative_axial_tension_resistance() {
        use ConcreteClassForCompression::*;
        use ConcreteType::*;

        let cases: &[(ConcreteType, ConcreteClassForCompression, Option<f64>)] = &[
            //
            (Heavy, B1_5, None),
            (Heavy, B2, None),
            (Heavy, B2_5, None),
            (Heavy, B3_5, Some(390_000.0)),
            (Heavy, B5, Some(550_000.0)),
            (Heavy, B7_5, Some(700_000.0)),
            (Heavy, B10, Some(850_000.0)),
            (Heavy, B12_5, Some(1_000_000.0)),
            (Heavy, B15, Some(1_100_000.0)),
            (Heavy, B20, Some(1_350_000.0)),
            (Heavy, B25, Some(1_550_000.0)),
            (Heavy, B30, Some(1_750_000.0)),
            (Heavy, B35, Some(1_950_000.0)),
            (Heavy, B40, Some(2_100_000.0)),
            (Heavy, B45, Some(2_250_000.0)),
            (Heavy, B50, Some(2_450_000.0)),
            (Heavy, B55, Some(2_600_000.0)),
            (Heavy, B60, Some(2_750_000.0)),
            (Heavy, B70, Some(3_000_000.0)),
            (Heavy, B80, Some(3_300_000.0)),
            (Heavy, B90, Some(3_600_000.0)),
            (Heavy, B100, Some(3_800_000.0)),
            (Heavy, B110, None),
            (Heavy, B120, None),
            //
            (Finegrained, B1_5, None),
            (Finegrained, B3_5, Some(390_000.0)),
            (Finegrained, B25, Some(1_550_000.0)),
            (Finegrained, B100, Some(3_800_000.0)),
            (Finegrained, B110, None),
            //
            (Prestressed, B1_5, None),
            (Prestressed, B2, None),
            (Prestressed, B2_5, None),
            (Prestressed, B3_5, Some(390_000.0 * 1.2)),
            (Prestressed, B5, Some(550_000.0 * 1.2)),
            (Prestressed, B7_5, Some(700_000.0 * 1.2)),
            (Prestressed, B10, Some(850_000.0 * 1.2)),
            (Prestressed, B12_5, Some(1_000_000.0 * 1.2)),
            (Prestressed, B15, Some(1_100_000.0 * 1.2)),
            (Prestressed, B20, Some(1_350_000.0 * 1.2)),
            (Prestressed, B25, Some(1_550_000.0 * 1.2)),
            (Prestressed, B30, Some(1_750_000.0 * 1.2)),
            (Prestressed, B35, Some(1_950_000.0 * 1.2)),
            (Prestressed, B40, Some(2_100_000.0 * 1.2)),
            (Prestressed, B45, Some(2_250_000.0 * 1.2)),
            (Prestressed, B50, Some(2_450_000.0 * 1.2)),
            (Prestressed, B55, Some(2_600_000.0 * 1.2)),
            (Prestressed, B60, Some(2_750_000.0 * 1.2)),
            (Prestressed, B70, Some(3_000_000.0 * 1.2)),
            (Prestressed, B80, Some(3_300_000.0 * 1.2)),
            (Prestressed, B90, Some(3_600_000.0 * 1.2)),
            (Prestressed, B100, Some(3_800_000.0 * 1.2)),
            (Prestressed, B110, None),
            (Prestressed, B120, None),
            //
            (Lightweight, B1_5, None),
            (Lightweight, B2, None),
            (Lightweight, B2_5, Some(290_000.0)),
            (Lightweight, B3_5, Some(390_000.0)),
            (Lightweight, B5, Some(550_000.0)),
            (Lightweight, B7_5, Some(700_000.0)),
            (Lightweight, B10, Some(850_000.0)),
            (Lightweight, B12_5, Some(1_000_000.0)),
            (Lightweight, B15, Some(1_100_000.0)),
            (Lightweight, B20, Some(1_350_000.0)),
            (Lightweight, B25, Some(1_550_000.0)),
            (Lightweight, B30, Some(1_750_000.0)),
            (Lightweight, B35, Some(1_950_000.0)),
            (Lightweight, B40, Some(2_100_000.0)),
            (Lightweight, B45, None),
            (Lightweight, B50, None),
            (Lightweight, B55, None),
            (Lightweight, B60, None),
            (Lightweight, B70, None),
            (Lightweight, B80, None),
            (Lightweight, B90, None),
            (Lightweight, B100, None),
            (Lightweight, B110, None),
            (Lightweight, B120, None),
            //
            (Cellular, B1_5, Some(220_000.0)),
            (Cellular, B2, Some(260_000.0)),
            (Cellular, B2_5, Some(310_000.0)),
            (Cellular, B3_5, Some(410_000.0)),
            (Cellular, B5, Some(550_000.0)),
            (Cellular, B7_5, Some(630_000.0)),
            (Cellular, B10, Some(890_000.0)),
            (Cellular, B12_5, Some(1_000_000.0)),
            (Cellular, B15, Some(1_050_000.0)),
            (Cellular, B20, None),
            (Cellular, B25, None),
            (Cellular, B30, None),
            (Cellular, B35, None),
            (Cellular, B40, None),
            (Cellular, B45, None),
            (Cellular, B50, None),
            (Cellular, B55, None),
            (Cellular, B60, None),
            (Cellular, B70, None),
            (Cellular, B80, None),
            (Cellular, B90, None),
            (Cellular, B100, None),
            (Cellular, B110, None),
            (Cellular, B120, None),
        ];

        for (concrete_type, class, expected) in cases {
            assert_eq!(
                concrete(*concrete_type, *class).normative_axial_tension_resistance(),
                *expected,
                "type={:?}, class={:?}",
                concrete_type,
                class,
            );
        }
    }

    #[test]
    fn calculated_axial_tension_by_second_group() {
        use ConcreteClassForCompression::*;
        use ConcreteType::*;

        let cases: &[(ConcreteType, ConcreteClassForCompression, Option<f64>)] = &[
            //
            (Heavy, B25, Some(1_550_000.0)),
            (Finegrained, B25, Some(1_550_000.0)),
            (Prestressed, B25, Some(1_550_000.0 * 1.2)),
            (Lightweight, B25, Some(1_550_000.0)),
            (Cellular, B10, Some(890_000.0)),
            //
            (Heavy, B1_5, None),
            (Lightweight, B45, None),
            (Cellular, B20, None),
            (Heavy, B110, None),
        ];

        for (concrete_type, class, expected) in cases {
            assert_eq!(
                concrete(*concrete_type, *class).calculated_axial_tension_by_second_group(),
                *expected,
                "type={:?}, class={:?}",
                concrete_type,
                class,
            );
        }
    }

    #[test]
    fn calculated_axial_tension_by_first_group() {
        use ConcreteClassForAxialTension::*;
        use ConcreteClassForCompression::*;
        use ConcreteType::*;

        let cases: &[(
            ConcreteType,
            ConcreteClassForCompression,
            Option<ConcreteClassForAxialTension>,
            Option<f64>,
        )] = &[
            //
            (Cellular, B1_5, None, Some(220_000.0 / 2.5)),
            (Cellular, B2, None, Some(260_000.0 / 2.5)),
            (Cellular, B2_5, None, Some(310_000.0 / 2.5)),
            (Cellular, B3_5, None, Some(410_000.0 / 2.5)),
            (Cellular, B5, None, Some(550_000.0 / 2.5)),
            (Cellular, B7_5, None, Some(630_000.0 / 2.5)),
            (Cellular, B10, None, Some(890_000.0 / 2.5)),
            (Cellular, B12_5, None, Some(1_000_000.0 / 2.5)),
            (Cellular, B15, None, Some(1_050_000.0 / 2.5)),
            (Cellular, B20, None, None),
            (Cellular, B25, None, None),
            (Cellular, B30, None, None),
            (Cellular, B35, None, None),
            (Cellular, B40, None, None),
            (Cellular, B45, None, None),
            (Cellular, B50, None, None),
            (Cellular, B55, None, None),
            (Cellular, B60, None, None),
            (Cellular, B70, None, None),
            (Cellular, B80, None, None),
            (Cellular, B90, None, None),
            (Cellular, B100, None, None),
            (Cellular, B110, None, None),
            (Cellular, B120, None, None),
            //
            (Heavy, B1_5, None, None),
            (Heavy, B25, None, Some(1_550_000.0 / 1.5)),
            (Heavy, B100, None, Some(3_800_000.0 / 1.5)),
            (Heavy, B110, None, None),
            (Finegrained, B25, None, Some(1_550_000.0 / 1.5)),
            (Prestressed, B25, None, Some(1_550_000.0 * 1.2 / 1.5)),
            (Lightweight, B25, None, Some(1_550_000.0 / 1.5)),
            (Lightweight, B45, None, None),
            //
            (Heavy, B1_5, Some(B2_0), None),
            (Heavy, B25, Some(B2_0), Some(1_550_000.0 / 1.3)),
            (Heavy, B100, Some(B2_0), Some(3_800_000.0 / 1.3)),
            (Heavy, B110, Some(B2_0), None),
            (Finegrained, B25, Some(B2_0), Some(1_550_000.0 / 1.3)),
            (Prestressed, B25, Some(B2_0), Some(1_550_000.0 * 1.2 / 1.3)),
            (Lightweight, B25, Some(B2_0), Some(1_550_000.0 / 1.3)),
            (Lightweight, B45, Some(B2_0), None),
        ];

        for (concrete_type, class, axial, expected) in cases {
            let c = match axial {
                Some(a) => concrete_with_axial_tension(*concrete_type, *class, *a),
                None => concrete(*concrete_type, *class),
            };
            assert_eq!(
                c.calculated_axial_tension_by_first_group(),
                *expected,
                "type={:?}, class={:?}, axial={:?}",
                concrete_type,
                class,
                axial,
            );
        }
    }
}
