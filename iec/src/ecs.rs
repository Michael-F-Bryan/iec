//! Super simple ECS-style building blocks, tailored for managing the various
//! side-tables and data structures generated when.

use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::{self, Formatter};
use std::sync::atomic::{AtomicUsize, Ordering};
use typename::TypeName;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct EntityId(u32);

/// Abstract component type.
pub trait Component: TypeName + Any + Debug + 'static {}

impl<C: TypeName + Any + Debug + 'static> Component for C {}

/// A counter which generates atomically incrementing [`EntityId`]s.
#[derive(Debug, Default, TypeName)]
pub struct EntityGenerator {
    last_id: AtomicUsize,
}

impl EntityGenerator {
    pub const fn new() -> EntityGenerator {
        EntityGenerator {
            last_id: AtomicUsize::new(0),
        }
    }

    pub fn next_id(&self) -> EntityId {
        let next_id = self.last_id.fetch_add(1, Ordering::Relaxed);
        EntityId(next_id as u32)
    }
}

/// A resource container used to access the various components stored inside.
///
/// There are two general categories of component which can be stored in a
/// [`Resources`]. Most of the time you'll be using "normal" [`Component`]s,
/// these are accessed with the usual [`Resources::get()`] and
/// [`Resources::get_mut()`] methods and let you associate data with a specific
/// [`EntityId`].
///
/// However there will be times when the usual [`EntityId`] -> [Component`]
/// relation doesn't make sense. In this case you can register a "singleton
/// component".
///
/// # Panics
///
/// Most of the methods on [`Resources`] rely on a component being registered
/// before you can access the [`Container`] which holds the various instances of
/// the component.
#[derive(Default)]
pub struct Resources {
    items: HashMap<TypeId, Box<Any>>,
    singletons: HashMap<TypeId, Box<Any>>,
    vtables: HashMap<TypeId, ContainerVtable>,
}

impl Resources {
    pub fn new() -> Resources {
        Resources::default()
    }

    /// Registers a [`Component`] type so we can set up containers and stash
    /// away some useful metadata.
    ///
    /// There is no way to "unregister" a component after it has been
    /// registered.
    pub fn register<C>(&mut self)
    where
        C: Component,
    {
        let type_id = TypeId::of::<C>();
        let boxed_container = Box::new(RefCell::new(Container::<C>::new()));
        self.items.insert(type_id, boxed_container as Box<Any>);
        self.vtables
            .insert(type_id, ContainerVtable::for_component_container::<C>());
    }

    pub fn register_singleton<C>(&mut self, value: C)
    where
        C: Component,
    {
        let type_id = TypeId::of::<C>();
        self.singletons
            .insert(type_id, Box::new(RefCell::new(value)));
        self.vtables
            .insert(type_id, ContainerVtable::for_singleton::<C>());
    }

    fn lookup<C: Component>(&self) -> &RefCell<Container<C>> {
        let type_id = TypeId::of::<C>();

        let container = match self.items.get(&type_id) {
            Some(c) => c,
            None => panic!("Unable to find the container for \"{}\", did you forget to register it?)", C::type_name()),
        };

        match container.downcast_ref::<RefCell<Container<C>>>() {
            Some(c) => c,
            None => unreachable!(
                "Something went really wrong when registering \"{}\"",
                C::type_name()
            ),
        }
    }

    fn lookup_singleton<C: Component>(&self) -> &RefCell<C> {
        let type_id = TypeId::of::<C>();

        let container = match self.singletons.get(&type_id) {
            Some(c) => c,
            None => panic!("Unable to find the \"{}\" singleton, did you forget to register it?)", C::type_name()),
        };

        match container.downcast_ref::<RefCell<C>>() {
            Some(c) => c,
            None => unreachable!(
                "Something went really wrong when registering \"{}\"",
                C::type_name()
            ),
        }
    }

    /// Look up the container for a particular component.
    pub fn get<C: Component>(&self) -> Ref<'_, Container<C>> {
        self.lookup::<C>().borrow()
    }

    /// Get a mutable reference to a component container.
    pub fn get_mut<C: Component>(&self) -> RefMut<'_, Container<C>> {
        self.lookup::<C>().borrow_mut()
    }

    /// Look up a singleton component.
    pub fn get_singleton<C: Component>(&self) -> Ref<'_, C> {
        self.lookup_singleton::<C>().borrow()
    }

    /// Get a mutable reference to a singleton component.
    pub fn get_singleton_mut<C: Component>(&self) -> RefMut<'_, C> {
        self.lookup_singleton::<C>().borrow_mut()
    }

    pub fn component_names(&self) -> impl Iterator<Item = &str> {
        self.vtables
            .values()
            .map(|vtable| vtable.component_name.as_str())
    }

    pub fn is_registered<C: Component>(&self) -> bool {
        let type_id = TypeId::of::<C>();
        self.vtables.contains_key(&type_id)
    }
}

impl Debug for Resources {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut map = f.debug_map();

        for (type_id, container) in &self.items {
            let vtable = &self.vtables[type_id];
            let debug = vtable.debug(&**container);
            map.entry(&vtable.component_name, &debug);
        }

        map.finish()
    }
}

/// A fancy lookup table mapping [`Component`]s to their correspondinng
/// [`EntityId`].
#[derive(Default, Clone, PartialEq, TypeName)]
pub struct Container<C: Component> {
    items: HashMap<EntityId, C>,
}

