// Интеграционные тесты (Integration testing)
// Cargo looks for integration tests in tests directory next to src.
#[test]
fn integration_test_add() {
    assert_eq!(step_1_1::my_doc_test_add(2, 3), 5) ;
}