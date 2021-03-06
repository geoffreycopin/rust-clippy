// run-rustfix

#[allow(clippy::string_add, unused)]
#[warn(clippy::string_add_assign)]
fn main() {
    // ignores assignment distinction
    let mut x = "".to_owned();

    for _ in 1..3 {
        x = x + ".";
    }

    let y = "".to_owned();
    let z = y + "...";

    assert_eq!(&x, &z);

    let mut x = 1;
    x = x + 1;
    assert_eq!(2, x);
}
