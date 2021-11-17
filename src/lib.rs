pub mod bodyfile3;
pub use bodyfile3::*;

#[cfg(test)]
mod tests {
    use super::Bodyfile3Line;

    #[test]
    fn sample1() {
        let bf = Bodyfile3Line::new();
        assert_eq!(bf.get_md5(), "0");
    }
}
