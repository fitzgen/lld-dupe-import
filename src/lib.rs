#[link(name = "dupeimport")]
extern "C" {
    fn bar(x: u32);
}

#[export_name = "foo"]
pub fn foo(x: u32) {
    println!("in Rust, x = {}", x);
    unsafe {
        bar(x);
    }
}
