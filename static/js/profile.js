const changePasswordForm = document.getElementById("change-password-form");

/**
 * @typedef {Object} detail
 * @property {XMLHttpRequest} xhr
 */
/**
 * @typedef {Object} ChangePasswordEvent
 * @property {detail} detail
 */
/**
 * Handles the change password response
 * 
 * @param {ChangePasswordEvent} e
 */
const handleChangePasswordRes = (e) => {
    turnstile.reset();

    const detail = e.detail;
    if (detail.xhr.status === 200) {
        changePasswordForm.reset();
    }
};
