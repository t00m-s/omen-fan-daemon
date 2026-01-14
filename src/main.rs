mod asserts;
use asserts::assert_modprobe;

fn main() {
    assert_modprobe();
    println!("hello, world!");
}
