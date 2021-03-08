use rand::Rng;
use std::io;
// Type class
use std::cmp::Ordering;
// HashMap
use std::collections::HashMap;

fn main() {

    #[derive(Debug, Clone)]
    enum Color {
        Red,
        Blue,
        White,
    };
    #[derive(Debug, Clone)]
    enum Engine {
        S,
        M,
        L,
    }
    trait Vehicle {}
    #[derive(Debug, Clone)]
    struct Car {
        //
        engine: Engine,
        color: Color,
        wheels: i16,
    };
    impl Vehicle for Car {}

    impl Car {
    // This is a static method
    // Static methods don't need to be called by an instance
    // These methods are generally used as constructors
        pub fn createCars() -> (Car, Car, Car) {
            let aDefaultCar: Car = Car {
                engine: Engine::M,
                color: Color::White,
                wheels: 4,
            };
            let aSmallCar: Car = Car {
                engine: Engine::S,
                color: Color::Red,
                wheels: 3,
            };

            let aMediumCar: Car = Car {
                engine: Engine::S,
                color: Color::Red,
                ..aDefaultCar
            };

            let aBigCar: Car = Car {
                engine: Engine::L,
                wheels: 4,
                ..aDefaultCar
            };
            return (aSmallCar, aMediumCar, aBigCar);
        }
    }

    #[derive(Debug, Clone)]
    struct Bicycle {
        eletricity: bool,
    };
    impl Vehicle for Bicycle {}

    #[derive(Debug, Clone)]
    struct Slot<T: Vehicle> {
        parkingSlot: HashMap<u32, T>,
    }

    #[derive(Debug)]
    pub struct Parking<T: Vehicle> {
        vehicles: Vec<Slot<T>>,
    };
    // Put detail into Generic types
    type ParkingForCars = Parking<Car>;
    type ParkingForBici = Parking<Bicycle>;

    // define a behaviour for a Parking
    trait ParkingTrait {
        fn max(&self) -> usize {
            return 100;
        }
        fn freeSlots(&self) -> usize;
        fn putAVehcileIntoSlot<T : Vehicle>(vehicle: T) -> Slot<T>;
    }

    // this is a concret implementation for a Cars
    // you need to provide a impl for you type - the rest is working on generic Vehicle

    impl ParkingTrait for ParkingForCars {
        fn freeSlots(&self) -> usize {
            return self.max() - self.vehicles.len();
        }
        fn putAVehcileIntoSlot<T : Vehicle>(vehicle: T) -> Slot<T> {
            const MIN: u32 = 1;
            const MAX: u32 = 100;
            fn randomNumber() -> u32 {
                return rand::thread_rng().gen_range(MIN, MAX);
            }
    
            let mut aVehcileIntoSlot = HashMap::new();
            aVehcileIntoSlot.insert(randomNumber(), vehicle);
            return Slot {
                parkingSlot: aVehcileIntoSlot,
            };
        }

    }

    // TODO :: implement Behaviour for your conjunction type
    impl ParkingTrait for ParkingForBici {
        fn freeSlots(&self) -> usize {unimplemented!(); }
        fn putAVehcileIntoSlot<T : Vehicle>(vehicle: T) -> Slot<T> { unimplemented!(); }
    }
   
    let aSmallParkingForCars: ParkingForCars = {
        let (aSmallCar, aMediumCar, aBigCar) = Car::createCars();
        ParkingForCars {
            vehicles: vec![
                ParkingForCars::putAVehcileIntoSlot(aSmallCar),
                ParkingForCars::putAVehcileIntoSlot(aMediumCar),
                ParkingForCars::putAVehcileIntoSlot(aBigCar),
            ],
        }
    };

    println!("Free Slots :: {:?}", aSmallParkingForCars.freeSlots());
    // panic!("crash and burn");
    println!("{:?}", printCats());
}

fn printCats() -> (char, char, char) {
    let heart_eyed_cat = '😻';
    return (heart_eyed_cat, heart_eyed_cat, heart_eyed_cat);
}
/*
*   An EXAMPLE OF TYPE CLASS
    use std::ops::Add;
    impl Add for Car {
        type Output = Car;

        fn add(self, other: Car) -> Car {
            return Car {
                engine: self.engine,
                color: self.color,
                wheels: self.wheels + other.wheels,
            };
        }
    }
    let c0 = Car {
        engine: Engine::M,
        color: Color::White,
        wheels: 4,
    };

    let c1 = Car {
        engine: Engine::M,
        color: Color::White,
        wheels: 4,
    };
    let sumACars = c0 + c1;

*/
