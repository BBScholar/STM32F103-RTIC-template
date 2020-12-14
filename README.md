## THIS DOESNT WORK YET

### Overview
This will generate an rtic project for an stm32f103c8 microcontroller. Adjustments to the desired toolchain or microcontroller will require changes to the configuration.

### Requirements:

Download the correct target via `rustup  ` (follow this guide if you dont have rustup: https://doc.rust-lang.org/book/ch01-01-installation.html )
Do this by running `rustup target add https://doc.rust-lang.org/book/ch01-01-installation.html` 

`Cargo` will handle the rest (you might have to install the rust `nightly` toolchain. Just look up how to do that)

### How to generate:
Run the following command to generate a project:
`cargo generate --git https://github.com/BBScholar/STM32F103-RTIC-template.git --name my-project`

(You'll need to install [cargo-generate][https://github.com/ashleygwilliams/cargo-generate] first)

### How to deploy
Run `cargo embed --release` to deploy the binary to the microcontroller
