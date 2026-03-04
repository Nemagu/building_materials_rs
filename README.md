# building_materials

Rust library providing types and normative material properties for concrete and rebar,
based on Russian construction standards.

## Contents

- **`concrete`** — `Concrete` struct with classification characteristics (compression class,
  tension classes, frost resistance grades, water resistance grade) per GOST 26633-2015.
- **`rebar`** — `Rebar` struct with rebar class and surface profile per GOST 34028-2016.
- **`standard::sp_63_13330_2018`** — normative and design resistances for concrete and rebar
  per SP 63.13330.2018.

## Features

- `serde` — enables `Serialize`/`Deserialize` for all types.

## Usage

```toml
[dependencies]
building_materials = "0.1"
```
