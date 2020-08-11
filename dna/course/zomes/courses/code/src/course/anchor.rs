use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

use super::entry::Course;
use crate::anchor_trait::AnchorTrait;
use crate::section::anchor::SectionAnchor;

pub const TEACHER_TO_COURSE_ANCHOR_LINK: &str = "teacher->course_anchor";
pub const STUDENT_TO_COURSE_ANCHOR_LINK: &str = "student->course_anchor";
pub const COURSE_ANCHOR_TO_STUDENT_LINK: &str = "course_anchor->student";
pub const COURSE_ANCHOR_TO_SECTION_ANCHOR_LINK: &str = "course_anchor->section_anchor";

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct CourseAnchor {
    // NOTE: these fields are here to ensure the uniqueness of every particular anchor
    //  and wouldn't be used to display data about course to a user
    pub title: String,
    pub teacher_address: Address,
    pub timestamp: u64,
}

impl AnchorTrait for CourseAnchor {
    fn entry_type() -> String {
        String::from("course_anchor")
    }
    fn link_to() -> String {
        Course::entry_type()
    }
    fn link_type() -> String {
        "course_anchor->course".to_owned()
    }
}

impl CourseAnchor {
    pub fn new(title: String, teacher_address: Address, timestamp: u64) -> Self {
        CourseAnchor {
            title: title,
            teacher_address: teacher_address,
            timestamp: timestamp,
        }
    }
}

pub fn course_anchor_def() -> ValidatingEntryType {
    entry!(
        name: CourseAnchor::entry_type(),
        description: "Anchor to the valid course",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<CourseAnchor>| {
            match validation_data{
                EntryValidationData::Create { .. } => {
                    Ok(())
                 },
                 EntryValidationData::Modify { .. } => {
                    Ok(())
                 },
                 EntryValidationData::Delete { .. } => {
                    Ok(())
                 }
            }
        },
        links:[
            // link that connects CourseAnchor to the latest Course entry
            // This is a necessary link that allows access to course data
            to!(
                CourseAnchor::link_to(),
                link_type: CourseAnchor::link_type(),
                validation_package:||{
                    hdk::ValidationPackageDefinition::Entry
                },
                validation:|_validation_data: hdk::LinkValidationData|{
                   Ok(())
                }
            ),
            // link from agent that is a teacher of this course
            // This is for teacher to keep track of all the courses that they're teaching
            from!(
                "%agent_id", // this is a special string that would automatically expand to the hdk::AGENT_ADDRESS
                link_type: TEACHER_TO_COURSE_ANCHOR_LINK,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                }              ,
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            ),
            // link from agent that is a student who enrolled in this course
            // This is for student to keep track of all the courses they've enrolled in
            from!(
                "%agent_id", // this is a special string that would automatically expand to the hdk::AGENT_ADDRESS
                link_type: STUDENT_TO_COURSE_ANCHOR_LINK,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                }              ,
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            ),
            // link to an agent who is a student enrolled in this course.
            // This is to keep track of student list from the course perspective
            to!(
                "%agent_id", // this is a special string that would automatically expand to the hdk::AGENT_ADDRESS
                link_type: COURSE_ANCHOR_TO_STUDENT_LINK,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            ),
            to!(
                SectionAnchor::entry_type(),
                link_type: COURSE_ANCHOR_TO_SECTION_ANCHOR_LINK,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )
        ]
    )
}
