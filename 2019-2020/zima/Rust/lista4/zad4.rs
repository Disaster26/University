fn my_first_interpreter(code: &str) -> String {
    let mut s = String::new();
    let mut cell : u16 = 0;
    for c in code.chars() {
        match c {
            '.' => s.push((cell as u8) as char),
            '+' => cell = (cell + 1) % 256,
            _ => (),
        }
    }
    s
}

#[test]
fn test0() {
    assert_eq!(my_first_interpreter(""), "");
}

#[test]
fn test1() {
    assert_eq!(my_first_interpreter("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+."), "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
}

#[test]
fn test2() {
    assert_eq!(my_first_interpreter("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++.+++++++..+++.+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++++++++++++++++++++++.+++.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++."), "Hello, World!");
}

#[test]
fn test3() {
    assert_eq!(my_first_interpreter("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.+.+."), "ABCD");
}

#[test]
fn test4() {
    assert_eq!(my_first_interpreter("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.+.+.+.+."), "ABCDEF");
}

#[test]
fn test5() {
    assert_eq!(my_first_interpreter("+++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++.+.+.+.+.+."), "5@ABCDE");
}

#[test]
fn test6() {
    assert_eq!(my_first_interpreter("+++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++.+.+.+.+.+.+++++++++."), "5@ABCDEN");
}

#[test]
fn test7() {
    assert_eq!(my_first_interpreter("+++++TEST+++++TEST+++TEST"), "");
}

#[test]
fn test8() {
    assert_eq!(my_first_interpreter("+++++TEST+++++TEST+++TEST++++++++++++++++++++++++++++++."), "+");
}


#[test]
fn test9() {
    assert_eq!(my_first_interpreter("+++++++++++++++++++++++++++++++++++++++++++......"), "++++++");
}
