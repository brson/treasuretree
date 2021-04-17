async function initWasm() {
    if (wasm_bindgen === undefined) {
        console.log("no wasm_bindgen loaded - build with 'bash ./build-wasm.sh'");
        return;
    }

    await wasm_bindgen("wasm/pkg/geonft_wasm_bg.wasm");

    wasm_bindgen.set_panic_hook();

    return wasm_bindgen;
}

initWasm().then((wasm) => {
    console.log("wasm inited");
}).catch((e) => {
    console.log(e);
});
