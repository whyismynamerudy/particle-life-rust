// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use rand::Rng;
use serde::{Serialize, Deserialize};
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
        let mut rng = rand::thread_rng();
        let keys: Vec<String> = self.particles.keys().cloned().collect();
    
        for key1 in &keys {
            let mut key_dict: HashMap<String, f64> = HashMap::new();
    
            for key2 in &keys {
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
        let window_width = self.width as f64;
        let window_height = self.height as f64;

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
    
                a.x += a.vx as i32;
                a.y += a.vy as i32;
    
                if a.x <= 2 || a.x >= (window_width as i32 - 30) {
                    a.vx *= -1.0;
                }
                if a.y <= 2 || a.y >= (window_height as i32 - 30) {
                    a.vy *= -1.0;
                }
            }
        }
    }
}

#[tauri::command]
fn init_particles() -> HashMap<String, Vec<Particle>> {

    let mut particle_system = PARTICLE_SYSTEM.lock().unwrap();
    
    if particle_system.particles.is_empty() {
        let colors = ["red", "magenta", "green", "yellow"];
        let num_atoms = 75;
        for color in colors {
            particle_system.create_particles(num_atoms, color);
        }
        particle_system.create_rules();
    }

    println!("{:?}", particle_system.rules);

    particle_system.get_particles()
}

#[tauri::command]
fn update_particles() -> HashMap<String, Vec<Particle>> {

    let mut particle_system = PARTICLE_SYSTEM.lock().unwrap();

    particle_system.apply_rules();
    particle_system.get_particles()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InteractionRules {
    rules: HashMap<String, HashMap<String, f64>>,
}

#[tauri::command]
fn get_interaction_rules() -> InteractionRules {
    let particle_system = PARTICLE_SYSTEM.lock().unwrap();
    InteractionRules {
        rules: particle_system.rules.clone(),
    }
}

#[tauri::command]
fn update_interaction_rules(new_rules: InteractionRules) -> Result<(), String> {
    let mut particle_system = PARTICLE_SYSTEM.lock().unwrap();
    particle_system.rules = new_rules.rules;
    Ok(())
}

#[tauri::command]
fn update_num_atoms(new_num: usize) -> Result<HashMap<String, Vec<Particle>>, String> {
    let mut particle_system = PARTICLE_SYSTEM.lock().unwrap();
    
    particle_system.particles.clear();
    
    let colors = ["red", "magenta", "green", "yellow"];
    for color in colors {
        particle_system.create_particles(new_num, color);
    }
    
    particle_system.create_rules();
    
    Ok(particle_system.get_particles())
}

#[tauri::command]
fn update_rules_only(new_rules: InteractionRules) -> Result<(), String> {
    let mut particle_system = PARTICLE_SYSTEM.lock().unwrap();
    particle_system.rules = new_rules.rules;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            init_particles, 
            update_particles, 
            get_interaction_rules, 
            update_interaction_rules,
            update_num_atoms,
            update_rules_only])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// CI=true npm run tauri build