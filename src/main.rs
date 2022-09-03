use eframe::{egui::{self}, egui::{Frame, style::Margin, TextureFilter}, epaint::{Color32, ColorImage, ImageData, Mesh, Rect, Pos2}};
use tiny_skia::{PixmapPaint, Transform};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    image: ColorImage,
    texture: Option<egui::TextureHandle>,
}

impl Default for MyApp {
    fn default() -> Self {
        let bytes = include_bytes!("c98-98.svg");
        let ci = load_svg_bytes(bytes, 0.1).unwrap();

        Self {
            image: ci,
            texture: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.texture.is_none() {
            let texture = ctx.load_texture("image", ImageData::Color(self.image.clone()), TextureFilter::Linear);
            self.texture = Some(texture);
        }

        egui::CentralPanel::default()
        .frame(
            Frame::default()
            .fill(Color32::LIGHT_GRAY)
            .inner_margin(Margin {top: 6., bottom: 6., left: 0., right: 0.})
        )
        .show(&ctx, |ui| {
            let t = self.texture.as_ref().unwrap();
            let mut mesh = Mesh::with_texture(self.texture.as_ref().unwrap().id());
            mesh.add_rect_with_uv(Rect::from_min_size(Pos2::new(100.0, 100.0), t.size_vec2()), Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::BLACK);
            ui.painter().add(mesh);
        });
    }        
}

pub fn load_svg_bytes(svg_bytes: &[u8], scale: f32) -> Result<egui::ColorImage, String> {
    let mut opt = usvg::Options::default();
    opt.fontdb.load_system_fonts();

    let rtree = usvg::Tree::from_data(svg_bytes, &opt.to_ref()).map_err(|err| err.to_string())?;

    let pixmap_size = rtree.svg_node().size.to_screen_size();
    let [w, h] = [pixmap_size.width(), pixmap_size.height()];

    let mut pixmap = tiny_skia::Pixmap::new(w, h)
        .ok_or_else(|| format!("Failed to create SVG Pixmap of size {}x{}", w, h))?;

    resvg::render(
        &rtree,
        usvg::FitTo::Original,
        Default::default(),
        pixmap.as_mut(),
    )
    .ok_or_else(|| "Failed to render SVG".to_owned())?;

    let mut scaled_pixmap = tiny_skia::Pixmap::new(((w as f32) * scale) as u32, ((h as f32) * scale) as u32).unwrap();
    scaled_pixmap.draw_pixmap(0, 0, pixmap.as_ref(), &PixmapPaint::default(), Transform::from_scale(0.1, 0.1), None);

    let image = egui::ColorImage::from_rgba_unmultiplied(
        [scaled_pixmap.width() as _, scaled_pixmap.height() as _],
        scaled_pixmap.data(),
    );

    Ok(image)
}
