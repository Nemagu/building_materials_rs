use crate::rebar::{Rebar, RebarClass};

/// Rebar material properties per SP 63.13330.2018.
pub struct RebarBySp63_13330_2018 {
    material: Rebar,
}

impl RebarBySp63_13330_2018 {
    pub fn new(material: Rebar) -> Self {
        Self { material }
    }

    /// Returns the elastic modulus **Es** (Pa).
    ///
    /// Source: SP 63.13330.2018.
    ///
    /// Returns `None` if the rebar class is not covered by the standard.
    pub fn elastic_modulus(&self) -> Option<f64> {
        match self.material.rebar_class() {
            RebarClass::A240
            | RebarClass::A400
            | RebarClass::A500
            | RebarClass::A600
            | RebarClass::A800
            | RebarClass::A1000
            | RebarClass::B500
            | RebarClass::Br500
            | RebarClass::Br1200
            | RebarClass::Br1300
            | RebarClass::Br1400
            | RebarClass::Br1500
            | RebarClass::Br1600 => Some(20_000_000_000.0),
            RebarClass::K1400
            | RebarClass::K1500
            | RebarClass::K1600
            | RebarClass::K1700
            | RebarClass::K1800
            | RebarClass::K1900 => Some(19_500_000_000.0),
            _ => None,
        }
    }

    /// Alias for [`Self::elastic_modulus`].
    pub fn es(&self) -> Option<f64> {
        self.elastic_modulus()
    }

    /// Returns the normative tensile resistance **Rsn, Rs,ser** (Pa)
    /// for second group of load.
    ///
    /// Source: SP 63.13330.2018, Table 6.13.
    ///
    /// Returns `None` if the rebar class is not covered by the standard.
    pub fn normative_tension_resistance(&self) -> Option<f64> {
        match self.material.rebar_class() {
            RebarClass::A240 => Some(240_000_000.0),
            RebarClass::A400 => Some(400_000_000.0),
            RebarClass::A500 => Some(500_000_000.0),
            RebarClass::A600 => Some(600_000_000.0),
            RebarClass::A800 => Some(800_000_000.0),
            RebarClass::A1000 => Some(1_000_000_000.0),
            RebarClass::B500 => Some(500_000_000.0),
            RebarClass::Br500 => Some(500_000_000.0),
            RebarClass::Br1200 => Some(1_200_000_000.0),
            RebarClass::Br1300 => Some(1_300_000_000.0),
            RebarClass::Br1400 => Some(1_400_000_000.0),
            RebarClass::Br1500 => Some(1_500_000_000.0),
            RebarClass::Br1600 => Some(1_600_000_000.0),
            RebarClass::K1400 => Some(1_400_000_000.0),
            RebarClass::K1500 => Some(1_500_000_000.0),
            RebarClass::K1600 => Some(1_600_000_000.0),
            RebarClass::K1700 => Some(1_700_000_000.0),
            RebarClass::K1800 => Some(1_800_000_000.0),
            RebarClass::K1900 => Some(1_900_000_000.0),
            _ => None,
        }
    }

    /// Alias for [`Self::normative_tension_resistance`].
    pub fn rsn(&self) -> Option<f64> {
        self.normative_tension_resistance()
    }

    /// Returns the calculated tensile resistance for second group of load **Rs,ser** (Pa).
    ///
    /// Equals [`Self::normative_tension_resistance`].
    pub fn calculated_tension_resistance_by_second_group(&self) -> Option<f64> {
        self.normative_tension_resistance()
    }

    /// Alias for [`Self::calculated_tension_resistance_by_second_group`].
    pub fn rsser(&self) -> Option<f64> {
        self.normative_tension_resistance()
    }

    /// Returns the calculated tensile resistance for first group of load **Rs** (Pa).
    ///
    /// Derived from [`Self::normative_tension_resistance`] divided by
    /// safety coefficient 6.13.
    pub fn calculated_tension_resistance_by_first_group(&self) -> Option<f64> {
        if let Some(normative_value) = self.normative_tension_resistance() {
            Some(normative_value / 1.15)
        } else {
            None
        }
    }

    /// Alias for [`Self::calculated_tension_resistance_by_first_group`].
    pub fn rs(&self) -> Option<f64> {
        self.calculated_tension_resistance_by_first_group()
    }

