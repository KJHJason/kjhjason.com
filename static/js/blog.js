let csrfHeaderName = null;
let csrfValue = null;

const isPublic = document.getElementById("is-public");

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

const saveFileInfo = (file, fileSlice) => {
    fileSlice.push({"file": file, "time": new Date().getTime()});
    localStorage.setItem("fileSlice", JSON.stringify(fileSlice));
};
const maxAge = 7 * 24 * 60 * 60 * 1000;
const loadFileInfo = () => {
    const fileSlice = localStorage.getItem("fileSlice");
    if (fileSlice === null) {
        return [];
    }
    const files = JSON.parse(fileSlice);
    const currentTime = new Date().getTime();
    return files.filter((file) => currentTime - file.time < maxAge);
};
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
content.addEventListener("input", () => {
    const value = content.value;
    if (useLocalStorage) {
        localStorage.setItem("content", value);
    }
    contentPreview.value = value;
});
const title = document.getElementById("title");
const titlePreivew = document.getElementById("blog-title");
title.addEventListener("input", () => {
    const value = title.value;
    titlePreivew.innerText = value;
    if (useLocalStorage) {
        localStorage.setItem("title", value);
    }
    localStorage.setItem("title", value);
});

const maxTags = 8;
const tagsInp = document.getElementById("tags");
tagsInp.addEventListener("input", () => {
    tagsSlice = tags.value.split(",");
    if (tagsSlice.length > 8) {
        tagsSlice = tagsSlice.slice(0, 8);
    }
    if (useLocalStorage) {
        localStorage.setItem("tags", JSON.stringify(tagsSlice));
    }
});
const loadTags = () => {
    const tagsSlice = localStorage.getItem("tags");
    if (tagsSlice === null) {
        return;
    }
    tags.value = JSON.parse(tagsSlice).join(",");
};
const parseTags = (tags) => {
    return tags.split(",")
        .map((tag) => tag.trim())
        .filter((tag) => tag.length > 0);
};
