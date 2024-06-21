let csrfHeaderName = "";
let csrfValue = "";

const copySecretToClipboard = () => {
    const secretEl = document.getElementById("two-fa-content-secret");
    if (!secretEl) {
        console.error("Failed to find 2FA secret!");
        return;
    }

    navigator.clipboard.writeText(secretEl.textContent);
    Swal.fire({
        icon: "success",
        title: "Copied!",
        text: "Secret copied to clipboard!",
        timer: 1000,
        timerProgressBar: true,
    })
};

const toggleShowHideSecretText = () => {
    const showHideText = document.getElementById("two-fa-showhide-text");
    if (!showHideText) {
        console.error("Failed to find 2FA show/hide text element!");
        return;
    }

    showHideText.textContent = showHideText.textContent === "Show" ? "Hide" : "Show";
};

let hasGenerated2FaSecret = false;
const get2FaSecretFromServer = async () => {
    if (hasGenerated2FaSecret) {
        return;
    }

    if (!csrfHeaderName || !csrfValue) {
        console.error("CSRF header name or value not set!");
        return;
    }

    const qrCodeImgEl = document.getElementById("two-fa-content-qr");
    const twoFaEl = document.getElementById("two-fa-content-secret");
    const twoFaInpEl = document.getElementById("two-fa-content-secret-input");
    if (!qrCodeImgEl || !twoFaEl || !twoFaInpEl) {
        console.error("Failed to find 2FA elements!");
        return;
    }

    const response = await fetch("/api/admin/generate-2fa", {
        method: "GET",
        headers: {
            [csrfHeaderName]: csrfValue,
        },
    });

    if (!response.ok) {
        Swal.fire({
            icon: "error",
            title: "Oops...",
            text: "Failed to generate 2FA secret! Please try again later.",
        })
        console.error("Failed to generate 2FA secret!", response);
        return;
    } 

    const data = await response.json();
    qrCodeImgEl.src = "data:image/png;base64, " + data.qr_code_data;
    twoFaEl.textContent = data.secret;
    twoFaInpEl.value = data.secret;
    hasGenerated2FaSecret = true;
};
