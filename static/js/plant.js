console.log("before click");

let treasureImageBin = null;

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
    const qrScanner = new QrScanner(video, (result) => {
        console.log(result);
        stopScanning();

        
    }, (error) => {
        //console.error(error);
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
