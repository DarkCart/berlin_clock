use chrono::prelude::*;
use std::{thread, time};
use colored::*;

fn main() {
	let mut current_time: DateTime<Local>;
	loop {
		current_time = Local::now();
		print!("\x1B[2J");
		display_clock(&(current_time.hour() / 5), &(current_time.hour() % 5), &(current_time.minute() / 5), &(current_time.minute() % 5), &current_time.second());
		thread::sleep(time::Duration::from_secs(1));
	}
}

fn display_clock(five_hr: &u32, one_hr: &u32, five_mn: &u32, one_mn: &u32, seconds: &u32) {
	//println!("S {}", seconds);
	//println!("Hx5 {}", five_hr);
	//println!("Hx1 {}", one_hr);
	//println!("Mx5 {}", five_mn);
	//println!("Mx1 {}", one_mn);		
	draw_sec_block(&(seconds % 2 == 0));
	draw_blocks(&five_hr, &4);
	draw_blocks(&one_hr, &4);
	draw_blocks(&five_mn, &11);
	draw_blocks(&one_mn, &4);
} 

fn draw_sec_block(on: &bool) {
	println!("╔══╗");
	if on == &true {
                println!("║{0}{0}║", "█".bright_yellow());
        } else {
        	println!("║{0}{0}║", "░".bright_yellow());
        }
	println!("╚══╝");
}

fn draw_blocks(current_val: &u32, range: &u32) {
	// looks hacky as hell, this is the only thing
	// i can think of to get this thing to work
	for _i in 1..=*range {
		print!("╔══╗");

	}
	println!("");
	
	for j in 1..=*range {
		if &j <= current_val {
			print!("║{0}{0}║", "█".bright_red());
		} else {
			print!("║{0}{0}║", "░".bright_red());
		}
	}

	println!("");
	for _k in 1..=*range {
		print!("╚══╝");
	}
	println!("");
}
