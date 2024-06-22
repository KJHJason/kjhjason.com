const totpInputDiv = document.getElementById("totp-input-div");
const handleLoginRequest = (e) => {
    const detail = e.detail;
    const xhr = detail.xhr;
    const loginError = xhr.getResponseHeader("X-Login-Error");
    if (loginError === "MissingTotp" || loginError === "InvalidTotp") {
        totpInputDiv.classList.remove("hidden")
    } else {
        totpInputDiv.classList.add("hidden")
    }

    turnstile.reset();
}
