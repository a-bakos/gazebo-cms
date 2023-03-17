use crate::app::App;
use crate::consts;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum ResourceType {
    User,
    Entry,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ResourceID {
    UserID(u32),
    EntryID(u32),
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct ResourceManager {
    allocated_ID: HashMap<ResourceType, Vec<ResourceID>>,
    last_allocated_ID: HashMap<ResourceType, ResourceID>,
}

#[allow(non_camel_case_types)]
pub trait ID_Allocator {
    fn allocate(app: &mut App) -> Self;
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            allocated_ID: HashMap::new(),
            last_allocated_ID: HashMap::new(),
        }
    }

    pub fn add_to_allocated(&mut self, resource_type: ResourceType, resource_id: ResourceID) {
        let resource_id = resource_id;

        // If resource type exists
        if self.allocated_ID.get(&resource_type).is_some() {
            let id_list = self.allocated_ID.get_mut(&resource_type).unwrap();

            if id_list.contains(&resource_id) {
                unreachable!()
            }

            id_list.push(resource_id);
            self.last_allocated_ID.insert(resource_type, resource_id);
        } else {
            self.allocated_ID.insert(resource_type, vec![resource_id]);
            self.last_allocated_ID.insert(resource_type, resource_id);
        }
    }

    fn remove_from_allocated(&mut self, resource_type: ResourceType, resource_id: ResourceID) {
        todo!()
    }

    pub fn get_next_available_id(app: &App, resource_type: ResourceType) -> ResourceID {
        match resource_type {
            ResourceType::User => {
                // acquire next available ID
                ResourceID::UserID(2)
            }
            ResourceType::Entry => {
                // Check if we have last allocated ID stored otherwise start from 1
                if let Some(id) = app.resources.last_allocated_ID.get(&resource_type) {
                    // todo
                    // https://stackoverflow.com/questions/41207885/using-generic-trait-methods-like-into-when-type-inference-is-impossible
                    let try_id = 1 + 1; //id.into() + 1;

                    if app.resources.allocated_ID.get(&resource_type).is_some() {
                        return ResourceID::EntryID(try_id);
                    } else {
                        return ResourceID::EntryID(0);
                    }
                } else {
                    return ResourceID::EntryID(consts::ID_START_VALUE);
                }
            }
        }
    }
}
