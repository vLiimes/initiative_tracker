use initiative_tracker::turn_order;

fn main() {
    let mut turn_order = turn_order::TurnOrder::new();
    turn_order.add_creature(String::from("Jeremy"), 10);

    println!("{turn_order}");
}
