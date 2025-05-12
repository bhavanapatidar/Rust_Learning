fn main(){
    let mut space = " ";
    space = space.len();
    println!("space={}",space);
}

// above code will give error because space is a string and we are trying to assign it to integer

fn main(){
    let mut space = " ";
    let space = space.len();
    println!("space={}",space);
}
// spaces is shadowed by space.len() and now space is an integer so it will not give error