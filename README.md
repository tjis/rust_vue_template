# Rust Vue template
This is a starter template to quickly get up and running with vue in rust.

This template mirrors the official webpack-simple vue template, but does
the component initialization from rust.

## Prerequisites
If you wish to use this template, make sure you have the following installed:
- rust (nightly)
- cargo
- yarn (or npm)

Furthermore, you'll need cargo-web, a cargo subcommand that handles the building to WebAssembly (among other things).

    cargo install cargo-web

## Building
run

    yarn install
    
or

    npm install
    
to install the js dependencies listed in package.json.

Then you can do either

    yarn run build
    
to run a production build, or

    yarn run dev
    
(or npm equivalent) to start a developer build and launch an instance of your application that automatically refreshes when it detects changes.

Incidentally, the first time you do this will be slow, as cargo takes some time to build all dependencies. Unfortunately, yarn does not inform us that this is happening, so be patient.

If you suspect something is wrong, you can run

    cargo web build
    
to launch a build just for the rust WebAssembly part. It'll actually display sensible errors.

## Explanation
I'll briefly go over all the relevant bits here. As I'm rather new to the frontend, there's a good chance some of this is wrong. Corrections are welcome.

### Webpack
[Webpack](https://webpack.js.org/) is our build system. It recursively finds all dependencies starting from a configured entry point (src/main.js in our case), then loads them using whatever mechanism configured. It is very flexible, but the flexibility comes with some extra complexity.

One of the loaders configured (in [webpack.config.js](webpack.config.js)) is [rust-native-wasm-loader](https://github.com/dflemstr/rust-native-wasm-loader), which will compile the crate to wasm. Note that the configured option `cargoWeb` is set to true, which will make sure the build is run through cargo web.

Other loaders are related to transpiling modern javascript dialects to plain old browser js, and to vue.

### yarn
[yarn](https://yarnpkg.com/en/) parses the [package.json](package.json) file, and downloads and installs the packages listed there, as well as their dependencies. This allows them to be found by Webpack, which in turn allows javascript files to import them. This is how we get Vue.js, among other things.

package.json also defines two 'scripts', `dev` and `build`. these define what happens when you do `yarn run dev` or `yarn run build` (or `npm run dev` or `npm run build`). In this case, starting a development server, or running a production build.

### Vue.js
[Vue.js](https://vuejs.org/) is a framework for building user interfaces. Vue allows for straightforward conversion of a javascript object (the viewmodel) to DOM elements using a declarative HTML templating approach.

Since we can produce javascript objects from our WebAssembly, it should be possible to use this mechanism with rust-provided viewmodels.

### WebAssembly
[WebAssembly](http://webassembly.org/) is an assembly format which modern compilers can target, and which modern browsers can transform into native instructions. Rust supports WebAssembly natively.

WebAssembly is still in a pretty early stage of development though, and what browsers currently ship is a minimal viable product (which does appear to be quite viable!). One of the consequences is that while we can cheaply call from javascript into a rust function, we can't cheaply do the other way around.

### cargo-web and stdweb
[cargo-web](https://github.com/koute/cargo-web) and [stdweb](https://github.com/koute/stdweb) are two rust projects that make it a lot easier to work with WebAssembly in rust. cargo-web is a cargo plugin that handles the build. While rust can build to WebAssembly without any help, cargo-web works around some of the limitations of the current implementation.

Specifically, cargo-web provides the plumbing required to call from rust into javascript. This is used by the stdweb library to provide a `js!` macro, which allows us to embed javascript directly in our rust code. I am not quite sure what voodoo was required to make this work, but it is pretty neat!

cargo-web is configured through the [Web.toml](Web.toml) file. The only thing configured there in this template is that we always wish to build with rust's native wasm backend (rather than using emscripten or asm.js).

### putting it all together
[main.js](src/main.js) imports vue, the application template and the rust webassembly. Since main.js is the webpack entry, webpack will ensure these dependencies are loaded into the final bundle and available at runtime. This also triggers the cargo web build of the webassembly, since lib.rs is imported.

[lib.rs](src/lib.rs) defines a function `start`, which accepts the vue module, as well as the application template as parameters. Because it is annotated with `#[js_export]`, cargo web will make sure this function is callable from javascript.

Next in main.js, the rust webassembly is compiled to native instructions, and when this is done, the `start` function is called with the imported modules.

The lib.rs `start` function is quite simple. It just logs a few messages and constructs the Vue instance in the same way one would do this in javascript.

With this setup, it should be possible to construct the entire application in rust, optionally making use of js dependencies which are injected through main.js. The setup itself isn't really vue specific; any framework that works with webpack can be substituted.

### misc. stuff
in [Cargo.toml](Cargo.toml), the crate is configured as a library crate. Also, the crate-type "cdylib" is configured. This is required for rust to actually produce a wasm file. A normal library type won't work.
