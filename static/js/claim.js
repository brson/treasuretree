import {
    initSecretScanner,
    treasureClaimUrl,
    secretKey,
    publicKey
} from "./secret-scan.js";

console.assert(initWasm);

initSecretScanner({
    onBeginSecretScan: onBeginSecretScan,
    onEndSecretScan: onEndSecretScan
});

let treasureClaimed = false;

let claimButton = document.getElementById("claim-button");

console.assert(claimButton);

claimButton.addEventListener("click", async () => {

    let claimSpinner = document.getElementById("claim-spinner");

    console.assert(claimSpinner);

    claimButton.disabled = true;

    console.assert(secretKey);
    console.assert(publicKey);

    claimSpinner.classList.remove("no-display");

    try {
        let wasm = await initWasm();

        let nonce = createNonce();
        let signature = wasm.sign_with_secret_key(secretKey, nonce);

        if (signature == null) {
            // TODO
        }

        let requestInfo = {
            nonce: nonce,
            public_key: publicKey,
            signature: signature
        };

        let response = await fetch("api/claim", {
            method: "POST",
            headers: {
                "Accept": "application/json",
                "Content-Type": "application/json"
            },
            body: JSON.stringify(requestInfo)
        });
        console.log(response);

        if (!response.ok) {
            // TODO
        }

        let jsonResponse = await response.json();
        console.log(jsonResponse);

        treasureClaimed = true;

        let claimedMessageElt = document.getElementById("claimed-message");
        console.assert(claimedMessageElt);
        claimedMessageElt.classList.remove("no-display");
    } finally {
        maybeEnableClaimButton();
        claimSpinner.classList.add("no-display");
    }
});

function createNonce() {
    // TODO
    return "hello world";
}

function maybeEnableClaimButton() {
    let dataReady =
        treasureClaimUrl &&
        secretKey &&
        publicKey;

    if (dataReady && !treasureClaimed) {
        claimButton.disabled = false;
    }
}


/* These handlers are required by secren-scan.js */

function onBeginSecretScan() {
    claimButton.disabled = true;
}

function onEndSecretScan() {
    maybeEnableClaimButton();
}
