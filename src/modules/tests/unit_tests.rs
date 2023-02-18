
#[test]
#[should_panic] // If the test should fail
pub fn lol() {
    assert!(1 == 1); // OK!
    panic!("Oh no!");
}

#[test]
pub fn test_equals() {
    assert_eq!(2,1+1);
    assert_ne!(2,3);
}