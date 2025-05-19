// enums in rust are a way to define a type that can be one of several different variants
// enums are similar to structs, but they can have multiple variants
// it is like custom data type

enum CarType {
    Sedan,
    SUV,
    Truck,
    Coupe,
}

fn printCars(car: CarType) {
    match car {
        CarType::Sedan => println!("This is a Sedan"),
        CarType::SUV => println!("This is an SUV"),
        CarType::Truck => println!("This is a Truck"),
        CarType::Coupe => println!("This is a Coupe"),
    }
}
fn main() {
    let my_car = CarType::SUV;
    printCars(my_car);
    printCars(CarType::Truck);
    printCars(CarType::Coupe);
}