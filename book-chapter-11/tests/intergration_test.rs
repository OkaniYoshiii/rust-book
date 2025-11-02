use book_chapter_11::adder::add;

#[test]
fn integration_add() {
    let result = add(10, 20);
    assert_eq!(result, 30);
}