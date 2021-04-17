console.log("before click");

let treasureImageBin = null;

let plantButton = document.getElementById("plant-button");

console.log(plantButton);

plantButton.addEventListener("click", async () => {
    console.log("click");

    let response = await fetch("api/plant");
    console.log(response);

    let jsonResponse = await response.json();
    console.log(jsonResponse);
});

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
