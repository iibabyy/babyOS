use crate::{print, println};

pub fn run_tests() {
	super::test_main();
	loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {

    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}