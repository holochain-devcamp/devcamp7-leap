use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::LinkMatch;
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    AGENT_ADDRESS,
};
use holochain_entry_utils::HolochainEntry;

use super::anchor::{
    CourseAnchor, COURSE_ANCHOR_TO_SECTION_ANCHOR_LINK, COURSE_ANCHOR_TO_STUDENT_LINK,
    STUDENT_TO_COURSE_ANCHOR_LINK, TEACHER_TO_COURSE_ANCHOR_LINK,
};
use super::catalog_anchor::CourseCatalogAnchor;
use super::entry::Course;
use crate::anchor_trait::AnchorTrait;
use crate::helper;
use crate::section::anchor::SectionAnchor;

pub fn create(title: String, timestamp: u64) -> ZomeApiResult<Address> {
    // if catalog anchor already exists, this function would just return it's address without actually writing anything
    // new to the DHT.
    let catalog_anchor_address = hdk::commit_entry(&CourseCatalogAnchor::new().entry())?;

    // just a helper variable because we'll need this value a few times
    let teacher_address = AGENT_ADDRESS.clone();

    // initialize CourseAnchor instance to represent this particular course
    let course_anchor = CourseAnchor::new(title.clone(), teacher_address.clone(), timestamp);
    // commit CourseAnchor to DHT
    let course_anchor_address = hdk::commit_entry(&course_anchor.entry())?;

    // create new Course entry
    let new_course = Course::new(
        title,
        teacher_address.to_owned().into(),
        timestamp,
        course_anchor_address.clone(),
    );
    // commit this entry to DHT and save it's address
    let new_course_address = hdk::commit_entry(&new_course.entry())?;

    // link CourseAnchor to Course entry
    hdk::link_entries(
        &course_anchor_address,
        &new_course_address,
        CourseAnchor::link_type(),
        "".to_owned(),
    )?;

    // link CourseCatalogAnchor to CourseAnchor entry for this course to be findable
    hdk::link_entries(
        &catalog_anchor_address,
        &course_anchor_address,
        CourseCatalogAnchor::link_type(),
        "".to_owned(),
    )?;

    // link address of the agent who called course::create to CourseAnchor
    // for this course to be findable in the list of courses that agent teaches
    hdk::link_entries(
        &AGENT_ADDRESS,
        &course_anchor_address,
        TEACHER_TO_COURSE_ANCHOR_LINK,
        "",
    )?;

    Ok(course_anchor_address)
}

// wrapper for a generic helper::get_latest_data_entry that instantiates it
// specifically for the Course datatype
pub fn get_latest_course(
    course_anchor_address: &Address,
) -> ZomeApiResult<Option<(Course, Address)>> {
    helper::get_latest_data_entry::<Course>(course_anchor_address, &CourseAnchor::link_type())
}

// wrapper for the get_latest_course that only returns Course entry
// and disregards it's address
pub fn get_latest_course_entry(course_anchor_address: Address) -> ZomeApiResult<Option<Course>> {
    let latest_course_result = get_latest_course(&course_anchor_address)?;
    match latest_course_result {
        Some((course_entry, _course_entry_address)) => {
            return Ok(Some(course_entry));
        }
        None => return Ok(None),
    }
}

// NOTE: this function isn't public because it's only needed in the current module
fn commit_update(
    course: Course,
    previous_course_address: &Address,
    course_anchor_address: &Address,
) -> ZomeApiResult<Address> {
    // commit updated course to DHT and get it's new address
    let new_course_address = hdk::update_entry(course.entry(), previous_course_address)?;

    // remove link to previous version of course
    hdk::remove_link(
        course_anchor_address,
        &previous_course_address,
        CourseAnchor::link_type(),
        "".to_owned(),
    )?;

    // create link to new version of course
    hdk::link_entries(
        course_anchor_address,
        &new_course_address,
        CourseAnchor::link_type(),
        "".to_owned(),
    )?;

    Ok(course_anchor_address.to_owned())
}

pub fn update(title: String, course_anchor_address: &Address) -> ZomeApiResult<Address> {
    let latest_course_result = get_latest_course(course_anchor_address)?;
    match latest_course_result {
        Some((mut previous_course, previous_course_address)) => {
            // update this course
            previous_course.title = title;

            commit_update(
                previous_course,
                &previous_course_address,
                course_anchor_address,
            )?;

            // returning address of the course anchor. Sure, it doesn't change, but it makes our API consistent with hdk:: API
            // that always returns address of an updated entry
            return Ok(course_anchor_address.clone());
        }
        None => {
            return Err(ZomeApiError::from(
                "Can't update a deleted course".to_owned(),
            ));
        }
    }
}

