[travis-badge]: https://img.shields.io/travis/zeyla/darksky.rs.svg?style=flat-square
[travis]: https://travis-ci.org/zeyla/darksky.rs
[license-badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
[license]: https://opensource.org/licenses/ISC
[docs-badge]: https://img.shields.io/badge/docs-online-2020ff.svg?style=flat-square
[docs]: https://docs.rs/darksky

[![travis-badge][]][travis] [![license-badge][]][license] [![docs-badge][]][docs]


# darksky.rs

An unofficial Rust wrapper for the [DarkSky] API.

**Note**: This package was previously named `forecast_io`. Due to a
[change in name], this package has been renamed to `darksky`, and can be found
on [crates.io] by the same name.


### Installation

Add the following dependency to your `Cargo.toml`:

```toml
darksky = "0.5"
```

And include it in your project:

```rust
extern crate darksky;
```

### Features

**hyper**: Enables an implementation of [`DarkskyRequester`] on hyper's
`Client` (enabled by default).

### License

License info can be found in the [LICENSE.md] file. Long story short, ISC.

[change in name]: http://status.darksky.net/2016/09/20/forecast-api-is-now-dark-sky-api.html
[crates.io]: https://crates.io/crates/darksky
[examples]: https://gitlab.com/kalasi/darksky.rs/tree/master/examples
[DarkSky]: https://darksky.net
[LICENSE.md]: https://gitlab.com/kalasi/darksky.rs/blob/master/LICENSE.md
