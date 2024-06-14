const parseUrlToMd = (file) => {
    if (!file.url.endsWith(".mp4")) {
        return `![${file.name}](${file.url})\n`;
    }
    // if the file uploaded is a mp4, use video tag
    return `<video controls>
    <source src="${file.url}" type="video/mp4">
</video>\n`;
};

const editDiv = document.getElementById("edit");
if (editDiv === null) {
    throw new Error("edit div not found");
}

document.onpaste = (e) => {
    const items = (e.clipboardData || e.originalEvent.clipboardData).items;
    for (index in items) {
        const item = items[index];
        if (item.kind === "file") {
            const file = item.getAsFile();
            uploadImage(file);
        }
    }
}
editDiv.addEventListener("drop", (e) => {
    e.preventDefault();
    const files = e.dataTransfer.files;
    for (const file of files) {
        uploadImage(file);
    }
});

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

const onSuccess = (useLocalStorage = true) => {
    if (useLocalStorage) {
        localStorage.removeItem("content");
        localStorage.removeItem("title");
    }
    window.location.href = "/admin/blogs"; // TODO: Change to the blog page
};
