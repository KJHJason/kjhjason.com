const GetErrorMsg = (e) => {
    const errorResponse = JSON.parse(e.detail.xhr.response);
    if (errorResponse.error) {
        return errorResponse.error;
    }
    return e.detail.error;
};
