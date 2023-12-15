use leptos::{
    create_rw_signal, RwSignal, SignalGet, SignalGetUntracked, SignalUpdate, SignalUpdateUntracked,
};

#[derive(Clone)]
pub struct ReactiveVec<T: 'static + Clone> {
    next_id: RwSignal<usize>,
    vec: RwSignal<Vec<(usize, RwSignal<T>)>>,
}
impl<T: 'static + Clone> ReactiveVec<T> {
    pub fn new() -> Self {
        Self {
            next_id: create_rw_signal(0),
            vec: create_rw_signal(Vec::new()),
        }
    }

    pub fn get(&self) -> Vec<(usize, RwSignal<T>)> {
        self.vec.get()
    }

    pub fn get_values_untracked(&self) -> Vec<T> {
        self.vec
            .get_untracked()
            .into_iter()
            .map(|(_, value)| value.get_untracked())
            .collect::<Vec<_>>()
    }

    pub fn push(&self, element: T) {
        self.vec
            .update(|vec| vec.push((self.next_id.get_untracked(), create_rw_signal(element))));
        self.next_id.update_untracked(|id| *id += 1);
    }
}
impl<T: Clone> Copy for ReactiveVec<T> {}
