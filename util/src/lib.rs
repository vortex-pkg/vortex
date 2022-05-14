pub fn unroll_version(version: &str) -> String {
    match version.parse::<i32>() {
        // Major
        Ok(n) => return format!("{}.0.0", n),
        _ => {

            // Only major + minor
            match version.parse::<f32>() {
                Ok(n) => return format!("{}.0", n),
                _ => return String::from(version)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn major_only() {
        assert_eq!(unroll_version("1"), String::from("1.0.0"));
    }

    #[test]
    fn major_and_minor_only() {
        assert_eq!(unroll_version("6.9"), String::from("6.9.0"));
    }

    #[test]
    fn complete() {
        assert_eq!(unroll_version("4.2.9"), String::from("4.2.9"));
    }
}