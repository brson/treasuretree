export {
    accountSecretKey,
    initAccount,
};
import { initWasm } from "./wasm-init.js";

let accountSecretKey = null;
let onAccountSecretKeyChanged = null;

function initAccount(callbacks) {
    console.assert(!onAccountSecretKeyChanged);
    console.assert(callbacks.onAccountSecretKeyChanged);

    onAccountSecretKeyChanged = callbacks.onAccountSecretKeyChanged;
}

let secretKeyInput = document.getElementById("account-secret-key");
let newAccountButton = document.getElementById("new-account-button");

console.assert(secretKeyInput);
console.assert(newAccountButton);

newAccountButton.addEventListener("click", async () => {

    secretKeyInput.disabled = true;
    newAccountButton.disabled = true;

    try {
        let wasm = await initWasm();

        let accountSecretKey_ = wasm.new_account_secret_key();
        secretKeyInput.value = accountSecretKey_;
        accountSecretKey = accountSecretKey_;

        console.assert(onAccountSecretKeyChanged);
        onAccountSecretKeyChanged();
    } finally {
        secretKeyInput.disabled = false;
        newAccountButton.disabled = false;
    }
});

secretKeyInput.addEventListener("input", async () => {
    accountSecretKey = null;
    console.assert(onAccountSecretKeyChanged);
    onAccountSecretKeyChanged();

    let wasm = await initWasm();

    let secretKeyInputValue = secretKeyInput.value;
    let publicKey = wasm.account_secret_key_to_public_key(secretKeyInputValue);

    if (publicKey != null) {
        console.log("secret key decoded");
        accountSecretKey = secretKeyInputValue;

        console.assert(onAccountSecretKeyChanged);
        onAccountSecretKeyChanged();
    } else {
        // todo: user feedback
        console.log("couldn't decode account secret key");
    }
});
