mod sample_module;
use sample_module::SampleImpl;

fn main() {
    let myObject = SampleImpl::new("Char".to_string());
    myObject.hello_world();
}
