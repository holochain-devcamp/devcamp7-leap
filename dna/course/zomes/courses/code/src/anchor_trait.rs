use hdk::prelude::*;
use std::convert::TryFrom;

// thanks to @HedayatAbedijoo for this trait
pub trait AnchorTrait: TryFrom<JsonString> + Into<JsonString> + Clone {
    fn entry_type() -> String;
    fn link_to() -> String;
    fn link_type() -> String;
    fn entry(self) -> Entry {
        Entry::App(Self::entry_type().into(), self.into())
    }

    fn address(&self) -> ZomeApiResult<Address> {
        hdk::entry_address(&self.clone().entry())
    }
}
