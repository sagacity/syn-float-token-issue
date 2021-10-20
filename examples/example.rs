use my_crate::my_macro;

#[my_macro]
struct MyStruct;

fn main() {
    let _ = MyStruct {};
}