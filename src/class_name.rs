pub struct ClassName {
    field: i32,
}

impl ClassName {
    pub fn new(value: i32) -> ClassName {
        ClassName {
            field: value
        }
    }

    pub fn public_method(&self) {
        println!("from public method, field is {}", self.field);
        self.private_method();
    }

    fn private_method(&self) {
        println!("form private method, field is {}", self.field);
    }
}