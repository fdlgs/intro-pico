#![no_std]
#![no_main]

// Halte notre programme en cas de panic!()
use panic_halt as _;

use rp_pico as bsp;     // Board Support Package
// La macro qui débute notre programme
use bsp::entry;
use embedded_hal::{
    // GPIO Traits
    digital::OutputPin,
    //  Le Trait du minuteur.
    delay::DelayNs,
};

mod pi_pico;

/// La macro `#[entry]` s'assure que cette fontion soit exécutée dès que toutes 
/// les variables globales sont initialiées.
///
/// Cette fonction capture les ressources du Pi Pico en vue d'écrire un programme simple
#[entry]
fn main_loop() -> ! {
    
    //  Capturer les ressources du Pi Pico
    let mut pi_pico = pi_pico::PiPico::take().unwrap();
    let pins = pi_pico.init_pins().unwrap();
    let mut timer = pi_pico.init_timer().unwrap();

    // Écrire votre programme à partir d'ici.

    loop {

    }
}

