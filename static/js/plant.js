console.assert(typeof treasureClaimUrl != "undefined");
console.assert(typeof secretKey != "undefined");
console.assert(typeof publicKey != "undefined");


let treasureImageEncoded = null;
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

    treasureImageEncoded = null;
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

        treasureImageEncoded = btoa(blob);

        maybeEnablePlantButton();
    } finally {
        imageUploadButton.disabled = false;
        useTestImageButton.disabled = false;
        fileSpinner.classList.add("no-display");
    }
});

useTestImageButton.addEventListener("click", async () => {

    plantButton.disabled = true;

    treasureImageEncoded = null;
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

        treasureImageEncoded = btoa(blob);

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

    console.assert(treasureImageEncoded);
    console.assert(secretKey);

    plantSpinner.classList.remove("no-display");

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
        plantSpinner.classList.add("no-display");
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
