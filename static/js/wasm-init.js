let wasmInited = false;

async function initWasm() {
    if (wasmInited === true) {
        return wasm_bindgen;
    }

    if (typeof wasm_bindgen === "undefined") {
        console.error("no wasm_bindgen loaded - build with 'bash ./build-wasm.sh'");
        return;
    }

    await wasm_bindgen("wasm/pkg/geonft_wasm_bg.wasm");

    wasm_bindgen.set_panic_hook();

    wasmInited = true;

    return wasm_bindgen;
}

initWasm().then((wasm) => {
    console.log("wasm initialized");
}).catch((e) => {
    console.log(e);
});
