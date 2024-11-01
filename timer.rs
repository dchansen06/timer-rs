use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::env;
use std::thread;

fn main() {
	let start: Duration = SystemTime::now().duration_since(UNIX_EPOCH).expect("Current time is pre-Unix Epoch");
	let dur: u64 = env::args().nth(1).expect("Duration required").parse().unwrap();

	assert!(dur != 0, "Duration cannot be zero");
	const OFFSET: Duration = Duration::from_millis(0);	// Sneaky error correction, brings sleep back this many ms earlier if you so choose
	let units: String = env::args().nth(2).expect("Units required");

	let duration = if matches!(&units[..], "ms" | "millis" | "milliseconds" ) { Duration::from_millis(dur) }
		else if matches!(&units[..], "s" | "secs" | "seconds" ) { Duration::from_secs(dur) }
		else if matches!(&units[..], "m" | "mins" | "minutes" ) { Duration::from_secs(dur * 60) }
		else { Duration::ZERO };

	let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Current time is pre-Unix Epoch");
	if duration + start > now + OFFSET { thread::sleep(duration + start - now - OFFSET); }
}
