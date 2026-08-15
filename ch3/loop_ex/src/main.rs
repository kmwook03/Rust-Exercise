fn main() {
	let mut count = 0;
	'counting_loop: loop {
		println!("count = {count}");
		let mut remaining = 10;

		loop {
			println!("remaining = {remaining}");
			if remaining == 9 {
				break;
			}
			if count == 2 {
				break 'counting_loop;
			}
			remaining -= 1;
		}
		count += 1;
	}
    println!("End count = {count}");
}