impl<C: Component> Container<C> {
    fn new() -> Container<C> {
        Container {
            items: HashMap::new(),
        }
    }

    pub fn get(&self, id: EntityId) -> Option<&C> {
        self.items.get(&id)
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut C> {
        self.items.get_mut(&id)
    }

    pub fn insert(&mut self, id: EntityId, item: C) {
        self.items.insert(id, item);
    }

    pub fn iter<'this>(
        &'this self,
    ) -> impl Iterator<Item = (EntityId, &'this C)> + 'this {
        self.items.iter().map(|(&id, c)| (id, c))
    }

    pub fn iter_mut<'this>(
        &'this mut self,
    ) -> impl Iterator<Item = (EntityId, &'this mut C)> + 'this {
        self.items.iter_mut().map(|(&id, c)| (id, c))
    }
}

impl<C: Component> Debug for Container<C> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_map().entries(self.items.iter()).finish()
    }
}

type DebugFunc = fn(container: &dyn Any, f: &mut Formatter) -> fmt::Result;

/// A vtable used to store container metadata and helper functions.
#[derive(Clone)]
struct ContainerVtable {
    debug: DebugFunc,
    /// The [`TypeId`] for the expected container. The container is usually a
    /// `RefCell<Container<C>>`.
    container_type_id: TypeId,
    component_type_id: TypeId,
    component_name: String,
}

impl ContainerVtable {
    fn for_component_container<C>() -> ContainerVtable
    where
        C: Component,
    {
        ContainerVtable {
            debug: |c, f| {
                c.downcast_ref::<RefCell<Container<C>>>()
                    .expect("Incorrect container type")
                    .borrow()
                    .fmt(f)
            },
            container_type_id: TypeId::of::<RefCell<Container<C>>>(),
            component_type_id: TypeId::of::<C>(),
            component_name: C::type_name(),
        }
    }

    fn for_singleton<C>() -> ContainerVtable
    where
        C: Component,
    {
        ContainerVtable {
            debug: |c, f| {
                c.downcast_ref::<RefCell<C>>()
                    .expect("Incorrect singleton type")
                    .borrow()
                    .fmt(f)
            },
            container_type_id: TypeId::of::<RefCell<C>>(),
            component_type_id: TypeId::of::<C>(),
            component_name: C::type_name(),
        }
    }

    fn debug<'a>(&self, container: &'a dyn Any) -> impl Debug + 'a {
        debug_assert_eq!(
            container.type_id(),
            self.container_type_id,
            "Expected a {} container",
            self.component_name
        );

        VtableDebug {
            func: self.debug,
            item: container,
        }
    }
}

struct VtableDebug<'a> {
    func: DebugFunc,
    item: &'a dyn Any,
}

impl<'a> Debug for VtableDebug<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        (self.func)(self.item, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default, Copy, Clone, PartialEq, TypeName)]
    struct RandomComponent(u32);

    #[test]
    fn generate_valid_vtables() {
        let vtable =
            ContainerVtable::for_component_container::<RandomComponent>();

        assert!(vtable.component_name.ends_with("RandomComponent"));
        assert_eq!(vtable.component_type_id, TypeId::of::<RandomComponent>());

        let container = RefCell::new(Container::default());
        container
            .borrow_mut()
            .insert(EntityId(0), RandomComponent(42));

        let debug_format = format!("{:?}", vtable.debug(&container));
        let actual = format!("{:?}", container.borrow());
        assert_eq!(actual, debug_format);
    }

    #[test]
    fn register_the_random_component() {
        let mut res = Resources::default();
        res.register::<RandomComponent>();

        let vtable = res.vtables.values().next().unwrap();

        assert_eq!(vtable.component_name, RandomComponent::type_name());
        assert_eq!(vtable.component_type_id, TypeId::of::<RandomComponent>());
        assert_eq!(
            vtable.container_type_id,
            TypeId::of::<RefCell<Container<RandomComponent>>>()
        );
    }

    #[test]
    fn get_a_component_container() {
        let mut res = Resources::default();
        res.register::<RandomComponent>();

        let _got = res.get::<RandomComponent>();

        assert_eq!(res.items.len(), 1);
        assert_eq!(res.vtables.len(), 1);
        let component_name = RandomComponent::type_name();
        assert_eq!(res.component_names().next().unwrap(), component_name);
    }

    #[test]
    fn debug_print_resources() {
        let mut res = Resources::default();
        res.register::<RandomComponent>();

        let got = format!("{:?}", res);

        let key = format!("\"{}\"", RandomComponent::type_name());
        assert!(got.contains(&key));
    }

    #[test]
    fn use_a_singleton_component() {
        let mut res = Resources::default();
        res.register_singleton(RandomComponent(42));

        assert!(res.items.is_empty());
        assert_eq!(res.vtables.len(), 1);
        assert_eq!(res.singletons.len(), 1);

        {
            let got = res.get_singleton::<RandomComponent>();
            assert_eq!(got.0, 42);
        }

        {
            let mut got = res.get_singleton_mut::<RandomComponent>();
            got.0 = 7;
        }

        {
            let got = res.get_singleton::<RandomComponent>();
            assert_eq!(got.0, 7);
        }
    }
}
