const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn version() -> String {
    VERSION.to_string()
}


#[cfg(test)]
mod tests {
    use crate::version;

    #[test]
    fn version_should_return_current_version() {
        assert_eq!(env!("CARGO_PKG_VERSION").to_string(), version());
    }
}