    /// Returns the calculated compressive resistance under long-term load **Rs'** (Pa)
    /// for first group of load.
    ///
    /// Source: SP 63.13330.2018, Table 6.14.
    ///
    /// Returns `None` if the rebar class is not covered by the standard.
    pub fn calculated_compression_resistance_by_long_term(&self) -> Option<f64> {
        match self.material.rebar_class() {
            RebarClass::A240 => Some(210_000_000.0),
            RebarClass::A400 => Some(350_000_000.0),
            RebarClass::A500 => Some(435_000_000.0),
            RebarClass::A600 => Some(470_000_000.0),
            RebarClass::A800 => Some(500_000_000.0),
            RebarClass::A1000 => Some(500_000_000.0),
            RebarClass::B500 => Some(415_000_000.0),
            RebarClass::Br500 => Some(390_000_000.0),
            RebarClass::Br1200 => Some(500_000_000.0),
            RebarClass::Br1300 => Some(500_000_000.0),
            RebarClass::Br1400 => Some(500_000_000.0),
            RebarClass::Br1500 => Some(500_000_000.0),
            RebarClass::Br1600 => Some(500_000_000.0),
            RebarClass::K1400 => Some(500_000_000.0),
            RebarClass::K1500 => Some(500_000_000.0),
            RebarClass::K1600 => Some(500_000_000.0),
            RebarClass::K1700 => Some(500_000_000.0),
            RebarClass::K1800 => Some(500_000_000.0),
            RebarClass::K1900 => Some(500_000_000.0),
            _ => None,
        }
    }

    /// Alias for [`Self::calculated_compression_resistance_by_long_term`].
    pub fn rs_long_term(&self) -> Option<f64> {
        self.calculated_compression_resistance_by_long_term()
    }

    /// Returns the calculated compressive resistance under short-term load **Rs'** (Pa)
    /// for first group of load.
    ///
    /// Source: SP 63.13330.2018, Table 6.14.
    ///
    /// Returns `None` if the rebar class is not covered by the standard.
    pub fn calculated_compression_resistance_by_short_term(&self) -> Option<f64> {
        match self.material.rebar_class() {
            RebarClass::A240 => Some(210_000_000.0),
            RebarClass::A400 => Some(350_000_000.0),
            RebarClass::A500 => Some(400_000_000.0),
            RebarClass::A600 => Some(400_000_000.0),
            RebarClass::A800 => Some(400_000_000.0),
            RebarClass::A1000 => Some(400_000_000.0),
            RebarClass::B500 => Some(380_000_000.0),
            RebarClass::Br500 => Some(360_000_000.0),
            RebarClass::Br1200 => Some(400_000_000.0),
            RebarClass::Br1300 => Some(400_000_000.0),
            RebarClass::Br1400 => Some(400_000_000.0),
            RebarClass::Br1500 => Some(400_000_000.0),
            RebarClass::Br1600 => Some(400_000_000.0),
            RebarClass::K1400 => Some(400_000_000.0),
            RebarClass::K1500 => Some(400_000_000.0),
            RebarClass::K1600 => Some(400_000_000.0),
            RebarClass::K1700 => Some(400_000_000.0),
            RebarClass::K1800 => Some(400_000_000.0),
            RebarClass::K1900 => Some(400_000_000.0),
            _ => None,
        }
    }

    /// Alias for [`Self::calculated_compression_resistance_by_short_term`].
    pub fn rs_short_term(&self) -> Option<f64> {
        self.calculated_compression_resistance_by_short_term()
    }

    /// Returns the calculated axial tensile resistance for clamps and bent bars **Rsw** (Pa)
    /// for first group of load.
    ///
    /// Source: SP 63.13330.2018, Table 6.15.
    ///
    /// Returns `None` if the rebar class is not covered by the standard.
    pub fn calculated_axial_tension_by_first_group_for_clamp_and_bent_rod(&self) -> Option<f64> {
        match self.material.rebar_class() {
            RebarClass::A240 => Some(170_000_000.0),
            RebarClass::A400 => Some(280_000_000.0),
            RebarClass::A500 => Some(300_000_000.0),
            RebarClass::A600 => Some(300_000_000.0),
            RebarClass::A800 => Some(300_000_000.0),
            RebarClass::A1000 => Some(300_000_000.0),
            RebarClass::B500 => Some(300_000_000.0),
            RebarClass::Br500 => Some(300_000_000.0),
            RebarClass::Br1200 => Some(300_000_000.0),
            RebarClass::Br1300 => Some(300_000_000.0),
            RebarClass::Br1400 => Some(300_000_000.0),
            RebarClass::Br1500 => Some(300_000_000.0),
            RebarClass::Br1600 => Some(300_000_000.0),
            RebarClass::K1400 => Some(300_000_000.0),
            RebarClass::K1500 => Some(300_000_000.0),
            RebarClass::K1600 => Some(300_000_000.0),
            RebarClass::K1700 => Some(300_000_000.0),
            RebarClass::K1800 => Some(300_000_000.0),
            RebarClass::K1900 => Some(300_000_000.0),
            _ => None,
        }
    }

