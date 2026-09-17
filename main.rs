fn main() {
    let mut balance:f64 = 1000.0;
    let mut resting_limit_orders: [(bool,f64,u32);4] = [
        (true,11.2,44),
        (false,22.34,11),
        (true,222.11,23),
        (false,33.11,22)
    ];

    loop{
        println!(r#"
-------------MENU---------------
1. View resting Limit Order Book.
2: Submit a Market Buy Order.
3: Submit a Market Sell Order.
4: Shut down engine.
                  "#);
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).expect("Failed to expect");
        let mut user_choice:u32 = match user_input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number entered...");
                continue;
            }
        };
        match user_choice {
            1=>{
                println!("\n--- Current Order Book ---");
                for order in resting_limit_orders{
                    let side = if order.0{"BUY"} else {"SELL"};
                    println!("{} | Price: {:.2} | Qty: {}",side,order.1,order.2);
            
                }
            }
            2=>{
                //BUY -> the goal is to buy everything that is for sell
                println!("BUY Ordered requested...Enter quantity to BUY-----> ");
                let mut user_input = String::new();
                std::io::stdin().read_line(&mut user_input).expect("Failed to expect");
                let mut qty_to_buy = match user_input.trim().parse(){
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid user input, please enter a valid quantity...");
                        continue;
                    }
                };
                for order in &mut resting_limit_orders{
                    if !(order.0) && (order.2) > 0{
                        //GOAL: mutate the array, remove all the quantity where we had false
                        //Two cases: requested qty_to_but > available qty AND normal case
                        let fill_qty = if qty_to_buy > order.2 {order.2} else {qty_to_buy};
                        //calculate the amount that we bought -> (qty*price)
                        let balance_spent = order.1 * (fill_qty as f64);
                        //mutating array elements->
                        if balance >= balance_spent{
                            //the balance
                            balance -= balance_spent;
                            //the volume
                            order.2 -= fill_qty;
                            //the real qty
                            qty_to_buy -= fill_qty;
                            println!("✅ Bought {} units at {:.2}", fill_qty, order.1);
                        } else {
                            println!("❌ Insufficient capital to buy {} units at {:.2}.", fill_qty, order.1);
                            break; 
                        }
                        if qty_to_buy ==0 {
                            break;
                        }
                    }//if clsed
                }//for closed
                println!("\nPress Enter to return to menu...");
                let mut pause = String::new();
                std::io::stdin().read_line(&mut pause).unwrap();
            }//case closed
            3=>{
                println!("SELL Ordered requested...Enter the qunatity to SELL-----> ");
                let mut user_input = String::new();
                std::io::stdin().read_line(&mut user_input).expect("Failed to expect");
                let mut qty_to_sell = match user_input.trim().parse(){
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid user input, please enter a valid quantity...");
                        continue;
                    }
                };
                for order in &mut resting_limit_orders{
                    if order.0 && (order.2) > 0{
                        let fill_qty = if qty_to_sell > order.2 {order.2} else {qty_to_sell};
                        let balance_earned = order.1 * (fill_qty as f64);
                        balance+=balance_earned;
                        order.2-=fill_qty;
                        //qty transc on my ledger
                        qty_to_sell -= fill_qty;
                        println!("✅ Sold {} units at {:.2}", fill_qty, order.1);
                    if qty_to_sell == 0 {
                            break;
                        }
                }//for closed
                    println!("\nPress Enter to return to menu...");
                    let mut pause = String::new();
                    std::io::stdin().read_line(&mut pause).unwrap();
            }//case closed

            4=>{
                println!("-----Shut Down!-----");
                break;
            }
            
            _ => println!("Invalid option");
                let mut pause = String::new();
                std::io::stdin().read_line(&mut pause).unwrap();
        }   
}
