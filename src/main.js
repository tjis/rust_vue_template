import Vue from 'vue';
import App from './App.vue';
import Rust from './lib.rs';

Rust.then(r=>r.start(Vue, App));
