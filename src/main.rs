use clipboard_win::{formats, get_clipboard, set_clipboard};
use leptess::{leptonica, tesseract};


fn main() {

    let result = match get_clipboard(formats::Bitmap) {
        Ok(v) => v,
        Err(e) => {
            dbg!(e);
            panic!("Failed to read clipboard as bitmap");
        }
    };

    let mut api = tesseract::TessApi::new(Some("tessdata"), "eng").unwrap();
    let pix = leptonica::pix_read_mem(&result).unwrap();

    api.set_image(&pix);

    api.set_source_resolution(1920);

    set_clipboard(formats::Unicode, api.get_utf8_text().unwrap()).unwrap();
}
