#![feature(proc_macro)]
#![recursion_limit="500"]

#[macro_use]
extern crate stdweb;

mod vue;
use stdweb::js_export;
use vue::VueModel;

#[js_export]
pub fn register(model: VueModel) {
    model.set("msg", "Hello from rust!");
}

#[js_export]
pub fn on_press(model: VueModel, event: stdweb::Value) {
    let mut new_msg : String = model.get("msg");
    new_msg.push('!');
    model.set("msg", &new_msg);
}
