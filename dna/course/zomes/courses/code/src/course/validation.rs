use super::{
    anchor::CourseAnchor,
    catalog_anchor::CourseCatalogAnchor,
    entry::{Course, MAX_TITLE_LEN},
};
use crate::anchor_trait::AnchorTrait;
use crate::helper;
use hdk::holochain_core_types::chain_header::ChainHeader;
use hdk::{LinkValidationData, ValidationData};
use holochain_entry_utils::HolochainEntry;

pub fn create(entry: Course, validation_data: ValidationData) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &entry.teacher_address,
        validation_data.sources(),
        "create their courses",
    )?;
    helper::validate_entity_title(&entry.title, &Course::entry_type(), MAX_TITLE_LEN)
}

pub fn modify(
    new_entry: Course,
    old_entry: Course,
    _old_entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &old_entry.teacher_address,
        validation_data.sources(),
        "modify their courses",
    )?;
    helper::validate_entity_title(&new_entry.title, &Course::entry_type(), MAX_TITLE_LEN)?;
    helper::validate_no_teacher_change(
        old_entry.teacher_address,
        new_entry.teacher_address,
        &Course::entry_type(),
    )
}

pub fn delete(
    entry: Course,
    _entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &entry.teacher_address,
        validation_data.sources(),
        "delete their courses",
    )
}

// =========================== CourseAnchor validation
pub fn anchor_create(entry: CourseAnchor, validation_data: ValidationData) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &entry.teacher_address,
        validation_data.sources(),
        "create their courses",
    )?;
    helper::validate_entity_title(&entry.title, &CourseAnchor::entry_type(), MAX_TITLE_LEN)
}

// NOTE: we don't accept any parameters here because we don't need them to always return an error
// because this anchor can never be modified
pub fn anchor_modify() -> Result<(), String> {
    Err(String::from(
        "Can't modify the CourseAnchor entry: it can only be created or deleted",
    ))
}

pub fn anchor_delete(
    entry: CourseAnchor,
    _entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &entry.teacher_address,
        validation_data.sources(),
        "delete their courses",
    )
}

// =========================== CourseCatalogAnchor validation
// Anyone can create the CourseCatalogAnchor and there isn't anything we need to validate about it
pub fn catalog_create(
    _entry: CourseCatalogAnchor,
    _validation_data: ValidationData,
) -> Result<(), String> {
    Ok(())
}

// NOTE: we don't accept any parameters here because we don't need them to always return an error
// because this anchor can never be modified
pub fn catalog_modify() -> Result<(), String> {
    Err(String::from("Can't modify the CourseAnchorCatalog entry"))
}

// NOTE: we don't accept any parameters here because we don't need them to always return an error
// because this anchor can never be deleted
pub fn catalog_delete() -> Result<(), String> {
    Err(String::from("Can't delete the CourseAnchorCatalog entry"))
}

//  =========================== CourseAnchor links validation

pub fn anchor_to_course_link(validation_data: LinkValidationData) -> Result<(), String> {
    match validation_data {
        hdk::LinkValidationData::LinkAdd {
            link,
            validation_data,
        } => {
            // get author of this entry
            let author = validation_data.package.chain_header.provenances()[0].source();
            // get link base: entry from which the link goes
            let base: CourseAnchor = hdk::utils::get_as_type(link.link.base().clone())?;
            // get link target: entry to which the link goes
            let target: Course = hdk::utils::get_as_type(link.link.target().clone())?;
            if base.teacher_address != target.teacher_address {
                // notice that we're using return and ending this statement with ; symbol
                // You can do both: skip ; symbol in the last fn statement or explicitly add return to it and then leave ; as is
                return Err(String::from(
                    "Can't link CourseAnchor to Course because their teacher addresses are different",
                ));
            } else if author != base.teacher_address {
                return Err(String::from(
                    "Can't link CourseAnchor to Course because your address isn't specified as teacher address for this course",
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
            let base: CourseAnchor = hdk::utils::get_as_type(link.link.base().clone())?;
            if author != base.teacher_address {
                return Err(String::from(
                    "Can't remove link from CourseAnchor to Course because your address isn't specified as teacher_address for this course",
                ));
            }
            Ok(())
        }
    }
}
