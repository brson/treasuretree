export {
    accountSecretKey
};
import { initWasm } from "./wasm-init.js";

let accountSecretKey = null;

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
    } finally {
        secretKeyInput.disabled = false;
        newAccountButton.disabled = false;
    }
});
