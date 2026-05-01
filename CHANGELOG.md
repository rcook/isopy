# Changelog

All notable changes to this project are documented here.
Generated from git commits by [git-cliff](https://git-cliff.org).

## [Unreleased]


### Internal
- More integration tests (6536345)
- Reduce scope of tokio dependency (a0d176f)
- Do not use forked decompress (91fb1c8)


### Tooling
- Use workspace dependencies and Clippy settings (e86a820)
- Pin toolchain to stable (5760fa7)
- Fix Clippy issues (b01f08e)

## [0.5.17] - 2026-05-01


### Dependencies
- Bump clap_complete from 4.5.66 to 4.6.3 (4e80a5c)
- Bump uuid from 1.23.0 to 1.23.1 (a8f1fc4)
- Bump tokio from 1.50.0 to 1.52.1 (73afee6)
- Bump clap from 4.5.60 to 4.6.1 (eab4b08)
- Bump sha2 from 0.10.9 to 0.11.0 (14cbf5e)


### Tooling
- Install x86_64-linux-musl-g++ via setup-cross-toolchain-action (6577c22)

## [0.5.16] - 2026-05-01


### Dependencies
- Bump quinn-proto from 0.11.13 to 0.11.14 (835f234)
- Bump tar from 0.4.44 to 0.4.45 (342619e)
- Bump uuid from 1.22.0 to 1.23.0 (56c5d9c)
- Bump tempfile from 3.26.0 to 3.27.0 (20e1e5e)
- Bump openssl-sys from 0.9.111 to 0.9.112 (a982900)
- Bump rand from 0.9.2 to 0.9.4 (276332b)
- Bump rustls-webpki from 0.103.9 to 0.103.13 (8f0419e)


### Features
- Add integrate tests (b0b55ee)


### Internal
- Download new checksums (d4c587a)
- Remove unused import (20e68fd)
- Use sort_by_key instead of sort_by (24487cf)
- Deduplicate version parsing across plugins (8f95cbb)
- Consolidate downcast_version macro (7ddf8af)
- Beef up unit tests (32eb9d2)
- Split up PackageManagerHelper (191cead)
- Tidy up imports (a5d9ffd)


### Tooling
- New workflows (814c2a5)
- Add permissions to workflow (86be2f3)
- Update aws-lc-rs (6cebc8c)

## [0.5.15] - 2026-03-07


### Tooling
- Rewrite release workflow (6be3bf9)

## [0.5.14] - 2026-03-06


### Bug Fixes
- Fix (e27a2cb)
- Fix filtering by tags (1a0a357)
- Fix warning on macOS and Linux (4b1f53c)
- Fix message (eefec62)
- Fix package metadata and bump to Rust 2024 edition (552a2fe)
- Fix typo (c595475)


### Dependencies
- Bump bytes from 1.10.1 to 1.11.1 (d6b37e1)


### Features
- Add unit tests (f78e27b)
- Add rudimentary support for configuration values (4725132)
- Add unit tests (5d9a13e)
- Add docs command (94d815e)


### Internal
- Rename commands (c34cf0f)
- Clean up tags (e25e783)
- Rename cache files (756aca2)
- Rename ReleaseGroup -> BuildLabel (d7e8d45)
- Return platform tags for Go package manager (2dd7ff7)
- Use cache in download_package (36f0e3c)
- Use cache in install_package (d66d750)
- Use cache throughout (2c88a3e)
- Choose "best" package where multiple match build label (dfc8612)
- Reformat (ef3e3df)
- Move checksums into GitHub Pages directory (8c86167)
- Get checksums from isopy GitHub Pages site (3400e93)
- Remove Display from VersionOps (bc20575)
- Remove Result (95d74ca)
- Remove label from PackageInfo (c61aec8)
- Remove label (572a5f8)
- Clean up code (1cf782c)
- Simplify handling of tags (40d5002)
- Simplify Metadata struct (022bc1c)
- Simplify code more (1cb02dd)
- Simplify more code (8fe610e)
- Simplify Item struct (1b8afee)
- Simplify more code (8d63658)
- Remove more unnecessary const fn functions (38b706e)
- Rename PythonPackageWithAvailability -> AvailabilityInfo (20d6275)
- More renames (8741baa)
- Simplify PythonVersion (fdfddd5)
- Renames (819bcbf)
- Make field private (028142d)
- Renames (2515786)
- Refactor (51a3fb7)
- Rename (75882e0)
- Sort Go packages properly (d34b2b3)
- Simplify code (eb025ef)
- Refactor local/remote package partitioning (bc8a994)
- Use config values for other commands using moniker (470460a)
- Generalize config value commands (0b037c2)
- Simplify tags (1be1a97)
- Display path to configuration values file (3991bef)
- Reorder output (126488f)
- Remove test value (3c9a73d)
- Improve messages (86adc9f)
- Change message (a39c1ae)
- Simplify label (3454215)
- Rename platform_hacks -> choose_best (797b343)
- Upgrade dependencies (343f6de)
- Use to_owned instead of clone (e341e78)
- Upgrade dependencies (e18d71f)
- Upgrade dependencies (adb4165)
- Do not hide elided lifetimes (d6a4423)
- Remove unused dependencies (97a1224)
- Upgrade dependencies (250bd88)
- Precheckin script (82c8700)
- Rationalize re-exports (51b587f)
- Use GitHub REST API pagination (de1eff1)
- Use unsafe (2ae9a71)
- Remove unused dependencies (65f2be5)
- Wrap unsafe functions (5bd0222)
- Do not link statically on Linux (b3ce175)
- Incorporate source code from my external libraries (ae6c842)
- Use LazyLock (3ead357)
- Rename packages (ccfbf1c)
- Remove unused dependencies (87b07d3)
- Remove old code (9d99781)
- Tidy up (419e0f6)
- Upgrade dependencies (0325365)
- Remove Error and Result (99a031c)
- Fold repo code into isopy (0ec0621)
- Fold isopy-util into isopy (730b89d)
- Simplify error handling (b14874c)
- Force remove_dir_all=1.0.0 (cb38e5a)
- Remove unused code (4dcfb51)
- Remove old code (1972f37)
- Move more code around (24b26cd)
- Tweaks (5c13e3c)
- Upgrade to tempfile (aaf05a7)
- Upgrade dependencies (46a1e88)
- Remove overcomplex ISOPY_BACKTRACE logic (a4cb9c4)
- Show correct call stack (ee4acfe)
- Handle no project file (10c7c5f)
- Upgrade dependencies (ef095a0)
- Remove unsupported target platforms (c5f0fe4)


### Tooling
- Update platform tags for Windows (297c5e0)
- Update documentation (987358f)
- Fix Clippy issues (5c06a34)
- Update to Rust 2024 edition (8c32f3c)
- Add Clippy rules and fix issues (f84c3ca)
- Fix Clippy issues (464eed0)
- Update dependencies (687d422)
- Fix more Clippy issues (9869b73)
- Update devtool configuration (8e764c0)
- Update dependencies (64b8dd5)

## [0.4.12] - 2025-04-15


### Internal
- Clean up after failed isopy env command (0d8922b)

## [0.4.11] - 2025-04-15


### Internal
- Simplify package manager context interface (dcf856a)
- Wrap URLs (1200bf1)
- Use FileUrl (caf5ae3)

## [0.4.10] - 2025-04-15


### Internal
- Download checksums (0c0c1ac)

## [0.4.9] - 2025-04-15


### Bug Fixes
- Fix caching (582017b)


### Internal
- Rename package cache file (e9b8907)
- Rename PackageState -> PackageInfo etc. (d254220)
- Skip empty fields (26e7449)
- Implement tags for Go plugin (af38699)
- Use cache in more places (6d6fc48)

## [0.4.8] - 2025-04-15


### Internal
- Combine FullVersion and ProjectVersion (3481c6a)
- Cache parsed package info (6c25202)

## [0.4.7] - 2025-04-14


### Features
- Add tests (5613f25)


### Internal
- Remove comment (920f11e)
- Simplify names (670ee5b)
- Upgrade dependencies (afe42c9)

## [0.4.6] - 2025-04-14


### Internal
- Include discriminator in version numbers (4f70b85)
- Rename discriminator -> discriminant (7d8fb09)
- Rename (4936608)
- Clean up version parsing (ea2b9b1)
- Clean up (58455ad)

## [0.4.5] - 2025-04-14


### Dependencies
- Bump regex from 1.10.6 to 1.11.1 (72925f3)
- Bump openssl-sys from 0.9.103 to 0.9.104 (08bc634)
- Bump serde from 1.0.210 to 1.0.214 (285a0bc)
- Bump anyhow from 1.0.89 to 1.0.92 (d5dff67)
- Bump serde_json from 1.0.128 to 1.0.132 (27411d3)


### Features
- Add missing argument (ca6b632)
- Support alpha versions (2ec9273)
- Add latest checksum files (69286cd)


### Internal
- Refactoring (a68f611)
- Simplify error handling (c16558f)
- Prettify (346cfe1)
- More tidy-up of error handling (b8336cc)
- Upgrade dependencies (8a04ac5)
- Upgrade dependencies (2c14753)
- Upgrade dependency versions (5a59702)


### Tooling
- Update thiserror (e6b6a52)
- Update indygreg -> astral-sh (f139a9d)
- Fix Clippy warning (ce67c17)

## [0.4.4] - 2024-10-19


### Bug Fixes
- Fix check for downloaded packages and error messages (387d5e1)


### Features
- Add --verbose flag to packages subcommand (164bebd)


### Internal
- Improve error messages (1b492fd)
- Improve error messages (8f8a619)
- Display user errors in yellow (5c87dae)
- Rename PackageKind -> PackageAvailability (10c8820)
- Rename PackageSummary -> PackageInfo (da1e224)
- Refactor (bd34c73)
- Nicer pattern match (77fcc62)
- Renames (023e6f7)
- Refactor (b447390)
- Refactor (38c65ef)
- More refactoring (2d6c7bf)
- Speed up PythonPackageInfo::read (6ad3247)
- Improve error messages (78a7dde)
- Format source files (f7a8f91)


### Tooling
- Clippy fixes (259adb9)
- Update .devtool.yaml (20ec869)
- Fix Clippy issues (4f07dd5)
- Fix Clippy issues (6b0f875)

## [0.4.3] - 2024-10-17


### Features
- Add 20241016.sha256sums (8588b34)


### Internal
- Upgrade dependencies (f3020b5)
- Revert "Upgrade dependencies" (7d83b38)
- Reformat source (95c36e1)
- Improve CLI (ae795af)
- Check that packages are available before trying to install them (eb10ee1)
- Refactor environment variable handling (e16023f)
- Show default (03ba6cc)
- Show RUST_BACKTRACE and ISOPY_ENV in info output (591b965)


### Tooling
- Fix Clippy issues (10f6396)

## [0.4.2] - 2024-10-09


### Features
- Add badge for most recent tag (57bcdef)
- Add SHA-256 checksums for 20241002 and 20241008 builds (e12efa8)

## [0.4.1] - 2024-10-08


### Bug Fixes
- Fix (575fd1e)
- Fix version (b5fc893)


### Features
- Add progress indicator to download (8c88be9)
- Add --download switch to init and env-init (9ddbf4f)
- Add some colour (efca230)
- Support optional release group in project configuration (ea0b9d2)
- Add --show-progress option (6d698a5)
- Add more colour (e71d530)
- Add isopy-go (0c3f0e8)
- Add URL helpers (6cfc042)
- Add default tags for Windows (04a4169)
- Add support for multifile cache directories (c985c22)
- Add SHA-256 checksums for 20240909 builds (59c9e3a)
- Add default tags for Linux (2fe6d18)


### Internal
- Use YAML instead of JSON for cache manifest (abdb7c0)
- Show progress (c9678e7)
- Tweak layout of progress indicator (f8dfb63)
- Show progress during download (2b53104)
- Display better error messages if version/build tag cannot be parsed (95f70eb)
- Handle RC version numbers (cbdbd89)
- Improve progress bar template (fdb3a6b)
- Rename config -> project (8ada961)
- Rename CacheInfo -> Cache (cce91a9)
- Display label in packages output (57680c1)
- Improve layout of packages output (b080f7d)
- Rename env-init -> env and env-list -> list (98dab59)
- Make --verbose the default for list command (77581c3)
- Tweak help strings (3ffda51)
- Clean up macros (7ab2421)
- Upgrade dependencies (9ab2348)
- Rename PackageFilter -> SourceFilter and OptionalTags -> TagFilter (472c188)
- Swap colours (6c15bf2)
- Remove on_before_install and on_after_install from trait (96416e2)
- Simplify code (057733b)
- Improve handling of wrapper file name (3557b16)
- Clean up macros (e7c10e7)
- Download Go index (d6f2a82)
- Extract Go package versions (fcfea71)
- Extract tags from index (71f038a)
- Partial implementation of install_package for Go (6e1172d)
- Implement download_package for Go (0d4141c)
- Move ArchiveType into isopy-lib (15e718a)
- Implement install_package for Go (0c81f28)
- Tidy up code (138d509)
- Remove old module (cc6907a)
- Environment for Go (38b8a3a)
- Remove old module (fb9d1b4)
- Rename serialization -> api (c02b29a)
- Remove more old code (77a7d93)
- Create isopy-java (65fd02e)
- Reformat (2963a2e)
- Rename types (1969fe7)
- Import old code (7fdf5dc)
- Disable Java plugin (3b0873a)
- Maven version handling (79c6261)
- Whitespace (6137751)
- Gate Java functionality behind ISOPY_JAVA environment variable (754f579)
- Simplify (13e3edf)
- Extract common code (e021a80)
- Bring LinkHeader back (f085afd)
- Enumerate files (b544b5e)
- Load pages (9fb0b55)
- Handle absence of files and directories (43560d1)
- Download index (1959280)
- Return package summaries (6cab1c8)
- Use rcook/rust-tool-action@v0.0.35 (8474f77)


### Tooling
- Fix Clippy issues (2516bf2)
- Fix Clippy issues (afa016f)
- Update documentation (b9997e4)

## [0.4.0] - 2024-09-13


### Bug Fixes
- Fix message (2baed47)
- Fix cache directory (5ead51f)
- Fix missing import (ea8ca7b)
- Fix names (0b04da5)
- Fix warning (3214655)
- Fix up configuration directory (f80c606)


### Dependencies
- Bump serde_json from 1.0.116 to 1.0.117 (4c9aff8)
- Bump serde from 1.0.200 to 1.0.202 (a5a1f56)
- Bump anyhow from 1.0.82 to 1.0.86 (e7633a4)
- Bump zip from 1.1.1 to 1.3.0 (4e20bd1)
- Bump thiserror from 1.0.59 to 1.0.61 (9bf6394)
- Bump sysinfo from 0.30.11 to 0.30.12 (2429ede)
- Bump serde from 1.0.202 to 1.0.203 (54b4613)
- Bump tokio from 1.37.0 to 1.38.0 (145fff8)
- Bump strum_macros from 0.26.2 to 0.26.4 (e60eb04)
- Bump rstest from 0.19.0 to 0.21.0 (358deb7)
- Bump url from 2.5.0 to 2.5.1 (c2fddf9)
- Bump clap_complete from 4.5.2 to 4.5.3 (f7cbf6e)
- Bump regex from 1.10.4 to 1.10.5 (4d59c9b)
- Bump tar from 0.4.40 to 0.4.41 (0f4146f)
- Bump clap from 4.5.4 to 4.5.7 (19fbecf)
- Bump strum from 0.26.2 to 0.26.3 (28ee43e)
- Bump url from 2.5.1 to 2.5.2 (251b22e)
- Bump clap_complete from 4.5.3 to 4.5.6 (d9c501e)
- Bump include_dir from 0.7.3 to 0.7.4 (8279617)
- Bump reqwest from 0.12.4 to 0.12.5 (86c78cf)
- Bump clap_complete from 4.5.6 to 4.5.7 (7094535)
- Bump lazy_static from 1.4.0 to 1.5.0 (b3c7907)
- Bump uuid from 1.8.0 to 1.9.1 (50afa4b)
- Bump log from 0.4.21 to 0.4.22 (70b353e)
- Bump clap from 4.5.7 to 4.5.8 (59ba9f7)
- Bump async-trait from 0.1.80 to 0.1.81 (08da89f)
- Bump sysinfo from 0.30.12 to 0.30.13 (10dc964)
- Bump serde from 1.0.203 to 1.0.204 (f37cb3c)
- Bump uuid from 1.9.1 to 1.10.0 (6138e06)
- Bump thiserror from 1.0.61 to 1.0.62 (73fcdc7)
- Bump clap from 4.5.8 to 4.5.9 (b8656cc)
- Bump serde_json from 1.0.117 to 1.0.120 (783ff3d)
- Bump openssl-sys from 0.9.102 to 0.9.103 (5610613)
- Bump tokio from 1.38.0 to 1.38.1 (434d954)
- Bump thiserror from 1.0.62 to 1.0.63 (3ab5096)
- Bump openssl from 0.10.55 to 0.10.66 (20ade38)
- Bump clap_complete from 4.5.7 to 4.5.8 (5c90cbc)
- Bump bytes from 1.6.0 to 1.6.1 (2baca64)
- Bump clap from 4.5.9 to 4.5.11 (6123f47)
- Bump tokio from 1.38.1 to 1.39.2 (0bbcffb)
- Bump clap from 4.5.11 to 4.5.13 (40143c8)
- Bump flate2 from 1.0.30 to 1.0.31 (aede093)
- Bump clap_complete from 4.5.8 to 4.5.12 (fe3052e)
- Bump rstest from 0.21.0 to 0.22.0 (a203465)
- Bump regex from 1.10.5 to 1.10.6 (79aaab6)
- Bump serde from 1.0.204 to 1.0.206 (5a01c78)
- Bump clap from 4.5.13 to 4.5.15 (790a867)
- Bump bytes from 1.6.1 to 1.7.1 (4292ea4)
- Bump serde_json from 1.0.124 to 1.0.125 (9f3e172)
- Bump ctrlc from 3.4.4 to 3.4.5 (9394097)
- Bump tokio from 1.39.2 to 1.39.3 (65edd41)
- Bump serde from 1.0.206 to 1.0.208 (029f2b1)
- Bump clap_complete from 4.5.15 to 4.5.23 (412fef2)
- Bump reqwest from 0.12.5 to 0.12.7 (792d335)
- Bump flate2 from 1.0.31 to 1.0.32 (da9c11d)
- Bump clap from 4.5.15 to 4.5.16 (aa05175)
- Bump serde from 1.0.208 to 1.0.209 (e7d74b1)
- Bump flate2 from 1.0.32 to 1.0.33 (e965a74)
- Bump sysinfo from 0.31.2 to 0.31.4 (600f44f)
- Bump clap_complete from 4.5.23 to 4.5.24 (654e86c)
- Bump tokio from 1.39.3 to 1.40.0 (11bdc1d)


### Features
- Support pwsh (9a36ad0)
- Add host (a175fe7)
- Add unit tests for PackageVersion (6c1ca86)
- Add DownloadOptions (4562f21)
- Add update download option (f21aa9b)
- Add copyright notice and licence header (6e7b5c6)
- Add platform keywords for macOS (fe0c064)
- Add platform keywords for Linux (21ba2bd)
- New constants (4b61c1a)
- Add Moniker struct (61a4b11)
- Add package filter (c206ea8)
- Add install subcommand (ba4aec6)
- Add tags subcommand (1987848)
- Add more filtering by tag (bb39896)


### Internal
- Annotate structs with unused fields (9ee45fd)
- Reinstate x86_64-apple-darwin target using macos-13 (7e7f719)
- Upgrade serde_json from 1.0.120 -> 1.0.124 (7c232e2)
- Upgrade clap_complete from 4.5.12 -> 4.5.13 (d17e0d0)
- Upgrade sysinfo from 0.30.13 -> 0.31.2 (1005ab5)
- Set Dependabot interval to monthly (520c023)
- Initial commit (7b4d668)
- Create api, lib and python subpackages (3d373e1)
- Package manager factory (5c868e2)
- Rename (6e48765)
- Implement basic factory model (744d629)
- Sanitize paths etc. (11b9fea)
- Download manifest or retrieve from cache (4b314bc)
- Fetch release using releases API (50788df)
- Dump archive names (c0baa4f)
- Parse version and group properly (9c0f6d7)
- Rename (70a8b10)
- Order versions etc. (934305d)
- Convert ArchiveGroup to struct (f83bbc9)
- Get rid of redundant regex check (928f3d3)
- Filter to latest version (58fbd67)
- Find unique match (2506328)
- Download archive (c65628b)
- Rename ArchiveVersion -> PackageVersion (c0f3afb)
- Remove regex parsing (8d85a0b)
- Tidy up (df648d3)
- Refactor (17f1a85)
- Refactor package manager structs (1088bcd)
- Use non-blocking APIs (9b6fe43)
- Change version (60a0750)
- Simplify main (7f1618e)
- Rename isopy-api -> isopy-lib (19872b7)
- Rearrange modules (c0d88ad)
- Refactor (f2e1aa4)
- Rewrite string/file name sanitizing code (5bf4ab4)
- Store download info in cache.json (dc7efd9)
- Archive with old-style tag (f5a636a)
- Refactor download (b558d48)
- Validate checksums (b0afefd)
- Rearrange code (ea3f074)
- Rename PackageManagerWrapper -> AppPackageManager (6a4276e)
- Move constants into dedicated module (1f28c08)
- Refactor (a06e901)
- Stub out install_package (a1cdd9f)
- Stub out unpack (96c321f)
- Stub out implementations (c9934d2)
- Delete file if checksum validation fails (1d9b5a4)
- Stub out unpack functions (f64d2b6)
- Decompress archives (182904f)
- Strip path components (d762bef)
- Rename packages (ca3aee4)
- Stub out Go package manager (f80c047)
- Rename isopy2 -> isopy-internal (75f28d6)
- Use forked version of decompress (21b13f5)
- Use latest decompress (ec75d65)
- Prune unneeded dependencies using cargo-machete (fccc36b)
- Use serde feature of url (7c3e5b2)
- Remove unnecessary async (178844b)
- Hang new app object off old (a3f83a2)
- Remove isopy-python2 (74b20be)
- Upgrade dependencies (0c7507b)
- Use ~/.config/.isopy for config on Linux/macOS and %APPDATA%\.isopy on Windows (5cb4ca1)
- Implement update command (c4eb23c)
- Implement new download command (83c8f2e)
- Report GitHub rate limit exceeded (61b3308)
- Rename PackageVersion -> VersionTriple (5f10f21)
- Rename package manager etc. (1d31141)
- Big refactor (503c1ad)
- Return &str instead of String (d900c5f)
- Clean up API with Deref (e281d72)
- Encapsulate moniker and version string code (1ff6dcc)
- Rename App -> PluginManager (4afef1e)
- Rename Context -> Host (d2f9528)
- Turn Host into newtype (4c0f675)
- Rename (7e1919b)
- More renamesg (f579d51)
- Refactor host/context (4ca31e9)
- Refactor (d91c66d)
- Rename Manager -> PackageManager etc. (784370f)
- Use config_dir accessor (ca72563)
- Refactor (759af46)
- Rename to_str -> as_str (e228938)
- Simplify code (3cae5be)
- Rename AppManagerContext -> PackageManagerHelper (0be5f76)
- Rename AppHost -> PluginHost (7fda2d3)
- Simplify code (8a7db80)
- Simplify even more (4ca0331)
- Remove unnecessary cyclic references (07e6452)
- Move new commands under subcommand (ffdbc4b)
- Move commands into incubating subcommand (e3a5823)
- Rename PythonManager -> PythonPackageManager (1ff4e13)
- Refactor list_packages (9596455)
- Make local default (a01e3ea)
- Allow override of tags (22229b6)
- Use term tags instead of keywords (08fc70b)
- Rename (12ce028)
- Tweak output (d2e2188)
- Improve list_tags (2e41a6f)
- Include build tag in filtering (b0516bf)
- Remove todo! (31a033d)
- Use sentence case for error messages (bc81f88)
- Hook up new package manager (d2bf0a0)
- Sentence case (4babcb6)
- Rename variables (735c004)
- Refactor (59e7230)
- Renames (b1d5b74)
- Move on_before_install and on_after_install into new plugin (ce84724)
- Remove dead code (debc265)
- Change visibility (158003a)
- Tidy up naming of serialization structs (4fc3fdd)
- Removed unused code (75880d5)
- Don't pass plugin host (ea5e8c8)
- Improve design of code (e75df11)
- Store package URL in environment configuration (c86640f)
- Refactor (78e9ec8)
- deref_wrapper macro (1a9a131)
- Tidy up (122153b)
- Rename id -> moniker (60d97f1)
- Simplify install (b5303ac)
- Use pub(crate) (2307370)
- Create incubating module (053296d)
- Remove old download command (1ec28d9)
- Remove old code (98b0a7e)
- Remove unused dependencies (ffedab6)
- Remove old package subcommand (9a7e1bb)
- Remove Go and Java support (f433392)
- Remove dead code (37971d0)
- Remove more unused code (39ae329)
- Remove unused dependencies (5d93bed)
- Remove dead code (20db21f)
- Improve dyn_trait_struct (e3d229c)
- Use new PackageId (9e15577)
- Remove more old code (ad953e3)
- Remove more old code (4b49e8e)
- Remove old code (6e02c25)
- Remove old code (10ff3a5)
- Remove old code (edbe917)
- Remove old code (da40e44)
- Remove old code (d1dfbc2)
- Remove old code (2d1dcaf)
- Flatten isopy-python (649aac7)
- More renames and deletes (2d5cc0a)
- More deletes (d3dbd84)
- Trim unused dependencies (df1e721)
- Remove some uses of lazy_static (98e82a4)
- Remove lazy_static dependency (1b1cb0f)
- Allow new_ret_no_self (8f522e4)
- Display unpacking progress (9bb6757)
- Move incubating commands to top level (5c33e8d)
- Flatten constants (8e54413)
- Remove dead code (92a8778)
- Remove more dead code (5d920b8)
- Rearrange modules (09349c2)
- Rearrange modules (52c1ece)
- Rearrange modules (5838d6c)
- Rename Package structs (532758a)
- Sentence case (aeb47db)
- Flatten commands (9f7c392)
- Rename commands (f3fb7f6)
- Simplify code (d1c3d56)
- Rename (6ad4166)
- Use PackageId (cffbee5)
- Tidy up (60c2a6e)
- Rearrange modules (9be588f)


### Tooling
- Fix Clippy issues (f084d84)
- Update dependencies (6099ede)
- Update dependencies (ed7202e)
- Update decompress dependency (83b081b)
- Fix some Clippy issues (e4a5815)
- More Clippy fixes (eab315d)
- Fix more Clippy issues (6fbe66e)
- Fix more Clippy issues (903b85f)
- Fix Clippy issues (07eae58)
- Update packages (f04a839)

## [0.3.25] - 2024-05-18


### Internal
- Ensure wrapper script ends with .bat or .cmd on Windows (32f2dea)

## [0.3.24] - 2024-05-18


### Bug Fixes
- Fix Windows compile errors (ac6fc9e)


### Dependencies
- Bump chrono from 0.4.37 to 0.4.38 (7ebd1a4)
- Bump serde from 1.0.197 to 1.0.200 (1a9f85e)
- Bump time from 0.3.34 to 0.3.36 (2e09f5d)
- Bump rstest from 0.18.2 to 0.19.0 (85dd9f7)
- Bump async-trait from 0.1.79 to 0.1.80 (18a29ed)


### Internal
- Do not attempt to create symlink if file already exists (3dde695)
- Swallow Ctrl+C from child processes (fdb5cf0)

## [0.3.23] - 2024-04-29


### Bug Fixes
- Fix executable_ext -> target_ext (bb77e08)

## [0.3.22] - 2024-04-29


### Internal
- Remove unused import (60f87ca)

## [0.3.21] - 2024-04-29


### Tooling
- Update executable_ext -> target_ext (6b792b2)

## [0.3.20] - 2024-04-29


### Tooling
- Update to rcook/rust-tool-action@v0.0.34 (3ecfe8a)

## [0.3.19] - 2024-04-29


### Dependencies
- Bump flate2 from 1.0.28 to 1.0.30 (2ca46b2)
- Bump zip from 0.6.6 to 1.1.1 (db3fdb8)
- Bump anyhow from 1.0.81 to 1.0.82 (031842c)
- Bump serde_json from 1.0.115 to 1.0.116 (897af09)
- Bump thiserror from 1.0.58 to 1.0.59 (c042c28)


### Internal
- Use rcook/rust-tool-action@v0.0.33 (2933dd3)
- Temporarily disable x86_64-apple-darwin target (8f32aa5)

## [0.3.18] - 2024-04-28


### Dependencies
- Bump reqwest from 0.12.0 to 0.12.1 (7086d20)
- Bump regex from 1.10.3 to 1.10.4 (40a2572)
- Bump bytes from 1.5.0 to 1.6.0 (96467d3)
- Bump async-trait from 0.1.78 to 0.1.79 (39878c2)
- Bump serde_yaml from 0.9.33 to 0.9.34+deprecated (99aa98f)
- Bump serde_json from 1.0.114 to 1.0.115 (0e4e503)
- Bump reqwest from 0.12.1 to 0.12.2 (4431d1c)
- Bump tokio from 1.36.0 to 1.37.0 (e91cda1)
- Bump chrono from 0.4.35 to 0.4.37 (cc27e0f)
- Bump openssl-sys from 0.9.101 to 0.9.102 (3882ce9)
- Bump h2 from 0.4.3 to 0.4.4 (9b81ac8)
- Bump clap from 4.5.3 to 4.5.4 (b43959f)
- Bump clap_complete from 4.5.1 to 4.5.2 (71f737a)
- Bump reqwest from 0.12.2 to 0.12.4 (6a4b722)
- Bump sysinfo from 0.30.7 to 0.30.11 (76d7d1c)


### Features
- Add tracing of asset name (725eb56)
- Add new SHA256 checksums (1986f63)


### Internal
- Handle ARM v7 and associated ABIs (b1a4a9f)


### Tooling
- Fix Clippy issues (928f1e0)
- Fix Clippy issues (db3a6df)
- Update GitHub action versions (1daef2c)

## [0.3.17] - 2024-03-20


### Bug Fixes
- Fixes to support sysinfo 0.30.x (5e187c0)


### Dependencies
- Bump strum from 0.25.0 to 0.26.2 (d94a2b2)
- Bump mio from 0.8.10 to 0.8.11 (c32f68a)
- Bump sysinfo from 0.29.11 to 0.30.7 (5af3840)


### Internal
- Upgrade dependencies (3d88d75)
- Upgrade dependencies (571a81f)
- Upgrade dependencies (4aa0ff1)

## [0.3.16] - 2024-03-11


### Bug Fixes
- Fix isopy-go version number (1bc43ca)
- Fix #74: Create python symlink on Linux/macOS (ca2af32)


### Features
- Add isopy-go to devtool manifest (5e25711)


### Internal
- Upgrade dependencies (20a91f4)
- Use try_hours instead of hours (3696c70)

## [0.3.15] - 2024-03-10


### Dependencies
- Bump reqwest from 0.11.20 to 0.11.24 (0aff66d)
- Bump futures-util from 0.3.28 to 0.3.30 (64ca9b5)
- Bump anyhow from 1.0.75 to 1.0.80 (432d49c)
- Bump serde_yaml from 0.9.25 to 0.9.29 (749728e)
- Bump home from 0.5.5 to 0.5.9 (5521599)


### Internal
- #74: Create python3.cmd on Windows (f26762d)

## [0.3.14] - 2024-02-23


### Bug Fixes
- Fix #63: isopy env delete subocommand (107e44b)

## [0.3.13] - 2024-02-22


### Bug Fixes
- Fix error handling (7735e9e)


### Tooling
- Fix warnings and Clippy issues (3f44942)

## [0.3.12] - 2024-02-22


### Bug Fixes
- Fix #67, Fix #68: Enable generation of bash script on Windows (9f500ed)


### Dependencies
- Bump openssl from 0.10.52 to 0.10.55 (d659350)
- Bump tokio from 1.32.0 to 1.36.0 (424cefa)
- Bump h2 from 0.3.19 to 0.3.24 (d51bb71)
- Bump rustix from 0.37.20 to 0.37.27 (ab7ed01)
- Bump unsafe-libyaml from 0.2.9 to 0.2.10 (c5e874c)
- Bump sysinfo from 0.29.10 to 0.29.11 (395a1f3)


### Internal
- Comment out unused exports (943214a)


### Tooling
- Fix Clippy issues (0be4c33)

## [0.3.11] - 2024-02-21


### Internal
- Pass in --platform too (b201350)

## [0.3.10] - 2024-02-21


### Bug Fixes
- Fix #61, Fix #66: Generate valid .cmd and allow override of script type from CLI (f3e01a9)

## [0.3.9] - 2024-02-12


### Bug Fixes
- Fix #62: Add SHA256 hash codes for newer builds of Python (5f45769)

## [0.3.8] - 2024-01-10


### Internal
- Correctly parse JDK version number in format 21.0.1+12-LTS (d1f2847)

## [0.3.7] - 2024-01-09


### Tooling
- Update to clap_complete 4.4.6 (6cb56ba)

## [0.3.6] - 2023-09-18


### Bug Fixes
- Fix warning on Windows (23e0027)


### Internal
- Start building Go plugin (77f251b)
- Move scratch code into GoPlugin (1f78203)
- Implement Descriptor (777cf7d)
- Start implementing download (5bcc48e)
- Download Go packages (b619261)
- Implement get_downloaded_packages (ff349cb)
- Upgrade dependencies and common metadata (e180218)

## [0.3.5] - 2023-08-10


### Bug Fixes
- Fix is_executable_file on Windows (1997ad1)
- Fix compiler warnings on Windows (e291c84)


### Internal
- Simply environment variable handling (192599c)
- Prepend name of Python binary for Python scripts (a012b7e)
- Only wrap script directly if it's executable (ee9742e)
- Combine 'wrap command' and 'wrap script' into 'wrap' (48a2305)
- Flatten wrap module (48ef733)
- Remove unused coverage script (fe7011d)

## [0.3.4] - 2023-08-09


### Bug Fixes
- Fix imports (d0c2ad0)


### Features
- Add headers (9f6b5a9)
- Add table_headings macro (8273ea5)
- Add isopy-go (1356477)
- Add checksums for 20230726 releases (a9e695d)


### Internal
- Rename table_row -> table_columns (69e2b21)
- Use unwrap_or_else (bdefdb6)
- Refactor format strings (e9b6551)
- Partial implementation of offline mode (645952d)
- Centralize management of environment variables (5ac0f20)
- Environment action (1588680)
- Display environment variable actions (6fac98e)
- Don't crash if metadirectory is missing (dbef319)
- Upgrade dependencies (105b8b9)
- Upgrade dependencies (c52c770)


### Tooling
- Update asset name parsing (c13bfc6)

## [0.3.3] - 2023-07-26


### Bug Fixes
- Fix use (cd91b92)


### Features
- New table macros (66a6195)
- New return_user_error macro (d86a1fe)
- New return_success macro (ab42d3e)
- Add parameterless overload of return_success macro (5d4cca1)
- Add brief output to env list command (67a9afe)
- Add unit test (04e2f3f)


### Internal
- Use the other kind of table (b651412)
- Display key-value pairs (e472197)
- Rename row! -> table_row! (c769552)
- Combine available/downloaded into single list subcommand (7af8368)
- Pretty-print value (a694cb9)
- Move utility functions around (4c90879)
- Hygiene (17cf073)
- Do not log from prompt command (426fbd1)
- Remove color-backtrace (1b0f32b)
- Use backtrace feature (7ac7ff7)
- Make macro variadic (e7d66db)
- Simplify query building with macro (ad8cb5f)
- serializable_string_newtype macro (762cb16)
- Use strum (fda2839)
- Rename and restructure serializable_string_newtype -> serializable_newtype (ef75d5d)
- Remove unused serde derives (f89e4da)
- Handle nanos vs. string representation of last modified time (1f9e4d2)
- Improve no-packages warning (b293a21)
- Upgrade dependencies (8bcd3de)
- Simplify prompt (53b2606)
- Tweak prompt to improve shell integration (57d909d)
- Allow messages to be customized (9f8655d)
- Make fields private (806ba56)
- A little bit more information (f9e76c5)
- Display (config) if config file is available (01a9b62)
- User error instead of program error (07bb46e)
- Report metadirectory ID if already in shell (1618741)
- Improve shell message (d3fcb24)

## [0.3.2] - 2023-07-20


### Features
- New table builder to pretty print tabular output (5620541)


### Internal
- Move all calls to println! into print module (f5a77ed)
- Upgrade dependencies (3582322)
- Start to group commands together (d81e4cb)
- Generate shell completions (d114864)
- macOS-compatible shell setup script (377c499)
- Simplify Table (8f638fe)
- Use Table (b530409)
- Upgrade dependencies (90e11b1)

## [0.3.1] - 2023-07-14


### Features
- Add missing import (42aa770)


### Internal
- Specify resolver (49d6d86)
- Reformat using latest cargo fmt (81d1d8d)


### Tooling
- Fix Clippy issues with latest clippy (2bece75)

## [0.3.0] - 2023-07-14


### Bug Fixes
- Fix indentation (a9bab79)
- Fix label (eff5756)
- Fix Python bin path on Windows (4d8d23a)


### Features
- Add environment variable support (de511ec)
- Add .isopy/bin to PATH (4ce5bc1)
- Add --verbose to shell and fix all Boolean CLI flags (2b5dfc4)


### Internal
- Upgrade dependencies (994df53)
- Descriptor is no longer optional (b52d416)
- Remove .python-version.yaml ignore rule (7be1ff2)
- Implement PluginFactory on PluginHost (300eb95)
- Use type alias (f57bf89)
- Rename isopy-openjdk -> isopy-java (9b30711)
- Renames (f3a07da)
- Split into JDK and JRE (d1c1928)
- Show full paths and file sizes for downloaded packages when passing --verbose (9e4c693)
- Sort downloaded packages list (b6c40d9)
- Get wrap working (4495904)
- Generate wrappers in bin directory (57e53e8)
- Clean up suppressions (290e418)
- Split wrap into wrap-command and wrap-script (3995917)
- Tweak colours (5ec3f4b)
- Extract DirInfo helper methods into DirInfoExt extension trait (b50eeec)
- Use 'existing' helper (859c8e5)
- Make function private (306f070)
- Refactor (2c6cb14)
- Simplify find_dir_info (20b4623)
- Improve pretty output (4d89f7d)
- Handle zip archives (e6e465d)
- Move ensure_file_executable_mode into util (2f69d2c)
- Generate valid .cmd file and require .bat or .cmd extension (509611d)
- Use logger in more places (8b1011b)


### Tooling
- Clippy fix (c71f716)
- Update examples in cookbook (237c651)
- Fix Clippy issues (3ae6af9)
- Fix Clippy issues on Windows (364bef5)
- Fix Clippy issues (5c49394)
- Update devtool configuration (07a5f8b)

## [0.2.2] - 2023-07-10


### Internal
- Rearrange comments (07a43cb)
- Consolidate project configuration files (deb2beb)
- Renames (738f2aa)
- Rename Download -> DownloadPackage (fb3a5e9)
- Renames (0f00b81)
- Rename get_env_info -> make_env_info (c019007)
- Remove .isopy.yaml (c9032a9)
- Store descriptor in configuration files (140c174)
- Rename commands (187dbd9)
- Rename functions (acb01b5)
- Show environment info (1ffed27)


### Tooling
- Update documentation (d136dd2)

## [0.2.1] - 2023-07-07


### Bug Fixes
- Fix more error handling (29fd7ed)
- Fix up error types (afc316c)
- Fix imports (35455d7)
- Fix OpenJDK paths on macOS (6adbc00)
- Fix (0e5cdf1)


### Features
- Add --verbose to available/downloaded commands (b9fd4ac)
- Add PluginFactory trait (f5c17af)


### Internal
- Simplify plugin API (b833884)
- Simplify errors (857907d)
- Rename PackageInfo -> Package (1ceac11)
- Store plugin-specific files and assets in isolated directories (2e8f774)
- Show descriptor for locally downloaded packages (7330724)
- Use OsString for file name (43ec045)
- Rename repository_url -> source_url (3b78fb0)
- Store full asset path (b4bf617)
- Clean up help text (c8df227)
- Swap lines (2370bf9)
- Refactoring (f5f5d11)
- Refactor plugins (3483f47)
- Rename Plugin -> PluginHost (a612b40)
- Renames (9eb11e9)
- Refactoring (ce6ff04)
- Refactor traits (16bd454)
- Tidy up Python plugin (eb34843)
- Tidy up OpenJDK plugin (1ff9694)
- Formatting (c5b6db2)
- Whitespace (d36d2cc)
- Use ASSETS_DIR (577af04)
- Refactor (f9aa865)
- Clean up errors (9a5babe)
- Tidy up (1efedb3)
- Whitespace (63dad13)
- Tweak versions (14a37e1)
- Reorder (868aa3c)
- More tidy-up (6bc5108)
- Handle no packages (4d9d595)

## [0.2.0] - 2023-07-05


### Bug Fixes
- Fix paths and environment variables (24aa0de)
- Fix struct names (35c81d3)
- Fix tests (b8a2048)
- Fix product descriptor (4525100)
- Fix available (8388b45)
- Fix uses (9d6c95d)
- Fix import (1a5aa00)


### Features
- Add prompt subcommand (ba154e7)
- Add OpenJDK version support (da21e72)
- Add Maven version range support (c8cc6d5)
- Add missing re-exports (206cbd7)
- Support macOS OpenJDK (3a9506a)
- Add AArch64 (db60f7c)
- Add Windows support (5e10d66)
- Add libraries (05f4619)
- Add .devtool.yaml (8d252e1)
- Add comment (b165ce3)
- Support multiple packages per environment (e432022)


### Internal
- Minimize tokio dependencies (3fe5780)
- Don't display prompt if it's empty (fd2ef82)
- Document basic shell integration (620409f)
- Extend find_dir_info to handle ISOPY_ENV (3f0c595)
- Abbreviate the prompt (f5d8436)
- Parse link headers (9d753a4)
- Serialize and deserialize OpenJDK versions (4fe74d2)
- Implement Clone for OpenJdkVersion (552f94e)
- Improve context message (824aa55)
- Split out reqwest-specific implementation (84e34c1)
- Use LogLevel instead of Option<LogLevel> (52dec98)
- Adoptium API (4aaa7e9)
- Upgrade dependencies (55ac717)
- Display version and tag for Python builds (134a661)
- Sort Python packages by version and tag (e2309f2)
- Display URL of source (3b815f8)
- Product descriptor (711c4a4)
- Use product descriptor for download command (aa514ce)
- Validate SHA256 checksums on OpenJDK downloads (8303757)
- Remove tracing (73ab310)
- Generate OpenJDK configuration file (3035fed)
- Create separate Python and OpenJDK product descriptors (69881e0)
- Remove PythonVersion struct (7bfd8b5)
- Refactor a bit (228c8b7)
- Unzip OpenJDK tarball (dc986b8)
- Log instead of print (6157e22)
- Unpack Java distribution (ebb7875)
- Set JAVA_HOME etc. (9246f51)
- Remove unneeded serializer/deserializer for Url (6f804f5)
- Split product descriptors (9d34045)
- Create api module (bf575fa)
- Rearrange sources (8658cf5)
- Move more files around (0c2c0e2)
- Rename MavenVersion -> MavenVersionRange (e117085)
- Rearrange into workspace (87acd62)
- Move profile into workspace (6c544cd)
- Move Python version code into isopy-python (8de22a5)
- Move OpenJDK code into isopy-openjdk (2761caa)
- Split into multiple sources (4c98c4c)
- Move more code around (b10cb98)
- Turn demo into tests (2b0dc68)
- Annotations (fd12ecf)
- Consolidate types (9024b51)
- Renames (5577fcf)
- Refactor (f16847b)
- Tidy up (8131bf6)
- Use registry (fe6fe6e)
- More refactoring (bb96a37)
- Move path transformation to Descriptor (4a59141)
- Extract configuration into products (cb6ce67)
- More refactoring (1313a8a)
- Move code around (71d16c5)
- Move more code around (542d4e9)
- Move OpenJDK serialization (6446a5a)
- Move more code (1a0ccef)
- Move download code into isopy-openjdk (e3c57bb)
- Move more code (35585ac)
- Extract project version code (32aa9e3)
- More refactoring (17e0fcd)
- Break apart dependencies (8984ad1)
- Move url module (3a649dc)
- Move constants around (f7a9901)
- Separate out Foo (83c3b76)
- Move RepositoryName (8385ff0)
- Move more code into isopy-python (3e41331)
- More refactoring (7ea001d)
- Create environment info (e96ace8)
- Print package dirs (708d7d0)
- Remove dead code (99c81ca)
- Partially push index code down (ef7196f)
- Extract Python index code (063286d)
- Refactor (8a64f50)
- Move Foo into isopy-python (e347390)
- Refactor (45c3b48)
- More refactoring (0ac3bc4)
- Move URL (1ad8e73)
- Move asset handling code (29f09cc)
- Minimize re-exports from isopy-python (292e7bd)
- Minimize re-exports from isopy-openjdk (ce1ca10)
- Renames (78de5a0)
- Rename ProductInfo -> Plugin (4d09f22)
- Global registry (fdfa22c)
- Use global registry (1695de1)
- Remove registry from app (024fb50)
- Refactor DescriptorId (ff1a28d)
- Simplify API (c3ca6e6)
- Rename DescriptorId -> PackageId (a721a74)
- Upgrade dependencies (bd9a3ae)
- Suppress warning (6e55b42)
- Join prompts properly (c95229a)


### Tooling
- Update dependencies (d44ba16)
- Fix Clippy issue (a8f3bae)
- Fix Clippy issues (b8c089d)
- Fix Clippy issues (babc5ca)
- Fix Clippy issues (d194b8e)
- Fix Clippy issues (ccae8f2)

## [0.1.4] - 2023-06-03


### Internal
- Implement Serialize and Deserialize on LastModified (6d723cf)
- Implement Serialize and Deserialize on RepositoryName (ed45b81)
- Implement Serialize and Deserialize on Tag (ac60359)
- Implement Serialize and Deserialize on Option<Tag> (886f848)
- Implement Serialize and Deserialize on Version (d00ff37)
- Rename helpers -> url_serde (65349d3)
- Upgrade dependencies (8af6206)

## [0.1.3] - 2023-05-30


### Features
- Add check command (2355018)


### Internal
- Rename cli -> args (ea342eb)

## [0.1.2] - 2023-05-22


### Bug Fixes
- Fix progress bar templates (890ab92)
- Fix typos (94530da)
- Fix ownership of indicator instance (aeeca58)
- Fix exec command (fc30390)
- Fix uses (5f6d4fc)


### Features
- New UI code (e8fd656)


### Internal
- Move backup code into download_stream (795fec3)
- Flatten module structure (d4e1378)
- Disclaimer (c2f812b)
- Remove banner (545303d)
- Remove old content (d224d02)
- Upgrade to newer time (ddd3dc8)
- Use FromStr (d32e5a6)
- Implement FromStr for AssetMeta (b0904c1)
- Lower-case error message (5b18467)
- Simplify functions (610442a)
- Whitespace and rename (37a2213)
- Code coverage script (bae1964)
- Improve styling of progress bar (a0f9848)
- Remove old Indicator struct (776634e)
- Use match (d8359a4)
- Remove extraneous whitespace (584b79c)
- Don't panic (45511bd)
- Simply ui2 and fix TOCTOU (fe3940d)
- Upgrade dependencies (285d5d3)
- Use joat-logger (75525ac)
- Flatten modules (7e5e263)
- Upgrade dependencies (c563ccf)
- Get wrap working again (d7fa1e5)


### Tooling
- Update docs (9e95d79)
- Update cookbook (65aae4c)
- Update to latest dependencies (7bb0c39)
- Update joat-repo (43ebc83)
- Apply Clippy fixes (2613f00)
- Turn Clippy up (98dab3f)

## [0.1.1] - 2023-05-18


### Bug Fixes
- Fix Windows build errors (daa6d14)


### Internal
- Tidy up URL handling (7e81d39)
- Test all lazy_static variables (325bc39)
- Clean up RepositoryName (a1c7fb5)
- Remove more expects (479d5f6)
- Remove debugging (0d1f804)
- Explicitly drop app before starting child process (ce7b2ad)

## [0.1.0] - 2023-05-17


### Bug Fixes
- Fix index update (abb08dc)
- Fix bug (a30e9e3)
- Fix labelled files (1b882b6)
- Fix construction of PATH (090c6d0)
- Fix warnings (5b9d188)
- Fix Windows (7833bd1)


### Features
- Add links to badges (8fb778b)
- Add missing Windows dependencies (29008c2)
- Add status (499d48e)
- Add --force to gen-config (cf06935)
- Add log level (b72d3f8)


### Internal
- Reorder badges (9358ef8)
- Remove unused dependencies (abe4efe)
- Upgrade dependencies (90cf41b)
- Upgrade dependencies (c771194)
- Remove Python implementation (0d9033e)
- Refactor to use joat-repo (b32b963)
- Clean up dependencies (08df708)
- Use show helpers (d9f894a)
- Upgrade to joatmon 0.0.25 (058e4b1)
- Get download working again (92e09c5)
- Remove some unused code (5e22fa4)
- Revert cache directory name (7589695)
- Clean-up output (79bd2af)
- Remove more unused code (4737db0)
- Renames (91050f3)
- Remove more unused code (e6b7df4)
- Improved backtrace (45801c9)
- Consolidate constants (f868881)
- Check for presence of index.json (753ee63)
- Rename index.json -> releases.json (e07ef03)
- Rename constants (cd972f1)
- Implement TryFrom (a08c5ac)
- Move code around (1496a31)
- Upgrade dependencies (dfc2b1e)
- Use label_file_name from joatmon (db59424)
- Move UI into new module (ccda988)
- Extract constant (531ca2c)
- Move more code around (34bb755)
- Separate backtrace module (a68507d)
- Separate run module (1d1b3a5)
- Refactor run (676cf21)
- Eliminate one use of path_to_str (b2362c4)
- Remove uuid dependency (f162752)
- Move find_dir_info (c78abc4)
- Remove repo.rs (a6e418e)
- Move unused imports (05c4b30)


### Tooling
- Update dependencies (1e97ab7)
- Update dependencies (e5745dd)
- Update command names (588a7ef)

## [0.0.30] - 2023-04-20


### Bug Fixes
- Fix use statements (390f2b4)


### Internal
- Replace swiss-army-knife with joatmon (b1e894e)


### Tooling
- Update to use rust-package-action and latest rust-tool-action (07e586a)

## [0.0.29] - 2023-04-17


### Bug Fixes
- Fix action version (eafdb9c)

## [0.0.28] - 2023-04-17


### Internal
- Enable code-signing on Windows (f2c8fce)

## [0.0.27] - 2023-04-16


### Internal
- Use rcook/rust-tool-action (44f8eea)

## [0.0.26] - 2023-04-16


### Internal
- Use shasum on macOS (00fc0c0)

## [0.0.25] - 2023-04-16


### Tooling
- Update SHA256 checksums (6986840)

## [0.0.24] - 2023-04-15


### Internal
- Pass ISOPY_BUILD_VERSION to Cross and set variables in right place (643b6d5)

## [0.0.23] - 2023-04-15


### Internal
- Tweak version output in help (b435f2b)

## [0.0.22] - 2023-04-15


### Bug Fixes
- Fix version of swiss-army-knife-rs (9c4d842)
- Fix Windows build (7f342cc)


### Dependencies
- Bump h2 from 0.3.16 to 0.3.17 (408b685)


### Features
- Add copyright headers (df3eff3)


### Internal
- Start using swiss-army-knife-rs (4bbe957)
- Remove unnecessary error handling (309d699)
- Don't show unlinked file notification (b236366)
- Use anyhow (6512a50)
- Use find_sentinel_file (85165c9)


### Tooling
- Fix bug - thanks Clippy! (2fad72c)
- Update dependencies (1ab10db)
- Fix Clippy issues (e4c5b90)

## [0.0.21] - 2023-04-11


### Internal
- Move MD5 code into util (7e0586a)
- Use Cross and build MUSL (5b687b3)

## [0.0.20] - 2023-04-11


### Internal
- Remove git describe (b098e4b)

## [0.0.19] - 2023-04-11


### Tooling
- Fix workflow (b5e2a38)

## [0.0.18] - 2023-04-11


### Bug Fixes
- Fixes #2: Add .yaml extension (733e067)


### Dependencies
- Bump tokio from 1.26.0 to 1.27.0 (b8758dd)
- Bump reqwest from 0.11.15 to 0.11.16 (9838ce5)
- Bump futures-util from 0.3.27 to 0.3.28 (ba5cc13)
- Bump serde from 1.0.158 to 1.0.159 (3c9c72f)
- Bump serde_json from 1.0.94 to 1.0.95 (1707f66)


### Internal
- Dependabot configuration (97af793)
- CI/release badges (2b96bc1)
- Remove linkedProjects (f375e0b)
- Move some source files around (feab5f3)
- Pass version in during build (9e411de)


### Tooling
- Get Clippy clean and enforce code format (5b8b556)

## [0.0.17] - 2023-04-10


### Internal
- Specify target (8e0ffd0)

## [0.0.16] - 2023-04-10


### Internal
- Configure static linking (af4c5b5)

## [0.0.15] - 2023-04-10


### Features
- Add aarch64-apple-darwin (a1b6bc8)

## [0.0.14] - 2023-04-10


### Features
- Add permissions (060a905)

## [0.0.13] - 2023-04-10


### Internal
- Remove config.toml (aebf9d5)
- Simplified GitHub actions (747cb75)

## [0.0.12] - 2023-04-08


### Internal
- Statically link on Linux (a6e3947)

## [0.0.11] - 2023-04-06


### Internal
- Construct sanitized environment name from project configuration path (1e53c9c)
- Start of next big rename (25b1108)
- Rename more things (7530c17)
- Split more modules up (2082a64)
- Split more modules (9446919)
- Use correct asset filter on AArch64 (181f22b)
- More renaming (4485ac1)
- Don't get trap in a symlink loop (9516aae)

## [0.0.10] - 2023-04-06


### Bug Fixes
- Fix (d74b5b4)


### Internal
- Rename .isopy.yaml -> .python-version.yaml (435948a)
- Translate another IO error (04dc7aa)
- Translate more IO errors (90462b1)
- Translate even more IO errors (cc1d89f)
- Whoops (768faf8)
- Map more errors (6aa6c11)
- Map more errors (3c73527)
- Tidy up errors (e5e7bbb)
- Reorder matches (51e3e97)
- Make dir global (9d75577)
- Dump out more information (7f83c8d)
- Link with lld (380d7a3)
- Remove config.toml (9c0624b)
- Use different exit codes (d12d223)
- Probe for project configuration files in parent directories (cf55d49)
- Show anonymous environment directory (e082bdc)


### Tooling
- Update to remove-dir-all 0.8.2 (db9600b)

## [0.0.9] - 2023-04-03


### Bug Fixes
- Fix whitespace (a9af046)


### Internal
- Describe Git commit (93156a2)
- Remove optional (656f8b4)

## [0.0.8] - 2023-04-03


### Internal
- Force tags (7ec209b)

## [0.0.7] - 2023-04-03


### Tooling
- Fetch tags in workflow (b8dceb6)

## [0.0.6] - 2023-04-03


### Bug Fixes
- Fix link (4caa086)


### Internal
- Link to Python reference implementation (7a16448)
- Use vendored OpenSSL (eb411c0)

## [0.0.5] - 2023-04-03


### Bug Fixes
- Fix releases link (1e2e2ea)


### Features
- Add repository to concepts (605f5aa)


### Internal
- Move concepts and cookbook into GitHub Pages web site (246313e)


### Tooling
- Update installation instructions (e95bcc7)

## [0.0.4] - 2023-04-03


### Bug Fixes
- Fix Windows use (9c4a251)


### Internal
- Rename to isopy and beef up README (8880ea4)

## [0.0.3] - 2023-04-03


### Bug Fixes
- Fix test (06002b2)


### Internal
- Rename error.rs -> result.rs (1ea22b2)
- Simplify error handling (19c9f49)
- POC for alternative repositories (8cc5fed)
- Remove debugging code (90caf09)
- Use safe_create_file (d0a708f)
- Use Self (e5ac517)
- Move more code into repository (a083180)
- Use Last-Modified header (29756f7)
- Tidy up traits (b1a4560)
- Handle trailing slashes in URLs (0e58a0a)
- Multiple repositories (eda47fa)
- Distinguish between default repo and others (b7fd4ef)
- Use repository name to read/write index YAML file (28d96f2)
- Cache Last-Modified headers etc. (48e445c)
- Get asset (5774781)
- Move code around (2300a5d)
- Filter repositories at App level (e205aae)
- Use new repository download mechanism (fedd55d)
- Rename to download_stream (10d5ac7)
- Move ISOPY_USER_AGENT into GitHub repo code (5a66e89)
- Use Indicator and remove ProgressIndicator (c63aa3b)
- Remove allow(unused) in AssetFilter (088e8cf)
- Remove more unused attributes (8eb84d0)
- Start improving the error handling (9972e94)
- Translate JSON errors (6d8eed3)
- Remove debugging (c35a339)
- Renames (42fd658)
- Include Git version in CLI output (5a170b4)


### Tooling
- Update available command to use new repository implementation (d8187e0)

## [0.0.2] - 2023-03-31


### Bug Fixes
- Fix binary name (ca8b5bd)


### Features
- Add permissions (f04bf0a)

## [0.0.1] - 2023-03-31


### Bug Fixes
- Fix-ups (cb49d66)
- Fix download command (90310c4)
- Fix shell command (3c08ce3)
- Fix init command (172e35e)
- Fix app path (a7a016c)
- Fix up list command (4d94463)
- Fix comment (ce9c393)
- Fix up exec command (58bb143)
- Fix up wrap command (71c2a72)
- Fix more outputs (789bc2c)
- Fix bugs (4c60855)
- Fix string (39385c9)
- Fix (18b8fc3)
- Fix (f83531a)
- Fix (11dad43)
- Fix None problem (e11ba0e)
- Fix (0ff03f9)
- Fix target directory (e966183)
- Fix path to Python (7064845)
- Fix bootstrap script (5e01b0a)
- Fix wrapper script (a432c73)
- Fix list command (a55b488)
- Fix tiny bug (16b8f2b)
- Fix error message (ee9390f)
- Fix Windows cmd (cc5b9e9)
- Fix install instructions for Linux/macOS (67c67cb)
- Fix italics (79d2b5b)
- Fix unused imports (fd5d212)
- Fix error reporting, status code and other tweaks (8a515f2)
- Fix tests (c11e8ef)
- Fix build under Windows (d48238c)
- Fix Python path (b4bd230)
- Fix use subcommand (25b64cc)
- Fix warning (0987b24)
- Fix CLI help (ab5e409)
- Fix init and new commands (1eeee6a)


### Features
- Add versions subcommand (a50f1a8)
- Add envs subcommand (201a9d7)
- Add EnvManifest (6fd97a0)
- Add more SHA256 checksums (8fd50fb)
- Add init subcommand (952e756)
- Add "new" command to create project manifest (55b3691)
- Add checksums into package (08e880d)
- Add info command (288e30c)
- Add imports (a34bf42)
- Add placeholders for each subcommand in README (65e9057)
- Add --refresh to available subcommand (a16e109)
- Add --env to sample commands (a3b9a5d)
- Add refresh option (178010d)
- Add create instructions (1af9914)
- New "use" command (018080b)
- Add stub for use command (8c7801f)
- Add "use" support to "shell" command (4bc6365)
- Add --detailed/--no-detailed switches (064bb98)
- Add --prune to prune Python directories from system search path (4a2470e)
- Add command-line help to bootstrap (9808a8c)
- Add help and arguments (ddad116)
- Add -- (6ba87fc)
- Add missing import (82c32ac)
- Add --detail switch (4bdda65)
- Support cmd.exe as well as PowerShell (f11bbd8)
- Add quotes (2a8521d)
- Add documentation links and move Python version info to debug command (3d7c26a)
- Add -UseBasicParsing (77eace8)
- Add boilerplate to docs (7819bc1)
- Add missing link (cd7b083)
- Add banner to README.md (67f8ad1)
- Add some data-driven tests for asset name parsing (de0a6ae)
- Add newtype wrappers (3884840)
- Support subcommands (95de098)
- Add size (cb92672)
- Add assets_dir and envs_dir to Config (20dfbc3)
- Add Config.read_envs (6991195)
- Add SHA256 checksums (a79733f)
- Add command-line help (8d3ba92)
- Add env_name to exec command (f3096eb)


### Internal
- Initial commit (cc50a36)
- Implement simple filtering of versions (b2f9eda)
- Pass in tag name and Python version (7c494e1)
- Spawn child shell (76073ff)
- Improve message and read SHELL (f095cf0)
- Platform check (02e5016)
- Handle macOS files (e0636d0)
- Handle macOS (3e08d80)
- Sort by version (603b29e)
- Parse version (44fa932)
- Use os.execle to create shell (1f849b1)
- Platform module (ba3561a)
- Extract more modules (e584d2f)
- Use download_file (a2eeec1)
- Remove unused import (6d5b733)
- Use exec (cf366e3)
- Handle no env directory (0837847)
- Use tag name from asset (371c409)
- Rename envs -> list (1ede3ed)
- Checksums (78112a8)
- Verify checksum (c75ce9d)
- Use checksums based on tag name (2ad5a6e)
- Convert line endings (81567b5)
- Move release index to assets directory (28892ec)
- Separate download command (4afb5c9)
- Extract filtering (59cf86f)
- Remove debugging code (4de7133)
- VSCode settings (0364ca4)
- Ensure LF (ad812d8)
- Big refactor (73fa2fc)
- More fix-ups (67d3844)
- Auto-description (ceccedb)
- Get list command working (663379b)
- Pass in log level (175ef9e)
- Set default log level (1b6f12a)
- add_python_version_arg (b3f1960)
- Remove wrapper (9f0768a)
- Implement exec command (b651457)
- Extract common exec code (14df0a8)
- Use execlpe (7480d58)
- Use colour (fb29252)
- Debug level (a570e12)
- More arg parsers (9b13520)
- Move add_subcommand (43480b0)
- Show Python interpreter info (1074062)
- Wrapper script (b8fc562)
- downloaded command (99bdb61)
- Show file size (3b30f3c)
- Read project manifests in shell command (8ddd62e)
- Constrain env name (810c97c)
- Disallow slashes (d1ed00c)
- Tidy up (ce9bd76)
- Move code around (5738fdc)
- Check that environment doesn't already exist (5099102)
- Combine new and init into init (ede070b)
- Show default logging (5760d5b)
- Show default force (16b02b9)
- Whitespace (8151240)
- Include isopy config file (13859b4)
- Bootstrap script (0ffd1ea)
- Create manifest (3286250)
- Create local project manifest (4a776d5)
- Wrap command (fb0233c)
- Improve wrap (44795fc)
- Makefile for pyinstaller (88a1a9a)
- PyInstaller spec (0284e5c)
- Debug command for showing runtime debugging information (ecaa038)
- List checksum files (4c6e575)
- Tidy up spec (8201b3f)
- Tidy up debug command (36e6a54)
- Remove EnvInfo (e510521)
- Big refactor (83f1abb)
- Extract ProgramInfo (fcbb4dc)
- Don't show which output when frozen (b66b0bb)
- Refactorg (83dfe20)
- Clean up downloaded command (75feb70)
- Sort in reverse Python version order (250a667)
- Download working (331ba25)
- Handle named environments (7b232b6)
- More named environment fixes (4955cb3)
- Slim down list output (76a8882)
- Use os.pathsep instead of colon (98e886d)
- Refactor (938de3f)
- Rename files (794441e)
- Use environment variable to prevent nested shells (06d8bd3)
- Get it working on Windows (4e0749b)
- Refactor some platform-dependent code (e14e044)
- Refactor shell code again (e460ac0)
- More exec work (1a4752e)
- Constants (d3c4849)
- Tidy up (be77b27)
- Exec works on Windows (8e3e0af)
- Create command (4ee2590)
- Stuff isopy-bootstrap away (f918def)
- Handled named environments (abe355a)
- More documentation for available command (164c209)
- Rename --tag-name to --tag (8fe3a0a)
- Rename bootstrap script (aae33db)
- Create wrapper script on PATH (18c6a3e)
- Extend bootstrapping instructions (e18de61)
- Instructions for modifying your .bashrc etc. (0156a47)
- More discussion (ebd2b08)
- More documentation (2170c3f)
- Start isopy shell (506630a)
- Show download command (0512aa2)
- Reuse code in asset.py to extract asset information from file names (714acbe)
- Use exec (74fbd42)
- Link to exec docs (04be666)
- Better manual page (8ba4274)
- Remove setup script (fa2022a)
- Remove __init__.py (18b3056)
- Remove scratch directory (bf11470)
- MIT licence file (0902717)
- Feature requests (44ac3be)
- Show PATH instructions (1f20ec3)
- Use correct file name extension (15bb08e)
- Create use config files (23a2efc)
- Allow force-overwrite of "use" (1f4e70f)
- Extend "exec" to respect "use" (e1ca5b8)
- Mention the \"use\" subcommand (3984ec6)
- Show platform information in debug output (3f98cdd)
- Separate out subcommands (dfe6b41)
- Compute version string (5cae951)
- Extend "info" command to handle "use" (db26787)
- Help strings (0512523)
- Don't include version numbers in Python directories (7481e7a)
- Tweak bootstrap script (0331d65)
- Show common Python programs accessible from system search path (0ba7e2e)
- Remove foo.py (ca237fa)
- Ignore build and dist directories (0680a8c)
- Report the directory (c8a1238)
- Partial implementation of PowerShell bootstrap script (a7192aa)
- Generate wrapper script for PowerShell (3b52bc8)
- Developer requirements (73ab2c4)
- Render PATH (f5cfa4f)
- Generate PowerShell wrapper scripts (2d20749)
- Rename --detailed to --detail (2f1f887)
- Windows command file wrapper (1e41662)
- Windows command file bootstrap script (4c3267a)
- Remove exit 1 (51e7a3c)
- Remove temporary file on unexpected exit (9f2aaa4)
- Improve CLI error messages (4243c28)
- Restructure bootstrap to pull source too (9040291)
- Write wrapper to stdout for curl-based bootstrapping (972a2ae)
- Bootstrap with curl (00d338a)
- Improve bootstrap output (b1873aa)
- Copy instead of move (5d03555)
- --quiet flag for shell (a67a40b)
- Change heading levels (ac16a59)
- Improve layout of shell instructions (9389cb8)
- Introduce temp (3296348)
- Simplify path (b50b456)
- Introduce more temporaries (1ed3c61)
- PowerShell bootstrapper (1d9b7ca)
- Indicate if this is a bootstrapped instance of isopy or not (134bb85)
- Generate Windows command script instead of PowerShell (3d179c4)
- Tweak comments (c858ab0)
- Remove PowerShell wrapper (e6d72f3)
- Rename bootstrap to setup (81ccfe4)
- Remove bootstrap.cmd (68d33d8)
- Remove Makefile (49e2f92)
- Remove pyinstaller dependency (13bf0c7)
- Remove pyinstaller spec (e4f9282)
- Remove unnecessary .gitignore entries (9dc8f42)
- Generate Windows command script wrappers instead of PowerShell (5f0d74e)
- Provide a dummy value on non-Windows platforms (de9984d)
- Index for GitHub pages (0d8cfa4)
- Link to documentation (44b9126)
- Get rid of module wrapper (3b0360f)
- Simplify Windows setup script (7a74a29)
- Handle the command script wrapper while detecting the current shell (d6e765e)
- Show process hierarchy (870c384)
- Display inferred shell in debug output (78f3019)
- A more reliable way to run setup.ps1 (50fc661)
- TBD section (7545043)
- Handle AccessDenied when trying to read process environment (d8208a5)
- More documentation (3d986e0)
- Streamline docs (b20dbf1)
- Mention bash and PowerShell (1104b61)
- Placeholders for each command (8ed9213)
- Get rid of table (098c646)
- Move usage information out of README (0bae9a9)
- Provide Set-ExecutionPolicy command line (052bb25)
- Handle double quotes (3d6589c)
- Extract URLs from help output and open them in default browser (eb1f9b5)
- Don't open the default help topic in browser (3d33f11)
- Feature requests moved to GitHub issue tracker (62970b4)
- More documentation for available command (ca51b62)
- Document create command (807e016)
- Move Python reference implementation into py directory (842532a)
- Initial commit (0504d28)
- Separate code into modules (17923c8)
- Deserialize Python Standalone Build index.json to Rust structs (07e74b2)
- Make web requests using reqwest (64d29cf)
- Deserialize to Url etc. (2e0670f)
- Print out asset name (e35685b)
- Rename asset_name -> asset_info (34359f1)
- Parse more fields (d70f8f3)
- Parse the rest of the asset name (18cd15c)
- Version struct (46689ce)
- Simplify code (d604f43)
- Parse more names (5237c58)
- Iterate over all packages (d84f07a)
- Handle tags better (39dd178)
- Filter out SHA256 sums etc. (11e17a3)
- Use enums for some asset info fields (ba58c2a)
- Parse asset names even better! (895f455)
- Remove debugging code (bad69a6)
- Simple filtering (8d83f4a)
- Rename parsing -> object_model (275b6b9)
- Create attributes module (597f46f)
- Change use statement (d9720b9)
- Implement AssetFilter (7d0cce2)
- More AssetFilter tests (d822a5f)
- Filter (b93e142)
- Default for platform (3e8ffce)
- Make it platform-specific (2b53336)
- Separate out subcommands (d6ea488)
- Dump out asset info (897684e)
- Rename subcommand for consistency with isopy (1b39396)
- Simplify Tag (df59289)
- Reorder match branches (69de5fe)
- Unwrap (bd9c979)
- Rename Package -> PackageRecord and Asset -> AssetRecord (f3438f5)
- Rename AssetInfo -> Asset (92df16c)
- More renames (f1bbaff)
- Rename asset_name -> asset_meta (dd8053c)
- Introduce Asset (afb70c1)
- Download file with progress bar (929e6bd)
- List command (e902fa4)
- Deserialize to struct (e86a7a5)
- Remove httpbin.rs (fe30feb)
- Deserialize fields better (83a199a)
- Include --version in CLI (c812398)
- Start building create command (0f4e1ec)
- Separate unpack_file helper (4d6a7fc)
- Rearrange (41b6f66)
- Unpacking with progress (6e1c46c)
- Unpack archives with progress (3ecdbd2)
- Create environment directory (320204c)
- Create environment YAML (b4916b1)
- Shell command (cea6c42)
- Use do_shell_platform on Linux (5d1d2ab)
- Check ISOPY_ENV before starting shell (ec40de0)
- Make environment name optional but unimplemented (9610b3c)
- Optimize for binary size (eb02c28)
- Remove AssetMeta.Tag (c4b263b)
- Derive Python bin directory from env.yaml etc. (d46448a)
- Implement info and new subcommands (1e4df19)
- Mostly implemented shell for projects (97f1c27)
- Ignore target directories (56212b6)
- Create hash map (b0e4d67)
- Perform SHA256 checksum validation (869d26d)
- Validate all checksums (020199f)
- Validate SHA256 checksum after downloading (a74d54e)
- Validate SHA256 checksum and delete downloaded file if it fails (709b3c1)
- Implement use command (8e7a543)
- Rename Config -> App (d138eb0)
- Shell for Windows (700ed72)
- Placeholder for exec command (8e9cbda)
- Explicitly name subcommands (547b336)
- Extract code (1f7c07c)
- Rearrange shell code (1864062)
- Refactor shell code again (22f4942)
- Exec kinda works now (bb43319)
- Turn errors into error results (8b92eb5)
- Download index.json in available command (a68e0e3)
- Download index.json (a6a5ea9)
- Save Last-Modified date in index.yaml (d2939e4)
- Pass environment name to use command (b2ea96a)
- Wrapper Indicatif in ProgressIndicator (bf12b7d)
- Remove use (8fd1557)
- Reuse client (61c740b)
- Use download_file for index.json (6658a38)
- Compute full path to Python binary directory (09415b2)
- Start cookbook (4158fba)
- Remove create_env_dir (ea5fcd7)
- Rename EnvRecord -> NamedEnvRecord and HashedEnvRecord -> AnonymousEnvRecord (179b8b4)
- Use named env naming convention (4969a7d)
- Tidy up path generation (3a23b21)
- Extend list command (5b66a23)
- Constant defining user agent (05349a7)
- Rename safe_write_to_file -> safe_write_file (eb09fa9)
- Allow init to perform download (fb1c15b)
- Tidy up code (a351d73)
- Move make_asset_path to App (64ed9a2)
- Get exec working properly (ea7c003)
- Show links (b1fb75b)
- Implement Windows exec (fab278b)
- Move helpers into root (485a913)
- Rename ShellInfo -> EnvInfo (fd8daf1)
- Remove exec-rs submodule (9edf95c)
- Rename scratch -> wrap (b4154ae)
- Remove unused function (f182932)
- Use TinyTemplate to generate bash wrapper (0008751)
- Small refactor (073c0e6)
- Move platform-specific code (933cc24)
- Generate batch file on Windows (fe7f239)
- Core concepts (af5941c)
- Remove unused reqwest features (71effaa)


### Tooling
- Update CLI (e9ef6a6)
- Update dependencies (685c2eb)
- Update README.md (ae86b45)
- Update README (cdc9e75)
- Update bootstrap to support macOS (e6a1934)
- Update README (67ec883)
- Update README (3e037bc)
- Install requirements (5266c81)
- Update dev instructions (5c40659)
- Update Windows bootstrap script (a1d787b)
- Update setup script to match setup.ps1 (5c5d87d)
- Rust build and test workflow (a8b1c31)
- Workflow to release Rust binaries (804eb9f)


