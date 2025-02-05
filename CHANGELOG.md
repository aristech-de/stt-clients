# Changelog

## NodeJS v1.2.0 - 2025-02-05
## Rust v2.2.0 - 2025-02-05
## Python v1.2.0 - 2025-02-05
- Added support for new alias field.

## NodeJS v1.1.3 - 2025-02-03
- Bumped dependencies.
## Rust v2.1.5 - 2025-02-03
- Bumped dependencies.

## NodeJS v1.1.2 - 2025-01-07
- Bumped dependencies.
## Rust v2.1.4 - 2025-01-07
- Bumped dependencies.


## Rust v2.1.3 - 2024-12-05
- Moved cpal to dev-dependencies because it is only used within the live example.

## Rust v2.1.2 - 2024-12-04
- Bumped dependencies.
## Node v1.1.1 - 2024-12-04
- Bumped dependencies.

## Rust v2.1.1 - 2024-11-21
- Fixed a bug where native roots were not being used correctly.
- The get_client function now automatically adds a schema if it is missing from the host based on the tls options.

## NodeJS v1.1.0 - 2024-10-31
- Added support for adding a prompt to the recognition request.
- Made readme examples links absolute
- Updated dependencies.
## Python v1.1.0 - 2024-10-31
- Added support for adding a prompt to the recognition request.
- Made readme examples link absolute
## Rust v2.1.0 - 2024-10-31
- Added support for adding a prompt to the recognition request.
- Made readme examples links absolute

## Python v1.0.1 - 2024-10-29
- Improved readme.

## Python v1.0.0 - 2024-10-29
- Initial release of the Aristech STT-Client for Python.

## Rust v2.0.1 - 2024-10-28
- Switched from tonic tls-roots to tls-native-roots because tls-roots is deprecated.

## NodeJS v1.0.3 - 2024-10-25
- Made accountInfo request optional.

## Rust v2.0.0 - 2024-10-23
- Updated dependencies.
- Added documentation.
- Modified calling signature to expect `&mut SttClient` instead of a mutable `SttClient` instance to make it easier to clone the client for future use cases.
## NodeJS v1.0.2 - 2024-10-23
- Updated dependencies.

## NodeJS v1.0.1 - 2024-10-21
- Fixed package to properly import generated code.

## Rust v1.0.0 - 2024-10-21
- Initial release of the Aristech STT-Client for Rust.
## NodeJS v1.0.0 - 2024-10-21
- Initial release of the Aristech STT-Client for NodeJS.