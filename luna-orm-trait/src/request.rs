use crate::{Entity, Location, Mutation, Primary};
use serde::{Deserialize, Serialize};

//#[derive(Serialize, Deserialize)]
//#[serde(untagged)]
pub enum WriteCommand {
    Insert {
        entity: Box<dyn Entity>,
    },
    Upsert {
        entity: Box<dyn Entity>,
    },
    Update {
        mutation: Box<dyn Mutation>,
        primary: Box<dyn Primary>,
    },
    Change {
        mutation: Box<dyn Mutation>,
        location: Box<dyn Location>,
    },
    Delete {
        primary: Box<dyn Primary>,
    },

    Purify {
        location: Box<dyn Location>,
    },
}
