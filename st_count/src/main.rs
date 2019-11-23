extern crate st_count;
fn main (){
    let value = "asASDsd asds as";
    let counts = st_count::count::capital(value.to_string());
    println!("space count {}",counts);
    println!("{}",value); 

}