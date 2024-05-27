pub use sdl2::rect::Rect;
use sdl2::pixels::Color;
use rand::Rng;
pub use sdl2::rect::Point;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

//use crate::vehicule;

// Direction possible pour les véhicules
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Dir {
    North,
    South,
    West,
    East,
}

// Fonction pour obtenir une aléatoire à partir d'une direction donnée
pub fn random(from: Dir) -> Dir {
    let all: [Dir; 3] = match from {
        Dir::East => [Dir::North, Dir::South, Dir::West],
        Dir::West => [Dir::North, Dir::South, Dir::East],
        Dir::North => [Dir::East, Dir::South, Dir::West],
        Dir::South => [Dir::North, Dir::East, Dir::West],
    };
    all[(rand::thread_rng().gen_range(0..3)) as usize]
}

// Fonction pour obtenir une direction aléatoire sans contrainte
pub fn true_random() -> Dir {
    let all: [Dir; 4] = [Dir::East, Dir::North, Dir::South, Dir::West];
    all[(rand::thread_rng().gen_range(0..4)) as usize]
}

// Structure représentant un véhicule 
#[derive(Debug, Eq, PartialEq,Clone)]
pub struct Vehicule {
    pub rect: Rect,
    pub from: Dir,
    pub to: Dir,
    pub speed: u8,
    pub time: u64,
    pub actual_dir: Dir,
    pub color: Color,
    pub stopped: bool,
}

// Définition de la structure Vehicule
impl Vehicule {
    
    pub fn est_dans_rectangle(&self, lowest_vehicule: &mut Vehicule, speedest_vehicule: &mut Vehicule) -> bool {
       if self.rect.x >= 0 && self.rect.x <= 1200 && self.rect.y >= 0 && self.rect.y <= 1000 {
        if self.time >= lowest_vehicule.time {
            *lowest_vehicule = self.clone();
        }
        if self.time <= speedest_vehicule.time {
            *speedest_vehicule = self.clone();
        }
        true
        } else {
        false
       }
    }
    // Constructeur pour créer une nouvelle instance de Vehicule
    pub fn new(from: Dir, to: Dir, speed: u8) -> Vehicule {
        // Déclaration de la variable pour stocker la couleur de virage
        let turn_color: Color;

        // Détermination de la couleur de virage en fonction des directions from et to
        if (from == Dir::South && to == Dir::East)
            || (from == Dir::East && to == Dir::North)
            || (from == Dir::North && to == Dir::West)
            || (from == Dir::West && to == Dir::South)
        {
            turn_color = Color::RGB(0, 0, 255);  // Bleu
        } else if (from == Dir::South && to == Dir::West)
            || (from == Dir::West && to == Dir::North)
            || (from == Dir::North && to == Dir::East)
            || (from == Dir::East && to == Dir::South)
        {
            turn_color = Color::RGB(128, 0, 128);  // Violet
        } else {
            turn_color = Color::RGB(0, 255, 0);  // Vert
        }


        

        // Création d'une instance de Vehicule en utilisant match pour définir les propriétés initiales en fonction de la direction from
        match from {
            // Si la direction `from` est `Dir::North`
            Dir::North => {
                let x = match to {
                    Dir::North => 0,
                    Dir::West => 513,
                    Dir::East => 577,
                    Dir::South => 547,
                };
                // Crée une nouvelle instance de Vehicule avec des propriétés spécifiques
                Vehicule {
                rect: Rect::new(x, 0, 10, 10),
                from: from, 
                to: to,
                speed: speed,
                time: 0,
                color: turn_color,
                actual_dir: Dir::South,  // La direction actuelle du véhicule est définie comme étant vers le Sud
                stopped: false,
                }
            },

            // Si la direction `from` est `Dir::South`
            Dir::South => {
                let x = match to {
                    Dir::South => 0,
                    Dir::West => 613,
                    Dir::East => 677,
                    Dir::North => 647,
                };
                // Crée une nouvelle instance de Vehicule avec des propriétés spécifiques
                Vehicule {
                rect: Rect::new(x, 1000, 10, 10),
                from: from,
                to: to,
                speed: speed,
                time: 0,
                color: turn_color,
                actual_dir: Dir::North,  // La direction actuelle du véhicule est définie comme étant vers le Nord
                stopped: false,
                }
            },

            // Si la direction `from` est `Dir::West`
            Dir::West => {
                    let y = match to {
                        Dir::South => 577,
                        Dir::West => 0,
                        Dir::East => 547,
                        Dir::North => 513,
                    };
                // Crée une nouvelle instance de Vehicule avec des propriétés spécifiques
                Vehicule {
                rect: Rect::new(0, y, 10, 10),
                from: from,
                to: to,
                speed: speed,
                time: 0,
                color: turn_color,
                actual_dir: Dir::East,  // La direction actuelle du véhicule est définie comme étant vers l'Est
                stopped: false,
                }
            }, 

            // Si la direction `from` est `Dir::East`
            Dir::East => {
                let y = match to {
                    Dir::East => 0,
                    Dir::North => 413,
                    Dir::South => 477,
                    Dir::West => 447,
                };
                // Crée une nouvelle instance de Vehicule avec des propriétés spécifiques
                Vehicule {
                rect: Rect::new(1150, y, 10, 10),
                from: from,
                to: to,
                speed: speed,
                time: 0,
                color: turn_color,
                actual_dir: Dir::West,  // La direction actuelle du véhicule est définie comme étant vers l'Ouest
                stopped: false,
                }
            },
        }
    }