pub fn delete(course_anchor_address: Address) -> ZomeApiResult<Address> {
    // retrieve course_anchor entry. If it doesn't exist, we'll fail with error here so we're also validating input
    let course_anchor: CourseAnchor = hdk::utils::get_as_type(course_anchor_address.clone())?;

    // remove link from CourseCatalogAnchor to CourseAnchor
    hdk::remove_link(
        &CourseCatalogAnchor::new().address()?,
        &course_anchor_address,
        CourseCatalogAnchor::link_type(),
        "".to_owned(),
    )?;

    // retrieve list of students that have enrolled in this course
    let students = get_students(course_anchor_address.clone())?;
    // go through all students and remove their links to this course
    for student in students {
        hdk::remove_link(
            &student,
            &course_anchor_address,
            STUDENT_TO_COURSE_ANCHOR_LINK,
            "",
        )?;
    }

    // NOTE: using the fact that course_anchor stores teacher_address and that we don't allow to change teacher's address ever
    // so we don't have to retrieve the latest Course entry to get the teacher address and it makes this method a little bit faster
    hdk::remove_link(
        &course_anchor.teacher_address,
        &course_anchor_address,
        TEACHER_TO_COURSE_ANCHOR_LINK,
        "",
    )?;

    // NOTE: let's try only deleting an anchor! (and don't touch links from anchor to Course entry and Course entry itself)
    // reasons:
    // 1) without it, we won't be able to reach the Course because everywhere we link to course we only use anchor address
    // 2) we'll avoid polluting DHT by new deletion metadata
    hdk::remove_entry(&course_anchor_address)
}

pub fn list_all_courses() -> ZomeApiResult<Vec<Address>> {
    let addresses = hdk::get_links(
        &CourseCatalogAnchor::new().address()?,
        LinkMatch::Exactly(&CourseCatalogAnchor::link_type()),
        LinkMatch::Any,
    )?
    .addresses();

    Ok(addresses)
}

pub fn get_my_courses() -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &AGENT_ADDRESS,
        LinkMatch::Exactly(TEACHER_TO_COURSE_ANCHOR_LINK),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}

pub fn get_my_enrolled_courses() -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &AGENT_ADDRESS,
        LinkMatch::Exactly(STUDENT_TO_COURSE_ANCHOR_LINK),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}

pub fn get_students(course_anchor_address: Address) -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &course_anchor_address,
        LinkMatch::Exactly(COURSE_ANCHOR_TO_STUDENT_LINK),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}

// NOTE: fun fact for fellow English learners: there isn't a typo because both "enrol" and "enroll" are valid!
//  See: https://grammarist.com/spelling/enrol-enroll/ for more details
pub fn enrol_in_course(course_anchor_address: Address) -> ZomeApiResult<Address> {
    // create a link that would allow student to find course they've enrolled into
    hdk::link_entries(
        &AGENT_ADDRESS,
        &course_anchor_address,
        STUDENT_TO_COURSE_ANCHOR_LINK,
        "",
    )?;
    // create a link that would allow course to list it's students
    hdk::link_entries(
        &course_anchor_address,
        &AGENT_ADDRESS,
        COURSE_ANCHOR_TO_STUDENT_LINK,
        "",
    )
}

pub fn add_section(
    course_anchor_address: &Address,
    section_anchor_address: &Address,
) -> ZomeApiResult<Address> {
    // retrieve course anchor to validate that it exists. We won't need the actual value so prefix it with _
    let _course_anchor: CourseAnchor = hdk::utils::get_as_type(course_anchor_address.clone())?;
    // retrieve section anchor to validate that it exists. We won't need the actual value so prefix it with _
    let _section_anchor: SectionAnchor = hdk::utils::get_as_type(section_anchor_address.clone())?;

    // now create an explicit link from course anchor to the section anchor
    hdk::link_entries(
        course_anchor_address,
        section_anchor_address,
        COURSE_ANCHOR_TO_SECTION_ANCHOR_LINK,
        "",
    );

    Ok(course_anchor_address.clone())
}

pub fn delete_section(
    course_anchor_address: &Address,
    section_anchor_address: &Address,
) -> ZomeApiResult<Address> {
    // retrieve course anchor to validate that it exists. We won't need the actual value so prefix it with _
    let _course_anchor: CourseAnchor = hdk::utils::get_as_type(course_anchor_address.clone())?;
    // retrieve section anchor to validate that it exists. We won't need the actual value so prefix it with _
    let _section_anchor: SectionAnchor = hdk::utils::get_as_type(section_anchor_address.clone())?;

    // now remove an explicit link from course anchor to the section anchor, therefore removing the section
    hdk::remove_link(
        course_anchor_address,
        section_anchor_address,
        COURSE_ANCHOR_TO_SECTION_ANCHOR_LINK,
        "",
    )?;

    Ok(course_anchor_address.clone())
}
