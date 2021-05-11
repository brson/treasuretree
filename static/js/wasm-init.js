export { initWasm };

import init, {
    set_panic_hook,
    account_secret_key_to_public_key,
    new_account_secret_key,
    sanity_check_treasure_secret_url,
    treasure_secret_url_to_secret_key,
    treasure_secret_url_to_public_key,
    treasure_secret_key_to_public_key,
    treasure_secret_key_to_secret_url,
    sign_with_secret_key,
    get_hash,
} from "../wasm/pkg/geonft_wasm.js";

let wasm = null;

async function initWasm() {
    if (wasm != null) {
        return wasm;
    }

    await init();

    wasm = {
        set_panic_hook,
        account_secret_key_to_public_key,
        new_account_secret_key,
        sanity_check_treasure_secret_url,
        treasure_secret_url_to_secret_key,
        treasure_secret_url_to_public_key,
        treasure_secret_key_to_public_key,
        treasure_secret_key_to_secret_url,
        sign_with_secret_key,
        get_hash,
    };

    wasm.set_panic_hook();

    return wasm;
}

initWasm().then((wasm) => {
    console.log("wasm initialized");
}).catch((e) => {
    console.log(e);
});
