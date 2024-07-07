#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// UserType enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum UserType {
    #[default]
    Owner,
    Shelter,
    Adopter,
}

// PetStatus enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum PetStatus {
    #[default]
    Available,
    Adopted,
}

// AdoptionStatus enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum AdoptionStatus {
    #[default]
    Pending,
    Approved,
    Rejected,
}

// User struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User {
    id: u64,
    name: String,
    contact: String,
    user_type: UserType,
    created_at: u64,
}

// Pet struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Pet {
    id: u64,
    owner_id: u64,
    name: String,
    species: String,
    breed: String,
    age: u32,
    description: String,
    status: PetStatus,
    created_at: u64,
}

// AdoptionRequest struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct AdoptionRequest {
    id: u64,
    pet_id: u64,
    adopter_id: u64,
    status: AdoptionStatus,
    requested_at: u64,
    approved_at: Option<u64>,
}

// PetCareEvent struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct PetCareEvent {
    id: u64,
    title: String,
    description: String,
    date_time: u64,
    location: String,
    organizer_id: u64,
    created_at: u64,
}

// Feedback struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Feedback {
    id: u64,
    user_id: u64,
    pet_id: Option<u64>,
    event_id: Option<u64>,
    feedback: String,
    rating: u8,
    created_at: u64,
}

// Donation struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Donation {
    id: u64,
    donor_id: u64,
    amount: u32,
    created_at: u64,
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Pet {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Pet {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for AdoptionRequest {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for AdoptionRequest {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for PetCareEvent {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for PetCareEvent {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Feedback {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Feedback {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Donation {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Donation {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static USERS_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static PETS_STORAGE: RefCell<StableBTreeMap<u64, Pet, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static ADOPTION_REQUESTS_STORAGE: RefCell<StableBTreeMap<u64, AdoptionRequest, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static PET_CARE_EVENTS_STORAGE: RefCell<StableBTreeMap<u64, PetCareEvent, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));

    static FEEDBACKS_STORAGE: RefCell<StableBTreeMap<u64, Feedback, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));

    static DONATIONS_STORAGE: RefCell<StableBTreeMap<u64, Donation, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6)))
    ));
}

// User Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct UserPayload {
    name: String,
    contact: String,
    user_type: UserType,
}

// Pet Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct PetPayload {
    owner_id: u64,
    name: String,
    species: String,
    breed: String,
    age: u32,
    description: String,
    status: PetStatus,
}

// AdoptionRequest Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct AdoptionRequestPayload {
    pet_id: u64,
    adopter_id: u64,
    status: AdoptionStatus,
}

// PetCareEvent Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct PetCareEventPayload {
    title: String,
    description: String,
    date_time: u64,
    location: String,
    organizer_id: u64,
}

// Feedback Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct FeedbackPayload {
    user_id: u64,
    pet_id: Option<u64>,
    event_id: Option<u64>,
    feedback: String,
    rating: u8,
}

// Donation Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct DonationPayload {
    donor_id: u64,
    amount: u32,
}

// Function to create a new user
#[ic_cdk::update]
fn create_user(payload: UserPayload) -> Result<User, String> {
    if payload.name.is_empty() || payload.contact.is_empty() {
        return Err("Name and contact cannot be empty".to_string());
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let user = User {
        id,
        name: payload.name,
        contact: payload.contact,
        user_type: payload.user_type,
        created_at: time(),
    };

    USERS_STORAGE.with(|storage| storage.borrow_mut().insert(id, user.clone()));
    Ok(user)
}

// Function to retrieve all users
#[ic_cdk::query]
fn get_all_users() -> Result<Vec<User>, String> {
    USERS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<User> = stable_btree_map
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No users found.".to_string())
        } else {
            Ok(records)
        }
    })
}

// Function to create a new pet
#[ic_cdk::update]
fn create_pet(payload: PetPayload) -> Result<Pet, String> {
    if payload.owner_id == 0
        || payload.name.is_empty()
        || payload.species.is_empty()
        || payload.breed.is_empty()
        || payload.description.is_empty()
    {
        return Err("All fields must be provided.".to_string());
    }

    let owner_exists =
        USERS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.owner_id));
    if !owner_exists {
        return Err("Owner ID does not exist.".to_string());
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let pet = Pet {
        id,
        owner_id: payload.owner_id,
        name: payload.name,
        species: payload.species,
        breed: payload.breed,
        age: payload.age,
        description: payload.description,
        status: payload.status,
        created_at: time(),
    };

    PETS_STORAGE.with(|storage| storage.borrow_mut().insert(id, pet.clone()));
    Ok(pet)
}

// Function to retrieve all pets
#[ic_cdk::query]
fn get_all_pets() -> Result<Vec<Pet>, String> {
    PETS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<Pet> = stable_btree_map
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No pets found.".to_string())
        } else {
            Ok(records)
        }
    })
}

