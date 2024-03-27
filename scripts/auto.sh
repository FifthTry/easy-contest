function build-wasm() {
    pushd "ec" || return 1
    cargo build --target wasm32-unknown-unknown --release || return 1
    cp ./target/wasm32-unknown-unknown/release/ec.wasm ../ || return 1

    popd || return 1
}

function upload-frontend-debug() {
    DEBUG_USE_TEJAR_FOLDER="/home/sidd/projects/work/ft/tejar-cache/" \
      FIFTHTRY_SITE_WRITE_TOKEN="fifthtry-write-token" \
      DEBUG_API_FIFTHTRY_COM="http://127.0.0.1" \
      clift upload localhost

    DEBUG_USE_TEJAR_FOLDER="/home/sidd/projects/work/ft/tejar-cache/" \
    FIFTHTRY_SITE_WRITE_TOKEN="fifthtry-write-token" \
    DEBUG_API_FIFTHTRY_COM="http://127.0.0.1" \
    clift upload localhost --file ec.wasm
}
