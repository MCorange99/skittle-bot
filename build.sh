#!/bin/bash


# core
cargo build --release

# core-module
pushd $(pwd)/modules/skittle-bot-core-module
cargo build --release
cp ./target/release/libskittle_bot_core_module.so ../libskittle_bot_core_module.so
popd