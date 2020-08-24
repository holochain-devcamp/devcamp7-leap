use super::{entry::Section, validation};
use crate::anchor_trait::AnchorTrait;
use crate::content::entry::Content;

use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

pub const SECTION_TO_CONTENT_LINK: &str = "section_anchor->content";

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct SectionAnchor {
    // NOTE: these fields are here to ensure the uniqueness of every particular anchor
    //  and wouldn't be used to display data about section to a user
    pub title: String,
    pub course_address: Address,
    pub timestamp: u64,
    // NOTE: storing teacher_address sure requires more memory for each and every SectionAnchor entry but instead
    // it gives us the ability to quickly validate that only the Course's teacher is deleting this SectionAnchor entry.
    // So we're trading smaller memory footprint for a less error-prone and less CPU-intensive validation, because it
    // won't now rely on data retrieval from DHT (that could be unavailable for so many reasons).
    // If you don't like neither of these options, there's another one: store each course in a separate DNA where teacher_address is
    // just a DNA property: it's retrieval has constant time (because it's always there on every device). But that's a totally different topic.
    pub teacher_address: Address,
}

impl AnchorTrait for SectionAnchor {
    fn entry_type() -> String {
        String::from("section_anchor")
    }
    fn link_to() -> String {
        Section::entry_type()
    }
    fn link_type() -> String {
        "section_anchor->section".to_string()
    }
}

impl SectionAnchor {
    pub fn new(
        title: String,
        course_address: Address,
        timestamp: u64,
        teacher_address: Address,
    ) -> Self {
        SectionAnchor {
            title: title,
            course_address: course_address,
            timestamp: timestamp,
            teacher_address: teacher_address,
        }
    }
}

pub fn section_anchor_def() -> ValidatingEntryType {
    entry!(
        name: SectionAnchor::entry_type(),
        description: "Anchor to the valid course section",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<SectionAnchor>| {
            match validation_data{
                EntryValidationData::Create { entry, validation_data } => {
                    validation::anchor_create(entry, validation_data)
                 },
                 // NOTE: the symbol .. means that we're skipping unpacking parameters that we receive here
                 // because we won't need them
                 EntryValidationData::Modify { .. } => {
                    validation::anchor_modify()
                 },
                 EntryValidationData::Delete { old_entry, old_entry_header, validation_data } => {
                    validation::anchor_delete(old_entry, old_entry_header, validation_data)
                 }
            }
        },
        links:[
            to!(
                SectionAnchor::link_to(),
                link_type: SectionAnchor::link_type(),
                validation_package:||{
                    hdk::ValidationPackageDefinition::Entry
                },
                validation:|validation_data: hdk::LinkValidationData|{
                   validation::anchor_to_section_link(validation_data)
                }
            ),
            to!(
                Content::entry_type(),
                link_type: SECTION_TO_CONTENT_LINK,
                validation_package:||{
                    hdk::ValidationPackageDefinition::Entry
                },
                validation:|_validation_data: hdk::LinkValidationData|{
                    Ok(())
                }
            )
        ]
    )
}
