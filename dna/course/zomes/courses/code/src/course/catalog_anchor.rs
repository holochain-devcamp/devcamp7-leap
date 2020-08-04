use hdk::prelude::*;
use hdk::{entry_definition::ValidatingEntryType, holochain_core_types::dna::entry_types::Sharing};

use super::anchor::CourseAnchor;
use crate::anchor_trait::AnchorTrait;

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct CourseCatalogAnchor {
    name: String,
}

impl AnchorTrait for CourseCatalogAnchor {
    fn entry_type() -> String {
        String::from("course_catalog_anchor")
    }
    fn link_to() -> String {
        CourseAnchor::entry_type()
    }
    fn link_type() -> String {
        // NOTE: ideas for a better name for this link are welcome!
        // there'll be a single link per every course
        "course_list".to_owned()
    }
}

impl CourseCatalogAnchor {
    pub fn new() -> Self {
        CourseCatalogAnchor {
            name: CourseCatalogAnchor::entry_type(),
        }
    }
}

//// Anchor Definition : This Anchor will be used to query all courses
pub fn catalog_anchor_entry_def() -> ValidatingEntryType {
    entry!(
        name: CourseCatalogAnchor::entry_type(),
        description:"Anchor that serves as a catalog for all Course entries",
        sharing: Sharing::Public,
        validation_package:||{
            hdk::ValidationPackageDefinition::Entry
        },
        validation:|_validation_data: hdk::EntryValidationData<CourseCatalogAnchor>|{
            Ok(())
        },
        links:[
            to!(
                CourseCatalogAnchor::link_to(),
                link_type: CourseCatalogAnchor::link_type(),
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
