console.log("before click");

let treasureImageBin = null;
let treasureClaimUrl = null;
let secretKey = null;
let publicKey = null;

let imageUploadButton = document.getElementById("image-upload-button");

imageUploadButton.addEventListener("change", async () => {
    let imgElt = document.getElementById("treasure-image");

    if (imageUploadButton.files.length == 0) {
        treasureImageBin = null;
        imgElt.src = "";
        return;
    }

    let file = imageUploadButton.files[0];
    let bin = await file.arrayBuffer();
    let blob = new Blob([bin], { type: file.type });

    imgElt.src = URL.createObjectURL(blob);
});

let qrScanButton = document.getElementById("qrscan-button");

QrScanner.WORKER_PATH = "js/lib/qr-scanner-worker.min.js";

qrScanButton.addEventListener("click", async () => {
    qrScanButton.enabled = false;
    let video = document.getElementById("qr-video");
    let stopScanning = null;
    const qrScanner = new QrScanner(video, async (result) => {
        console.log(result);
        stopScanning();

        let wasm = await initWasm();

        let url = result;
        let sanityCheck = wasm.sanity_check_url(url);

        if (sanityCheck != null) {
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

        let treasureClaimUrlElt = document.getElementById("treasure-claim-url");
        let secretKeyElt = document.getElementById("secret-key");
        let publicKeyElt = document.getElementById("public-key");

        treasureClaimUrlElt.innerText = url;
        secretKeyElt.innerText = secretKey_;
        publicKeyElt.innerText = publicKey_;

        treasureClaimUrl = url;
        secretKey = secretKey_;
        publicKey = publicKey_;
        
    }, (error) => {
        console.error(error);
        //stopScanning();
    });
    stopScanning = () => {
        qrScanner.stop();
        qrScanner.destroy();
        qrScanButton.enabled = true;
    }
    qrScanButton.enabled = false;
    qrScanner.start();
});

let plantButton = document.getElementById("plant-button");

plantButton.addEventListener("click", async () => {
    console.log("click");

    let response = await fetch("api/plant");
    console.log(response);

    let jsonResponse = await response.json();
    console.log(jsonResponse);
});
