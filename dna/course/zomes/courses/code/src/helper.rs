use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_persistence_api::cas::content::Address,
    prelude::LinkMatch,
};
use holochain_entry_utils::HolochainEntry;

// gets latest data entry that is linked to anchor at entry_anchor_address
// This is a helper for anchor-first pattern entries
pub fn get_latest_data_entry<T: HolochainEntry>(
    entry_anchor_address: &Address,
    link_type: &str,
) -> ZomeApiResult<Option<(T, Address)>> {
    // since we're only deleting anchor when deletining entry with anchor (and leave data
    // entires and links to them as is), we need to check if anchor is deleted.
    // And get_entry won't return anything if anchor at entry_anchor_address is deleted
    let get_entry_result = hdk::get_entry(entry_anchor_address)?;
    match get_entry_result {
        // anchor isn't deleted and get_entry returned instance of T type
        Some(_entry_anchor) => {
            let entry_addresses = hdk::get_links(
                entry_anchor_address,
                LinkMatch::Exactly(link_type),
                // this parameter is for link tags. since we don't tag anchor->data entry link (see method create above)
                //  we need to ask for all tags
                LinkMatch::Any,
            )?
            .addresses();

            // NOTE: this could be handled by:
            // 1. versioning all links from the anchor from the beginning in course::create and course::update
            // 2. retrieveing all deleted links
            // 3. sorting them in increasing order and finding the latest one
            // 4. using Course entry address from this link as latest_entry_address
            // We're not going into that right now to not overload people with the content
            if entry_addresses.len() != 1 {
                return Err(ZomeApiError::from(
                    "Something is wrong with links from CourseAnchor to Course".to_owned(),
                ));
            } else {
                let latest_entry_address = entry_addresses[0].clone();
                let latest_entry: T = hdk::utils::get_as_type(latest_entry_address.clone())?;
                // our return value is a Result container on the outside that holds Option container that holds a tuple
                // we write Ok() to init Result's value, Some to init Option's value and then inside we have our tuple
                return Ok(Some((latest_entry, latest_entry_address)));
            }
        }
        // anchor is deleted so we're returning None
        None => return Ok(None),
    }
}
