import os
import logging

def import_to_flyio() -> None:
    logging.info("Importing the secrets to fly.io")
    args = ["flyctl", "secrets", "set"]
    with open(".env") as f:
        for line in f:
            key, value = line.split("=", 1)
            if key == "DEBUG_MODE":
                value = '"false"'

            logging.info(f"Adding secret \"{key}\"")
            args.append(f"{key}={value.strip()}")

    logging.info("Setting the secrets")
    try:
        # Somehow subprocess.run causes weird 
        # string issues esp with mongodb uri
        os.system(" ".join(args))
    except Exception as e:
        logging.error(f"Coud not set the secrets: {e}")
        return
    logging.info("Imported the secrets to fly.io")

# convert that ps1 to python
def main() -> None:
    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s - %(levelname)s - %(message)s",
    )
    import_to_flyio()

if __name__ == "__main__":
    main()
