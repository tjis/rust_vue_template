#![feature(proc_macro)]
#![recursion_limit="500"]

#[macro_use]
extern crate stdweb;
use stdweb::js_export;

#[js_export]
pub fn start() {
    console!(log, "hello from rust!");
}
