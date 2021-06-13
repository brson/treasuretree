import { initWasm } from "./wasm-init.js";
import {
    initAccount,
    accountSecretKey,
    accountPublicKey
} from "./account.js";
import {
    initSecretScanner,
    treasureClaimUrl,
    treasureSecretKey,
    treasurePublicKey
} from "./secret-scan.js";

initAccount({
    onAccountSecretKeyChanged: onAccountSecretKeyChanged
});

let treasureClaimed = false;

let claimButton = document.getElementById("claim-button");

console.assert(claimButton);

claimButton.addEventListener("click", async () => {

    let claimSpinner = document.getElementById("claim-spinner");

    console.assert(claimSpinner);

    claimButton.disabled = true;

    console.assert(treasureSecretKey);
    console.assert(treasurePublicKey);

    claimSpinner.classList.remove("no-display");

    try {
        let wasm = await initWasm();
        
        let treasureSignature = wasm.sign_claim_with_treasure_secret_key(treasureSecretKey, accountPublicKey);
        let accountSignature = wasm.sign_claim_with_account_secret_key(accountSecretKey, treasurePublicKey);

        if (treasureSignature == null) {
            // TODO
            console.log("treasure signature == null");
        }

        if (accountSignature == null) {
            // TODO
            console.log("account signature == null");
        }

        let requestInfo = {
            account_public_key: accountPublicKey,
            treasure_public_key: treasurePublicKey,
            account_signature: accountSignature,
            treasure_signature: treasureSignature
        };

        let response = await fetch("api/claim", {
            method: "POST",
            headers: {
                "Accept": "application/json",
                "Content-Type": "application/json"
            },
            body: JSON.stringify(requestInfo)
        });
        console.log("log geonft response:");
        console.log(response);

        if (!response.ok) {
            // TODO
            console.log("something wrong with response");
        }

        let jsonResponse = await response.json();
        console.log(jsonResponse);

        treasureClaimed = true;

        let claimedMessageElt = document.getElementById("claimed-message");
        console.log(claimedMessageElt);
        console.assert(claimedMessageElt);
        claimedMessageElt.classList.remove("no-display");
    } finally {
        maybeEnableClaimButton();
        claimSpinner.classList.add("no-display");
    }
});

function maybeEnableClaimButton() {
    let dataReady =
        treasureClaimUrl &&
        treasureSecretKey &&
        treasurePublicKey &&
        accountSecretKey;

    if (dataReady && !treasureClaimed) {
        claimButton.disabled = false;
    }
}

function onAccountSecretKeyChanged() {
    maybeEnableClaimButton();
}

function onBeginSecretScan() {
    claimButton.disabled = true;
}

function onEndSecretScan() {
    maybeEnableClaimButton();
}

async function treasureExists(){
    if (!treasurePublicKey) {
        return;
    }

    let requestInfo = {
        treasure_public_key: treasurePublicKey
    };

    let response = await fetch("api/exists", {
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
        console.log("something wrong with search response");
    }

    let jsonResponse = await response.json();

    if (jsonResponse.treasure_exists == false) {
        if (window.confirm("Treasure doesn't exist, do you want to plant one?")) {
            window.open("plant?key="+treasureSecretKey, "_self");
        } 
    }
}



window.addEventListener('DOMContentLoaded', async () => {
    await initSecretScanner({
        onBeginSecretScan: onBeginSecretScan,
        onEndSecretScan: onEndSecretScan
    });
    await treasureExists();

    console.log("initSecretScanner");
});
