/*
    Note: Minim font created by Raul Montala. 

    TODO:   Allow screen resizing
            Clean code to look better
            Make input easier
            Make this a web-tool.
*/

use ggez::{
    ContextBuilder,
    GameResult,
    Context,

    event,
    graphics,
    input::mouse::{self, MouseButton},
    nalgebra as alg,
};

const CELL_ON_COLOR:  graphics::Color = graphics::Color::new(1.0, 1.0, 1.0, 1.0);
const CELL_OFF_COLOR: graphics::Color = graphics::Color::new(0.15, 0.15, 0.15, 1.0);

const GRID_RATIO: f32 = 0.9;

#[derive(Default, Clone, Copy)]
struct Cell {
    is_active: bool,
}

struct GameState {
    cells: [Cell; 16],

    cell_distance: f32,
    cell_size: f32,
    gap_size: f32,

    screen_mesh: Option<graphics::Mesh>,
    screen_text: Option<graphics::Text>,
    text_font: graphics::Font,

    mouse_held: bool,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let (size_x, size_y) = graphics::drawable_size(ctx);

        let min_size = if size_x < size_y {
            size_x * 0.9
        } else {
            size_y * 0.9
        };

        let cell_distance = (min_size / 4.0).floor();
        let cell_size = (cell_distance * GRID_RATIO).floor();
        let gap_size = cell_distance - cell_size;

        //have to use this workaround because graphics::Font::new(); sucks. 
        let text_font = {
            use std::fs::File;
            use std::io::prelude::*;
            
            let mut f = File::open("resources/font.ttf")?;
            let mut buffer = vec!();

            f.read_to_end(&mut buffer)?;

            graphics::Font::new_glyph_font_bytes(ctx, &buffer)?
        };

        Ok(GameState {
            cells: [Cell::default(); 16],
            
            cell_distance,
            cell_size,
            gap_size,

            screen_mesh: None,
            screen_text: None,
            text_font,

            mouse_held: false,
        })
    }

    fn get_cell_at_position(&self, pos: (f32, f32)) -> Option<usize> {
        let position_in_cell = (pos.0 % self.cell_distance, pos.1 % self.cell_distance);
        if position_in_cell.0 >= self.gap_size 
        && position_in_cell.0 <= self.cell_size + self.gap_size
        && position_in_cell.1 >= self.gap_size 
        && position_in_cell.1 <= self.cell_size + self.gap_size
        {
            let cell_position = ((pos.0 / self.cell_distance).floor(), (pos.1 / self.cell_distance).floor());
            if cell_position.0 < 4.0 && cell_position.1 < 4.0 {
                return Some((cell_position.1 * 4.0 + cell_position.0) as usize);
            }
        }

        None
    }

    fn get_decimal_value(&mut self) -> u16 {
        let mut value = 0;
        for i in 0..16 {
            if self.cells[i].is_active {
                value += (2u16).pow((15 - i) as u32); //TODO: Find out why this has to be u32?
            }
        }
        
        value
    }

    fn update_screen(&mut self, ctx: &mut Context) -> GameResult {
        let mut frame_mesh_builder = graphics::MeshBuilder::new();
        let mut text = String::new();

        for y in (0..4).map(|a| a as f32) {
            for x in (0..4).map(|a| a as f32) {
                //build text representation
                let cell_num = (4.0 * y + x) as usize;
                let cell_is_active = self.cells[cell_num].is_active;
                text.push(match cell_is_active {
                    true => '1',
                    false => '0',
                });
                if (cell_num + 1) % 4 == 0 {
                    text.push(' ');
                }
                
                //build squares
                let cell_rect = graphics::Rect::new(
                    self.gap_size + self.cell_distance * x, 
                    self.gap_size + self.cell_distance * y, 
                    self.cell_size, 
                    self.cell_size
                );
                frame_mesh_builder.rectangle(
                    graphics::DrawMode::fill(), 
                    cell_rect, 
                    match cell_is_active {
                        true => CELL_ON_COLOR,
                        false => CELL_OFF_COLOR,
                    }    
                );
            }
        }

        self.screen_mesh = Some(frame_mesh_builder.build(ctx)?);

        text.push_str(&format!(" = {}", self.get_decimal_value()));
        let mut screen_text = graphics::Text::new(text);
        screen_text.set_font(self.text_font, graphics::Scale::uniform(25.0));

        self.screen_text = Some(screen_text);

        Ok(())
    }
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if mouse::button_pressed(ctx, MouseButton::Left) {
            if !self.mouse_held {
                let pos = mouse::position(ctx);
                if let Some(id) = self.get_cell_at_position((pos.x, pos.y)) {
                    self.cells[id].is_active = !self.cells[id].is_active;
                    self.update_screen(ctx)?;
                }

                self.mouse_held = true;
            }
        } else {
            self.mouse_held = false;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        //TODO: make it so this doesn't break everything just because the GameState hasn't been initialized
        //      with update_screen().
        graphics::draw(ctx, self.screen_mesh.as_ref().unwrap(), graphics::DrawParam::default())?;
        graphics::draw(
            ctx, 
            self.screen_text.as_ref().unwrap(), 
            graphics::DrawParam::default().dest(alg::Point2::new(self.gap_size, self.gap_size + self.cell_distance * 4.0))
        )?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Grid Value", "Me")   
        .build()?;
    graphics::set_window_title(&ctx, "Grid Value");

    let mut game_state = GameState::new(&mut ctx)?;
    game_state.update_screen(&mut ctx)?;

    event::run(&mut ctx, &mut event_loop, &mut game_state)?;

    Ok(())
}
