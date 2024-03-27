function build-wasm() {
    pushd "ec" || return 1
    cargo build --target wasm32-unknown-unknown --release || return 1
    cp ./target/wasm32-unknown-unknown/release/ec.wasm ../ || return 1

    popd || return 1
}

