#[cfg(test)]
mod tests {
    #[test]
    fn fails_with_default_features() {
        #[cfg(feature = "red")]
        assert!(false)
    }
}
