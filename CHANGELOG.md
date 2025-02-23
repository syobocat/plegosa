# v0.4

## Changed

- `url` in `config.toml` must be include schema (`example.tld` → `https://example.tld`)
- Removed `timelines.home`, `timelines.local`, `timelines.public`. Please migrate to `timeline.targets`
- `openssl` should no longer be needed
- Plegosa no longer relies on `LazyLock`, allowing you to compile with the lower version of Rust

## Added

- Added illumos support

# v0.3

## Fixed

- Plegosa no longer skips Quotes

# v0.2.3

## Added

- Added unicode normalization support

## Removed

- Removed Solaris 10 support (Solaris 11 support is currently unavailable)

# v0.2.2

## Fixed

- `include` is now working properly

# v0.2.1

## Fixed

- `exclude` is now working properly

# v0.2.0

## Changed

- .env is obsolete; Please migrate to config.toml
- `Discord` logger will now use embeds

## Added

- Multiple logger support
- Added an option to always use Discord embed
- Added Linux AArch64 support

## Fixed

- Reposts will no longer appear in the log

# v0.1.2

## Added

- Added Solaris support
- Added feature flag to enable static link for OpenSSL

# v0.1.1

## Added

- Added NetBSD support
- Dynamically link OpenSSL except for linux-musl and netbsd

# v0.1.0

Initial release.
