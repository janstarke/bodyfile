pub mod bodyfile3;
pub use bodyfile3::*;

#[cfg(test)]
mod tests {
    use super::Bodyfile3;

    #[test]
    fn sample1() {
        let bf = Bodyfile3::new();
        assert_eq!(bf.get_md5(), "0");
    }
}
