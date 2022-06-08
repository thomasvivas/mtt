from PIL import Image

IMAGE_TARGET = "target.jpg"

def resize_image(file_name):
    image = Image.open(file_name)
    image.resize((100, 50)).save(IMAGE_TARGET)

def get_brightness(R, G, B):
    return 0.2126*R + 0.7152*G + 0.0722*B

def brightness_chr(brightness: float) -> str:
    PIXELS = " .:-=+*#%@"
    return PIXELS[int(brightness // (256 / len(PIXELS)))]

def print_pixel(RGB:tuple) -> None:
    BRIGHTNESS = get_brightness(*RGB)
    print(brightness_chr(BRIGHTNESS), end="")

def print_image(file_name):
    resize_image(file_name)

    image = Image.open(IMAGE_TARGET)
    WIDTH, HEIGHT = image.size

    for row in range(HEIGHT):
        for column in range(WIDTH):
            print_pixel(image.getpixel((column, row)))
        print()


def main():
    print_image("sample.jpg")

if __name__ == "__main__":
    main()