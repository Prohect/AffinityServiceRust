# Tesseract OCR

**Version:** 5.4.0.20240606  
**Installation Path:** `C:\Program Files\Tesseract-OCR\`  
**Installed via:** winget (`UB-Mannheim.TesseractOCR`)

## Usage

Extract text from an image:

```sh
tesseract image.png output       # outputs to output.txt
tesseract image.png stdout       # outputs directly to console
```

## Common Options

- `-l <lang>` - Specify language (e.g., `-l eng`, `-l chi_sim` for Simplified Chinese)
- `--psm <n>` - Page segmentation mode (0-13)
- `--oem <n>` - OCR engine mode (0-3)

## Notes

- May need to open a new terminal session for PATH to update after installation
- Additional language packs can be installed separately

## Best Preprocessing Steps for OCR

For better accuracy on screenshots, low-resolution, or noisy images:

1. **Resize for higher DPI**: Upscale to at least 200% to improve text recognition.
   ```
   magick input.png -resize 200% resized.png
   ```

2. **Convert to grayscale**: Removes color noise and enhances contrast.
   ```
   magick resized.png -colorspace Gray gray.png
   ```

3. **Optional binarization**: Use threshold to create black-and-white image if text is clear.
   ```
   magick gray.png -threshold 50% binary.png
   ```

4. **Run Tesseract**: Use PSM 6 for tabular data, LSTM engine, and English language.
   ```
   tesseract binary.png output --psm 6 -l eng --oem 1
   ```

Example full pipeline:
```
magick input.png -resize 200% -colorspace Gray preprocessed.png
tesseract preprocessed.png temp/output.txt --psm 6 -l eng --oem 1
```
