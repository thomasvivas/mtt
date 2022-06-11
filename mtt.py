from PIL import Image
from enum import Enum
import sys

IMAGE_TARGET = "target.jpg"

def resize_image(file_name: str) -> None:
    image = Image.open(file_name)
    LENGTH, WIDTH = (100, 50)
    image.resize((LENGTH, WIDTH)).save(IMAGE_TARGET)

def get_brightness(R: int, G: int, B: int) -> float:
    return 0.2126*R + 0.7152*G + 0.0722*B

def brightness_chr(brightness: float) -> str:
    PIXELS = " .:-=+*#%@"
    return PIXELS[int(brightness // (256 / len(PIXELS)))]

def print_pixel(RGB:tuple) -> None:
    BRIGHTNESS = get_brightness(*RGB)
    print(brightness_chr(BRIGHTNESS), end="")

def print_image(file_name: str) -> None:
    resize_image(file_name)

    image = Image.open(IMAGE_TARGET)
    WIDTH, HEIGHT = image.size

    for row in range(HEIGHT):
        for column in range(WIDTH):
            print_pixel(image.getpixel((column, row)))
        print()

def play_video(file_name: str):
    # Placeholder for video playing implementation
    pass

class FileType(Enum):
    UNSUPPORTED = 1
    IMAGE = 2
    VIDEO = 3

def file_type(file_name: str) -> FileType:
    EXTENSION = file_name.split(".")[-1]
    match EXTENSION:
        case "jpg" | "jpeg" | "png":
            return FileType.IMAGE
        case "mp4" | "mov":
            return FileType.VIDEO
        case _:
            return FileType.UNSUPPORTED


def main() -> None:
    if len(sys.argv) == 1:
        raise LookupError("No file was supplied")

    SELECTED_FILE = sys.argv[1]
    SELECTED_TYPE = file_type(SELECTED_FILE)

    if SELECTED_TYPE is FileType.UNSUPPORTED:
        raise ValueError("File type is not supported")
    
    match SELECTED_TYPE:
        case FileType.IMAGE:
            print_image(SELECTED_FILE)    
        case FileType.VIDEO:
            play_video(SELECTED_FILE)

if __name__ == "__main__":
    main()