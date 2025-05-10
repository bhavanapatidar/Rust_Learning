fn main(){
    let mut v= vec![1,2,3,4,5];
    v.push(10); //1,2,3,4,5,10
    v.pop(); // last element will pop 1,2, 3, 4, 5
    println!("Vector v = {:?}",v)
}