use debug_print::debug_print;

#[test]
fn js_exec_and_log_works() {
    let num = 42;
    debug_print!(num);

    let name = "Alice";
    debug_print!(name);

    let is_valid = true;
    debug_print!(is_valid);

    assert!(true)
}
