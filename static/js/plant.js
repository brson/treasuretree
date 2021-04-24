let treasureImageBin = null;
let treasureClaimUrl = null;
let secretKey = null;
let publicKey = null;




let imageUploadButton = document.getElementById("image-upload-button");

console.assert(imageUploadButton);

imageUploadButton.addEventListener("change", async () => {

    let imageElt = document.getElementById("treasure-image");

    console.assert(imageElt);

    treasureImageBin = null;
    imageElt.src = "";

    if (imageUploadButton.files.length == 0) {
        return;
    }

    let file = imageUploadButton.files[0];
    let bin = await file.arrayBuffer();
    let blob = new Blob([bin], { type: file.type });

    imageElt.src = URL.createObjectURL(blob);
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
    }

    qrScanner.start();
});

qrCancelButton.addEventListener("click", async () => {
    stopScanning();
});

secretKeyInput.addEventListener("input", async () => {

    let wasm = await initWasm();

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
});


let plantButton = document.getElementById("plant-button");

plantButton.addEventListener("click", async () => {
    console.log("click");

    let treasure_info = {
        image: "foobarimage",
        private_key: "testprivatekey"
    };
    
    let response = await fetch("api/plant",
                               {
                                   method: "POST",
                                   headers: {
                                       'Accept': 'application/json',
                                       'Content-Type': 'application/json'
                                   },
                                   body: JSON.stringify(treasure_info)
                               });
    console.log(response);

    // let jsonResponse = await response.json();
    let jsonResponse = await response.text();
    console.log(jsonResponse);
});
