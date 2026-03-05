# building_materials

Rust library providing types and normative material properties for concrete and rebar,
based on Russian construction standards.

## Installation

```toml
[dependencies]
building_materials = "0.1"
```

With serde support:

```toml
[dependencies]
building_materials = { version = "0.1", features = ["serde"] }
```

## Feature flags

- `serde` — enables `Serialize`/`Deserialize` for all types.

---

## `concrete`

### `Concrete`

Concrete with its classification characteristics. Built via the `bon` builder.
Compression class is mandatory; all other fields are optional.

```rust
use building_materials::concrete::{
    Concrete, ConcreteClassForCompression, ConcreteClassForAxialTension,
    ConcreteGradeForFrostResistanceByFirstMethod, ConcreteGradeForWaterResistance,
};

let c = Concrete::builder()
    .class_for_compression(ConcreteClassForCompression::B25)
    .class_for_axial_tension(ConcreteClassForAxialTension::B2_0)
    .grade_for_frost_resistance_by_first_method(ConcreteGradeForFrostResistanceByFirstMethod::F200)
    .grade_for_water_resistance(ConcreteGradeForWaterResistance::W6)
    .build();

let class = c.class_for_compression();
let axial = c.class_for_axial_tension(); // Option<_>
```

| Method | Returns |
|---|---|
| `class_for_compression()` | `ConcreteClassForCompression` |
| `class_for_axial_tension()` | `Option<ConcreteClassForAxialTension>` |
| `class_for_flexural_tension()` | `Option<ConcreteClassForFlexuralTension>` |
| `grade_for_frost_resistance_by_first_method()` | `Option<ConcreteGradeForFrostResistanceByFirstMethod>` |
| `grade_for_frost_resistance_by_second_method()` | `Option<ConcreteGradeForFrostResistanceBySecondMethod>` |
| `grade_for_water_resistance()` | `Option<ConcreteGradeForWaterResistance>` |

### Enums

| Type | Variants |
|---|---|
| `ConcreteClassForCompression` | `B1_5` … `B120` |
| `ConcreteClassForAxialTension` | `B0_8` … `B4_8` |
| `ConcreteClassForFlexuralTension` | `B1_2` … `B10_0` |
| `ConcreteGradeForFrostResistanceByFirstMethod` | `F50` … `F1000` |
| `ConcreteGradeForFrostResistanceBySecondMethod` | `F100` … `F500` |
| `ConcreteGradeForWaterResistance` | `W2` … `W20` |

---

## `rebar`

### `Rebar`

A reinforcing bar. Built via the `bon` builder.
Rebar class is mandatory; configuration profile is optional.

```rust
use building_materials::rebar::{Rebar, RebarClass, RebarConfigurationProfile};

let r = Rebar::builder()
    .rebar_class(RebarClass::A400)
    .configuration_profile(RebarConfigurationProfile::F1)
    .build();

let class = r.rebar_class();
let profile = r.configuration_profile(); // Option<_>
```

| Method | Returns |
|---|---|
| `rebar_class()` | `RebarClass` |
| `configuration_profile()` | `Option<RebarConfigurationProfile>` |

### Enums

| Type | Variants |
|---|---|
| `RebarClass` | `A240`, `A400`, `A500`, `A600`, `Ap600`, `A800`, `A1000`, `B500`, `Br500`, `Br1200`…`Br1600`, `K1400`…`K1900` |
| `RebarConfigurationProfile` | `F1`, `F2`, `F3`, `F4` |

---

## `standard::sp_63_13330_2018`

### `ConcreteBySp63_13330_2018`

Provides design and normative resistances for a concrete material.
Takes a `Concrete` and a `ConcreteType`. All methods return `Option<f64>` in **Pa** —
`None` if the class/type combination is not covered by the standard.

```rust
use building_materials::concrete::{Concrete, ConcreteClassForCompression};
use building_materials::standard::sp_63_13330_2018::{ConcreteBySp63_13330_2018, ConcreteType};

let c = ConcreteBySp63_13330_2018::new(
    Concrete::builder()
        .class_for_compression(ConcreteClassForCompression::B25)
        .build(),
    ConcreteType::Heavy,
);

let rb  = c.rb();   // or c.calculated_axial_compression_by_first_group()
let rbn = c.rbn();  // or c.normative_axial_compression_resistance()
let rbt = c.rbt();  // or c.calculated_axial_tension_by_first_group()
```

#### `ConcreteType`

| Variant | Note |
|---|---|
| `Heavy` | — |
| `Finegrained { fineness_modulus: f64 }` | Tension resistance multiplied by 0.8 when `fineness_modulus ≤ 2.0` |
| `Prestressed` | Tension resistance multiplied by 1.2 |
| `Lightweight` | — |
| `Cellular` | — |

#### Methods

| Method | Short alias | Description |
|---|---|---|
| `normative_axial_compression_resistance()` | `rbn()` | Rbn / Rb,ser |
| `calculated_axial_compression_by_second_group()` | `rbser()` | equals `rbn()` |
| `calculated_axial_compression_by_first_group()` | `rb()` | Rb |
| `normative_axial_tension_resistance()` | `rbtn()` | Rbtn / Rbt,ser |
| `calculated_axial_tension_by_second_group()` | `rbtser()` | equals `rbtn()` |
| `calculated_axial_tension_by_first_group()` | `rbt()` | Rbt; uses axial tension class when set |

---

### `RebarBySp63_13330_2018`

Provides design and normative resistances for a rebar material.
Takes a `Rebar`. All methods return `Option<f64>` in **Pa** —
`None` if the rebar class is not covered by the standard.

```rust
use building_materials::rebar::{Rebar, RebarClass};
use building_materials::standard::sp_63_13330_2018::RebarBySp63_13330_2018;

let r = RebarBySp63_13330_2018::new(
    Rebar::builder().rebar_class(RebarClass::A400).build(),
);

let rs  = r.rs();   // or r.calculated_tension_resistance_by_first_group()
let rsn = r.rsn();  // or r.normative_tension_resistance()
let es  = r.es();   // or r.elastic_modulus()
```

#### Methods

| Method | Short alias | Description |
|---|---|---|
| `elastic_modulus()` | `es()` | Es |
| `normative_tension_resistance()` | `rsn()` | Rsn / Rs,ser |
| `calculated_tension_resistance_by_second_group()` | `rsser()` | equals `rsn()` |
| `calculated_tension_resistance_by_first_group()` | `rs()` | Rs = Rsn / 1.15 |
| `calculated_compression_resistance_by_long_term()` | `rs_long_term()` | Rs' under long-term load |
| `calculated_compression_resistance_by_short_term()` | `rs_short_term()` | Rs' under short-term load |
| `calculated_axial_tension_by_first_group_for_clamp_and_bent_rod()` | `rsw()` | Rsw for clamps and bent bars |
