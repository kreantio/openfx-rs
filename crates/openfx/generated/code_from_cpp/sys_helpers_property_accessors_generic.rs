openfx_internal_macros::sys_helpers_make_property_accessors_by_types! {
    Double: ... pub { set get }, 1 pub { set get }, (2|3|4) pub(crate) { set get };
    Int: ... pub { set get }, 1 pub { set get }, (2|4) pub(crate) { set get };
    Pointer: ... pub { set get }, 1 pub { set get };
    String: ... pub { set get }, 1 pub { set get }, 2 pub(crate) { set get };
}
