#![crate_type="lib"]
pub fn example(x: Option<usize>) {
    match x {
        Some(0 | 1 | 2) => {}
        //~^ ERROR: or_patterns syntax is experimental
        _ => {}
    }
}
