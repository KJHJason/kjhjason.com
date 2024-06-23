// LocalStorage keys
const titleKey = "title";
const seoDescKey = "seoDesc";
const contentKey = "content";
const tagsKey = "tags";
const fileSliceKey = "fileSlice";

let csrfHeaderName = null;
let csrfValue = null;

const isPublic = document.getElementById("is-public");

/**
 * @typedef {object} FileResponse
 * @property {string} name
 * @property {string} url
 * @property {string} signed_url
 */
/**
 * 
 * @param {FileResponse} file 
 * @returns {string}
 */
const parseUrlToMd = (file) => {
    if (!file.url.endsWith(".mp4")) {
        return `![${file.name}](${file.signed_url})\n`;
    }
    // if the file uploaded is a mp4, use video tag
    return `<video controls>
    <source src="${file.signed_url}" type="video/mp4">
</video>\n`;
};

const editDiv = document.getElementById("edit");
if (editDiv === null) {
    throw new Error("edit div not found");
}

const maxSize = 1024 * 1024 * 100; // 100MB

/**
 * Check if the file size is less than maxSize
 * 
 * Note: Since I am using Cloudflare behind the web server,
 * Cloudflare free plan only allows 100MB in a single request
 * Hence, if there's a need to upload a file larger than 100MB,
 * chunk upload should be implemented instead.
 * 
 * @param {File} file
 * @returns {boolean}
 */
const checkFileSize = (file) => {
    if (file.size > maxSize) {
        alert("File size is too large");
        return false;
    }
    return true;
};

let fileUploadResponseHandler = null;
document.onpaste = (e) => {
    const items = (e.clipboardData || e.originalEvent.clipboardData).items;
    for (index in items) {
        const item = items[index];
        if (item.kind === "file") {
            const file = item.getAsFile();
            if (!checkFileSize(file)) {
                return;
            }
            uploadImage(file);
        }
    }
};
editDiv.addEventListener("drop", (e) => {
    e.preventDefault();
    const files = e.dataTransfer.files;
    for (const file of files) {
        if (!checkFileSize(file)) {
            return;
        }
        uploadImage(file);
    }
});

/**
 * Uploads the file to the server
 * 
 * @param {File} file
 * @returns {void}
 */
const uploadImage = (file) => {
    if (csrfValue === null || csrfHeaderName === null) {
        throw new Error("csrf header name or value is null");
    }
    if (fileUploadResponseHandler === null) {
        throw new Error("fileUploadResponseHandler is null");
    }

    const formData = new FormData();
    formData.append("file", file);
    fetch("/api/blog/upload/files", {
        method: "POST",
        body: formData,
        headers: {
            [csrfHeaderName]: csrfValue,
        },  
    })
        .then((response) => response.json())
        .then((data) => {
            data.files.forEach((file) => {
                fileUploadResponseHandler(file);
                content.value += parseUrlToMd(file);
                content.dispatchEvent(new Event("input", {
                    bubbles: true,
                    cancelable: true,
                }));
            });
        })
        .catch((error) => {
            console.error("Error:", error);
        });
}

/* Fn mainly for the new blog route */

/**
 * @typedef {object} FileSlice
 * @property {FileResponse} file
 * @property {number} time
 */
/**
 * 
 * @param {FileResponse} file 
 * @param {FileSlice[]} fileSlice
 * @returns {void}
 */
const saveFileInfo = (file, fileSlice) => {
    fileSlice.push({"file": file, "time": new Date().getTime()});
    localStorage.setItem(fileSliceKey, JSON.stringify(fileSlice));
};

const maxAge = 7 * 24 * 60 * 60 * 1000;

/**
 * Loads the file info from localStorage
 * no longer than maxAge (7 days)
 * 
 * @returns {FileSlice[]}
 */
const loadFileInfo = () => {
    const fileSlice = localStorage.getItem(fileSliceKey);
    if (fileSlice === null) {
        return [];
    }
    const files = JSON.parse(fileSlice);
    const currentTime = new Date().getTime();
    return files.filter((file) => currentTime - file.time < maxAge);
};

/**
 * Converts the fileSlice to an array of files
 * to be used for the upload logic.
 * 
 * @param {FileSlice[]} fileSlice
 * @returns {File[]}
 */
const parseFileSliceForUpload = (fileSlice) => {
    return fileSlice.map((file) => file.file);
};

/* End of file fn for the new blog route */

const previewDiv = document.getElementById("preview");
const editBtnEvt = () => {
    editDiv.classList.remove("hidden");
    previewDiv.classList.add("hidden");
};
const previewBtnEvt = () => {
    editDiv.classList.add("hidden");
    previewDiv.classList.remove("hidden");
};

let useLocalStorage = true;
const content = document.getElementById("content");
const contentPreview = document.getElementById("content-preview");

/**
 * Updates the content of the blog
 * 
 * @param {string} value
 * @returns {void}
 */
const updateContent = (value) => {
    content.value = value;
    contentPreview.value = value;
}
content.addEventListener("input", () => {
    const value = content.value;
    if (useLocalStorage) {
        localStorage.setItem(contentKey, value);
    }
    contentPreview.value = value;
});

const title = document.getElementById("title");
const titlePreivew = document.getElementById("blog-title");

/**
 * Updates the title of the blog
 * 
 * @param {string} value
 * @returns {void}
 */
const updateTitle = (value) => {
    title.value = value;
    titlePreivew.innerText = value;
}
title.addEventListener("input", () => {
    const value = title.value;
    titlePreivew.innerText = value;
    if (useLocalStorage) {
        localStorage.setItem(titleKey, value);
    }
    localStorage.setItem(titleKey, value);
});

const seoDesc = document.getElementById("seo-desc");
seoDesc.addEventListener("input", () => {
    if (useLocalStorage) {
        localStorage.setItem(seoDescKey, seoDesc.value);
    }
});
const loadSeoDesc = () => {
    const seoDescValue = localStorage.getItem(seoDescKey);
    if (seoDescValue === null) {
        return;
    }
    seoDesc.value = seoDescValue;
};

const maxTags = 8;
const tagsInp = document.getElementById("tags");
tagsInp.addEventListener("input", () => {
    tagsSlice = tags.value.split(",");
    if (tagsSlice.length > 8) {
        tagsSlice = tagsSlice.slice(0, 8);
    }
    if (useLocalStorage) {
        localStorage.setItem(tagsKey, JSON.stringify(tagsSlice));
    }
});

/**
 * Loads the tags from localStorage
 * to the tags input field
 * 
 * @returns {string}
 */
const loadTags = () => {
    const tagsSlice = localStorage.getItem(tagsKey);
    if (tagsSlice === null) {
        return;
    }
    tags.value = JSON.parse(tagsSlice).join(",");
};

/**
 * Parse the tag input value to an array of tags
 * @param {string} tags 
 * @returns {string[]}
 */
const parseTags = (tags) => {
    return tags.split(",")
        .map((tag) => tag.trim())
        .filter((tag) => tag.length > 0);
};