    // Méthode pour simuler le mouvement du véhicule en fonction de sa direction actuelle
    pub fn motor(&mut self, other_vehicles: &[Vehicule]) {
        // verifie la collision avec d'autres vehicules
        for other_vehicle in other_vehicles {
            if self.collides_with(other_vehicle) {
                //verifie si le vehicule approchant vient de la droite
                if self.is_approaching_from_right(other_vehicle){
                    self.stopped = true;
                    return;
                }
            }
        }

        if  !self.stopped {
            self.manage_traffic(other_vehicles);
            // Met à jour la position du véhicule en fonction de sa direction
            match self.actual_dir {
                Dir::East => self.rect.x += self.speed as i32,
                Dir::West => self.rect.x -= self.speed as i32,
                Dir::North => self.rect.y -= self.speed as i32,
                Dir::South => self.rect.y += self.speed as i32,
            }
        }
    }

    pub fn is_approaching_from_right(&self, other: &Vehicule) -> bool {
        match self.actual_dir {
            Dir::East => self.rect.x < other.rect.x,
            Dir::West => self.rect.x > other.rect.x,
            Dir::North | Dir::South => false, 
        }
    }

    pub fn collides_with(&self, other: &Vehicule) -> bool {    
        self.rect.x < (other.rect.x + other.rect.width() as i32)
            && (self.rect.x + self.rect.width() as i32) > other.rect.x
            && self.rect.y < (other.rect.y + other.rect.height() as i32)
            && (self.rect.y + self.rect.height() as i32) > other.rect.y
    }

    pub fn manage_traffic(&mut self, other_vehicles: &[Vehicule]) {
        for other in other_vehicles {
            if self.is_in_front_or_close(other) {
                self.decide_to_stop_or_go(other);
                self.adjust_speed(other_vehicles);
                return;
            }
        }
        self.stopped = false;
        self.adjust_speed(other_vehicles);
    }

    pub fn is_in_front_or_close(&mut self, other: &Vehicule) -> bool {
        // Implémente la logique pour déterminer si un véhicule est en face ou proche
        // Retourne true si le véhicule est en face ou proche, sinon false
        // Tu peux ajuster cette logique en fonction de tes besoins
        // La logique actuelle considère simplement que tout véhicule sur la même voie (direction) est en face
        match (self.actual_dir, other.actual_dir) {
            (Dir::North, Dir::South) => true,
            (Dir::South, Dir::North) => true,
            (Dir::West, Dir::East) => true,
            (Dir::East, Dir::West) => true,
            _ => false,
        }
    }

    // Méthode pour décider de s'arrêter ou de continuer en fonction de la situation
    pub fn decide_to_stop_or_go(&mut self, other: &Vehicule) {
          // Calcule la distance euclidienne entre les deux véhicules
          let distance = ((self.rect.x - other.rect.x).pow(2) + (self.rect.y - other.rect.y).pow(2)) as f64;
          let distance_threshold = 50.0; // Ajuste cette valeur selon tes besoins
  
          // Si la distance entre les deux véhicules est inférieure à la limite, s'arrête
          if distance < (distance_threshold as f64).powi(2) {
              self.stopped = true;
          } else {
              // Sinon, repart
              self.stopped = false;
          }
       }

    // Méthode pour réadapter la vitesse si nécessaire
    pub fn adjust_speed(&mut self, other: &[Vehicule]) {
          // Si la voiture est déjà arrêtée, ne rien faire
          if self.stopped {
            return;
        }

        // Vérifie s'il y a des véhicules en face
        let vehicles_in_front = other
            .iter()
            .any(|other| self.is_in_front_or_close(other));

        // Ajuste la vitesse en fonction de la présence de véhicules en face
        self.speed = if vehicles_in_front {
            1 // Si des véhicules sont en face, ajuste la vitesse à 1
        } else {
            6 // Sinon, maintient la vitesse à 3
        };
    }
}