// Function to create a new adoption request
#[ic_cdk::update]
fn create_adoption_request(payload: AdoptionRequestPayload) -> Result<AdoptionRequest, String> {
    if payload.pet_id == 0 || payload.adopter_id == 0 {
        return Err("Pet ID and Adopter ID must be provided.".to_string());
    }

    let pet_exists = PETS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.pet_id));
    if !pet_exists {
        return Err("Pet ID does not exist.".to_string());
    }

    let adopter_exists =
        USERS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.adopter_id));
    if !adopter_exists {
        return Err("Adopter ID does not exist.".to_string());
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let adoption_request = AdoptionRequest {
        id,
        pet_id: payload.pet_id,
        adopter_id: payload.adopter_id,
        status: payload.status,
        requested_at: time(),
        approved_at: None,
    };

    ADOPTION_REQUESTS_STORAGE
        .with(|storage| storage.borrow_mut().insert(id, adoption_request.clone()));
    Ok(adoption_request)
}

// Function to retrieve all adoption requests
#[ic_cdk::query]
fn get_all_adoption_requests() -> Result<Vec<AdoptionRequest>, String> {
    ADOPTION_REQUESTS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<AdoptionRequest> = stable_btree_map
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No adoption requests found.".to_string())
        } else {
            Ok(records)
        }
    })
}

// Function to create a new pet care event
#[ic_cdk::update]
fn create_pet_care_event(payload: PetCareEventPayload) -> Result<PetCareEvent, String> {
    if payload.title.is_empty()
        || payload.description.is_empty()
        || payload.location.is_empty()
        || payload.organizer_id == 0
    {
        return Err("All fields must be provided.".to_string());
    }

    let organizer_exists =
        USERS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.organizer_id));
    if !organizer_exists {
        return Err("Organizer ID does not exist.".to_string());
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let pet_care_event = PetCareEvent {
        id,
        title: payload.title,
        description: payload.description,
        date_time: payload.date_time,
        location: payload.location,
        organizer_id: payload.organizer_id,
        created_at: time(),
    };

    PET_CARE_EVENTS_STORAGE.with(|storage| storage.borrow_mut().insert(id, pet_care_event.clone()));
    Ok(pet_care_event)
}

// Function to retrieve all pet care events
#[ic_cdk::query]
fn get_all_pet_care_events() -> Result<Vec<PetCareEvent>, String> {
    PET_CARE_EVENTS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<PetCareEvent> = stable_btree_map
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No pet care events found.".to_string())
        } else {
            Ok(records)
        }
    })
}

// Function to create a new feedback
#[ic_cdk::update]
fn create_feedback(payload: FeedbackPayload) -> Result<Feedback, String> {
    if payload.user_id == 0 || payload.feedback.is_empty() || payload.rating == 0 {
        return Err("User ID, feedback, and rating must be provided.".to_string());
    }

    let user_exists = USERS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.user_id));
    if !user_exists {
        return Err("User ID does not exist.".to_string());
    }

    if let Some(ref pid) = payload.pet_id {
        let pet_exists = PETS_STORAGE.with(|storage| storage.borrow().contains_key(pid));
        if !pet_exists {
            return Err("Pet ID does not exist.".to_string());
        }
    }

    if let Some(ref eid) = payload.event_id {
        let event_exists =
            PET_CARE_EVENTS_STORAGE.with(|storage| storage.borrow().contains_key(eid));
        if !event_exists {
            return Err("Event ID does not exist.".to_string());
        }
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let feedback = Feedback {
        id,
        user_id: payload.user_id,
        pet_id: payload.pet_id,
        event_id: payload.event_id,
        feedback: payload.feedback,
        rating: payload.rating,
        created_at: time(),
    };

    FEEDBACKS_STORAGE.with(|storage| storage.borrow_mut().insert(id, feedback.clone()));
    Ok(feedback)
}

// Function to retrieve all feedback
#[ic_cdk::query]
fn get_all_feedback() -> Result<Vec<Feedback>, String> {
    FEEDBACKS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<Feedback> = stable_btree_map
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No feedback found.".to_string())
        } else {
            Ok(records)
        }
    })
}

// Function to create a new donation
#[ic_cdk::update]
fn create_donation(payload: DonationPayload) -> Result<Donation, String> {
    if payload.donor_id == 0 || payload.amount == 0 {
        return Err("Donor ID and amount must be provided.".to_string());
    }

    let donor_exists =
        USERS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.donor_id));
    if !donor_exists {
        return Err("Donor ID does not exist.".to_string());
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let donation = Donation {
        id,
        donor_id: payload.donor_id,
        amount: payload.amount,
        created_at: time(),
    };

    DONATIONS_STORAGE.with(|storage| storage.borrow_mut().insert(id, donation.clone()));
    Ok(donation)
}

// Function to retrieve all donations
#[ic_cdk::query]
fn get_all_donations() -> Result<Vec<Donation>, String> {
    DONATIONS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<Donation> = stable_btree_map
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No donations found.".to_string())
        } else {
            Ok(records)
        }
    })
}

// Error types
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    UnAuthorized { msg: String },
}

// need this to generate candid
ic_cdk::export_candid!();
