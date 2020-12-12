use quicksilver::{
    geom::Vector,
    graphics::{Color, VectorFont},
    input::{Event, Key},
    Graphics, Input, Result, Timer, Window,
};

use crate::structs::game_data::{GameData, FG_COLOR};
use crate::systems::scene_mgr::SceneManager;

pub struct GameLoop {
    running: bool,
    window: Window,
    gfx: Graphics,
    input: Input,
    gd: GameData,
    scene_manager: SceneManager,
}

impl GameLoop {
    pub fn new(window: Window, gfx: Graphics, input: Input) -> GameLoop {
        GameLoop {
            running: true,
            window: window,
            gfx: gfx,
            input: input,
            gd: GameData {
                mouse_pos: Vector::new(0., 0.),
                last_mouse_pos: Vector::new(0., 0.),
                timer: Timer::time_per_second(60.),
            },
            scene_manager: SceneManager::new(),
        }
    }

    pub fn init(mut self) -> Self {
        //*~*~*~todo*~*~*~*~*~*~*~*~*~
        // ☐ start network stuff
        // ☐ init asset cacher
        // ☐ load initial assets
        // ☑ init scene manager
        // *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~

        self.scene_manager.init();

        self
    }

    pub async fn run(&mut self) -> Result<()> {
        let ttf = VectorFont::load("NASDAQER.ttf").await?;
        let mut font = ttf.to_renderer(&mut self.gfx, 72.)?;

        while self.running {
            self.handle_input().await;
            self.gfx.clear(Color::BLACK);

            self.scene_manager
                .draw_scene(&self.gd, &mut self.gfx, &self.window);

            font.draw(&mut self.gfx, "THE NET", FG_COLOR, Vector::new(500., 500.))?;

            match self.gfx.present(&self.window) {
                Ok(_) => {}
                Err(e) => println!("err {}", e), // ☒ ~~add logger~~ use quicksilver's logger
                                                 // ☒ maybe crash program at this point?
                                                 //     - why would it give an error?
            }
        }

        Ok(())
    }

    async fn handle_input(&mut self) {
        while let Some(event) = self.input.next_event().await {
            match event {
                Event::KeyboardInput(k) if k.is_down() => {
                    if k.key() == Key::Escape {
                        self.running = false;
                    }
                    #[cfg(debug_assertions)]
                    println!("{:?}", k.key());

                    //#todo: pull in config for key mappings
                    // for c in config
                    // if e.key is c.Key
                    // c.action(e, &mut self.gd)
                }
                Event::ReceivedCharacter(_c) => {}
                _ => {}
            }
        }
        self.gd.set_mouse_pos(self.input.mouse().location());
        #[cfg(debug_assertions)]
        self.gd.print();
    }
}
