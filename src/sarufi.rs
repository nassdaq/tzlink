#[cfg(feature="sarufi")]
fn main(){
    println!("feature on")
}

#[cfg(not(feature="sarufi"))]
fn main(){
    println!("feature off")
}
