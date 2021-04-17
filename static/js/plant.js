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
    if (imageUploadButton.files.length == 0) {
        treasureImageBin = null;
        return;
    }

    let file = imageUploadButton.files[0];
    let bin = await file.arrayBuffer();
    let blob = new Blob([bin], { type: file.type });
    //let urlCreator = window.URL
});