// Cette fonction dessine une route avec des lignes continues et des pointillés.
pub fn draw_road(canvas: &mut Canvas<Window>) {
    // Définit la couleur de fond de la route en noir
    canvas.set_draw_color(Color::RGB(0, 0, 0));

    // Dessine un rectangle en haut de l'écran représentant la route
    canvas.fill_rect(Rect::new(500, 0, 200, 1000)).unwrap();
    
    // Dessine un rectangle à gauche de l'écran représentant la route
    canvas.fill_rect(Rect::new(0, 400, 1200, 200)).unwrap();

    // Change la couleur pour les lignes pointillées (blanc)
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    // Dessine des lignes pointillées verticales au centre de la route
    let mut dashed_line_y = 0;
    for _ in 0..14 {
        canvas
            .draw_line(Point::new(533, dashed_line_y), Point::new(533, dashed_line_y + 10))
            .unwrap();
        dashed_line_y += 30;
    }

    let mut dashed_line_y = 0;
    for _ in 0..14 {
        canvas
            .draw_line(Point::new(566, dashed_line_y), Point::new(566, dashed_line_y + 10))
            .unwrap();
        dashed_line_y += 30;
    }

    let mut dashed_line_y = 0;
    for _ in 0..14 {
        canvas
            .draw_line(Point::new(600, dashed_line_y), Point::new(600, dashed_line_y + 10))
            .unwrap();
        dashed_line_y += 30;
    }

    let mut dashed_line_y = 0;
    for _ in 0..14 {
        canvas
            .draw_line(Point::new(633, dashed_line_y), Point::new(633, dashed_line_y + 10))
            .unwrap();
        dashed_line_y += 30;
    }

    let mut dashed_line_y = 0;
    for _ in 0..14 {
        canvas
            .draw_line(Point::new(666, dashed_line_y), Point::new(666, dashed_line_y + 10))
            .unwrap();
        dashed_line_y += 30;
    }

    // Dessine deux lignes continues délimitant le côté gauche de la route
    canvas.draw_line(Point::new(500, 0), Point::new(500, 400)).unwrap();
    canvas.draw_line(Point::new(0, 400), Point::new(500, 400)).unwrap();

    // Réinitialise la position pour les lignes pointillées verticales inférieures
    dashed_line_y = 1000;
    for _ in 0..14 {
        canvas
            .draw_line(Point::new(533, dashed_line_y), Point::new(533, dashed_line_y - 10))
            .unwrap();
        dashed_line_y -= 30;
    }

    dashed_line_y = 1000;
    for _ in 0..14 {
        canvas
            .draw_line(Point::new(566, dashed_line_y), Point::new(566, dashed_line_y - 10))
            .unwrap();
        dashed_line_y -= 30;
    }

    dashed_line_y = 1000;
    for _ in 0..14 {
        canvas
            .draw_line(Point::new(600, dashed_line_y), Point::new(600, dashed_line_y - 10))
            .unwrap();
        dashed_line_y -= 30;
    }

    dashed_line_y = 1000;
    for _ in 0..14 {
        canvas
            .draw_line(Point::new(633, dashed_line_y), Point::new(633, dashed_line_y - 10))
            .unwrap();
        dashed_line_y -= 30;
    }

    dashed_line_y = 1000;
    for _ in 0..14 {
        canvas
            .draw_line(Point::new(666, dashed_line_y), Point::new(666, dashed_line_y - 10))
            .unwrap();
        dashed_line_y -= 30;
    }

    // Dessine deux lignes continues délimitant le côté droit de la route
    canvas.draw_line(Point::new(700, 600), Point::new(700, 1000)).unwrap();
    canvas.draw_line(Point::new(700, 600), Point::new(1200, 600)).unwrap();

    // Dessine des lignes pointillées horizontales au centre de la route
    // Initialise une variable mutable pour suivre la position en X de la ligne pointillée
    let mut dashed_line_x = 0;

    // Utilise une boucle for pour répéter le dessin de la ligne pointillée 13 fois
    for _ in 0..17 {
        // Dessine une ligne pointillée horizontale sur le canevas
        canvas
            .draw_line(Point::new(dashed_line_x, 433), Point::new(dashed_line_x + 10, 433))
            .unwrap();
        
        // Incrémente la position en X pour la prochaine ligne pointillée
        dashed_line_x += 30;
    }

    let mut dashed_line_x = 0;

    for _ in 0..17 {
        canvas
            .draw_line(Point::new(dashed_line_x, 466), Point::new(dashed_line_x + 10, 466))
            .unwrap();
        
        // Incrémente la position en X pour la prochaine ligne pointillée
        dashed_line_x += 30;
    }

    let mut dashed_line_x = 0;

    for _ in 0..17 {
        canvas
            .draw_line(Point::new(dashed_line_x, 500), Point::new(dashed_line_x + 10, 500))
            .unwrap();
        
        // Incrémente la position en X pour la prochaine ligne pointillée
        dashed_line_x += 30;
    }

    let mut dashed_line_x = 0;

    for _ in 0..17 {
        canvas
            .draw_line(Point::new(dashed_line_x, 533), Point::new(dashed_line_x + 10, 533))
            .unwrap();
        
        // Incrémente la position en X pour la prochaine ligne pointillée
        dashed_line_x += 30;
    }

    let mut dashed_line_x = 0;

    for _ in 0..17 {
        canvas
            .draw_line(Point::new(dashed_line_x, 566), Point::new(dashed_line_x + 10, 566))
            .unwrap();
        
        // Incrémente la position en X pour la prochaine ligne pointillée
        dashed_line_x += 30;
    }


    // Dessine deux lignes continues délimitant le bas de la route
    canvas.draw_line(Point::new(0, 600), Point::new(500, 600)).unwrap();
    canvas.draw_line(Point::new(500, 600), Point::new(500, 1000)).unwrap();

    // Réinitialise la position pour les lignes pointillées horizontales de droite à gauche
    dashed_line_x = 1200;
    for _ in 0..17 {
        canvas
            .draw_line(Point::new(dashed_line_x, 433), Point::new(dashed_line_x - 10, 433))
            .unwrap();
        dashed_line_x -= 30;
    }

    dashed_line_x = 1200;
    for _ in 0..17 {
        canvas
            .draw_line(Point::new(dashed_line_x, 466), Point::new(dashed_line_x - 10, 466))
            .unwrap();
        dashed_line_x -= 30;
    }

    dashed_line_x = 1200;
    for _ in 0..17 {
        canvas
            .draw_line(Point::new(dashed_line_x, 500), Point::new(dashed_line_x - 10, 500))
            .unwrap();
        dashed_line_x -= 30;
    }


    dashed_line_x = 1200;
    for _ in 0..17 {
        canvas
            .draw_line(Point::new(dashed_line_x, 533), Point::new(dashed_line_x - 10, 533))
            .unwrap();
        dashed_line_x -= 30;
    }

    dashed_line_x = 1200;
    for _ in 0..17 {
        canvas
            .draw_line(Point::new(dashed_line_x, 566), Point::new(dashed_line_x - 10, 566))
            .unwrap();
        dashed_line_x -= 30;
    }

    // Dessine deux lignes continues délimitant le côté supérieur de la route
    canvas.draw_line(Point::new(700, 400), Point::new(1200, 400)).unwrap();
    canvas.draw_line(Point::new(700, 400), Point::new(700, 0)).unwrap();
}

