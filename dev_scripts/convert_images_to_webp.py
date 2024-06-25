import pathlib
import logging
import subprocess

QUALITY = "80"
REMOVE_METADATA = True
IMG_DIR = r"E:\Codes\Github Projects\kjhjason.com\res\projects"

def convert_jpeg_to_webp(img_file: pathlib.Path, replaced: pathlib.Path) -> None:
    logging.info(f"Converting {img_file} to webp using ImageMagick to {replaced}")
    try:
        # Since the cwebp doesn't preserve the image orientation for jpg/jpeg despite 
        # using -metadata all and using exiftool to copy the metadata from the original 
        # image to the webp, decided to use ImageMagick instead.
        subprocess.run(
            [
                "magick", # Using https://www.imagemagick.org/script/download.php#windows
                img_file,
                "-auto-orient",
                "-quality",
                QUALITY,
                replaced,
            ],
            check=True,
        )
    except subprocess.CalledProcessError as e:
        logging.error(f"Could not convert {img_file} to webp: {e}")
        return

    if REMOVE_METADATA:
        logging.info(f"Removing metadata from {replaced}")
        try:
            subprocess.run(
                ["exiftool", "-overwrite_original", "-all=", replaced],
                check=True,
            )
        except subprocess.CalledProcessError as e:
            logging.error(f"Could not remove metadata from {replaced}: {e}")
            return

def convert_gif_to_webp(img_file: pathlib.Path, replaced: pathlib.Path) -> None:
    logging.info(f"Converting {img_file} to webp using gif2webp to {replaced}")
    try:
        # https://developers.google.com/speed/webp/docs/gif2webp
        subprocess.run(
            ["gif2webp", "-q", QUALITY, "-mt", img_file, "-o", replaced],
            check=True,
        )
    except subprocess.CalledProcessError as e:
        logging.error(f"Could not convert {img_file} to webp: {e}")
        return

def convert_png_to_webp(img_file: pathlib.Path, replaced: pathlib.Path) -> None:
    logging.info(f"Converting {img_file} to webp using cwebp to {replaced}")
    try:
        # https://developers.google.com/speed/webp/docs/using
        subprocess.run(
            ["cwebp", "-q", QUALITY, "-mt", img_file, "-o", replaced],
            check=True,
        )
    except subprocess.CalledProcessError as e:
        logging.error(f"Could not convert {img_file} to webp: {e}")
        return

def convert() -> None:
    img_dir = pathlib.Path(IMG_DIR)
    if not img_dir.exists():
        logging.error(f"Directory {img_dir} does not exist")
        return

    allowed_suffixes = [".png", ".gif", ".jpg", ".jpeg"]
    img_files = img_dir.glob("*")
    for img_file in img_files:
        if img_file.suffix not in allowed_suffixes:
            continue

        new_folder = img_dir / "converted"
        new_folder.mkdir(exist_ok=True)

        replaced = new_folder / f"{img_file.stem}.webp"
        if img_file.suffix in [".jpg", ".jpeg"]:
            convert_jpeg_to_webp(img_file, replaced)
        elif img_file.suffix == ".gif":
            convert_gif_to_webp(img_file, replaced)
        else:
            convert_png_to_webp(img_file, replaced)

def main() -> None:
    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s - %(levelname)s - %(message)s",
    )
    convert()

if __name__ == "__main__":
    main()
