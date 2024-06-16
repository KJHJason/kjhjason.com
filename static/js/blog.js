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

const previewDiv = document.getElementById("preview");
const editBtnEvt = () => {
    editDiv.classList.remove("hidden");
    previewDiv.classList.add("hidden");
};
const previewBtnEvt = () => {
    editDiv.classList.add("hidden");
    previewDiv.classList.remove("hidden");
};

const content = document.getElementById("content");
const contentPreview = document.getElementById("content-preview");
content.addEventListener("input", () => {
    const value = content.value;
    localStorage.setItem("content", value);
    contentPreview.value = value;
});
const title = document.getElementById("title");
const titlePreivew = document.getElementById("blog-title");
title.addEventListener("input", () => {
    const value = title.value;
    titlePreivew.innerText = value;
    localStorage.setItem("title", value);
});

const tagsInp = document.getElementById("tags");
tagsInp.addEventListener("input", () => {
    tagsSlice = tags.value.split(",");
    if (tagsSlice.length > 8) {
        tagsSlice = tagsSlice.slice(0, 8);
    }
    localStorage.setItem("tags", JSON.stringify(tagsSlice));
});
const loadTags = () => {
    const tagsSlice = localStorage.getItem("tags");
    if (tagsSlice === null) {
        return;
    }
    tags.value = JSON.parse(tagsSlice).join(",");
};
