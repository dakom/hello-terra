use dominator::DomBuilder;

pub type MixinStub<T: 'static> = fn(DomBuilder<T>) -> DomBuilder<T>;