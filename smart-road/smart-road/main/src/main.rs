// Importation du module vehicule.rs
mod vehicule;
use vehicule::*;

// importation des modules sdl2 nécessaires 
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
//use std::num::ParseIntError;
use std::time::Duration;
use rand::Rng;
use std::path::Path;


fn main() {
    // initialisation du contexte sdl
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // création d'une fenetre  avec le sous-système vidéo sdl 
    let window = video_subsystem
        .window("Voiture de trois", 1200, 1000)
        .position_centered()
        .build()
        .unwrap();

    // création d'un canvas pour dessiner dans la fenetre 
    let mut canvas = window.into_canvas().build().unwrap();

    // configuration de la couleur de fond du canvas 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // initialisation de la simulation 
    let mut rect: Vec<Vehicule> = Vec::new(); // vecteur de véhicules
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut cooldown_spawn = false;
    let mut cooldown_time = 0;

    let autres_vehicules: Vec<Vehicule> = Vec::new();

    let mut lowest_vehicule = Vehicule {
        rect: Rect::new(0, 0, 0, 10),
        from: Dir::East,
        to: Dir::North,
        speed: 3,
        time: 0,
        actual_dir: Dir::East,
        color: Color{r:0,g:0,b:0,a:0},
        stopped: true,
    };
    
    let mut speedest_vehicule = Vehicule {
        rect: Rect::new(0, 0, 0, 10),
        from: Dir::East,
        to: Dir::North,
        speed: 1,
        time: 999999,
        actual_dir: Dir::East,
        color: Color{r:0,g:0,b:0,a:0},
        stopped: true,
    };
    let texture_creator = canvas.texture_creator();
    let mut echap_first_press = false;
    let font_path: &Path = Path::new("./src/font/PKMN_RBYGSC.ttf");
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let mut font = ttf_context.load_font(font_path, 128).unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    // boucle principale du programme 
    'running: loop {
        
        rect.retain(|vehicule: &Vehicule| vehicule.est_dans_rectangle(&mut lowest_vehicule, &mut speedest_vehicule));
        // gestion du cooldown pour limiter la creation de véhicule
        if cooldown_spawn {
            cooldown_time += 1;
            if cooldown_time >= 40 {
                cooldown_spawn = false;
                cooldown_time = 0;
            }

        }

        
        
        // gestion des événements (clavier)
        for event in event_pump.poll_iter() {
            // (creation de nouveau vehicules en fonction des touches pressées)
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if !cooldown_spawn {
                        rect.push(Vehicule::new(Dir::West, random(Dir::West), rand::thread_rng().gen_range(1..=3)));
                        cooldown_spawn = true;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if !cooldown_spawn {
                        rect.push(Vehicule::new(Dir::East, random(Dir::East), rand::thread_rng().gen_range(1..=3)));
                        cooldown_spawn = true;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if !cooldown_spawn {
                        rect.push(Vehicule::new(Dir::North, random(Dir::North), rand::thread_rng().gen_range(1..=3)));
                        cooldown_spawn = true;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if !cooldown_spawn {
                        rect.push(Vehicule::new(Dir::South, random(Dir::South), rand::thread_rng().gen_range(1..=3)));
                        cooldown_spawn = true;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    if !cooldown_spawn {
                        let dir = true_random();
                        rect.push(Vehicule::new(dir, random(dir), rand::thread_rng().gen_range(1..=3)));
                        cooldown_spawn = true;
                    }
                }
                
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    if !echap_first_press{
                        write_stats(
                            &font,
                            &texture_creator,
                            &mut canvas,
                            speedest_vehicule.time,
                            lowest_vehicule.time
                        );
                        echap_first_press = true;
                    } else {
                        break 'running
                    }
                }
                _ => {}
            }
        }
        if !echap_first_press {
                    // Effacement de l'ecran
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        // Dessin de la route 
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        draw_road(&mut canvas);
        
        let mut car_x: Vec<i32> = Vec::new();
        let mut car_y: Vec<i32> = Vec::new();
        
        //gestion des vehicule
        for vehicule in &mut rect {
            vehicule.time += 1;
  
            print!("{}, ", vehicule.speed);
            car_x.push(vehicule.rect.x);
            car_y.push(vehicule.rect.y);
            vehicule.motor(&autres_vehicules);
            match vehicule.from {
                Dir::North => {
                    if vehicule.to == Dir::East && vehicule.rect.y >= 510 && vehicule.rect.y <= 517 {
                        vehicule.actual_dir = Dir::East;
                    } else if vehicule.to == Dir::West && vehicule.rect.y >= 410 && vehicule.rect.y <= 417 {
                        vehicule.actual_dir = Dir::West;
                    } else if vehicule.to == Dir::South && vehicule.rect.y >= 547 && vehicule.rect.y <= 584 {
                        vehicule.actual_dir = Dir::South;
                    }
                }
                
                Dir::South => {
                    if vehicule.to == Dir::East && vehicule.rect.y >= 570 && vehicule.rect.y <= 577 {
                        vehicule.actual_dir = Dir::East;
                    } else if vehicule.to == Dir::West && vehicule.rect.y >= 477 && vehicule.rect.y <= 481 {
                        vehicule.actual_dir = Dir::West;
                    } else if vehicule.to == Dir::North && vehicule.rect.y >= 647 && vehicule.rect.y <= 655 {
                        vehicule.actual_dir= Dir::North;
                    }
                }
                
                Dir::West => {
                    if vehicule.to == Dir::North && vehicule.rect.x >= 613 && vehicule.rect.x <= 620 {
                        vehicule.actual_dir = Dir::North;
                    } else if vehicule.to == Dir::South && vehicule.rect.x >= 510 && vehicule.rect.x <= 517{
                        vehicule.actual_dir = Dir::South;
                    } else if vehicule.to == Dir::East && vehicule.rect.x >= 513 && vehicule.rect.x <= 520 {
                        vehicule.actual_dir= Dir::East;
                    }
                }
                
                Dir::East => {
                    if vehicule.to == Dir::North && vehicule.rect.x >= 677 && vehicule.rect.x <= 684{
                        vehicule.actual_dir = Dir::North;
                    } else if vehicule.to == Dir::South && vehicule.rect.x >= 577 && vehicule.rect.x <= 584{
                        vehicule.actual_dir = Dir::South;
                    } else if vehicule.to == Dir::West && vehicule.rect.x >= 513 && vehicule.rect.x <= 520{
                        vehicule.actual_dir = Dir::West;
                    }
                }
            }
            // dessin du vehicule dur le canvas 
            canvas.set_draw_color(vehicule.color);
            canvas.fill_rect(vehicule.rect).unwrap();
            
        }
        print!("\n");
        }

        
        // pause pour ralentir la boucle principale 
        std::thread::sleep(Duration::from_millis(8));
        
        // rafrachissement de l'ecran 
        canvas.present();
        
    }
}