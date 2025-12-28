use crate::sound_gen::WavGenerator;
use macroquad::audio::{Sound, load_sound_from_bytes};
use macroquad::prelude::*;

// Embed assets at compile time
const FONT_DATA: &[u8] = include_bytes!("../assets/font/kenny_future.ttf");


// UI Assets
const BAR_BG: &[u8] = include_bytes!("../assets/PNG/Grey/Default/bar_round_large.png");
const BAR_RED: &[u8] = include_bytes!("../assets/PNG/Red/Default/bar_round_large.png");
const BAR_YELLOW: &[u8] = include_bytes!("../assets/PNG/Yellow/Default/bar_round_large.png");

// Character Bodies
const BODY_BLUE: &[u8] = include_bytes!("../assets/characters/PNG/Default/blue_body_squircle.png");
const BODY_GREEN: &[u8] =
    include_bytes!("../assets/characters/PNG/Default/green_body_squircle.png");
const BODY_PINK: &[u8] = include_bytes!("../assets/characters/PNG/Default/pink_body_squircle.png");
const BODY_PURPLE: &[u8] =
    include_bytes!("../assets/characters/PNG/Default/purple_body_squircle.png");
const BODY_RED: &[u8] = include_bytes!("../assets/characters/PNG/Default/red_body_squircle.png");
const BODY_YELLOW_C: &[u8] =
    include_bytes!("../assets/characters/PNG/Default/yellow_body_squircle.png");
const BODY_BLUE_R: &[u8] = include_bytes!("../assets/characters/PNG/Default/blue_body_rhombus.png");
const BODY_GREEN_R: &[u8] =
    include_bytes!("../assets/characters/PNG/Default/green_body_rhombus.png");
const BODY_PINK_R: &[u8] = include_bytes!("../assets/characters/PNG/Default/pink_body_rhombus.png");
const BODY_PURPLE_R: &[u8] =
    include_bytes!("../assets/characters/PNG/Default/purple_body_rhombus.png");
const BODY_RED_R: &[u8] = include_bytes!("../assets/characters/PNG/Default/red_body_rhombus.png");
const BODY_YELLOW_R: &[u8] =
    include_bytes!("../assets/characters/PNG/Default/yellow_body_rhombus.png");

pub struct GameAssets {
    pub font: Font,
  
    pub bar_bg: Texture2D,
    pub bar_red: Texture2D,
    pub bar_yellow: Texture2D,

    pub char_bodies: Vec<Texture2D>,

    // Sounds
    pub snd_click: Sound,
    pub snd_start: Sound,
    pub snd_over: Sound,
}

impl GameAssets {
    pub async fn load() -> Self {
        let font = load_ttf_font_from_bytes(FONT_DATA).expect("Failed to load font");
       
        let bar_bg = Texture2D::from_file_with_format(BAR_BG, Some(ImageFormat::Png));
        let bar_red = Texture2D::from_file_with_format(BAR_RED, Some(ImageFormat::Png));
        let bar_yellow = Texture2D::from_file_with_format(BAR_YELLOW, Some(ImageFormat::Png));

        // Procedural Sound Generation
        let snd_click = load_sound_from_bytes(&WavGenerator::generate_beep(1000.0, 0.05, 0.5))
            .await
            .expect("Failed click gen");
        let snd_start = load_sound_from_bytes(&WavGenerator::generate_beep(600.0, 0.5, 0.5))
            .await
            .expect("Failed start gen");
        let snd_over = load_sound_from_bytes(&WavGenerator::generate_beep(300.0, 0.8, 0.5))
            .await
            .expect("Failed over gen");

        let char_bodies = vec![
            Texture2D::from_file_with_format(BODY_BLUE, Some(ImageFormat::Png)),
            Texture2D::from_file_with_format(BODY_GREEN, Some(ImageFormat::Png)),
            Texture2D::from_file_with_format(BODY_PINK, Some(ImageFormat::Png)),
            Texture2D::from_file_with_format(BODY_PURPLE, Some(ImageFormat::Png)),
            Texture2D::from_file_with_format(BODY_RED, Some(ImageFormat::Png)),
            Texture2D::from_file_with_format(BODY_YELLOW_C, Some(ImageFormat::Png)),
            Texture2D::from_file_with_format(BODY_BLUE_R, Some(ImageFormat::Png)),
            Texture2D::from_file_with_format(BODY_GREEN_R, Some(ImageFormat::Png)),
            Texture2D::from_file_with_format(BODY_PINK_R, Some(ImageFormat::Png)),
            Texture2D::from_file_with_format(BODY_PURPLE_R, Some(ImageFormat::Png)),
            Texture2D::from_file_with_format(BODY_RED_R, Some(ImageFormat::Png)),
            Texture2D::from_file_with_format(BODY_YELLOW_R, Some(ImageFormat::Png)),
        ];

        Self {
            font,
           
            bar_bg,
            bar_red,
            bar_yellow,
            char_bodies,
            snd_click,
            snd_start,
            snd_over,
        }
    }
}
