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

def _replace_hosted_viewer_origins_in_file(file_path: str, value: str, to_replace_with: str) -> None:
    logging.info(f"Replacing the HOSTED_VIEWER_ORIGINS value in {file_path}")

    with open(file_path, "r", encoding="utf-8") as f:
        content = f.read()
        if value not in content:
            logging.error(f"Could not find the HOSTED_VIEWER_ORIGINS in {file_path}")
            return

    with open(file_path, "w", encoding="utf-8") as f:
        f.write(content.replace(value, to_replace_with))
    logging.info(f"Replaced the HOSTED_VIEWER_ORIGINS value in {file_path}")

def replace_hosted_viewer_origins_for_pdfjs() -> None:
    _replace_hosted_viewer_origins_in_file(
        "./static/pdfjs/web/viewer.mjs",
        'const HOSTED_VIEWER_ORIGINS = ["null", "http://mozilla.github.io", "https://mozilla.github.io"];',
        'const HOSTED_VIEWER_ORIGINS = ["null", "https://storage.kjhjason.com"];',
    )
    _replace_hosted_viewer_origins_in_file(
        "./static/pdfjs/web/viewer.mjs.map",
        r'const HOSTED_VIEWER_ORIGINS = [\n    \"null\",\n    \"http://mozilla.github.io\",\n    \"https://mozilla.github.io\",\n  ];',
        r'const HOSTED_VIEWER_ORIGINS = [\n    "null",\n    "https://storage.kjhjason.com",\n  ];',
    )

def _remove_file_origin_checks_in_file(file_path: str, value: str) -> None:
    logging.info(f"Removing the file origin checks in {file_path}")

    with open(file_path, "r", encoding="utf-8") as f:
        content = f.read()
        if value not in content:
            logging.error(f"Could not find the file origin check in {file_path}")
            return

    with open(file_path, "w", encoding="utf-8") as f:
        f.write(content.replace(value, ""))

    logging.info(f"Removed the file origin check in {file_path}")

def remove_file_origin_checks_for_pdfjs() -> None:
    mjs_map_value = r"if (fileOrigin !== viewerOrigin) {\n        throw new Error(\"file origin does not match viewer's\");\n      }\n"
    mjs_value = r"""      if (fileOrigin !== viewerOrigin) {
        throw new Error("file origin does not match viewer's");
      }"""

    _remove_file_origin_checks_in_file("./static/pdfjs/web/viewer.mjs", mjs_value)
    _remove_file_origin_checks_in_file("./static/pdfjs/web/viewer.mjs.map", mjs_map_value)

def _change_default_options_in_file(file_path: str, value: str, to_replace_with: str) -> None:
    logging.info(f"Changing the default options in {file_path}")

    common_value = "enablePermissions = false"
    with open(file_path, "r", encoding="utf-8") as f:
        content = f.read()
        if value not in content:
            logging.error(f"Could not find the default options in {file_path}")
            return
        if common_value not in content:
            logging.error(f"Could not find the common default options in {file_path}")
            return

    with open(file_path, "w", encoding="utf-8") as f:
        content = content.replace(common_value, "enablePermissions = true")
        content = content.replace(value, to_replace_with)
        f.write(content)

    logging.info(f"Changed the default options in {file_path}")

def change_default_options_for_pdfjs() -> None:
    map_value = r"enablePermissions: {\n    /** @type {boolean} */\n    value: false,"
    value = r"enablePermissions: false,"

    _change_default_options_in_file(
        "./static/pdfjs/web/viewer.mjs", 
        value,
        r"enablePermissions: true,",
    )
    _change_default_options_in_file(
        "./static/pdfjs/web/viewer.mjs.map", 
        map_value,
        r"enablePermissions: {\n    /** @type {boolean} */\n    value: true,"
    )

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

    # Modify the pdfjs source files
    replace_hosted_viewer_origins_for_pdfjs()

    # Remove the file origin checks so 
    # that the pdf can use storage.kjhjason.com
    remove_file_origin_checks_for_pdfjs()

    # Change the default options for pdfjs.
    # Mainly to disable editing of the pdf.
    change_default_options_for_pdfjs()

if __name__ == "__main__":
    main()
