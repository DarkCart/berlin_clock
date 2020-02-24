use chrono::prelude::*;

fn main() {
	let time: DateTime<Local> = Local::now();
	println!("{}", time);
	display_clock(&(time.hour() / 5), &(time.hour() % 5), &(time.minute() / 5), &(time.minute() % 5), &time.second());
}

fn display_clock(five_hr: &u32, one_hr: &u32, five_mn: &u32, one_mn: &u32, seconds: &u32) {
		
} 
