# ImageMagick

**Version:** 7.1.2.9  
**Installation Path:** `C:\Program Files\ImageMagick-7.1.2-Q16-HDRI\`  
**Installed via:** winget (`ImageMagick.ImageMagick`)

## Usage

Preprocess images for better OCR or other tasks:

```sh
magick input.png -resize 200% -colorspace Gray output.png
```

## Common Options

- `-resize <percentage>%` - Scale image
- `-colorspace Gray` - Convert to grayscale
- `-threshold <value>%` - Binarize image
- `-deskew <angle>%` - Fix tilted text

## Notes

- Useful for enhancing images before OCR with Tesseract
- Installed with proxy if needed: `export HTTP_PROXY=http://127.0.0.1:25563 && winget install ImageMagick.ImageMagick`
