let createButton = document.getElementById("create-button");
let qrCodeContainer = document.getElementById("display-qrcode");
let secretKeyContainer = document.getElementById("secret-key");

console.assert(createButton);
console.assert(qrCodeContainer);
console.assert(secretKeyContainer);

createButton.addEventListener("click", async () => {
    qrCodeContainer.innerHTML = null;
    secretKeyContainer.innerHTML = null;
    createButton.disabled = true;

    try {
        let response = await fetch("api/create");

        console.log(response);

        let jsonResponse = await response.json();

        console.log(jsonResponse);
        console.assert(jsonResponse.qrcode);
        console.assert(jsonResponse.secret_key);

        qrCodeContainer.innerHTML = jsonResponse.qrcode;
        secretKeyContainer.innerHTML = jsonResponse.secret_key;
    } finally {
        createButton.disabled = false;
    }
});

