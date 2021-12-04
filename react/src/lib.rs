use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use std::iter::FromIterator;

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct InputCellID(Uuid);

/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct ComputeCellID(Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub struct CallbackID(Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

impl CellID {
    fn input_cell_id(self) -> Uuid {
        if let CellID::Input(InputCellID(uuid)) = self {
            uuid
        } else {
            panic!("Cell is not an input cell!");
        }
    }

    fn compute_cell_id(self) -> Uuid {
        if let CellID::Compute(ComputeCellID(uuid)) = self {
            uuid
        } else {
            panic!("Cell is not an compute cell!");
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<'reactor, T> {
    input_cell_map: HashMap<Uuid, T>,
    compute_cell_map_deps: HashMap<Uuid, Vec<CellID>>,
    compute_cell_map_fn: HashMap<Uuid, Box<dyn Fn(&[T]) -> T + 'reactor>>,
    compute_cell_map_cb: HashMap<Uuid, Vec<Uuid>>,
    callback_map: HashMap<Uuid, Box<dyn FnMut(T) + 'reactor>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'reactor, T: Copy + PartialEq + Default> Reactor<'reactor, T> {
    pub fn new() -> Self {
        Reactor {
            input_cell_map: HashMap::new(),
            compute_cell_map_deps: HashMap::new(),
            compute_cell_map_fn: HashMap::new(),
            compute_cell_map_cb: HashMap::new(),
            callback_map: HashMap::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        let input_cell_id = InputCellID(Uuid::new_v4());
        self.input_cell_map.insert(input_cell_id.0, initial);
        input_cell_id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'reactor>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        let compute_cell_id = ComputeCellID(Uuid::new_v4());

        // Verifies if all dependencies exist and throws an error otherwise
        let deps = self.verify_dependencies(dependencies)?;

        self.compute_cell_map_deps.insert(compute_cell_id.0, deps);
        self.compute_cell_map_fn.insert(compute_cell_id.0, Box::new(compute_func));
        Ok(compute_cell_id)
    }

    fn verify_dependencies(&self, deps: &[CellID]) -> Result<Vec<CellID>, CellID> {
        deps.iter().try_fold(Vec::new(), |mut acc, val| {
            match val {
                CellID::Input(id) => {
                    let cell_id = CellID::Input(InputCellID(id.0));
                    if !self.input_cell_map.contains_key(&id.0) {
                        return Err(cell_id);
                    }
                    acc.push(cell_id);
                    Ok(acc)
                },
                CellID::Compute(id) => {
                    let cell_id = CellID::Compute(ComputeCellID(id.0));
                    if !self.compute_cell_map_deps.contains_key(&id.0) {
                        return Err(cell_id);
                    }
                    acc.push(cell_id);
                    Ok(acc)
                }
            }
        })
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(id) => self.input_cell_map.get(&id.0).map(|id| id.clone()),
            CellID::Compute(id) => self.compute_cell_value(id),
        }
    }

    fn compute_cell_value(&self, id: ComputeCellID) -> Option<T> {
        self.compute_cell_map_deps.get(&id.0)
            .map_or(None, |deps| {

                // Compute each dependency cell final value
                let input = deps.iter()
                    .map(|dep| self.value(*dep))
                    // TODO: Possible that a dependency may not exist. What do we do???
                    .map(|val| val.unwrap())
                    .collect::<Vec<T>>();

                // TODO: Same as above. It is possible that this compute function may
                //  not exist. What do we do???
                let compute = self.compute_cell_map_fn.get(&id.0).unwrap();
                Some(compute(&input[..]))
            })
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        if self.input_cell_map.contains_key(&id.0) {
            // Find dependant compute cells
            let dep_compute_cells = self.find_top_lvl_dep_compute_cells(id);

            // Find dependant compute cell current values so that we can compare them to new values
            let mut ocv_store = HashMap::new();
            dep_compute_cells.iter().for_each(|id| {
                if let Some(val) = self.value(*id) {
                    ocv_store.insert(*id, val);
                }
            });

            *self.input_cell_map.entry(id.0).or_insert(T::default()) = new_value;

            // Find dependant compute cell new values and call registered callbacks for compute
            // cells whose values changed
            ocv_store.iter().for_each(|(cell_id, v)| {
                let val = self.value(*cell_id)
                    .expect(format!("Could not find compute cell with id: {:?}", cell_id).as_str());

                if val != *v {
                    let cb_ids = self.compute_cell_map_cb.get(&(*cell_id).compute_cell_id());
                    if cb_ids.is_some() {
                        let cb_ids = cb_ids.unwrap();
                        // Have to loop callback ids this way as I cannot take a mutable reference
                        // to self while I hold an immutable reference to it
                        for i in 0..cb_ids.len() {
                            let cb_id = cb_ids[i];
                            let callback = self.callback_map.get_mut(&cb_id).unwrap();
                            Self::call_registered_callback(val, callback);
                        }
                    }
                }
            });
            return true;
        }
        false
    }

    fn call_registered_callback(val: T, callback: &mut Box<dyn FnMut(T) + 'reactor>) {
        let mut a = Box::new(5);
        let b = &mut a;
        let c = **b;
        callback(val);
    }

    fn find_top_lvl_dep_compute_cells(&self, id: InputCellID) -> Vec<CellID> {
        let mut result = HashSet::new();
        let input_cell_id = CellID::Input(id);
        self.compute_cell_map_deps.iter().for_each(|(id, deps)| {
            let deps = HashSet::from_iter(deps.iter().map(|d| *d));

            if deps.contains(&input_cell_id) {
                result.insert(CellID::Compute(ComputeCellID(*id)));
            } else {
                let intersection = result
                    .intersection(&deps)
                    .map(|id| *id)
                    .collect::<HashSet<CellID>>();

                if !intersection.is_empty() {
                    result.retain(|id| !intersection.contains(id));
                    result.insert(CellID::Compute(ComputeCellID(*id)));
                }
            }
        });
        result.into_iter().collect()
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'reactor>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        if self.compute_cell_map_deps.contains_key(&id.0) {
            let callback_id = CallbackID(Uuid::new_v4());
            self.compute_cell_map_cb.entry(id.0).or_default().push(callback_id.0);
            self.callback_map.insert(callback_id.0, Box::new(callback));
            return Some(callback_id);
        }

        None
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        self.compute_cell_map_cb.get_mut(&cell.0)
            .map_or(Err(RemoveCallbackError::NonexistentCell), |cb_ids| {
                if let Some(i) = cb_ids.iter().position(|id| *id == callback.0) {
                    cb_ids.remove(i);
                } else {
                    return Err(RemoveCallbackError::NonexistentCallback);
                }

                Ok(())
            })?;

            return self.callback_map
                .remove(&callback.0)
                .map_or(Err(RemoveCallbackError::NonexistentCallback), |_| Ok(()));
    }
}
