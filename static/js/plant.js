import { initWasm } from "./wasm-init.js";
import {
    initAccount,
    accountSecretKey,
    accountPublicKey,
} from "./account.js";
import {
    initSecretScanner,
    treasureClaimUrl,
    treasureSecretKey,
    treasurePublicKey
} from "./secret-scan.js";

initAccount({
    onAccountSecretKeyChanged: onAccountSecretKeyChanged,
});

initSecretScanner({
    onBeginSecretScan: onBeginSecretScan,
    onEndSecretScan: onEndSecretScan
});

let treasureImageBlob = null;
let treasurePlanted = false;

let plantButton = document.getElementById("plant-button");

console.assert(plantButton);

let imageUploadButton = document.getElementById("image-upload-button");
let useTestImageButton = document.getElementById("use-test-image-button")
let imageElt = document.getElementById("treasure-image");
let fileSpinner = document.getElementById("file-spinner");

console.assert(imageUploadButton);
console.assert(useTestImageButton);
console.assert(imageElt);
console.assert(fileSpinner);

imageUploadButton.addEventListener("change", async () => {

    plantButton.disabled = true;

    treasureImageBlob = null;
    imageElt.src = "";
    imageElt.classList.add("no-display");

    imageUploadButton.disabled = true;
    useTestImageButton.disabled = true;

    fileSpinner.classList.remove("no-display");

    try {

        if (imageUploadButton.files.length == 0) {
            return;
        }

        let file = imageUploadButton.files[0];
        let bin = await file.arrayBuffer();
        let blob = new Blob([bin], { type: file.type });

        imageElt.src = URL.createObjectURL(blob);
        imageElt.classList.remove("no-display");

        treasureImageBlob = blob;

        maybeEnablePlantButton();
    } finally {
        imageUploadButton.disabled = false;
        useTestImageButton.disabled = false;
        fileSpinner.classList.add("no-display");
    }
});

useTestImageButton.addEventListener("click", async () => {

    plantButton.disabled = true;

    treasureImageBlob = null;
    imageElt.scr = "";
    imageElt.classList.add("no-display");

    imageUploadButton.disabled = true;
    useTestImageButton.disabled = true;

    fileSpinner.classList.remove("no-display");

    try {
        let response = await fetch("images/coconut-tree.png");

        if (!response.ok) {
            // TODO
        }

        let blob = await response.blob();

        imageElt.src = URL.createObjectURL(blob);
        imageElt.classList.remove("no-display");

        treasureImageBlob = blob;

        maybeEnablePlantButton();
    } finally {
        imageUploadButton.disabled = false;
        useTestImageButton.disabled = false;
        fileSpinner.classList.add("no-display");
    }
});

plantButton.addEventListener("click", async () => {

    let plantSpinner = document.getElementById("plant-spinner");

    console.assert(plantSpinner);

    plantButton.disabled = true;

    plantSpinner.classList.remove("no-display");

    try {
        let encoder = new Promise((resolve) => {
            let reader = new FileReader();
            reader.readAsBinaryString(treasureImageBlob);
            reader.addEventListener("loadend", () => {
                resolve(reader.result);
            });
        });

        let treasureImageBuffer = await encoder;
        let treasureImageEncoded = btoa(treasureImageBuffer);
        
        let wasm = await initWasm();

        // todo:
        // let treasureHash = wasm.get_hash(treasureImageBuffer);

        let treasureHash = wasm.get_hash(treasureImageEncoded);
        let treasureSignature = wasm.sign_with_secret_key(treasureSecretKey, treasureHash);

        let accountSignature = "todo";

        let requestInfo = {
            account_public_key: accountPublicKey,
            treasure_public_key: treasurePublicKey,
            image: treasureImageEncoded,
            account_signature: accountSignature,
            treasure_signature: treasureSignature,
        };

        let response = await fetch("api/plant", {
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

        treasurePlanted = true;

        let plantedMessageElt = document.getElementById("planted-message");
        console.assert(plantedMessageElt);
        plantedMessageElt.classList.remove("no-display");
    } finally {
        maybeEnablePlantButton();
        plantSpinner.classList.add("no-display");
    }
});

function maybeEnablePlantButton() {
    let dataReady =
        treasureImageBlob &&
        treasureClaimUrl &&
        treasureSecretKey &&
        treasurePublicKey &&
        accountSecretKey;

    if (dataReady && !treasurePlanted) {
        plantButton.disabled = false;
    }
}

function onAccountSecretKeyChanged() {
    maybeEnablePlantButton();
}

function onBeginSecretScan() {
    plantButton.disabled = true;
}

function onEndSecretScan() {
    maybeEnablePlantButton();
}
