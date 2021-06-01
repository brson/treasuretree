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
let secretKeyInput = document.getElementById("treasure-secret-key");

console.assert(qrScanButton);
console.assert(qrCancelButton);
console.assert(secretKeyInput);

let treasureLinkPElt = document.getElementById("scan-treasure-link-p");
let treasureLinkAElt = document.getElementById("scan-treasure-link-a");
let treasureLinkSpanElt = document.getElementById("scan-treasure-link-span");

console.assert(treasureLinkPElt);
console.assert(treasureLinkAElt);
console.assert(treasureLinkSpanElt);

console.assert(initWasm);

QrScanner.WORKER_PATH = "js/lib/qr-scanner-worker.min.js";

let stopScanning = null;

qrScanButton.addEventListener("click", async () => {

    let video = document.getElementById("qr-video");

    console.assert(video);    

    console.assert(onBeginSecretScan);
    onBeginSecretScan();

    secretKeyInput.value = null;

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

        secretKeyInput.value = secretKey_;

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

        doEndSecretScan(wasm);
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

    treasureClaimUrl = null;
    treasureSecretKey = null;
    treasurePublicKey = null;

    let secretKey_ = secretKeyInput.value;
    let publicKey_ = wasm.treasure_secret_key_to_public_key(secretKey_);
    let treasureClaimUrl_ = wasm.treasure_secret_key_to_secret_claim_url(secretKey_);

    if (publicKey_ == null || treasureClaimUrl_ == null) {
        console.error("unable to decode key");
        // TODO
        return;
    }

    treasureClaimUrl = treasureClaimUrl_;
    treasureSecretKey = secretKey_;
    treasurePublicKey = publicKey_;

    doEndSecretScan(wasm);
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

        treasureSecretKey = secretKey_;
        treasurePublicKey = publicKey_;
        treasureClaimUrl = url;

        doEndSecretScan(wasm);
    }    
}

function doEndSecretScan(wasm) {
    if (treasurePublicKey) {
        let treasureLinkUrl = wasm.treasure_public_key_to_treasure_url(treasurePublicKey);
        let treasureAbbrev = wasm.treasure_public_key_to_abbrev(treasurePublicKey);

        treasureLinkAElt.href = treasureLinkUrl;
        treasureLinkSpanElt.innerText = treasureAbbrev;
        treasureLinkPElt.classList.remove("no-display");
    } else {
        treasureLinkPElt.classList.add("no-display");
    }

    console.assert(onEndSecretScan);
    onEndSecretScan();
}