pub fn write_stats(font: &sdl2::ttf::Font,texture_creator: &TextureCreator<WindowContext>, canvas: &mut Canvas<Window>, time_speedest: u64, time_lowest: u64) {
    canvas.set_draw_color(Color::GRAY);
    let _ = canvas.fill_rect(Rect::new(0,0,1200,1000));
    render_text("Min speed : 1".to_string(), font, &texture_creator, canvas, 0, 0, 35);
    render_text("Max speed : 3".to_string(), font, &texture_creator, canvas, 0, 40, 35);
    render_text("Max time : 400".to_string(), font, texture_creator, canvas, 0, 80, 35);
    render_text("Min time : ".to_string() + time_lowest.to_string().as_str(), font, texture_creator, canvas, 0, 120, 35);
    render_text("Collisions : 0".to_string(), font, &texture_creator, canvas, 0, 160, 35);
}

fn render_text(text : String, font: &sdl2::ttf::Font,texture_creator: &TextureCreator<WindowContext>, canvas: &mut Canvas<Window> , x:i32,y:i32,height:u32) {
    let surface = font
        .render(&text)
        .blended(Color::RGBA(0, 0, 0, 255))
        .map_err(|e| e.to_string()).unwrap();
    let texture_font = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string()).unwrap();
let target = Rect::new(x,y,(height/2)*text.len()as u32,height);
    canvas.copy(&texture_font, None ,target).expect("work");

}