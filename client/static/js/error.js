const getErrorMsg = (e) => {
    let errorResponse;
    const xhr = e.detail.xhr;
    try {
        errorResponse = JSON.parse(xhr.response);
    } catch (e) {
        return xhr.response;
    }

    if (errorResponse.error) {
        return errorResponse.error;
    }
    return e.detail.error;
};