    /// Alias for [`Self::calculated_axial_tension_by_first_group_for_clamp_and_bent_rod`].
    pub fn rsw(&self) -> Option<f64> {
        self.calculated_axial_tension_by_first_group_for_clamp_and_bent_rod()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rebar::{Rebar, RebarClass};

    fn rebar(class: RebarClass) -> RebarBySp63_13330_2018 {
        RebarBySp63_13330_2018::new(Rebar::builder().rebar_class(class).build())
    }

    #[test]
    fn elastic_modulus() {
        use RebarClass::*;

        let cases: &[(RebarClass, Option<f64>)] = &[
            //
            (A240, Some(20_000_000_000.0)),
            (A400, Some(20_000_000_000.0)),
            (A500, Some(20_000_000_000.0)),
            (A600, Some(20_000_000_000.0)),
            (A800, Some(20_000_000_000.0)),
            (A1000, Some(20_000_000_000.0)),
            (B500, Some(20_000_000_000.0)),
            (Br500, Some(20_000_000_000.0)),
            (Br1200, Some(20_000_000_000.0)),
            (Br1300, Some(20_000_000_000.0)),
            (Br1400, Some(20_000_000_000.0)),
            (Br1500, Some(20_000_000_000.0)),
            (Br1600, Some(20_000_000_000.0)),
            //
            (K1400, Some(19_500_000_000.0)),
            (K1500, Some(19_500_000_000.0)),
            (K1600, Some(19_500_000_000.0)),
            (K1700, Some(19_500_000_000.0)),
            (K1800, Some(19_500_000_000.0)),
            (K1900, Some(19_500_000_000.0)),
            //
            (Ap600, None),
        ];

        for (class, expected) in cases {
            assert_eq!(
                rebar(*class).elastic_modulus(),
                *expected,
                "class={:?}",
                class,
            );
        }
    }

    #[test]
    fn normative_tension_resistance() {
        use RebarClass::*;

        let cases: &[(RebarClass, Option<f64>)] = &[
            (A240, Some(240_000_000.0)),
            (A400, Some(400_000_000.0)),
            (A500, Some(500_000_000.0)),
            (A600, Some(600_000_000.0)),
            (A800, Some(800_000_000.0)),
            (A1000, Some(1_000_000_000.0)),
            (B500, Some(500_000_000.0)),
            (Br500, Some(500_000_000.0)),
            (Br1200, Some(1_200_000_000.0)),
            (Br1300, Some(1_300_000_000.0)),
            (Br1400, Some(1_400_000_000.0)),
            (Br1500, Some(1_500_000_000.0)),
            (Br1600, Some(1_600_000_000.0)),
            (K1400, Some(1_400_000_000.0)),
            (K1500, Some(1_500_000_000.0)),
            (K1600, Some(1_600_000_000.0)),
            (K1700, Some(1_700_000_000.0)),
            (K1800, Some(1_800_000_000.0)),
            (K1900, Some(1_900_000_000.0)),
            //
            (Ap600, None),
        ];

        for (class, expected) in cases {
            assert_eq!(
                rebar(*class).normative_tension_resistance(),
                *expected,
                "class={:?}",
                class,
            );
        }
    }

    #[test]
    fn calculated_tension_resistance_by_first_group() {
        use RebarClass::*;

        let cases: &[(RebarClass, Option<f64>)] = &[
            (A240, Some(240_000_000.0 / 1.15)),
            (A400, Some(400_000_000.0 / 1.15)),
            (A500, Some(500_000_000.0 / 1.15)),
            (A600, Some(600_000_000.0 / 1.15)),
            (A800, Some(800_000_000.0 / 1.15)),
            (A1000, Some(1_000_000_000.0 / 1.15)),
            (B500, Some(500_000_000.0 / 1.15)),
            (Br500, Some(500_000_000.0 / 1.15)),
            (Br1200, Some(1_200_000_000.0 / 1.15)),
            (Br1300, Some(1_300_000_000.0 / 1.15)),
            (Br1400, Some(1_400_000_000.0 / 1.15)),
            (Br1500, Some(1_500_000_000.0 / 1.15)),
            (Br1600, Some(1_600_000_000.0 / 1.15)),
            (K1400, Some(1_400_000_000.0 / 1.15)),
            (K1500, Some(1_500_000_000.0 / 1.15)),
            (K1600, Some(1_600_000_000.0 / 1.15)),
            (K1700, Some(1_700_000_000.0 / 1.15)),
            (K1800, Some(1_800_000_000.0 / 1.15)),
            (K1900, Some(1_900_000_000.0 / 1.15)),
            //
            (Ap600, None),
        ];

        for (class, expected) in cases {
            assert_eq!(
                rebar(*class).calculated_tension_resistance_by_first_group(),
                *expected,
                "class={:?}",
                class,
            );
        }
    }

    #[test]
    fn calculated_compression_resistance_by_long_term() {
        use RebarClass::*;

        let cases: &[(RebarClass, Option<f64>)] = &[
            (A240, Some(210_000_000.0)),
            (A400, Some(350_000_000.0)),
            (A500, Some(435_000_000.0)),
            (A600, Some(470_000_000.0)),
            (A800, Some(500_000_000.0)),
            (A1000, Some(500_000_000.0)),
            (B500, Some(415_000_000.0)),
            (Br500, Some(390_000_000.0)),
            (Br1200, Some(500_000_000.0)),
            (Br1300, Some(500_000_000.0)),
            (Br1400, Some(500_000_000.0)),
            (Br1500, Some(500_000_000.0)),
            (Br1600, Some(500_000_000.0)),
            (K1400, Some(500_000_000.0)),
            (K1500, Some(500_000_000.0)),
            (K1600, Some(500_000_000.0)),
            (K1700, Some(500_000_000.0)),
            (K1800, Some(500_000_000.0)),
            (K1900, Some(500_000_000.0)),
            //
            (Ap600, None),
        ];

        for (class, expected) in cases {
            assert_eq!(
                rebar(*class).calculated_compression_resistance_by_long_term(),
                *expected,
                "class={:?}",
                class,
            );
        }
    }

    #[test]
    fn calculated_compression_resistance_by_short_term() {
        use RebarClass::*;

        let cases: &[(RebarClass, Option<f64>)] = &[
            (A240, Some(210_000_000.0)),
            (A400, Some(350_000_000.0)),
            (A500, Some(400_000_000.0)),
            (A600, Some(400_000_000.0)),
            (A800, Some(400_000_000.0)),
            (A1000, Some(400_000_000.0)),
            (B500, Some(380_000_000.0)),
            (Br500, Some(360_000_000.0)),
            (Br1200, Some(400_000_000.0)),
            (Br1300, Some(400_000_000.0)),
            (Br1400, Some(400_000_000.0)),
            (Br1500, Some(400_000_000.0)),
            (Br1600, Some(400_000_000.0)),
            (K1400, Some(400_000_000.0)),
            (K1500, Some(400_000_000.0)),
            (K1600, Some(400_000_000.0)),
            (K1700, Some(400_000_000.0)),
            (K1800, Some(400_000_000.0)),
            (K1900, Some(400_000_000.0)),
            //
            (Ap600, None),
        ];

        for (class, expected) in cases {
            assert_eq!(
                rebar(*class).calculated_compression_resistance_by_short_term(),
                *expected,
                "class={:?}",
                class,
            );
        }
    }

    #[test]
    fn calculated_axial_tension_by_first_group_for_clamp_and_bent_rod() {
        use RebarClass::*;

        let cases: &[(RebarClass, Option<f64>)] = &[
            (A240, Some(170_000_000.0)),
            (A400, Some(280_000_000.0)),
            (A500, Some(300_000_000.0)),
            (A600, Some(300_000_000.0)),
            (A800, Some(300_000_000.0)),
            (A1000, Some(300_000_000.0)),
            (B500, Some(300_000_000.0)),
            (Br500, Some(300_000_000.0)),
            (Br1200, Some(300_000_000.0)),
            (Br1300, Some(300_000_000.0)),
            (Br1400, Some(300_000_000.0)),
            (Br1500, Some(300_000_000.0)),
            (Br1600, Some(300_000_000.0)),
            (K1400, Some(300_000_000.0)),
            (K1500, Some(300_000_000.0)),
            (K1600, Some(300_000_000.0)),
            (K1700, Some(300_000_000.0)),
            (K1800, Some(300_000_000.0)),
            (K1900, Some(300_000_000.0)),
            //
            (Ap600, None),
        ];

        for (class, expected) in cases {
            assert_eq!(
                rebar(*class).calculated_axial_tension_by_first_group_for_clamp_and_bent_rod(),
                *expected,
                "class={:?}",
                class,
            );
        }
    }
}
