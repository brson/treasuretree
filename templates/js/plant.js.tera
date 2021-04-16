console.log("before click");
let plantButton = document.getElementById("plant-button");
console.log(plantButton);
plantButton.addEventListener("click", async () => {
    console.log("click");

    let response = await fetch("api/plant");
    console.log(response);

    let jsonResponse = await response.json();
    console.log(jsonResponse);
});

