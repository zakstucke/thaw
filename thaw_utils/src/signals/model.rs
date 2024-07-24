mod option_model;

pub use option_model::OptionModel;

use leptos::reactive_graph::{
    computed::Memo,
    signal::{ReadSignal, RwSignal, WriteSignal},
    traits::{DefinedAt, IsDisposed, Set, Update, With, WithUntracked},
    wrappers::read::Signal,
};

pub struct Model<T>
where
    T: 'static,
{
    read: Signal<T>,
    write: WriteSignal<T>,
    on_write: Option<WriteSignal<T>>,
}

impl<T: Default + Send + Sync> Default for Model<T> {
    fn default() -> Self {
        RwSignal::new(Default::default()).into()
    }
}

impl<T> Clone for Model<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Model<T> {}

impl<T: Send + Sync> Model<T> {
    fn new(value: T) -> Self {
        let rw_signal = RwSignal::new(value);
        rw_signal.into()
    }

    pub fn signal(&self) -> Signal<T> {
        self.read
    }
}

impl<T> DefinedAt for Model<T> {
    fn defined_at(&self) -> Option<&'static std::panic::Location<'static>> {
        todo!()
    }
}

impl<T: Send + Sync> With for Model<T> {
    type Value = T;

    fn try_with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        self.read.try_with(f)
    }
}

impl<T: Send + Sync> WithUntracked for Model<T> {
    type Value = T;

    fn try_with_untracked<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        self.read.try_with_untracked(f)
    }
}

// TODO
impl<T: Send + Sync + Clone> Update for Model<T> {
    type Value = T;

    fn try_maybe_update<U>(&self, fun: impl FnOnce(&mut Self::Value) -> (bool, U)) -> Option<U> {
        let value = self.write.try_maybe_update(fun);

        if let Some(on_write) = self.on_write.as_ref() {
            on_write.set(self.read.with_untracked(|read| read.clone()));
        }

        value
    }
}

impl<T> IsDisposed for Model<T> {
    fn is_disposed(&self) -> bool {
        self.write.is_disposed()
    }
}

impl<T: Send + Sync> From<T> for Model<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Send + Sync> From<RwSignal<T>> for Model<T> {
    fn from(rw_signal: RwSignal<T>) -> Self {
        let (read, write) = rw_signal.split();
        Self {
            read: read.into(),
            write,
            on_write: None,
        }
    }
}

impl<T> From<(Signal<T>, WriteSignal<T>)> for Model<T> {
    fn from((read, write): (Signal<T>, WriteSignal<T>)) -> Self {
        Self {
            read,
            write,
            on_write: None,
        }
    }
}

impl<T: Send + Sync> From<(ReadSignal<T>, WriteSignal<T>)> for Model<T> {
    fn from((read, write): (ReadSignal<T>, WriteSignal<T>)) -> Self {
        Self {
            read: read.into(),
            write,
            on_write: None,
        }
    }
}

impl<T: Send + Sync> From<(Memo<T>, WriteSignal<T>)> for Model<T> {
    fn from((read, write): (Memo<T>, WriteSignal<T>)) -> Self {
        Self {
            read: read.into(),
            write,
            on_write: None,
        }
    }
}

impl<T: Default + Send + Sync> From<(Option<T>, WriteSignal<T>)> for Model<T> {
    fn from((read, write): (Option<T>, WriteSignal<T>)) -> Self {
        let mut model = Self::new(read.unwrap_or_default());
        model.on_write = Some(write);
        model
    }
}

// TODO
// #[cfg(test)]
// mod test {
//     use super::Model;
//     use leptos::*;

//     #[test]
//     fn from() {
//         let runtime = create_runtime();

//         // T
//         let model: Model<i32> = 0.into();
//         assert_eq!(model.get_untracked(), 0);
//         model.set(1);
//         assert_eq!(model.get_untracked(), 1);

//         // RwSignal
//         let rw_signal = RwSignal::new(0);
//         let model: Model<i32> = rw_signal.into();
//         assert_eq!(model.get_untracked(), 0);
//         model.set(1);
//         assert_eq!(model.get_untracked(), 1);

//         // Read Write
//         let (read, write) = create_signal(0);
//         let model: Model<i32> = (read, write).into();
//         assert_eq!(model.get_untracked(), 0);
//         model.set(1);
//         assert_eq!(model.get_untracked(), 1);

//         runtime.dispose();
//     }
// }
