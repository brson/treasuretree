let createButton = document.getElementById("create-button");

console.assert(createButton);

createButton.addEventListener("click", async () => {

    let qrCodeContainer = document.getElementById("display-qrcode");
    let secretKeyContainer = document.getElementById("secret-key");
    let spinner = document.getElementById("create-spinner");

    console.assert(qrCodeContainer);
    console.assert(secretKeyContainer);

    qrCodeContainer.innerHTML = null;
    secretKeyContainer.innerHTML = null;

    createButton.disabled = true;
    spinner.classList.remove("no-display");

    try {
        let response = await fetch("api/create");

        console.log(response);

        if (!response.ok) {
            // TODO
        }

        let jsonResponse = await response.json();

        console.log(jsonResponse);
        console.assert(jsonResponse.qrcode);
        console.assert(jsonResponse.secret_key);

        qrCodeContainer.innerHTML = jsonResponse.qrcode;
        secretKeyContainer.innerHTML = jsonResponse.secret_key;
    } finally {
        createButton.disabled = false;
        spinner.classList.add("no-display");
    }
});

