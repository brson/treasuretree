/*

Companion script to secret-scan.html

This file expects two global non-async functions to be defined elsewhere:

- onBeginSecretScan
- onEndSecretScan

*/

export {
    initSecretScanner,
    treasureClaimUrl,
    treasureSecretKey,
    treasurePublicKey
};

import { initWasm } from "./wasm-init.js";
import QrScanner from "./lib/qr-scanner.min.js";

/* These three globals are for access outside this file */

let treasureClaimUrl = null;
let treasureSecretKey = null;
let treasurePublicKey = null;

let onBeginSecretScan = null;
let onEndSecretScan = null;


async function initSecretScanner(callbacks) {
    console.assert(!onBeginSecretScan);
    console.assert(!onEndSecretScan);
    console.assert(callbacks.onBeginSecretScan);
    console.assert(callbacks.onEndSecretScan);

    onBeginSecretScan = callbacks.onBeginSecretScan;
    onEndSecretScan = callbacks.onEndSecretScan;

    await loadFromUrl();
}

/* The rest of the globals are implementation details */

let qrScanButton = document.getElementById("qrscan-button");
let qrCancelButton = document.getElementById("qrscan-cancel-button");
let treasureClaimUrlElt = document.getElementById("treasure-claim-url");
let secretKeyInput = document.getElementById("treasure-secret-key");
let publicKeyElt = document.getElementById("treasure-public-key");

console.assert(qrScanButton);
console.assert(qrCancelButton);
console.assert(secretKeyInput);
console.assert(treasureClaimUrlElt);
console.assert(publicKeyElt);

console.assert(initWasm);

QrScanner.WORKER_PATH = "js/lib/qr-scanner-worker.min.js";

let stopScanning = null;

qrScanButton.addEventListener("click", async () => {

    let video = document.getElementById("qr-video");

    console.assert(video);    

    console.assert(onBeginSecretScan);
    onBeginSecretScan();

    treasureClaimUrlElt.innerText = null;
    secretKeyInput.value = null;
    publicKeyElt.innerText = null;

    treasureClaimUrl = null;
    treasureSecretKey = null;
    treasurePublicKey = null;

    let wasm = await initWasm();

    const qrScanner = new QrScanner(video, (result) => {
        console.log(result);

        // Don't do async work after this to avoid races updated the UI
        stopScanning();

        let url = result;
        let sanityCheck = wasm.sanity_check_treasure_secret_url(url);

        if (sanityCheck === false) {
            console.error("QR code looks bogus");
            // TODO
            return;
        }

        let secretKey_ = wasm.treasure_secret_url_to_secret_key(url);
        let publicKey_ = wasm.treasure_secret_url_to_public_key(url);

        if (secretKey_ == null || publicKey_ == null) {
            console.error("unable to decode key from URL");
            // TODO
            return;
        }

        treasureClaimUrlElt.innerText = url;
        secretKeyInput.value = secretKey_;
        publicKeyElt.innerText = publicKey_;

        treasureClaimUrl = url;
        treasureSecretKey = secretKey_;
        treasurePublicKey = publicKey_;
        
    }, (error) => {
        console.error(error);
    });

    
    qrScanButton.disabled = true;
    qrCancelButton.disabled = false;
    secretKeyInput.disabled = true;
    video.classList.remove("no-display");

    stopScanning = () => {
        qrScanner.stop();
        qrScanner.destroy();
        qrScanButton.disabled = false;
        qrCancelButton.disabled = true;
        secretKeyInput.disabled = false;
        video.classList.add("no-display");
        console.assert(onEndSecretScan);
        onEndSecretScan();
    }

    qrScanner.start();
});

qrCancelButton.addEventListener("click", async () => {
    stopScanning();
});

secretKeyInput.addEventListener("input", async () => {

    let wasm = await initWasm();

    console.assert(onBeginSecretScan);
    onBeginSecretScan();

    treasureClaimUrlElt.innerText = null;
    publicKeyElt.innerText = null;

    treasureClaimUrl = null;
    treasureSecretKey = null;
    treasurePublicKey = null;

    let secretKey_ = secretKeyInput.value;
    let publicKey_ = wasm.treasure_secret_key_to_public_key(secretKey_);
    let treasureClaimUrl_ = wasm.treasure_secret_key_to_secret_url(secretKey_);

    if (publicKey_ == null || treasureClaimUrl_ == null) {
        console.error("unable to decode key");
        // TODO
        return;
    }

    publicKeyElt.innerText = publicKey_;
    treasureClaimUrlElt.innerText = treasureClaimUrl_;

    treasureClaimUrl = treasureClaimUrl_;
    treasureSecretKey = secretKey_;
    treasurePublicKey = publicKey_;

    console.assert(onEndSecretScan);
    onEndSecretScan();
});


async function loadFromUrl() {
    let params = new URLSearchParams(document.location.search.substring(1));
    let key = params.get("key");
    if (key != null) {

        onBeginSecretScan();
        
        let url = document.location.href;
        console.log(url);

        let wasm = await initWasm();

        let secretKey_ = wasm.treasure_secret_url_to_secret_key(url);
        let publicKey_ = wasm.treasure_secret_url_to_public_key(url);

        if (secretKey_ == null || publicKey_ == null) {
            console.error("unable to decode key from URL");
        }

        secretKeyInput.value = secretKey_;
        publicKeyElt.innerText = publicKey_;
        treasureClaimUrlElt.innerText = url;

        treasureSecretKey = secretKey_;
        treasurePublicKey = publicKey_;
        treasureClaimUrl = url;

        onEndSecretScan();    
    }    
}
