// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;

// Define a global ParticleSystem instance wrapped in a Mutex
lazy_static! {
    static ref PARTICLE_SYSTEM: Mutex<ParticleSystem> = Mutex::new(ParticleSystem::new(800, 600));
}

#[derive(Debug, Clone, Serialize)]
struct Particle {
    x: i32,
    y: i32,
    vx: f64,
    vy: f64,
    color: String
}

impl Particle {
    fn new(x: i32, y: i32, color: String) -> Self {
        Particle {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            color
        }
    }
}

struct ParticleSystem {
    particles: HashMap<String, Vec<Particle>>,
    rules: HashMap<String, HashMap<String, f64>>,
    width: i32,
    height: i32
}


impl ParticleSystem {
    fn new(width: i32, height: i32) -> Self {
        ParticleSystem {
            particles: HashMap::new(),
            rules: HashMap::new(),
            width,
            height
        }
    }

    fn create_particles(&mut self, num: usize, color: &str) {
        // assume that color is not a part of the particle system yet, so we want a new Vec group to store particles of that color
        let mut rng = rand::thread_rng();
        let mut group: Vec<Particle> = Vec::new();

        for _ in 0..num {
            let x = self.rand_x(&mut rng);
            let y = self.rand_y(&mut rng);
            let particle = Particle::new(x, y, color.to_string());
            group.push(particle);
        }

        self.particles.insert(color.to_string(), group);
    }

    fn create_rules(&mut self) {
        // nested for loop over all keys of self.partcles
        // for each, call rand_rule and store
        let mut rng = rand::thread_rng();

        for key1 in self.particles.keys() {
            let mut key_dict: HashMap<String, f64> = HashMap::new();

            for key2 in self.particles.keys() {
                let rule_val = ParticleSystem::rand_rule(&mut rng);
                key_dict.insert(key2.to_string(), rule_val);
            }

            self.rules.insert(key1.to_string(), key_dict);
        }
    }

    fn rand_x(&self, rng: &mut impl Rng) -> i32 {
        rng.gen_range(0..self.width)
    }

    fn rand_y(&self, rng: &mut impl Rng) -> i32 {
        rng.gen_range(0..self.height)
    }

    fn rand_rule(rng: &mut impl Rng) -> f64 {
        rng.gen_range(-1.0..1.0)
    }

    fn get_particles(&self) -> HashMap<String, Vec<Particle>> {
        self.particles.clone()
    }

    fn apply_rules(&mut self) {
        let window_size = self.width as f64;
        let p = self.particles.clone();
    
        for (_, group_a) in &mut self.particles {
            for a in group_a {
                let mut fx = 0.0;
                let mut fy = 0.0;
    
                for (_, group_b) in &p {
                    for b in group_b {
                        let g = self.rules[&a.color.to_string()][&b.color.to_string()];
                        let dx = a.x - b.x;
                        let dy = a.y - b.y;
                        let d = ((dx * dx + dy * dy) as f64).sqrt();
    
                        if d > 0.0 && d < 80.0 {
                            let F = g / d;
                            fx += F * dx as f64;
                            fy += F * dy as f64;
                        }
                    }
                }
    
                a.vx = (a.vx + fx) * 0.5;
                a.vy = (a.vy + fy) * 0.5;
    
                // Update position
                a.x += a.vx as i32;
                a.y += a.vy as i32;
    
                // Handle boundary conditions
                if a.x <= 0 || a.x >= window_size as i32 {
                    a.vx *= -1.0;
                }
                if a.y <= 0 || a.y >= window_size as i32 {
                    a.vy *= -1.0;
                }
            }
        }
    }
    
    
    
}

#[tauri::command]
fn init_particles() -> HashMap<String, Vec<Particle>> {
    // println!("Initializing particles...");

    let mut particle_system = PARTICLE_SYSTEM.lock().unwrap();
    
    // let mut particle_system = ParticleSystem::new(800, 600);

    // let colors = ["red", "blue", "green", "yellow"];
    // let num_atoms = 200;
    // for color in colors {
    //     println!("Creating particles for color: {}", color);
    //     particle_system.create_particles(num_atoms, color);
    // }
    // particle_system.create_rules();

    if particle_system.particles.is_empty() {
        let colors = ["red", "blue", "green", "yellow"];
        let num_atoms = 50;
        for color in colors {
            // println!("Creating particles for color: {}", color);
            particle_system.create_particles(num_atoms, color);
        }
        particle_system.create_rules();
    }


    particle_system.get_particles()
}

#[tauri::command]
fn update_particles() -> HashMap<String, Vec<Particle>> {

    // println!("got into update system");

    // Lock the global ParticleSystem instance
    let mut particle_system = PARTICLE_SYSTEM.lock().unwrap();

    // println!("got particle system");

    // Apply rules to update particles' positions and velocities
    particle_system.apply_rules();

    // println!("applied rules");

    // Return the updated particles
    particle_system.get_particles()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![init_particles, update_particles])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}