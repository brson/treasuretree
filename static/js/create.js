console.log("before click");
let createButton = document.getElementById("create-button");
console.log(createButton);
createButton.addEventListener("click", async () => {
    console.log("click");

    let response = await fetch("api/create");
    console.log(response);

    let jsonResponse = await response.json();
    console.log(jsonResponse);

    document.getElementById("display-qrcode").innerHTML = jsonResponse.qrcode;
});

