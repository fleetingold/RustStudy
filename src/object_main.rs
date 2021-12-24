mod class_name;
use class_name::ClassName;

fn object_main() {
    let object = ClassName::new(1024);
    object.public_method();
}