pub mod foo {
    pub fn bar() -> i32 {
        let x = 5;
        println!("Mod test");
        x
    }
}

#[cfg(test)]
mod our_tests {
    use super::*;


    #[test]
    fn test_bar() {
        assert_eq!(foo::bar(), 5);
    }

    #[test]
    fn test_bar_2() {
        assert_ne!(foo::bar(), 5);
    }
}