pub mod extensiontypes;

#[no_mangle]
pub extern "C" fn test(){
    println!("Testing extension lib");
}