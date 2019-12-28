initSidebarItems({"struct":[["CreateIter","An iterator for entity creation. Please note that you have to consume it because iterators are lazy."],["CreateIterAtomic","An iterator for entity creation. Please note that you have to consume it because iterators are lazy."],["EntitiesRes","The entities of this ECS. This is a resource, stored in the `World`. If you just want to access it in your system, you can also use the `Entities` type def."],["Entity","`Entity` type, as seen by the user."],["EntityBuilder","The entity builder, allowing to build an entity together with its components."],["EntityResBuilder","An entity builder from `EntitiesRes`.  Allows building an entity with its components if you have mutable access to the component storages."],["Generation","Index generation. When a new entity is placed at an old index, it bumps the `Generation` by 1. This allows to avoid using components from the entities that were deleted."],["LazyBuilder","Like `EntityBuilder`, but inserts the component lazily, meaning on `maintain`. If you need those components to exist immediately, you have to insert them into the storages yourself."],["LazyUpdate","Lazy updates can be used for world updates that need to borrow a lot of resources and as such should better be done at the end. They work lazily in the sense that they are dispatched when calling `world.maintain()`."],["World","A [Resource] container, which provides methods to insert, access and manage the contained resources."]],"trait":[["Builder","A common trait for `EntityBuilder` and `LazyBuilder`, allowing either to be used. Entity is definitely alive, but the components may or may not exist before a call to `World::maintain`."],["Component","Abstract component type. Doesn't have to be Copy or even Clone."],["WorldExt","This trait provides some extension methods to make working with shred's [World] easier."]],"type":[["Entities","A wrapper for a read `Entities` resource. Note that this is just `Read<Entities>`, so you can easily use it in your system:"],["Index","An index is basically the id of an `Entity`."]]});