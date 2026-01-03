use std::io::{self, Write};

const MAX_ATTEMPTS: u8 = 3;
const CORRECT_PIN: &str = "1234"; // Set your PIN here

fn main() {
    let mut attempts = 0;
    let mut is_blocked = false;

    println!("=== SIM Unlock System ===");
    println!(
        "You have {} attempts to enter the correct PIN.\n",
        MAX_ATTEMPTS
    );

    while attempts < MAX_ATTEMPTS && !is_blocked {
        print!("Enter PIN: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let pin = input.trim();
        attempts += 1;

        if pin == CORRECT_PIN {
            println!("\n✓ PIN correct! SIM unlocked.");
            return;
        } else {
            let remaining = MAX_ATTEMPTS - attempts;
            if remaining > 0 {
                println!("✗ Incorrect PIN. {} attempt(s) remaining.\n", remaining);
            } else {
                is_blocked = true;
            }
        }
    }

    if is_blocked {
        println!("\n⚠ ACCOUNT BLOCKED! Maximum attempts exceeded.");
        println!("Please contact your service provider to unlock.");
    }
}
