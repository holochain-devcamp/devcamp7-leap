use super::entry::Content;
use crate::helper;
use hdk::holochain_core_types::chain_header::ChainHeader;
use hdk::ValidationData;
use holochain_entry_utils::HolochainEntry;

pub fn create(entry: Content, validation_data: ValidationData) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &entry.teacher_address,
        validation_data.sources(),
        "create content in the section of this course",
    )
}

pub fn modify(
    new_entry: Content,
    old_entry: Content,
    _old_entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &old_entry.teacher_address,
        validation_data.sources(),
        "modify content in the section of this course",
    )?;
    helper::validate_no_teacher_change(
        old_entry.teacher_address,
        new_entry.teacher_address,
        &Content::entry_type(),
    )
}

pub fn delete(
    entry: Content,
    _entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &entry.teacher_address,
        validation_data.sources(),
        "delete content in the section of this course",
    )
}
