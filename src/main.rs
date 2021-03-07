use rand::Rng;
use std::io;
// Type class
use std::cmp::Ordering;
// HashMA
use std::collections::HashMap;

fn main() {
    // let secret_number: u32 = randomNumber();
    // let ourNum: u32 = randomNumber();

    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128
    // arch	isize	usize

    let result: i32 = match 1.cmp(&3) {
        Ordering::Less => -1,
        Ordering::Greater => 1,
        Ordering::Equal => 0,
    };

    println!("{}", result);

    let (aGato1, aCat2, aKotek3): (char, char, char) = printCats();
    println!("{:?}", printCats());

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
    pub trait VehicleB {
        fn start(&self) -> String;
    }
    impl VehicleB for Car {
        fn start(&self) -> String {
            return String::from("Start a Car V");
        }
    }

    #[derive(Debug, Clone)]
    struct Car {
        engine: Engine,
        color: Color,
        wheels: i16,
    };

    #[derive(Debug, Clone)]
    struct Slot<T> {
        parkingSlot: HashMap<u32, T>,
    }

    #[derive(Debug)]
    pub struct Parking<T> {
        vehicles: Vec<Slot<T>>,
    };
    type ParkingForCars = Parking<Car>;
    pub trait ParkingB {
        fn max(&self) -> usize {
            return 100;
        }
        fn freeSlots(&self) -> usize;
    }

    impl ParkingB for ParkingForCars {
        fn freeSlots(&self) -> usize {
            return self.max() - self.vehicles.len();
        }
    }
    fn prepareData() -> (Car, Car, Car) {
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
    fn createSlotForCar<T: VehicleB>(vehicle: T) -> Slot<T> {
        fn randomNumber() -> u32 {
            return rand::thread_rng().gen_range(1, 100);
        }

        let mut slotForAVehicle = HashMap::new();
        slotForAVehicle.insert(randomNumber(), vehicle);
        return Slot {
            parkingSlot: slotForAVehicle,
        };
    }

    let aSmallParking: ParkingForCars = {
        let (aSmallCar, aMediumCar, aBigCar) = prepareData();
        ParkingForCars {
            vehicles: vec![
                createSlotForCar(aSmallCar),
                createSlotForCar(aMediumCar),
                createSlotForCar(aBigCar),
            ],
        }
    };

    println!("Free Slots :: {:?}", aSmallParking.freeSlots());

    // panic!("crash and burn");

    // println!("{:?}", aSmallParking);
    // println!("{:?}", aBigCar);
}

fn printCats() -> (char, char, char) {
    let heart_eyed_cat = '😻';
    return (heart_eyed_cat, heart_eyed_cat, heart_eyed_cat);
}
