#[cfg(test)]
mod tests {
    #[test]
    fn fails_with_default_features() {
        #[cfg(feature = "blue")]
        dbg!("blue feature is enabled");
        #[cfg(feature = "red")]
        assert!(false)
    }
}
