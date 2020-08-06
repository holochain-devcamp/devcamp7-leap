// allowing for this Rust project to have dead code on a crate level
#![allow(dead_code)]
// unstable Rust feature
// See more at: https://doc.rust-lang.org/nightly/unstable-book/language-features/proc-macro-hygiene.html
#![feature(proc_macro_hygiene)]
// This isn't a mistake that there are multiple #[macro_use] below: each applies to a particular crate that follows it
// specifying that we want to import macros defined in this crate too.
// See more at: https://doc.rust-lang.org/reference/macros-by-example.html
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::prelude::*;

use hdk_proc_macros::zome;

mod anchor_trait;
mod course;

#[zome]
mod courses {

    // Things to be done on an hApp init, we skip this for now
    #[init]
    fn init() {
        Ok(())
    }

    // Things to be done to validate each agent in the network, we skip this for now
    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    //  ====================== Course definitions
    #[entry_def]
    fn course_catalog_anchor_entry_definition() -> ValidatingEntryType {
        course::catalog_anchor::catalog_anchor_entry_def()
    }

    #[entry_def]
    fn course_anchor_definition() -> ValidatingEntryType {
        course::anchor::course_anchor_def()
    }

    #[entry_def]
    fn course_entry_definition() -> ValidatingEntryType {
        course::entry::course_entry_def()
    }

    // Section
    // TODO: implement section entry definitions

    // Content
    // TODO: implement content entry definition
}
