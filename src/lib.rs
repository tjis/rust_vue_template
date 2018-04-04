#![feature(proc_macro)]
#![recursion_limit="500"]

#[macro_use]
extern crate stdweb;
use stdweb::js_export;

#[js_export]
pub fn start(vue: stdweb::Value, app: stdweb::Value) {
    console!(log, "starting..");

    js! {
        let Vue = @{vue};
        let App = @{app};
        new Vue({
            el: "#app",
            render: h => h(App)
        });
    }

    console!(log, "started!");
}
