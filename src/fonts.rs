use eframe::egui::FontData;
use eframe::egui::FontDefinitions;
use eframe::egui::FontFamily;

pub(crate) fn custom_fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    let font_data =
        FontData::from_static(include_bytes!("../assets/LilexNerdFontMono-Regular.ttf"));
    fonts.font_data.insert("lilex".to_owned(), font_data.into());
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "lilex".to_owned());
    fonts
}
