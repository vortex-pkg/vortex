use lazy_static::lazy_static;
use regex::Regex;
use urlencoding::encode;

const BLACKLIST: [&str; 45] = [
    "node_modules",
    "favicon.ico",
    "assert",
    "buffer",
    "child_process",
    "cluster",
    "console",
    "constants",
    "crypto",
    "dgram",
    "dns",
    "domain",
    "events",
    "fs",
    "http",
    "https",
    "module",
    "net",
    "os",
    "path",
    "punycode",
    "querystring",
    "readline",
    "repl",
    "stream",
    "string_decoder",
    "sys",
    "timers",
    "tls",
    "tty",
    "url",
    "util",
    "vm",
    "zlib",
    "freelist",
    "v8",
    "process",
    "inspector",
    "async_hooks",
    "http2",
    "perf_hooks",
    "trace_events",
    "worker_threads",
    "wasi",
    "diagnostics_channel",
];

lazy_static! {
    static ref SPECIAL_CHARACTER_REGEX: Regex = Regex::new(r"[~'!()*]").unwrap();
}

/// Validate an npm package name
///
/// This **does not** accept legacy names!
///
/// This also assumes that all experimental modules
/// are enabled.
pub fn validate(name: &str) -> Result<(), &str> {
    if name.is_empty() {
        return Err("Package name must not be zero-length");
    }

    if name.to_lowercase() != name {
        return Err("Package name cannot contain capital letters");
    }

    for item in BLACKLIST.iter() {
        if &name == item {
            return Err("Package name is blacklisted");
        }
    }

    if name.contains(' ') {
        return Err("Package name cannot contain spaces");
    }

    if name.starts_with('_') || name.starts_with('.') {
        return Err("Package name cannot start with a period/underscore")
    }

    if SPECIAL_CHARACTER_REGEX.is_match(name.split('/').last().unwrap()) {
        return Err("Package name cannot contain special characters ('~\'!()*')");
    }

    if encode(name) != name {
        let matches: Vec<&str> = name.split('/').collect();

        if matches.len() != 2 {
            return Err("Package name must be URL-friendly");
        }

        let user = matches[0];
        let package = matches[1];

        if !user.starts_with('@') {
            return Err("Package name must be URL-friendly");
        }

        let mut user = user.to_string();
        user.remove(0);

        match (validate(user.as_str()), validate(package)) {
            (Ok(()), Ok(())) => return Ok(()),
            _ => return Err("Package name must be URL-friendly"),
        };
    }

    if name.len() > 214 {
        return Err("Package name cannot be longer than 214 characters")
    }

    Ok(())
}

// Please run the tests, I spent way too long writing these
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn valid_scoped_package() {
        assert_eq!(validate("@npm/cool"), Ok(()));
    }

    #[test]
    fn valid_kebab_case() {
        assert_eq!(validate("some-package"), Ok(()));
    }

    #[test]
    fn valid_domain_package() {
        assert_eq!(validate("example.com"), Ok(()));
    }

    #[test]
    fn valid_underscore_package() {
        assert_eq!(validate("under_score"), Ok(()));
    }

    #[test]
    fn valid_periods_in_package() {
        assert_eq!(validate("package.js"), Ok(()));
    }

    #[test]
    fn invalid_no_special_chars() {
        assert_eq!(
            validate("@npm-zors/money!time.js"),
            Err("Package name cannot contain special characters ('~\'!()*')")
        );
    }

    #[test]
    fn invalid_empty() {
        assert_eq!(validate(""), Err("Package name must not be zero-length"));
    }

    #[test]
    fn invalid_start_with_period() {
        assert_eq!(
            validate(".cool"),
            Err("Package name cannot start with a period/underscore")
        );
    }

    #[test]
    fn invalid_start_with_underscore() {
        assert_eq!(
            validate("_underscore"),
            Err("Package name cannot start with a period/underscore")
        );
    }

    #[test]
    fn invalid_no_spaces() {
        assert_eq!(
            validate(" discord"),
            Err("Package name cannot contain spaces")
        );
    }

    #[test]
    fn invalid_no_node_modules() {
        assert_eq!(validate("node_modules"), Err("Package name is blacklisted"));
    }

    #[test]
    fn invalid_no_internal_modules() {
        assert_eq!(validate("sys"), Err("Package name is blacklisted"));
    }

    #[test]
    fn invalid_longer_than_214_chars() {
        assert_eq!(validate("ifyouwanttogetthesumoftwonumberswherethosetwonumbersarechosenbyfindingthelargestoftwooutofthreenumbersandsquaringthemwhichismultiplyingthembyitselfthenyoushouldinputthreenumbersintothisfunctionanditwilldothatforyouvortexpkg"), Err("Package name cannot be longer than 214 characters"));
    }

    #[test]
    fn invalid_mixed_case() {
        assert_eq!(
            validate("COOL-PACKAGE"),
            Err("Package name cannot contain capital letters")
        )
    }

    #[test]
    fn must_be_url_friendly() {
        assert_eq!(
            validate("n/itro/package/manager"),
            Err("Package name must be URL-friendly")
        )
    }
}
