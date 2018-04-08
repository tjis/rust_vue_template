import Vue from 'vue';
import App from './App.vue';
import Rust from './lib.rs';

Rust.then(r=> {
    // Make the rust module available throughout the application
    Vue.prototype.$rust = r;

    // Create the root vue instance
    new Vue({
        el: "#app",
        render: h => h(App),
    })
});
