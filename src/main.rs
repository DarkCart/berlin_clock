use chrono::prelude::*;

fn main() {
	let time: DateTime<Local> = Local::now();
	println!("{}", time);
	display_clock(&(time.hour() / 5), &(time.hour() % 5), &(time.minute() / 5), &(time.minute() % 5), &time.second());
}

fn display_clock(five_hr: &u32, one_hr: &u32, five_mn: &u32, one_mn: &u32, seconds: &u32) {
	println!("S {}", seconds);
	draw_block(&(seconds % 2 == 0));
	println!("Hx5 {}", five_hr);
	println!("Hx1 {}", one_hr);
	println!("Mx5 {}", five_mn);
	println!("Mx1 {}", one_mn);		
} 

fn draw_block(on: &bool) {
	println!("╔══╗");
	if on == &true {
		println!("║{0}{0}║", "█");
	} else {
		println!("║  ║");
	}
	println!("╚══╝");
}
