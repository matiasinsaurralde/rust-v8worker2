use binding;

pub struct Handler {
}

pub fn new() -> Handler {
    Handler{}
}


impl Handler {
    pub fn init(&mut self) {
        unsafe {
            binding::v8_init();
        };
    }
}