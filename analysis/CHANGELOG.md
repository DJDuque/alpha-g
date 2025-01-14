# Changelog

Note that this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html) and all notable
changes will be documented in this file.

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.5.9] - 2024-10-27

### Fixed

- Bump `alpha_g_physics` to version `0.1.5`, see
  [its changelog](https://github.com/ALPHA-g-Experiment/alpha-g/blob/main/physics/CHANGELOG.md#015---2024-10-27)
  for more details. This updates the wire gain calibration.

## [0.5.8] - 2024-10-16

### Fixed

- Bump `alpha_g_physics` to version `0.1.4`, see
  [its changelog](https://github.com/ALPHA-g-Experiment/alpha-g/blob/main/physics/CHANGELOG.md#014---2024-10-16)
  for more details. This updates the wire and pad calibration for 2024 data.

## [0.5.7] - 2024-08-23

### Fixed

- Bump `alpha_g_detector` to version `0.5.1`, see
  [its changelog](https://github.com/ALPHA-g-Experiment/alpha-g/blob/main/detector/CHANGELOG.md#051---2024-08-22)
  for more details. This fixes a panic caused by missing AWB and PWB maps.
- Bump `alpha_g_physics` to version `0.1.3`, see
  [its changelog](https://github.com/ALPHA-g-Experiment/alpha-g/blob/main/physics/CHANGELOG.md#013---2024-08-22)
  for more details. This fixes a panic caused by missing wire and pad
  calibration files.

## [0.5.6] - 2024-08-09

### Fixed

- Installing pre-built binaries when the glibc version is too old (e.g.
  `alpha03`) is now possible using musl.

## [0.5.5] - 2024-08-07

Nothing changed for this release. It is just made to test the new release
workflow.

<!-- next-url -->
[Unreleased]: https://github.com/ALPHA-g-Experiment/alpha-g/compare/alpha-g-analysis-v0.5.9...HEAD
[0.5.9]: https://github.com/ALPHA-g-Experiment/alpha-g/compare/alpha-g-analysis-v0.5.8...alpha-g-analysis-v0.5.9
[0.5.8]: https://github.com/ALPHA-g-Experiment/alpha-g/compare/alpha-g-analysis-v0.5.7...alpha-g-analysis-v0.5.8
[0.5.7]: https://github.com/ALPHA-g-Experiment/alpha-g/compare/alpha-g-analysis-v0.5.6...alpha-g-analysis-v0.5.7
[0.5.6]: https://github.com/ALPHA-g-Experiment/alpha-g/compare/alpha-g-analysis-v0.5.5...alpha-g-analysis-v0.5.6
[0.5.5]: https://github.com/ALPHA-g-Experiment/alpha-g/compare/alpha-g-analysis-v0.5.4...alpha-g-analysis-v0.5.5
