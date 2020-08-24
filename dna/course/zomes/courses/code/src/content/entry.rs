use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

use super::validation;

// NOTE: using self::DefaultJson to disambiguate usage of DefaultJson from this module (hdk::prelude imports it)
#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct Content {
    pub name: String,
    pub url: String,
    pub description: String,
    pub timestamp: u64,
    pub section_anchor_address: Address,
    // NOTE: storing teacher_address sure requires more memory for each and every Content entry but instead
    // it gives us the ability to quickly validate that only the Course's teacher is modifying / deleting this Content entry.
    // So we're trading smaller memory footprint for a less error-prone and less CPU-intensive validation, because it
    // won't now rely on data retrieval from DHT (that could be unavailable for so many reasons).
    // If you don't like neither of these options, there's another one: store each course in a separate DNA where teacher_address is
    // just a DNA property: it's retrieval has constant time (because it's always there on every device). But that's a totally different topic.
    pub teacher_address: Address,
}

impl Content {
    pub fn new(
        name: String,
        section_anchor_address: Address,
        url: String,
        timestamp: u64,
        description: String,
        teacher_address: Address,
    ) -> Self {
        Content {
            name,
            url,
            description,
            timestamp,
            section_anchor_address,
            teacher_address: teacher_address,
        }
    }
}

impl HolochainEntry for Content {
    fn entry_type() -> String {
        String::from("content")
    }
}

pub fn content_entry_def() -> ValidatingEntryType {
    entry!(
        name: Content::entry_type(),
        description: "this is the content for each section",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Content>| {
            match  validation_data {
                EntryValidationData::Create { entry, validation_data } => {
                    validation::create(entry, validation_data)
                },
                EntryValidationData::Modify { new_entry, old_entry, old_entry_header, validation_data } => {
                    validation::modify(new_entry, old_entry, old_entry_header, validation_data)
                },
                EntryValidationData::Delete { old_entry, old_entry_header, validation_data } => {
                    validation::delete(old_entry, old_entry_header, validation_data)
                }
            }
        }
    )
}
