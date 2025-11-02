pub mod adder {
    pub fn add(left: u64, right: u64) -> u64 {
        left + right
    }
}

#[cfg(test)]
mod tests {
    use super::adder::*;
    #[test]
    fn is_add_correct() {
        let result = add(10, 20);
        assert_eq!(result, 30, "Add expected to return 30 but returned {result}");
    }
}