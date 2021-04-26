console.assert(typeof treasureClaimUrl != "undefined");
console.assert(typeof secretKey != "undefined");
console.assert(typeof publicKey != "undefined");


let treasureClaimed = false;

let claimButton = getElementById("claim-button");

console.assert(claimButton);


function maybeEnableClaimButton() {
    let dataReady =
        treasureClaimUrl &&
        secretKey &&
        publicKey;

    if (dataReady && !treasureClaimed) {
        claimButton.disabled = false;
    }
}


/* These handlers are required by secren-scan.js */

function onBeginSecretScan() {
    plantButton.disabled = true;
}

function onEndSecretScan() {
    maybeEnableClaimButton();
}
