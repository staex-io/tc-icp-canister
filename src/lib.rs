use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt::Display;

use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::StableBTreeMap;
use ic_stable_structures::{memory_manager::MemoryManager, DefaultMemoryImpl, Storable};

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    static TELEMETRY_DATA: RefCell<StableBTreeMap<String, TelemetryData, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))));
}

/*
    -----
    NOTE: IN CASE YOU CHANGE SOMETHING YOU NEED TO UPDATE ICP WORKER.
    NOTE: IN CASE YOU CHANGE SOMETHING YOU NEED TO UPDATE ICP WORKER.
    NOTE: IN CASE YOU CHANGE SOMETHING YOU NEED TO UPDATE ICP WORKER.
    -----
*/
pub type CResult<T> = Result<T, CError>;

#[derive(CandidType, Deserialize, Default)]
pub enum CError {
    #[default]
    Internal,
    NotFound,
}

impl Display for CError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c_error_str: &str = match self {
            CError::Internal => "internal",
            CError::NotFound => "not_found",
        };
        write!(f, "{c_error_str}")
    }
}
/*
    -----
    NOTE: IN CASE YOU CHANGE SOMETHING YOU NEED TO UPDATE ICP WORKER.
    NOTE: IN CASE YOU CHANGE SOMETHING YOU NEED TO UPDATE ICP WORKER.
    NOTE: IN CASE YOU CHANGE SOMETHING YOU NEED TO UPDATE ICP WORKER.
    -----
*/

#[derive(CandidType, Deserialize)]
struct TelemetryData {
    hash: String,
    signature: String,
}
impl_storable!(TelemetryData);

#[ic_cdk::update]
fn store(id: String, hash: String, signature: String) -> CResult<()> {
    let caller = ic_cdk::api::caller();
    if caller.to_text() != "pfkhr-mutkw-nrub5-la442-jyhrk-fl7ht-tc7qp-qsihq-d7wdk-6insr-6qe" {
        return Err(CError::Internal);
    }
    TELEMETRY_DATA.with(|inner| {
        inner
            .borrow_mut()
            .insert(id.clone(), TelemetryData { hash, signature });
    });
    Ok(())
}

#[ic_cdk::query]
fn get_data(id: String) -> CResult<TelemetryData> {
    TELEMETRY_DATA.with(|inner| inner.borrow().get(&id).ok_or(CError::NotFound))
}

#[macro_export]
macro_rules! impl_storable {
    ($struct_name:ident) => {
        impl Storable for $struct_name {
            const BOUND: Bound = Bound::Bounded {
                max_size: u32::MAX,
                is_fixed_size: false,
            };

            fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
                Decode!(bytes.as_ref(), Self).unwrap()
            }

            fn to_bytes(&'_ self) -> std::borrow::Cow<'_, [u8]> {
                Cow::Owned(Encode!(self).unwrap())
            }
        }
    };
}

ic_cdk::export_candid!();
