#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Define types for memory management
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Define Adopter struct
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Adopter {
    id: u64,
    name: String,
    contact_details: String,
    desired_animal_type: String,
}

// Implement Storable trait for Adopter
impl Storable for Adopter {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement BoundedStorable trait for Adopter
impl BoundedStorable for Adopter {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define Animal struct
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Animal {
    id: u64,
    name: String,
    species: String,
    age: u32,
    is_adopted: bool,
}

// Implement Storable trait for Animal
impl Storable for Animal {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement BoundedStorable trait for Animal
impl BoundedStorable for Animal {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define thread-local storage for memory management and data storage
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static ADOPTER_STORAGE: RefCell<StableBTreeMap<u64, Adopter, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static ANIMAL_STORAGE: RefCell<StableBTreeMap<u64, Animal, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));
}

// Define custom error types
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidInput { msg: String },
}

// Query function to get adopter by ID
#[ic_cdk::query]
fn get_adopter(adopter_id: u64) -> Result<Adopter, Error> {
    match _get_adopter(&adopter_id) {
        Some(adopter) => Ok(adopter),
        None => Err(Error::NotFound {
            msg: format!("Adopter with id={} not found", adopter_id),
        }),
    }
}

// Update function to register a new adopter
#[ic_cdk::update]
fn register_adopter(name: String, contact_details: String, desired_animal_type: String) -> Result<Adopter, Error> {
    // Validate input data
    if name.is_empty() || contact_details.is_empty() || desired_animal_type.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Name, contact details, and desired animal type cannot be empty".to_string(),
        });
    }

    // Generate unique ID for the adopter
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    // Create Adopter instance
    let adopter = Adopter { id, name, contact_details, desired_animal_type };

    // Store adopter in the adopter storage
    ADOPTER_STORAGE.with(|service| service.borrow_mut().insert(id, adopter.clone()));
    Ok(adopter)
}

// Query function to get animal by ID
#[ic_cdk::query]
fn get_animal(animal_id: u64) -> Result<Animal, Error> {
    match _get_animal(&animal_id) {
        Some(animal) => Ok(animal),
        None => Err(Error::NotFound {
            msg: format!("Animal with id={} not found", animal_id),
        }),
    }
}

// Update function to add a new animal
#[ic_cdk::update]
fn add_animal(name: String, species: String, age: u32) -> Result<Animal, Error> {
    // Validate input data
    if name.is_empty() || species.is_empty() || age == 0 {
        return Err(Error::InvalidInput {
            msg: "Name, species, and age cannot be empty".to_string(),
        });
    }

    // Generate unique ID for the animal
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    // Create Animal instance
    let animal = Animal { id, name, species, age, is_adopted: false };

    // Store animal in the animal storage
    ANIMAL_STORAGE.with(|service| service.borrow_mut().insert(id, animal.clone()));
    Ok(animal)
}

// Update function to perform animal adoption
#[ic_cdk::update]
fn adopt_animal(adopter_id: u64, animal_id: u64) -> Result<(), Error> {
    // Retrieve adopter by ID
    let _adopter = match _get_adopter(&adopter_id) {
        Some(adopter) => adopter,
        None => return Err(Error::NotFound {
            msg: format!("Adopter with id={} not found", adopter_id),
        }),
    };

    // Retrieve animal by ID
    let mut animal = match _get_animal(&animal_id) {
        Some(animal) => animal,
        None => return Err(Error::NotFound {
            msg: format!("Animal with id={} not found", animal_id),
        }),
    };

    // Check if the animal is already adopted
    if animal.is_adopted {
        return Err(Error::InvalidInput {
            msg: "Animal is already adopted".to_string(),
        });
    }

    // Perform adoption process
    animal.is_adopted = true;
    ANIMAL_STORAGE.with(|service| service.borrow_mut().insert(animal_id, animal));

    Ok(())
}

// Helper function to get adopter by ID
fn _get_adopter(adopter_id: &u64) -> Option<Adopter> {
    ADOPTER_STORAGE.with(|service| service.borrow().get(adopter_id))
}

// Helper function to get animal by ID
fn _get_animal(animal_id: &u64) -> Option<Animal> {
    ANIMAL_STORAGE.with(|service| service.borrow().get(animal_id))
}

