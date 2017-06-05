use std::io;

mod objects;
mod matching_engine;

fn get_orders(match_eng: &matching_engine::MatchingEngine) {
	println!("Enter price: ");
	let mut m_price = String::new();
	io::stdin().read_line(&mut m_price);
	let m_price = m_price.trim().parse::<i64>().unwrap();

	println!("Side: (1 = buy, 2 = sell)");
	let mut m_side = String::new();
	io::stdin().read_line(&mut m_side);
	let m_side: char = m_side.chars().nth(0).unwrap();

	let mut orders: Vec<objects::Order>;
	if m_side == '1' {
		orders = match_eng.get_buy_orders(&m_price);
	}
	else {
		orders = match_eng.get_sell_orders(&m_price);	
	}
}

fn insert_new_order(match_eng: &mut matching_engine::MatchingEngine) {
	// Ask user to input new order
	println!("Insert new order: ");

	println!("Enter account id: ");
	let mut m_id = String::new();
	io::stdin().read_line(&mut m_id);
	let m_id: String = m_id.trim().to_string();

	println!("Side: (1 = buy, 2 = sell)");
	let mut m_side = String::new();
	io::stdin().read_line(&mut m_side);
	let m_side: char = m_side.chars().nth(0).unwrap();
	// println!("{}", m_side);

	println!("Price: ");
	let mut m_price = String::new();
	io::stdin().read_line(&mut m_price);
	let m_price = m_price.trim().parse::<i64>().unwrap();
	// println!("{}", m_price);

	println!("Quantity: ");
	let mut m_qty = String::new();
	io::stdin().read_line(&mut m_qty);
	let m_qty = m_qty.trim().parse::<i64>().unwrap();
	// println!("{}", m_qty);		

	match_eng.insert(&objects::Order::new(m_id, m_qty, m_price, m_side));

	match_eng.print_status();
}

fn get_market_status(match_eng: &matching_engine::MatchingEngine) {
	match_eng.print_status();
}

fn main() {
	let mut match_eng = matching_engine::MatchingEngine::new();
	
	loop {
		println!("{:*<1$}", "", 80);
		println!("Continue? (y/n) ");
		let mut continue_cmd = String::new();
		io::stdin().read_line(&mut continue_cmd);
		
		match &continue_cmd.trim() as &str {
			"y" => { },
			"n" => break,
			_ 	=> {
				println!("Enter y or n!");
				continue;
			}
		}

		println!("OPTIONS:");
		println!("1. Insert new order");
		println!("2. Get market status");
		println!("3. Query price");
		println!("Enter 1 option (1 or 2 or 3)");

		let mut option_cmd = String::new();
		io::stdin().read_line(&mut option_cmd);

		match &option_cmd.trim() as &str {
			"1" => insert_new_order(&mut match_eng),
			"2" => get_market_status(&match_eng),
			"3" => get_orders(&match_eng),
			_	=> {
				println!("Invalid option!");
				continue;
			}
		}
	}
}