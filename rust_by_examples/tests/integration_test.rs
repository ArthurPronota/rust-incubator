// Интеграционные тесты (Integration testing)
// Cargo looks for integration tests in tests directory next to src.
#[test]
fn integration_test_add() {
    assert_eq!(rust_by_examples::my_doc_test_add(2, 3), 5) ;
}