// Update function to update adopter details
#[ic_cdk::update]
fn update_adopter(adopter_id: u64, name: String, contact_details: String, desired_animal_type: String) -> Result<Adopter, Error> {
    // Retrieve adopter by ID
    let mut adopter = match _get_adopter(&adopter_id) {
        Some(adopter) => adopter,
        None => return Err(Error::NotFound {
            msg: format!("Adopter with id={} not found", adopter_id),
        }),
    };

    // Update adopter details
    adopter.name = name;
    adopter.contact_details = contact_details;
    adopter.desired_animal_type = desired_animal_type;

    // Store updated adopter in the adopter storage
    ADOPTER_STORAGE.with(|service| service.borrow_mut().insert(adopter_id, adopter.clone()));
    Ok(adopter)
}

// Update function to delete an adopter
#[ic_cdk::update]
fn delete_adopter(adopter_id: u64) -> Result<(), Error> {
    // Check if the adopter exists
    if _get_adopter(&adopter_id).is_none() {
        return Err(Error::NotFound {
            msg: format!("Adopter with id={} not found", adopter_id),
        });
    }

    // Remove adopter from the adopter storage
    ADOPTER_STORAGE.with(|service| service.borrow_mut().remove(&adopter_id));
    Ok(())
}

// Update function to update animal details
#[ic_cdk::update]
fn update_animal(animal_id: u64, name: String, species: String, age: u32) -> Result<Animal, Error> {
    // Retrieve animal by ID
    let mut animal = match _get_animal(&animal_id) {
        Some(animal) => animal,
        None => return Err(Error::NotFound {
            msg: format!("Animal with id={} not found", animal_id),
        }),
    };

    // Update animal details
    animal.name = name;
    animal.species = species;
    animal.age = age;

    // Store updated animal in the animal storage
    ANIMAL_STORAGE.with(|service| service.borrow_mut().insert(animal_id, animal.clone()));
    Ok(animal)
}

// Update function to delete an animal
#[ic_cdk::update]
fn delete_animal(animal_id: u64) -> Result<(), Error> {
    // Check if the animal exists
    if _get_animal(&animal_id).is_none() {
        return Err(Error::NotFound {
            msg: format!("Animal with id={} not found", animal_id),
        });
    }

    // Remove animal from the animal storage
    ANIMAL_STORAGE.with(|service| service.borrow_mut().remove(&animal_id));
    Ok(())
}

// Query function to list all adopters
#[ic_cdk::query]
fn list_adopters() -> Vec<Adopter> {
    let mut adopters = Vec::new();

    ADOPTER_STORAGE.with(|service| {
        let map_ref = service.borrow();
        for (_, adopter) in map_ref.iter() {
            adopters.push(adopter.clone());
        }
    });

    adopters
}

// Query function to list all animals
#[ic_cdk::query]
fn list_animals() -> Vec<Animal> {
    let mut animals = Vec::new();

    ANIMAL_STORAGE.with(|service| {
        let map_ref = service.borrow();
        for (_, animal) in map_ref.iter() {
            animals.push(animal.clone());
        }
    });

    animals
}

// Update function to mark an animal as not adopted
#[ic_cdk::update]
fn release_animal(animal_id: u64) -> Result<(), Error> {
    // Retrieve animal by ID
    let mut animal = match _get_animal(&animal_id) {
        Some(animal) => animal,
        None => return Err(Error::NotFound {
            msg: format!("Animal with id={} not found", animal_id),
        }),
    };

    // Check if the animal is already released
    if !animal.is_adopted {
        return Err(Error::InvalidInput {
            msg: "Animal is already released".to_string(),
        });
    }

    // Mark the animal as not adopted
    animal.is_adopted = false;
    ANIMAL_STORAGE.with(|service| service.borrow_mut().insert(animal_id, animal));

    Ok(())
}

// Query function to list all adopters who adopted a specific animal
#[ic_cdk::query]
fn list_adopters_of_animal(animal_id: u64) -> Vec<Adopter> {
    let mut adopters = Vec::new();

    ADOPTER_STORAGE.with(|service| {
        let map_ref = service.borrow();
        for (_, adopter) in map_ref.iter() {
            if adopter.desired_animal_type == _get_animal(&animal_id).unwrap().species {
                adopters.push(adopter.clone());
            }
        }
    });

    adopters
}

// Export Candid interface
ic_cdk::export_candid!();
