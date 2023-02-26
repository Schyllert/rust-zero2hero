/*crate can also be used as in conjunction with pub to signify that the item
it’s attached to is public only to other members of the same crate it’s in. */
use crate::give_number;
use crate::swedish;
use crate::load_resource;


#[test]
#[should_panic] // If the test should fail
pub fn lol() {
  assert!(1 == 1); // OK!
  panic!("Oh no!");
}

#[test]
pub fn test_equals() {
  assert_eq!(2, 1 + 1);
  assert_ne!(2, 3);

  assert_ne!(give_number(), 3);
}

#[test]
fn swedish_greetings_test() {
  swedish::swedish_greetings::greeting();
}

fn can_we_access_main_functions() {
  give_number();
}


