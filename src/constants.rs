pub const RELEASES_URL: &'static str =
    "https://api.github.com/repos/indygreg/python-build-standalone/releases";

pub type ExitCode = i32;

pub const OK: ExitCode = 0;

pub const GENERAL_ERROR: ExitCode = 1;

pub const USAGE: ExitCode = 2;
