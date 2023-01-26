use rand::Rng;
use std::io;

fn main() {
    // Zero out the machine.
    let mut dice_in_play: u32 = 0;

    // Create dice
    let dice_1_active: bool = true;
    let dice_2_active: bool = true;
    let dice_3_active: bool = true;

    // Set maximum dice for game.
    const MAX_DICE: u32 = 3;
    /// Rolls all dice added to current game
    fn roll_dice(dice_1_active: bool, dice_2_active: bool, dice_3_active: bool) {
        
        if dice_1_active == true {
            print!("Dice 1 shows: ");
            roll()
        }
        if dice_2_active == true {
            print!("Dice 2 shows: ");
            roll()
        }
        if dice_3_active == true {
            print!("Dice 3 shows: ");
            roll()
        }

        
    }
    loop {
        // Set number of dice if user changes the amount.
        println!("Press enter to roll dice");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Problem reading line");

        roll_dice(dice_1_active, dice_2_active, dice_3_active);
        
        }
    
    }


fn roll() {
    let mut rng = rand::thread_rng();
    let roll: u32 = rng.gen_range(1..7);
    println!("{}", roll);
}