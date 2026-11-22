//! Build metadata. The release build sets the `BUILD_*` variables at compile
//! time; local builds fall back to the crate version.

/// The release version, e.g. `1.2.3`.
pub(crate) const VERSION: &str = match option_env!("BUILD_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};

const COMMIT: &str = match option_env!("BUILD_COMMIT") {
    Some(commit) => commit,
    None => "HEAD",
};

const DATE: &str = match option_env!("BUILD_DATE") {
    Some(date) => date,
    None => "now",
};

/// The `--version` text: version, short commit and build date.
pub(crate) fn long_version() -> String {
    let short_commit = COMMIT.get(..7).unwrap_or(COMMIT);
    format!("{VERSION} ({short_commit}) {DATE}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_version_starts_with_version() {
        assert!(
            long_version().starts_with(VERSION),
            "long version should lead with the release version"
        );
    }
}
