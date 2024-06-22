const denyAnalyticsStorage = () => {
    gtag("consent", "update", {
        "analytics_storage": "denied",
    });
};

const allowAnalyticsStorage = () => {
    gtag("consent", "update", {
        "analytics_storage": "granted",
    });
};

const gtagLocalStorageKey = "cookie-consent";
const declineCookieConsent = () => {
    localStorage.setItem(gtagLocalStorageKey, "false");
    document.getElementById("cookie-consent").classList.remove("opacity-100");
    document.getElementById("cookie-consent").classList.add("opacity-0");
    denyAnalyticsStorage();
};
const acceptCookieConsent = () => {
    localStorage.setItem(gtagLocalStorageKey, "true");
    document.getElementById("cookie-consent").classList.remove("opacity-100");
    document.getElementById("cookie-consent").classList.add("opacity-0");
    allowAnalyticsStorage();
};

const initGtagSettings = () => {
    const cookieConsentValue = localStorage.getItem(gtagLocalStorageKey);
    if (cookieConsentValue === "true") {
        allowAnalyticsStorage();
    } else {
        denyAnalyticsStorage();
    }
};
initGtagSettings();

const cookieConsentLogic = () => {
    const cookieConsentDiv = document.getElementById("cookie-consent");
    if (!cookieConsentDiv) {
        console.error("Cookie consent div not found");
        return;
    }

    if (localStorage.getItem(gtagLocalStorageKey) === null) {
        cookieConsentDiv.classList.remove("opacity-0");
        cookieConsentDiv.classList.add("opacity-100");
        return;
    };
}

document.addEventListener("DOMContentLoaded", () => {
    cookieConsentLogic();
});
