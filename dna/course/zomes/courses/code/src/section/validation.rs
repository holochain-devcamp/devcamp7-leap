use crate::anchor_trait::AnchorTrait;
use hdk::holochain_core_types::chain_header::ChainHeader;
use hdk::{LinkValidationData, ValidationData};
use holochain_entry_utils::HolochainEntry;

use super::{
    anchor::SectionAnchor,
    entry::{Section, MAX_TITLE_LEN},
};
use crate::helper;

pub fn create(entry: Section, validation_data: ValidationData) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &entry.teacher_address,
        validation_data.sources(),
        "create sections in this course",
    )?;
    helper::validate_entity_title(&entry.title, &Section::entry_type(), MAX_TITLE_LEN)
}

pub fn modify(
    new_entry: Section,
    old_entry: Section,
    _old_entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &old_entry.teacher_address,
        validation_data.sources(),
        "modify sections in this course",
    )?;
    if new_entry.course_address != old_entry.course_address {
        return Err(String::from(
            "Cannot change course to which the section belongs",
        ));
    }
    helper::validate_entity_title(&new_entry.title, &Section::entry_type(), MAX_TITLE_LEN)
}

pub fn delete(
    entry: Section,
    _entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &entry.teacher_address,
        validation_data.sources(),
        "delete sections in this course",
    )
}

// =========================== SectionAnchor validation
pub fn anchor_create(entry: SectionAnchor, validation_data: ValidationData) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &entry.teacher_address,
        validation_data.sources(),
        "create sections in this course",
    )?;
    helper::validate_entity_title(&entry.title, &SectionAnchor::entry_type(), MAX_TITLE_LEN)
}

// NOTE: we don't accept any parameters here because we don't need them to always return an error
// because this anchor can never be modified
pub fn anchor_modify() -> Result<(), String> {
    Err(String::from(
        "Can't modify the SectionAnchor entry: it can only be created or deleted".to_owned(),
    ))
}

pub fn anchor_delete(
    entry: SectionAnchor,
    _entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &entry.teacher_address,
        validation_data.sources(),
        "delete sections in this course",
    )
}

//  =========================== SectionAnchor links validation
pub fn anchor_to_section_link(validation_data: LinkValidationData) -> Result<(), String> {
    match validation_data {
        hdk::LinkValidationData::LinkAdd {
            link,
            validation_data,
        } => {
            // get author of this entry
            let author = validation_data.package.chain_header.provenances()[0].source();
            // get link base: entry from which the link goes
            let base: SectionAnchor = hdk::utils::get_as_type(link.link.base().clone())?;
            // get link target: entry to which the link goes
            let target: SectionAnchor = hdk::utils::get_as_type(link.link.target().clone())?;
            if base.teacher_address != target.teacher_address {
                // notice that we're using return and ending this statement with ; symbol
                // You can do both: skip ; symbol in the last fn statement or explicitly add return to it and then leave ; as is
                return Err(String::from(
                    "Can't link SectionAnchor to Section because their teacher addresses are different",
                ));
            } else if author != base.teacher_address {
                return Err(String::from(
                    "Can't link SectionAnchor to Section because your address isn't specified as teacher address for this course",
                ));
            }
            if base.course_address != target.course_address {
                return Err(String::from(
                    "Can't link SectionAnchor to Section because they belong to different courses",
                ));
            }
            Ok(())
        }
        hdk::LinkValidationData::LinkRemove {
            link,
            validation_data,
        } => {
            // get author of this entry
            let author = validation_data.package.chain_header.provenances()[0].source();
            // get link base: entry from which the link goes
            let base: SectionAnchor = hdk::utils::get_as_type(link.link.base().clone())?;
            if author != base.teacher_address {
                return Err(String::from(
                    "Can't remove link from SectionAnchor to Section because your address isn't specified as teacher_address for this course",
                ));
            }
            Ok(())
        }
    }
}
