console.log("before click");

let treasureImageBin = null;
let treasureClaimUrl = null;
let secretKey = null;
let publicKey = null;

let imageUploadButton = document.getElementById("image-upload-button");
let imageElt = document.getElementById("treasure-image");

console.assert(imageUploadButton);
console.assert(imageElt);

imageUploadButton.addEventListener("change", async () => {

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

QrScanner.WORKER_PATH = "js/lib/qr-scanner-worker.min.js";

qrScanButton.addEventListener("click", async () => {
    let treasureClaimUrlElt = document.getElementById("treasure-claim-url");
    let secretKeyElt = document.getElementById("secret-key");
    let publicKeyElt = document.getElementById("public-key");

    treasureClaimUrlElt.innerText = null;
    secretKeyElt.innerText = null;
    publicKeyElt.innerText = null;

    let video = document.getElementById("qr-video");
    let stopScanning = null;
    const qrScanner = new QrScanner(video, async (result) => {
        console.log(result);
        stopScanning();

        let wasm = await initWasm();

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
        qrScanButton.disabled = false;
    }
    qrScanButton.disabled = true;
    qrScanner.start();
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
