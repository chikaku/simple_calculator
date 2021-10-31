use simple_calculator::VM;

#[test]
fn test_parser() {
    let mut vm = VM::new(true);

    assert!(vm.exec_line("-1+2").is_ok());
}
