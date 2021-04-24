let treasureImageEncoded = null;
let treasureClaimUrl = null;
let secretKey = null;
let publicKey = null;
let treasurePlanted = false;

let plantButton = document.getElementById("plant-button");

console.assert(plantButton);


let imageUploadButton = document.getElementById("image-upload-button");
let useTestImageButton = document.getElementById("use-test-image-button")
let imageElt = document.getElementById("treasure-image");

console.assert(imageUploadButton);
console.assert(useTestImageButton);
console.assert(imageElt);

imageUploadButton.addEventListener("change", async () => {

    plantButton.disabled = true;

    treasureImageEncoded = null;
    imageElt.src = "";
    imageElt.classList.add("no-display");

    imageUploadButton.disabled = true;
    useTestImageButton.disabled = true;

    try {

        if (imageUploadButton.files.length == 0) {
            return;
        }

        let file = imageUploadButton.files[0];
        let bin = await file.arrayBuffer();
        let blob = new Blob([bin], { type: file.type });

        imageElt.src = URL.createObjectURL(blob);
        imageElt.classList.remove("no-display");

        treasureImageEncoded = btoa(blob);

        maybeEnablePlantButton();
    } finally {
        imageUploadButton.disabled = false;
        useTestImageButton.disabled = false;
    }
});

useTestImageButton.addEventListener("click", async () => {

    plantButton.disabled = true;

    treasureImageEncoded = null;
    imageElt.scr = "";
    imageElt.classList.add("no-display");

    imageUploadButton.disabled = true;
    useTestImageButton.disabled = true;

    try {
        let response = await fetch("images/coconut-tree.png");

        if (!response.ok) {
            // TODO
        }

        let blob = await response.blob();

        imageElt.src = URL.createObjectURL(blob);
        imageElt.classList.remove("no-display");

        treasureImageEncoded = btoa(blob);

        maybeEnablePlantButton();
    } finally {
        imageUploadButton.disabled = false;
        useTestImageButton.disabled = false;
    }
});




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

QrScanner.WORKER_PATH = "js/lib/qr-scanner-worker.min.js";

let stopScanning = null;

qrScanButton.addEventListener("click", async () => {

    let video = document.getElementById("qr-video");

    console.assert(video);    

    plantButton.disabled = true;

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
        maybeEnablePlantButton();
    }

    qrScanner.start();
});

qrCancelButton.addEventListener("click", async () => {
    stopScanning();
});

secretKeyInput.addEventListener("input", async () => {

    let wasm = await initWasm();

    plantButton.disabled = true;

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

    maybeEnablePlantButton();
});


plantButton.addEventListener("click", async () => {

    plantButton.disabled = true;

    console.assert(treasureImageEncoded);
    console.assert(secretKey);

    try {
        let treasureInfo = {
            image: treasureImageEncoded,
            private_key: secretKey
        };

        let response = await fetch("api/plant", {
            method: "POST",
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(treasureInfo)
        });
        console.log(response);

        if (!response.ok) {
            // TODO
        }

        // let jsonResponse = await response.json();
        let jsonResponse = await response.json();
        console.log(jsonResponse);

        treasurePlanted = true;

        let plantedMessageElt = document.getElementById("planted-message");
        console.assert(plantedMessageElt);
        plantedMessageElt.classList.remove("no-display");
    } finally {
        maybeEnablePlantButton();
    }
});

function maybeEnablePlantButton() {
    let dataReady =
        treasureImageEncoded &&
        treasureClaimUrl &&
        secretKey &&
        publicKey;

    if (dataReady && !treasurePlanted) {
        plantButton.disabled = false;
    }
}
