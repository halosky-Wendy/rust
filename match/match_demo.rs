fn add_fancy_hat(){
    println!("add_fancy_hat");    
}

fn remove_fancy_hat(){
    println!("remove_fancy_hat");
}

fn move_fancy_hat(num_space:u8){
    println!("move_fancy_hat:{}",num_space);
}

fn main(){

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_fancy_hat(other),
    }

}