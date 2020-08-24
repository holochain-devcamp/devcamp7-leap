use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{dna::entry_types::Sharing, validation::EntryValidationData},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};
use holochain_entry_utils::HolochainEntry;

use super::validation;

pub const MAX_TITLE_LEN: usize = 200;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Section {
    pub title: String,
    pub course_address: Address,
    pub timestamp: u64,
    pub anchor_address: Address,
    // NOTE: storing teacher_address sure requires more memory for each and every Section entry but instead
    // it gives us the ability to quickly validate that only the Course's teacher is modifying / deleting this Section entry.
    // So we're trading smaller memory footprint for a less error-prone and less CPU-intensive validation, because it
    // won't now rely on data retrieval from DHT (that could be unavailable for so many reasons).
    // If you don't like neither of these options, there's another one: store each course in a separate DNA where teacher_address is
    // just a DNA property: it's retrieval has constant time (because it's always there on every device). But that's a totally different topic.
    pub teacher_address: Address,
}

impl Section {
    pub fn new(
        title: String,
        course_address: Address,
        timestamp: u64,
        anchor_address: Address,
        teacher_address: Address,
    ) -> Self {
        Section {
            title: title,
            course_address: course_address,
            timestamp: timestamp,
            anchor_address: anchor_address,
            teacher_address: teacher_address,
        }
    }
}

impl HolochainEntry for Section {
    fn entry_type() -> String {
        String::from("section")
    }
}

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: Section::entry_type(),
        description: "this is the definition of section",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Section>| {
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
        },
        // Since now Section entry is a data entry that is hidden behind the SectionAnchor,
        // there won't be any links that it has.
        links:[]
    )
}
