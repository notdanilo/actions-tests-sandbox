#[cfg(test)]
mod tests {
    use cross_test::prelude::*;

    cross_test_configure!();

    #[cross_test]
    async fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
