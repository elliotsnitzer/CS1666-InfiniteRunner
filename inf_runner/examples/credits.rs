extern crate inf_runner;

use sdl2::event::Event;
use sdl2::pixels::Color;
// use sdl2::rect::Point;
use sdl2::rect::Rect;
// use sdl2::render::BlendMode;
use sdl2::render::TextureQuery;

use inf_runner::Game;
use inf_runner::SDLCore;

const TITLE: &str = "Credit scene - Dane Halle";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

pub struct Credits {
    core: SDLCore,
}

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

impl Game for Credits {
    fn init() -> Result<Self, String> {
        let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
        Ok(Credits { core })
    }

    fn run(&mut self) -> Result<(), String> {
        let mut count = CAM_H;

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let mut font = ttf_context.load_font("./assets/DroidSansMono.ttf", 128)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let texture_creator = self.core.wincan.texture_creator();

        let surface = font
            .render("Dane Halle - Debug Guru")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let texture_michael = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let surface = font
            .render("Caleb Kessler")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let texture_dane = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let surface = font
            .render("Andrew Wiesen")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let texture_caleb = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let surface = font
            .render("Benjamin Ungar")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let texture_andrew = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let surface = font
            .render("Dominic Karras")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let texture_benjamin = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let surface = font
            .render("Mateen Kasim")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let texture_dominic = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let surface = font
            .render("Elliot Snitzer")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let texture_Mateen = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let surface = font
            .render("Michael Daley")
            .blended(Color::RGBA(119, 3, 252, 255))
            .map_err(|e| e.to_string())?;
        let texture_elliot = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let team = [
            texture_dane,
            texture_caleb,
            texture_andrew,
            texture_benjamin,
            texture_dominic,
            texture_Mateen,
            texture_elliot,
            texture_michael,
        ];

        let mut index = 0;

        'gameloop: loop {
            for event in self.core.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'gameloop,
                    _ => {}
                }
            }
            count = self.credit_demo_text(&count, &team[index])?;
            if count == 0 {
                count = CAM_H;
                index += 1;
                if index == team.len() {
                    index = 0;
                }
            } else {
                continue;
            }
        }
        Ok(())
    }
}

impl Credits {
    fn credit_demo_text(
        &mut self,
        count: &u32,
        texture: &sdl2::render::Texture,
    ) -> Result<u32, String> {
        let m_count = count - 1;

        self.core
            .wincan
            .set_draw_color(Color::RGBA(3, 252, 206, 255));
        self.core.wincan.clear();

        let TextureQuery { width, height, .. } = texture.query();

        let padding = 64;

        let wr = width as f32 / (CAM_W - padding) as f32;
        let hr = height as f32 / (CAM_H - padding) as f32;

        let (w, h) = if wr > 1f32 || hr > 1f32 {
            if wr > hr {
                let h = (height as f32 / wr) as i32;
                ((CAM_W - padding) as i32, h)
            } else {
                let w = (width as f32 / hr) as i32;
                (w, (CAM_H - padding) as i32)
            }
        } else {
            (width as i32, height as i32)
        };

        let cx = (CAM_W as i32 - w) / 2;

        self.core
            .wincan
            .copy(&texture, None, Some(rect!(cx, m_count, w, h)));
        self.core.wincan.present();

        Ok(m_count)
    }
}

fn main() {
    inf_runner::runner(TITLE, Credits::init);
}