# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

### [0.7.0] - 2017-12-07

### Added

- Add methods to get Options' inner value [c:4d67790]
- Add reqwest support [c:8946cc2] ([@zeyla]), [c:0e825b0] ([@mklein994])
- Update to hyper 0.11 [c:ef39b9f]

### Changed

- `API_URL` is now located at `constants::API_URL` and is now a constant
  [c:14c172a]
- Models are now located in their own publicly exported `models` module
  [c:819fe80]
- Modularize HTTP client support [c:8fb8419]

### Misc.

- Add missing documentation [c:2353387]

## [0.6.1] - 2017-09-18

### Added

- 4 missing uv index / wind gust Datapoint fields ([@rparrett]) [c:bbe3a1d]
- 4 missing temperature low / high Datapoint fields ([@rparrett]) [c:0cec145]
- More entries to .gitignore [c:7ef66b1]

## [0.6.0] - 2017-08-20

### Added

- Missing Datapoint fields [c:aedafd9]
- More documentation [c:64fc545], [c:6f49baa]

### Fixes

- Performance optimizations [c:63b14a7]
- Make `Alert::expires` optional [c:1e693e6]
- Camel case fix [c:945e86c] ([@rparrett])

### Changed

- Use the question mark operator (bump to `rustc 1.13`) [c:a7b6438]
- Add a trait implemented on Hyper [c:792c151]

### Misc.

- Updated `hyper` to `v0.10` and `serde_json` to `v1.0` [c:a32b4fd] ([@sb89])
- Use `serde_derive` [c:f5b0269]

## [0.5.0] - 2016-10-04

### Changed

- Internal API endpoint changed to reflect forecast.io name's change to DarkSky;
- Package name changed to "darksky".

## [0.4.0] - 2016-09-13

### Added

- Added a function, `get_forecast_with_options`, to get the forecast
with specified options.

### Changed

- All error fields have been changed from strings to optional f64's


## [0.3.0] - 2016-08-30

### Added

### Changed

- Structs have been replaced with near-fully-optional datapoints and
datablocks.


## [0.2.0] - 2016-08-29

### Added

### Changed

- Forecast::minutely is now optional, as non-UK/USA areas do not contain this
data;
- HourlyData::visibility is now optional due to an oversight


## [0.1.1] - 2016-08-26

### Added

### Changed

- Fixed a typo in an icon definition.


## [0.1.0] - 2016-08-23

Initial commit.

[c:0cec145]: https://github.com/zeyla/darksky.rs/commit/0cec1452396b658281b47df9548360708c4caa60
[c:14c172a]: https://github.com/zeyla/darksky.rs/commit/14c172ae6b62b981df030ab2f58b9f5293117809
[c:1e693e6]: https://github.com/zeyla/darksky.rs/commit/1e693e640bf43eb8157d91b4b66e7f5088bced70
[c:2353387]: https://github.com/zeyla/darksky.rs/commit/2353387f5c1d3820a4e8c6ccfefb8c49dd216b88
[c:4d67790]: https://github.com/zeyla/darksky.rs/commit/4d677905b51feaff3e5544cd9004bdb5ba9f90cd
[c:63b14a7]: https://github.com/zeyla/darksky.rs/commit/63b14a7b5e92f26778a43813f69972ac9aa3835a
[c:64fc545]: https://github.com/zeyla/darksky.rs/commit/64fc545886cd883e8d502cb7336dc6bcde0345d1
[c:6f49baa]: https://github.com/zeyla/darksky.rs/commit/6f49baa2469d891aee847f6178a853e6ef4ba6b7
[c:792c151]: https://github.com/zeyla/darksky.rs/commit/792c1518291c77f7c5669ae8bdea3cda084688e5
[c:7ef66b1]: https://github.com/zeyla/darksky.rs/commit/7ef66b1b8894dbe391cf9f5ce51d7de258726593
[c:819fe80]: https://github.com/zeyla/darksky.rs/commit/819fe803463427fc7d6cd24eaff14432b2da8f29
[c:8946cc2]: https://github.com/zeyla/darksky.rs/commit/8946cc2ddaf67e577e62c943f4451b2c9fa9a7a1
[c:8fb8419]: https://github.com/zeyla/darksky.rs/commit/8fb84190310ec2638a5ab119240b7922bf8a6bc3
[c:945e86c]: https://github.com/zeyla/darksky.rs/commit/945e86c99735732e3709c2517e8ad36284dcbe59
[c:a32b4fd]: https://github.com/zeyla/darksky.rs/commit/a32b4fde289c0db4e636808d288fad7156179891
[c:a7b6438]: https://github.com/zeyla/darksky.rs/commit/a7b6438ee7fe663c9bf33e62c3bcb6117926779c
[c:aedafd9]: https://github.com/zeyla/darksky.rs/commit/aedafd9fce4d45280518e3f8f209a837ecacdc4f
[c:bbe3a1d]: https://github.com/zeyla/darksky.rs/commit/bbe3a1d41cea96bd923d8fe2e685c114c25fc314
[c:ef39b9f]: https://github.com/zeyla/darksky.rs/commit/ef39b9f9d34110ded507f93ef01290ba6e4aece4
[c:f5b0269]: https://github.com/zeyla/darksky.rs/commit/f5b0269b6fe0cf0643f942893368436ef14b6b68

[@mklein994]: https://github.com/mklein994
[@rparrett]: https://github.com/rparrett
[@sb89]: https://github.com/sb89

[0.7.0]: https://github.com/zeyla/darksky.rs/compare/v0.6.1...v0.7.0
[0.6.1]: https://github.com/zeyla/darksky.rs/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/zeyla/darksky.rs/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/zeyla/darksky.rs/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/zeyla/darksky.rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/zeyla/darksky.rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/zeyla/darksky.rs/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/zeyla/darksky.rs/compare/v0.1.0...v0.1.1
