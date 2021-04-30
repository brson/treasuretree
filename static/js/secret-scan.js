/*

Companion script to secret-scan.html

This file expects two global non-async functions to be defined elsewhere:

- onBeginSecretScan
- onEndSecretScan

*/


/* These three globals are for access outside this file */

let treasureClaimUrl = null;
let secretKey = null;
let publicKey = null;

let onBeginSecretScan = null;
let onEndSecretScan = null;

function initSecretScanner(callbacks) {
    console.assert(callbacks.onBeginSecretScan);
    console.assert(callbacks.onEndSecretScan);

    onBeginSecretScan = callbacks.onBeginSecretScan;
    onEndSecretScan = callbacks.onEndSecretScan;
}

/* The rest of the globals are implementation details */

let qrScanButton = document.getElementById("qrscan-button");
let qrCancelButton = document.getElementById("qrscan-cancel-button");
let secretKeyInput = document.getElementById("secret-key");
let treasureClaimUrlElt = document.getElementById("treasure-claim-url");
let publicKeyElt = document.getElementById("public-key");

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
    secretKey = null;
    publicKey = null;

    let wasm = await initWasm();

    const qrScanner = new QrScanner(video, (result) => {
        console.log(result);

        // Don't do async work after this to avoid races updated the UI
        stopScanning();

        let url = result;
        let sanityCheck = wasm.sanity_check_url(url);

        if (sanityCheck === false) {
            console.error("QR code looks bogus");
            // TODO
            return;
        }

        let secretKey_ = wasm.secret_url_to_secret_key(url);
        let publicKey_ = wasm.secret_url_to_public_key(url);

        if (secretKey_ == null || publicKey_ == null) {
            console.error("unable to decode key from URL");
            // TODO
            return;
        }

        treasureClaimUrlElt.innerText = url;
        secretKeyInput.value = secretKey_;
        publicKeyElt.innerText = publicKey_;

        treasureClaimUrl = url;
        secretKey = secretKey_;
        publicKey = publicKey_;
        
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
    secretKey = null;
    publicKey = null;

    let secretKey_ = secretKeyInput.value;
    let publicKey_ = wasm.secret_key_to_public_key(secretKey_);
    let treasureClaimUrl_ = wasm.secret_key_to_secret_url(secretKey_);

    if (publicKey_ == null || treasureClaimUrl_ == null) {
        console.error("unable to decode key");
        // TODO
        return;
    }

    publicKeyElt.innerText = publicKey_;
    treasureClaimUrlElt.innerText = treasureClaimUrl_;

    treasureClaimUrl = treasureClaimUrl_;
    secretKey = secretKey_;
    publicKey = publicKey_;

    console.assert(onEndSecretScan);
    onEndSecretScan();
});
