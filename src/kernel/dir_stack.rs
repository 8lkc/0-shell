use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref DIR_STACK: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

#[allow(non_camel_case_types)]
pub(super) struct DIRECTORY_STACK;

impl DIRECTORY_STACK {
    pub(super) fn init() {
        DIR_STACK.lock().unwrap().push("~".to_string());
    }

    // pub(super) fn pop() -> Option<String> {
    //     if DIR_STACK.lock().unwrap().len() <= 1 { return None; }
    //     DIR_STACK.lock().unwrap().pop()
    // }

    // pub(super) fn push(dir: &str) {
    //     DIR_STACK.lock().unwrap().push(dir.to_string());
    // }

    pub(super) fn to_string() -> String {
        DIR_STACK.lock().unwrap().join("/")
    }
}
