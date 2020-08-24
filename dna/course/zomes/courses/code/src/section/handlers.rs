use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

use super::anchor::SectionAnchor;
use super::entry::Section;
use crate::anchor_trait::AnchorTrait;
use crate::course;
use crate::helper;

pub fn create(
    title: String,
    course_anchor_address: &Address,
    timestamp: u64,
) -> ZomeApiResult<Address> {
    // retrieve latest course at course_address. If this address isn't valid or the course entry is deleted,
    // this returns None. We run a match to make sure the course exist and return error if it doesn't.
    let latest_course_result = course::handlers::get_latest_course(course_anchor_address)?;

    match latest_course_result {
        Some((previous_course, _previous_course_address)) => {
            // initialize SectionAnchor instance
            let section_anchor = SectionAnchor::new(
                title.clone(),
                course_anchor_address.clone(),
                timestamp,
                previous_course.teacher_address.clone(),
            );
            // commit SectionAnchor to DHT
            let section_anchor_address = hdk::commit_entry(&section_anchor.entry())?;

            // initialize Section instance without commiting it to DHT: we'll need it to commit anchor
            let new_section = Section::new(
                title,
                course_anchor_address.clone(),
                timestamp,
                section_anchor_address.clone(),
                previous_course.teacher_address,
            );
            // commit Section to DHT
            let new_section_address = hdk::commit_entry(&new_section.entry())?;

            hdk::link_entries(
                &section_anchor_address,
                &new_section_address,
                SectionAnchor::link_type(),
                "".to_owned(),
            )?;

            course::handlers::add_section(
                &course_anchor_address,
                &section_anchor_address,
                timestamp,
            )?;
            // SectionAnchor serves as this section's ID so we return it
            Ok(section_anchor_address)
        }
        None => {
            return Err(ZomeApiError::from(
                "Can't create a section in deleted course".to_owned(),
            ));
        }
    }
}

pub fn get_latest_section(
    section_anchor_address: &Address,
) -> ZomeApiResult<Option<(Section, Address)>> {
    helper::get_latest_data_entry::<Section>(section_anchor_address, &SectionAnchor::link_type())
}

pub fn get_latest_section_entry(section_anchor_address: Address) -> ZomeApiResult<Option<Section>> {
    let latest_section_result = get_latest_section(&section_anchor_address)?;
    match latest_section_result {
        Some((section_entry, _section_entry_address)) => {
            return Ok(Some(section_entry));
        }
        None => return Ok(None),
    }
}

pub fn update(
    title: String,
    section_anchor_address: &Address,
    timestamp: u64,
) -> ZomeApiResult<Address> {
    let latest_section_result = get_latest_section(section_anchor_address)?;
    match latest_section_result {
        Some((mut previous_section, previous_section_address)) => {
            // update the section
            previous_section.title = title;
            previous_section.timestamp = timestamp;
            // commit this update to the DHT.
            let new_section_address =
                hdk::update_entry(previous_section.entry(), &previous_section_address)?;

            // remove link to previous version of section
            hdk::remove_link(
                section_anchor_address,
                &previous_section_address,
                SectionAnchor::link_type(),
                "".to_owned(),
            )?;

            // create link to new version of section
            hdk::link_entries(
                section_anchor_address,
                &new_section_address,
                SectionAnchor::link_type(),
                "".to_owned(),
            )?;

            Ok(section_anchor_address.clone())
        }
        None => {
            return Err(ZomeApiError::from(
                "Can't update a deleted section".to_owned(),
            ));
        }
    }
}

pub fn delete(section_anchor_address: Address, timestamp: u64) -> ZomeApiResult<Address> {
    let section_anchor: SectionAnchor = hdk::utils::get_as_type(section_anchor_address.clone())?;

    // NOTE: we're using the fact that anchor contains course_address and that we don't allow
    //  to change course_address in a section entry.
    // By doing so, we avoid necessity to query links of the section_anchor to retrieve the latest section entry
    // which makes this method a little bit faster
    course::handlers::delete_section(
        &section_anchor.course_address,
        &section_anchor_address,
        timestamp,
    )?;

    // NOTE: let's try only deleting an anchor! (and don't touch links from anchor to section entry and section entry itself)
    // reasons:
    // 1) without it, we won't be able to reach the section because everywhere we link to section we only use anchor address
    // 2) we'll avoid polluting DHT by new deletion metadata
    let result = hdk::remove_entry(&section_anchor_address)?;
    Ok(result)
}
