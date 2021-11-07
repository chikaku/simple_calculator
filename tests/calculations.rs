use simple_calculator_cmd::{Error, VM};

#[test]
fn test_calculation() -> Result<(), Error> {
    let mut vm = VM::new(true);

    assert_eq!(vm.exec_line("1+1")?, 2.0);
    assert_eq!(vm.exec_line("-1+1")?, 0.0);
    assert_eq!(vm.exec_line("9/3/3")?, 1.0);
    assert_eq!(vm.exec_line("2^2^2/5")?, 3.2);
    assert_eq!(vm.exec_line("(-1)+(-2)*(-3)")?, 5.0);

    Ok(())
}
