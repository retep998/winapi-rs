use um::winnt::{HRESULT};
use winrt::inspectable::{IInspectable, IInspectableVtbl};

RIDL!{#[uuid(0x00000035, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IActivationFactory(IActivationFactoryVtbl): IInspectable(IInspectableVtbl) {
    fn ActivateInstance(instance: *mut *mut  IInspectable) ->  HRESULT
}}
