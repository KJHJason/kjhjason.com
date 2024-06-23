import json
import shutil
import pathlib
import logging
from urllib import request as urllib_request

def init_env_file() -> None:
    env_file_path = pathlib.Path(".env")
    if env_file_path.exists():
        logging.info("The .env file already exists.")
        return

    env_keys = [
        "DEBUG_MODE",
        "MINIFY_HTML",
        "MONGODB_URI",
        "R2_ACCOUNT_ID",
        "AWS_ACCESS_KEY_ID",
        "AWS_SECRET_ACCESS_KEY",
        "CF_TURNSTILE_SECRET_KEY",
        "SECRET_KEY",
        "SECRET_KEY_SALT",
        "CSRF_KEY_SALT",
        "BLOG_ADMIN_USERNAME",
        "BLOG_ADMIN_EMAIL",
        "BLOG_ADMIN_PASSWORD",
        "DB_ENCRYPTION_KEY",
        "DB_ENCRYPTION_KEY_AAD",
    ]
    with open(env_file_path, "w") as f:
        for key in env_keys:
            f.write(f'{key}=""\n')

    logging.info("Initialised the .env file with keys.")

def replace_hosted_viewer_origins_in_viewer_mjs() -> None:
    logging.info("Replacing the HOSTED_VIEWER_ORIGINS value in viewer.mjs")

    viewer_mjs_path = pathlib.Path("./static/pdfjs/web/viewer.mjs")
    if not viewer_mjs_path.exists():
        logging.error("viewer.mjs does not exist")
        return

    with open(viewer_mjs_path, encoding="utf-8") as f:
        content = f.read()

    to_replace = 'const HOSTED_VIEWER_ORIGINS = ["null", "http://mozilla.github.io", "https://mozilla.github.io"];'
    if to_replace not in content:
        logging.error("Could not find the HOSTED_VIEWER_ORIGINS in viewer.mjs")
        return

    content = content.replace(
        to_replace,
        'const HOSTED_VIEWER_ORIGINS = ["null", "https://storage.kjhjason.com"];',
    )
    with open(viewer_mjs_path, "w", encoding="utf-8") as f:
        f.write(content)
    logging.info("Replaced the HOSTED_VIEWER_ORIGINS value in viewer.mjs")

def download_pdfjs() -> None:
    # https://mozilla.github.io/pdf.js/getting_started/#download
    extract_dir = pathlib.Path("./static/pdfjs")
    if extract_dir.exists():
        # remove the existing pdfjs directory
        logging.warning("Removing the existing pdfjs directory")
        shutil.rmtree(extract_dir)

    # download the latest version of pdfjs
    logging.info("Downloading the latest version of pdfjs")
    url = "https://api.github.com/repos/mozilla/pdf.js/releases/latest"
    req = urllib_request.Request(url)
    json_response: dict = {}
    with urllib_request.urlopen(req) as response:
        data = response.read()
        json_response = json.loads(data)

    download_url: str = ""
    assets: list[dict] = json_response["assets"]
    for asset in assets:
        name: str = asset["name"]
        if "legacy" in name:
            continue

        download_url = asset["browser_download_url"]

    if download_url == "":
        logging.error("Could not find the download url for pdfjs")
        return

    # download the pdfjs zip
    logging.info(f"Downloading pdfjs from {download_url}")
    zip_path = pathlib.Path("./static/pdfjs.zip")
    with urllib_request.urlopen(download_url) as res, open(zip_path, "wb") as f:
        shutil.copyfileobj(res, f)

    # extract the zip
    logging.info(f"Extracting pdfjs to {extract_dir}")
    shutil.unpack_archive(zip_path, extract_dir)
    zip_path.unlink()

    logging.info("Downloaded and extracted pdfjs")


def main() -> None:
    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s - %(levelname)s - %(message)s",
    )
    init_env_file()
    download_pdfjs()
    replace_hosted_viewer_origins_in_viewer_mjs()

if __name__ == "__main__":
    main()
