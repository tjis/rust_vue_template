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
