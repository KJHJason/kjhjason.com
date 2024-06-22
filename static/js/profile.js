const changePasswordForm = document.getElementById("change-password-form");
const handleChangePasswordRes = (e) => {
    turnstile.reset();

    const detail = e.detail;
    if (detail.xhr.status === 200) {
        changePasswordForm.reset();
    }
};
