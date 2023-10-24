use std::collections::HashMap;

pub use as_form_derive::AsForm;
pub use as_jsonb_derive::AsJsonb;
pub use as_table_derive::AsTable;
pub use meta_data_derive::MetaData;
pub use reflective_derive::Reflective;
pub use set_cipher_derive::SetCipher;
pub use set_enum_derive::SetEnum;
pub use set_is_empty_derive::SetIsEmpty;
pub use set_mutate_derive::SetMutate;

pub trait AsForm {}

pub trait AsJsonb {}

pub trait AsTable {}

pub trait MetaData {
    fn author(&self) -> &'static str;
    fn serial_version(&self) -> usize;
    fn field_authors(&self) -> HashMap<&'static str, &'static str>;
}

pub trait Reflective {
    fn name(&self) -> &'static str;
    fn fields(&self) -> Vec<&'static str>;
}

pub trait SetCipher {}

pub trait SetEnum {
    fn new<T: ToString>(s: T) -> Self;
    fn from_str<T: ToString>(s: T) -> Self;
    fn to_str(&self) -> &'static str;
}

pub trait SetIsEmpty {}

pub trait SetMutate {}

