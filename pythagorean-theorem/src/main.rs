mod pytheorem;
fn main() {
    println!("hello");
    let result = pytheorem::py::threed(1.0, 2.0, 3.0);
    println!("{}", result);
}
