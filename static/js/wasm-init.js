export { initWasm };

import init, {
    set_panic_hook,
    new_account_secret_key,
    sanity_check_url,
    secret_url_to_secret_key,
    secret_url_to_public_key,
    secret_key_to_public_key,
    secret_key_to_secret_url,
    sign_with_secret_key
} from "../wasm/pkg/geonft_wasm.js";

let wasm = null;

async function initWasm() {
    if (wasm != null) {
        return wasm;
    }

    await init();

    wasm = {
        set_panic_hook,
        new_account_secret_key,
        sanity_check_url,
        secret_url_to_secret_key,
        secret_url_to_public_key,
        secret_key_to_public_key,
        secret_key_to_secret_url,
        sign_with_secret_key
    };

    wasm.set_panic_hook();

    return wasm;
}

initWasm().then((wasm) => {
    console.log("wasm initialized");
}).catch((e) => {
    console.log(e);
});
