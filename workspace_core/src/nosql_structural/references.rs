use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, bincode_next::Encode, bincode_next::Decode)]
pub struct DbRef {
    pub collection: String,
    pub doc_id: Uuid,
    pub ref_type: RefType,
}

#[derive(Debug, Deserialize, Serialize, Clone, bincode_next::Encode, bincode_next::Decode)]
pub enum RefType {
    OneToOne,
    OneToMany,
    ManyToMany,
}

#[derive(Debug, Deserialize, Serialize, Clone, bincode_next::Encode, bincode_next::Decode)]
pub struct ForeignKey {
    pub from: DbRef,
    pub to: DbRef,
    pub cascade_delete: bool,
}
