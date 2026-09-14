use std::ffi::{CStr, c_int, c_uint, c_void};

use crate::{
    low::enums::{DrawLineStipplePattern, DrawPrimitive, DrawTextAlignment, StandardColour},
    low_plugin::{
        objects::{
            DrawContext, ImageClipDescriptor, ImageClipInstance, ImageEffectDescriptor,
            ImageEffectInstance, ImageMemory, InteractDescriptor, InteractInstance, Mutex,
            ParamSetDescriptor, ParamSetInstance,
        },
        property_sets::{
            ImageClipDescriptorPropertySet, ImageClipInstancePropertySet,
            ImageEffectDescriptorPropertySet, ImageEffectInstancePropertySet,
            ImageInstancePropertySet, InteractDescriptorPropertySet, InteractInstancePropertySet,
            ParamSetDescriptorPropertySet,
        },
    },
    sys_umbrella::{
        OfxImageClipHandle, OfxImageEffectHandle, OfxImageMemoryHandle, OfxInteractHandle,
        OfxMutexHandle, OfxParamHandle, OfxParamSetHandle, OfxPointD, OfxPropertySetHandle,
        OfxRGBAColourF, OfxRangeD, OfxRectD, OfxTime,
    },
};

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/generated/code_from_c/low_suites_plugin.rs",
));

#[openfx_internal_macros::low_impl_suite]
impl DialogSuiteV1 {
    /// ## SAFETY
    ///
    /// `self` must be valid.
    pub unsafe fn notify_redraw_pending(&self) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!(pascal) };

        crate::low::Status::result_from(unsafe { sys_fn() })
    }

    /// ## SAFETY
    ///
    /// `self` must be valid.
    pub unsafe fn request_dialog(&self, user_data: *mut c_void) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!(pascal) };

        crate::low::Status::result_from(unsafe { sys_fn(user_data) })
    }
}

#[openfx_internal_macros::low_impl_suite]
impl DrawSuiteV1 {
    /// ## SAFETY
    ///
    /// `self` and `context` must be valid.
    pub unsafe fn draw(
        &self,
        context: &DrawContext,
        primitive: DrawPrimitive,
        points: &[OfxPointD],
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(
                context.sys_handle(),
                primitive.as_sys(),
                points.as_ptr(),
                points.len() as c_int,
            )
        })
    }

    /// ## SAFETY
    ///
    /// `self` and `context` must be valid.
    pub unsafe fn draw_text(
        &self,
        context: &DrawContext,
        text: &CStr,
        pos: &OfxPointD,
        alignment: DrawTextAlignment,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(
                context.sys_handle(),
                text.as_ptr(),
                pos,
                alignment.as_sys() as c_int,
            )
        })
    }

    /// ## SAFETY
    ///
    /// `self` and `context` must be valid.
    pub unsafe fn get_colour(
        &self,
        context: &DrawContext,
        std_colour: StandardColour,
    ) -> crate::low::Result<OfxRGBAColourF> {
        let sys_fn = unsafe { sys_fn!() };

        let mut colour = std::mem::MaybeUninit::<OfxRGBAColourF>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(
                context.sys_handle(),
                std_colour.as_sys(),
                colour.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { colour.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` and `context` must be valid.
    pub unsafe fn set_colour(
        &self,
        context: &DrawContext,
        colour: OfxRGBAColourF,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(context.sys_handle(), &colour) })
    }

    /// ## SAFETY
    ///
    /// `self` and `context` must be valid.
    pub unsafe fn set_line_stipple(
        &self,
        context: &DrawContext,
        pattern: DrawLineStipplePattern,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(context.sys_handle(), pattern.as_sys()) })
    }

    /// ## SAFETY
    ///
    /// `self` and `context` must be valid.
    pub unsafe fn set_line_width(
        &self,
        context: &DrawContext,
        width: f32,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(context.sys_handle(), width) })
    }
}

#[openfx_internal_macros::low_impl_suite]
impl ImageEffectOpenGLRenderSuiteV1 {
    /// ## SAFETY
    ///
    /// `self` and `texture_handle` must be valid.
    ///
    /// ## TODO
    ///
    /// dedicate object `Texture`.
    pub unsafe fn clip_free_texture(
        &self,
        texture_handle: OfxPropertySetHandle,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(texture_handle) })
    }

    /// ## SAFETY
    ///
    /// `self` and `clip` must be valid.
    pub unsafe fn clip_load_texture(
        &self,
        clip: &ImageClipInstance,
        time: OfxTime,
        format: &CStr,
        region: &OfxRectD,
    ) -> crate::low::Result<OfxPropertySetHandle> {
        let sys_fn = unsafe { sys_fn!() };

        let mut texture_handle = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(
                clip.sys_handle(),
                time,
                format.as_ptr(),
                region,
                texture_handle.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { texture_handle.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` must be valid.
    pub unsafe fn flush_resources(&self) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn() })
    }
}

#[openfx_internal_macros::low_impl_suite]
impl ImageEffectSuiteV1 {
    /// ## SAFETY
    ///
    /// `self` and `image_effect` must be valid.
    pub unsafe fn abort(&self, image_effect: &ImageEffectInstance) -> bool {
        let sys_fn = unsafe { sys_fn!() };

        (unsafe { sys_fn(image_effect.sys_handle()) }) != 0
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect` must be valid.
    pub unsafe fn clip_define(
        &self,
        image_effect: &ImageEffectDescriptor,
        name: &CStr,
    ) -> crate::low::Result<ImageClipDescriptorPropertySet> {
        let sys_fn = unsafe { sys_fn!() };

        let mut property_set = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(
                image_effect.sys_handle(),
                name.as_ptr(),
                property_set.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { ImageClipDescriptorPropertySet::from(property_set.assume_init()) })
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect` must be valid.
    pub unsafe fn clip_get_handle(
        &self,
        image_effect: &ImageEffectInstance,
        name: &CStr,
    ) -> crate::low::Result<(ImageClipInstance, ImageClipInstancePropertySet)> {
        let sys_fn = unsafe { sys_fn!() };

        let mut clip = std::mem::MaybeUninit::<OfxImageClipHandle>::uninit();
        let mut property_set = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(
                image_effect.sys_handle(),
                name.as_ptr(),
                clip.as_mut_ptr(),
                property_set.as_mut_ptr(),
            )
        })?;

        Ok((
            unsafe { ImageClipInstance::from_sys_handle(clip.assume_init()) },
            unsafe { ImageClipInstancePropertySet::from(property_set.assume_init()) },
        ))
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect` must be valid.
    pub unsafe fn clip_get_clip_handle(
        &self,
        image_effect: &ImageEffectInstance,
        name: &CStr,
    ) -> crate::low::Result<ImageClipInstance> {
        let sys_fn = unsafe { sys_fn!("clipGetHandle") };

        let mut clip = std::mem::MaybeUninit::<OfxImageClipHandle>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(
                image_effect.sys_handle(),
                name.as_ptr(),
                clip.as_mut_ptr(),
                std::ptr::null_mut(),
            )
        })?;

        Ok(unsafe { ImageClipInstance::from_sys_handle(clip.assume_init()) })
    }

    /// ## SAFETY
    ///
    /// `self` and `clip` must be valid.
    pub unsafe fn clip_get_image(
        &self,
        clip: &ImageClipInstance,
        time: OfxTime,
        region: &Option<OfxRectD>,
    ) -> crate::low::Result<ImageInstancePropertySet> {
        let sys_fn = unsafe { sys_fn!() };

        let mut image_handle = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(
                clip.sys_handle(),
                time,
                region
                    .as_ref()
                    .map_or(std::ptr::null(), |r| r as *const OfxRectD),
                image_handle.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { ImageInstancePropertySet::from(image_handle.assume_init()) })
    }

    /// ## SAFETY
    ///
    /// `self` and `sys_clip` must be valid.
    unsafe fn clip_get_property_set(
        &self,
        sys_clip: OfxImageClipHandle,
    ) -> crate::low::Result<OfxPropertySetHandle> {
        let sys_fn = unsafe { sys_fn!() };

        let mut prop_handle = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        crate::low::Status::result_from(unsafe { sys_fn(sys_clip, prop_handle.as_mut_ptr()) })?;

        Ok(unsafe { prop_handle.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` and `clip` must be valid.
    #[no_sys_fn]
    pub unsafe fn clip_get_descriptor_property_set(
        &self,
        clip: &ImageClipDescriptor,
    ) -> crate::low::Result<ImageClipDescriptorPropertySet> {
        Ok(ImageClipDescriptorPropertySet::from(unsafe {
            self.clip_get_property_set(clip.sys_handle())?
        }))
    }

    /// ## SAFETY
    ///
    /// `self` and `clip` must be valid.
    #[no_sys_fn]
    pub unsafe fn clip_get_instance_property_set(
        &self,
        clip: &ImageClipInstance,
    ) -> crate::low::Result<ImageClipInstancePropertySet> {
        Ok(ImageClipInstancePropertySet::from(unsafe {
            self.clip_get_property_set(clip.sys_handle())?
        }))
    }

    /// ## SAFETY
    ///
    /// `self` and `clip` must be valid.
    pub unsafe fn clip_get_region_of_definition(
        &self,
        clip: &ImageClipInstance,
        time: OfxTime,
    ) -> crate::low::Result<OfxRectD> {
        let sys_fn = unsafe { sys_fn!() };

        let mut bounds = std::mem::MaybeUninit::<OfxRectD>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(clip.sys_handle(), time, bounds.as_mut_ptr())
        })?;

        Ok(unsafe { bounds.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` and `image_handle` must be valid.
    ///
    /// ## TODO
    ///
    /// Since all operations on it after this will be invalid, should we consume
    /// `image_handle` here? Or is that out of the scope of the low layer?
    pub unsafe fn clip_release_image(
        &self,
        image_handle: &ImageClipDescriptorPropertySet,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(image_handle.sys_handle()) })?;

        Ok(())
    }

    /// ## SAFETY
    ///
    /// `self` and `sys_image_effect` must be valid.
    unsafe fn get_param_set(
        &self,
        sys_image_effect: OfxImageEffectHandle,
    ) -> crate::low::Result<OfxParamSetHandle> {
        let sys_fn = unsafe { sys_fn!() };

        let mut param_set = std::mem::MaybeUninit::<OfxParamSetHandle>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(sys_image_effect, param_set.as_mut_ptr())
        })?;

        Ok(unsafe { param_set.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect` must be valid.
    #[no_sys_fn]
    pub unsafe fn get_param_set_descriptor(
        &self,
        image_effect: &ImageEffectDescriptor,
    ) -> crate::low::Result<ParamSetDescriptor> {
        Ok(ParamSetDescriptor::from_sys_handle(unsafe {
            self.get_param_set(image_effect.sys_handle())?
        }))
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect` must be valid.
    #[no_sys_fn]
    pub unsafe fn get_param_set_instance(
        &self,
        image_effect: &ImageEffectInstance,
    ) -> crate::low::Result<ParamSetInstance> {
        Ok(ParamSetInstance::from_sys_handle(unsafe {
            self.get_param_set(image_effect.sys_handle())?
        }))
    }

    /// ## SAFETY
    ///
    /// `self` and `sys_image_effect` must be valid.
    unsafe fn get_property_set(
        &self,
        sys_image_effect: OfxImageEffectHandle,
    ) -> crate::low::Result<OfxPropertySetHandle> {
        let sys_fn = unsafe { sys_fn!() };

        let mut prop_handle = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(sys_image_effect, prop_handle.as_mut_ptr())
        })?;

        Ok(unsafe { prop_handle.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect` must be valid.
    #[no_sys_fn]
    pub unsafe fn get_descriptor_property_set(
        &self,
        image_effect: &ImageEffectDescriptor,
    ) -> crate::low::Result<ImageEffectDescriptorPropertySet> {
        Ok(ImageEffectDescriptorPropertySet::from(unsafe {
            self.get_property_set(image_effect.sys_handle())?
        }))
    }

    /// ## SAFETY
    ///
    /// `self` and `image_effect` must be valid.
    #[no_sys_fn]
    pub unsafe fn get_instance_property_set(
        &self,
        image_effect: &ImageEffectInstance,
    ) -> crate::low::Result<ImageEffectInstancePropertySet> {
        Ok(ImageEffectInstancePropertySet::from(unsafe {
            self.get_property_set(image_effect.sys_handle())?
        }))
    }

    /// ## SAFETY
    ///
    /// `self` and `instance_handle` must be valid.
    pub unsafe fn image_memory_alloc(
        &self,
        instance_handle: &ImageEffectInstance,
        n_bytes: usize,
    ) -> crate::low::Result<ImageMemory> {
        let sys_fn = unsafe { sys_fn!() };

        let mut image_memory = std::mem::MaybeUninit::<OfxImageMemoryHandle>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(
                instance_handle.sys_handle(),
                n_bytes,
                image_memory.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { ImageMemory::from_sys_handle(image_memory.assume_init()) })
    }

    /// ## SAFETY
    ///
    /// `self` and `memory_handle` must be valid.
    pub unsafe fn image_memory_free(&self, memory_handle: &ImageMemory) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(memory_handle.sys_handle()) })
    }

    /// ## SAFETY
    ///
    /// `self` and `memory_handle` must be valid.
    pub unsafe fn image_memory_lock(
        &self,
        memory_handle: &ImageMemory,
    ) -> crate::low::Result<*mut c_void> {
        let sys_fn = unsafe { sys_fn!() };

        let mut returned_ptr = std::mem::MaybeUninit::<*mut c_void>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(memory_handle.sys_handle(), returned_ptr.as_mut_ptr())
        })?;

        Ok(unsafe { returned_ptr.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` and `memory_handle` must be valid.
    pub unsafe fn image_memory_unlock(
        &self,
        memory_handle: &ImageMemory,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(memory_handle.sys_handle()) })
    }
}

#[openfx_internal_macros::low_impl_suite]
impl InteractSuiteV1 {
    /// ## SAFETY
    ///
    /// `self` and `sys_interact_instance` must be valid.
    ///
    /// ## Note
    ///
    /// The first parameter is called `interactInstance` in the original code,
    /// so I kept the name here, but it clearly can be a handle to both a
    /// descriptor and an instance.
    unsafe fn interact_get_property_set(
        &self,
        sys_interact_instance: OfxInteractHandle,
    ) -> crate::low::Result<OfxPropertySetHandle> {
        let sys_fn = unsafe { sys_fn!() };

        let mut property = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(sys_interact_instance, property.as_mut_ptr())
        })?;

        Ok(unsafe { property.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` and `interact_instance` must be valid.
    ///
    /// ## Note
    ///
    /// See [`Self::interact_get_property_set`] about the weird name of the
    /// first parameter.
    #[no_sys_fn]
    pub unsafe fn interacte_get_descriptor_property_set(
        &self,
        interact_instance: &InteractDescriptor,
    ) -> crate::low::Result<InteractDescriptorPropertySet> {
        Ok(InteractDescriptorPropertySet::from(unsafe {
            self.interact_get_property_set(interact_instance.sys_handle())?
        }))
    }

    /// ## SAFETY
    ///
    /// `self` and `interact_instance` must be valid.
    #[no_sys_fn]
    pub unsafe fn interacte_get_instance_property_set(
        &self,
        interact_instance: &InteractInstance,
    ) -> crate::low::Result<InteractInstancePropertySet> {
        Ok(InteractInstancePropertySet::from(unsafe {
            self.interact_get_property_set(interact_instance.sys_handle())?
        }))
    }

    /// ## SAFETY
    ///
    /// `self` and `interact_instance` must be valid.
    pub unsafe fn interact_redraw(
        &self,
        interact_instance: &InteractInstance,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(interact_instance.sys_handle()) })
    }

    /// ## SAFETY
    ///
    /// `self` and `interact_instance` must be valid.
    pub unsafe fn interact_swap_buffers(
        &self,
        interact_instance: &InteractInstance,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(interact_instance.sys_handle()) })
    }
}

#[openfx_internal_macros::low_impl_suite]
impl MemorySuiteV1 {
    /// ## SAFETY
    ///
    /// `self` and `handle` must be valid.
    pub unsafe fn memory_alloc(
        &self,
        handle: Option<&ImageEffectInstance>,
        n_bytes: usize,
    ) -> crate::low::Result<*mut c_void> {
        let sys_fn = unsafe { sys_fn!() };

        let handle = match handle {
            Some(handle) => handle.sys_handle() as *mut c_void,
            None => std::ptr::null_mut(),
        };

        let mut allocated_data = std::mem::MaybeUninit::<*mut c_void>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(handle, n_bytes, allocated_data.as_mut_ptr())
        })?;

        Ok(unsafe { allocated_data.assume_init() })
    }
    /// ## SAFETY
    ///
    /// `self` and `allocated_data` must be valid.
    pub unsafe fn memory_free(&self, allocated_data: *mut c_void) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(allocated_data) })
    }
}

#[openfx_internal_macros::low_impl_suite]
impl MessageSuiteV1 {
    /// ## SAFETY
    ///
    /// `self` and `sys_handle` must be valid.
    ///
    /// ## Note
    ///
    /// The parameters of this function diverge from the original function,
    /// because the original function uses variadic arguments which are not
    /// directly representable in Rust.
    ///
    /// ## TODO
    ///
    /// We currently don't have an enum type for `kOfxMessageType*`?
    unsafe fn message(
        &self,
        sys_handle: OfxImageEffectHandle,
        message_type: &CStr,
        message_id: Option<&CStr>,
        message: &CStr,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(
                sys_handle as *mut c_void,
                message_type.as_ptr(),
                message_id.map_or(std::ptr::null(), |s| s.as_ptr()),
                c"%s".as_ptr(),
                message.as_ptr(),
            )
        })
    }

    /// ## SAFETY
    ///
    /// `self` must be valid.
    #[no_sys_fn]
    pub unsafe fn message_with_null(
        &self,
        message_type: &CStr,
        message_id: &CStr,
        message: &CStr,
    ) -> crate::low::Result<()> {
        unsafe {
            self.message(
                std::ptr::null_mut(),
                message_type,
                Some(message_id),
                message,
            )
        }
    }

    /// ## SAFETY
    ///
    /// `self` and `handle` must be valid.
    #[no_sys_fn]
    pub unsafe fn message_with_image_effect_descriptor(
        &self,
        handle: &ImageEffectDescriptor,
        message_type: &CStr,
        message_id: Option<&CStr>,
        message: &CStr,
    ) -> crate::low::Result<()> {
        unsafe { self.message(handle.into(), message_type, message_id, message) }
    }

    /// ## SAFETY
    ///
    /// `self` and `handle` must be valid.
    #[no_sys_fn]
    pub unsafe fn message_with_image_effect_instance(
        &self,
        handle: &ImageEffectInstance,
        message_type: &CStr,
        message_id: Option<&CStr>,
        message: &CStr,
    ) -> crate::low::Result<()> {
        unsafe { self.message(handle.into(), message_type, message_id, message) }
    }
}

#[openfx_internal_macros::low_impl_suite]
impl MessageSuiteV2 {
    /// ## SAFETY
    ///
    /// `self` and `sys_handle` must be valid.
    ///
    /// ## Note
    ///
    /// For `clear_persistent_message_with_*`, don't use
    /// `impl Into<OfxImageEffectHandle>`, because that would allow NULL.
    unsafe fn clear_persistent_message(
        &self,
        sys_handle: OfxImageEffectHandle,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(sys_handle as *mut c_void) })
    }

    /// ## SAFETY
    ///
    /// `self` and `handle` must be valid.
    #[no_sys_fn]
    pub unsafe fn clear_persistent_message_with_image_effect_descriptor(
        &self,
        handle: &ImageEffectDescriptor,
    ) -> crate::low::Result<()> {
        unsafe { self.clear_persistent_message(handle.into()) }
    }

    /// ## SAFETY
    ///
    /// `self` and `handle` must be valid.
    #[no_sys_fn]
    pub unsafe fn clear_persistent_message_with_image_effect_instance(
        &self,
        handle: &ImageEffectInstance,
    ) -> crate::low::Result<()> {
        unsafe { self.clear_persistent_message(handle.into()) }
    }

    /// ## SAFETY
    ///
    /// `self` and `sys_handle` must be valid.
    ///
    /// ## Note
    ///
    /// See [`MessageSuiteV1::message`].
    ///
    /// ## TODO
    ///
    /// See [`MessageSuiteV1::message`].
    unsafe fn message(
        &self,
        sys_handle: OfxImageEffectHandle,
        message_type: &CStr,
        message_id: Option<&CStr>,
        message: &CStr,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(
                sys_handle as *mut c_void,
                message_type.as_ptr(),
                message_id.map_or(std::ptr::null(), |s| s.as_ptr()),
                c"%s".as_ptr(),
                message.as_ptr(),
            )
        })
    }

    /// ## SAFETY
    ///
    /// `self` must be valid.
    #[no_sys_fn]
    pub unsafe fn message_with_null(
        &self,
        message_type: &CStr,
        message_id: &CStr,
        message: &CStr,
    ) -> crate::low::Result<()> {
        unsafe {
            self.message(
                std::ptr::null_mut(),
                message_type,
                Some(message_id),
                message,
            )
        }
    }

    /// ## SAFETY
    ///
    /// `self` and `handle` must be valid.
    #[no_sys_fn]
    pub unsafe fn message_with_image_effect_descriptor(
        &self,
        handle: &ImageEffectDescriptor,
        message_type: &CStr,
        message_id: Option<&CStr>,
        message: &CStr,
    ) -> crate::low::Result<()> {
        unsafe { self.message(handle.into(), message_type, message_id, message) }
    }

    /// ## SAFETY
    ///
    /// `self` and `handle` must be valid.
    #[no_sys_fn]
    pub unsafe fn message_with_image_effect_instance(
        &self,
        handle: &ImageEffectInstance,
        message_type: &CStr,
        message_id: Option<&CStr>,
        message: &CStr,
    ) -> crate::low::Result<()> {
        unsafe { self.message(handle.into(), message_type, message_id, message) }
    }

    /// ## SAFETY
    ///
    /// `self` and `sys_handle` must be valid.
    ///
    /// ## Note
    ///
    /// For `set_persistent_message_with_*`, don't use
    /// `impl Into<OfxImageEffectHandle>`, because that would allow NULL.
    ///
    /// See [`MessageSuiteV1::message`] for more.
    ///
    /// ## TODO
    ///
    /// See [`MessageSuiteV1::message`] for more.
    unsafe fn set_persistent_message(
        &self,
        sys_handle: OfxImageEffectHandle,
        message_type: &CStr,
        message_id: Option<&CStr>,
        message: &CStr,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(
                sys_handle as *mut c_void,
                message_type.as_ptr(),
                message_id.map_or(std::ptr::null(), |s| s.as_ptr()),
                c"%s".as_ptr(),
                message.as_ptr(),
            )
        })
    }

    /// ## SAFETY
    ///
    /// `self` and `handle` must be valid.
    #[no_sys_fn]
    pub unsafe fn set_persistent_message_with_image_effect_descriptor(
        &self,
        handle: &ImageEffectDescriptor,
        message_type: &CStr,
        message_id: Option<&CStr>,
        message: &CStr,
    ) -> crate::low::Result<()> {
        unsafe { self.set_persistent_message(handle.into(), message_type, message_id, message) }
    }

    /// ## SAFETY
    ///
    /// `self` and `handle` must be valid.
    #[no_sys_fn]
    pub unsafe fn set_persistent_message_with_image_effect_instance(
        &self,
        handle: &ImageEffectInstance,
        message_type: &CStr,
        message_id: Option<&CStr>,
        message: &CStr,
    ) -> crate::low::Result<()> {
        unsafe { self.set_persistent_message(handle.into(), message_type, message_id, message) }
    }
}

#[openfx_internal_macros::low_impl_suite]
impl MultiThreadSuiteV1 {
    /// ## SAFETY
    ///
    /// `self` must be valid.
    unsafe fn multi_thread(
        &self,
        sys_func: unsafe extern "C" fn(
            thread_index: c_uint,
            thread_max: c_uint,
            custom_arg: *mut c_void,
        ),
        n_threads: c_uint,
        custom_arg: *mut c_void,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(Some(sys_func), n_threads, custom_arg) })
    }

    /// ## SAFETY
    ///
    /// `self` must be valid.
    ///
    /// ## TODO
    ///
    /// Find a better name for this function?
    #[no_sys_fn]
    pub unsafe fn multi_thread_<T: Send + Sync>(
        &self,
        func: extern "C" fn(thread_index: c_uint, thread_max: c_uint, custom_arg: &T),
        n_threads: c_uint,
        custom_arg: &T,
    ) -> crate::low::Result<()> {
        let func: unsafe extern "C" fn(
            thread_index: c_uint,
            thread_max: c_uint,
            custom_arg: *mut c_void,
        ) = unsafe { std::mem::transmute(func) };

        unsafe { self.multi_thread(func, n_threads, custom_arg as *const T as *mut c_void) }
    }

    /// ## SAFETY
    ///
    /// `self` must be valid.
    pub unsafe fn multi_thread_index(&self) -> crate::low::Result<c_uint> {
        let sys_fn = unsafe { sys_fn!() };

        let mut thread_index = std::mem::MaybeUninit::<c_uint>::uninit();

        crate::low::Status::result_from(unsafe { sys_fn(thread_index.as_mut_ptr()) })?;

        Ok(unsafe { thread_index.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` must be valid.
    pub unsafe fn multi_thread_is_spawned_thread(&self) -> bool {
        let sys_fn = unsafe { sys_fn!() };

        (unsafe { sys_fn() }) != 0
    }

    /// ## SAFETY
    ///
    /// `self` must be valid.
    pub unsafe fn multi_thread_num_cpus(&self) -> crate::low::Result<c_uint> {
        let sys_fn = unsafe { sys_fn!("multiThreadNumCPUs") };

        let mut num_cpus = std::mem::MaybeUninit::<c_uint>::uninit();

        crate::low::Status::result_from(unsafe { sys_fn(num_cpus.as_mut_ptr()) })?;

        Ok(unsafe { num_cpus.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` must be valid.
    pub unsafe fn mutex_create(&self, lock_count: c_int) -> crate::low::Result<Mutex> {
        let sys_fn = unsafe { sys_fn!() };

        let mut mutex = std::mem::MaybeUninit::<OfxMutexHandle>::uninit();

        crate::low::Status::result_from(unsafe { sys_fn(mutex.as_mut_ptr(), lock_count) })?;

        Ok(unsafe { Mutex::from_sys_handle(mutex.assume_init()) })
    }

    /// ## SAFETY
    ///
    /// `self` and `mutex` must be valid.
    ///
    /// ## TODO
    ///
    /// Should we consume `mutex` here? Or is that out of the scope of the low
    /// layer?
    pub unsafe fn mutex_destroy(&self, mutex: &Mutex) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(mutex.sys_handle()) })
    }

    /// ## SAFETY
    ///
    /// `self` and `mutex` must be valid.
    pub unsafe fn mutex_lock(&self, mutex: &Mutex) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(mutex.sys_handle()) })
    }

    /// ## SAFETY
    ///
    /// `self` and `mutex` must be valid.
    pub unsafe fn mutex_try_lock(&self, mutex: &Mutex) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(mutex.sys_handle()) })
    }

    /// ## SAFETY
    ///
    /// `self` and `mutex` must be valid.
    pub unsafe fn mutex_unlock(&self, mutex: &Mutex) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!("mutexUnLock") };

        crate::low::Status::result_from(unsafe { sys_fn(mutex.sys_handle()) })
    }
}

#[openfx_internal_macros::low_impl_suite]
impl OpenCLProgramSuiteV1 {}

#[openfx_internal_macros::low_impl_suite]
impl ParameterSuiteV1 {
    /// ## SAFETY
    ///
    /// `self`, `sys_param_to` and `sys_param_from` must be valid.
    pub(crate) unsafe fn param_copy(
        &self,
        sys_param_to: OfxParamHandle,
        sys_param_from: OfxParamHandle,
        dst_offset: OfxTime,
        frame_range: Option<&OfxRangeD>,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(
                sys_param_to,
                sys_param_from,
                dst_offset,
                frame_range.map_or(std::ptr::null(), |r| r),
            )
        })
    }

    /// ## SAFETY
    ///
    /// `self` and `param_set` must be valid.
    ///
    /// ## TODO
    ///
    /// Enum type `kOfxParamType*`?
    pub(crate) unsafe fn param_define(
        &self,
        param_set: ParamSetDescriptor,
        param_type: &CStr,
        name: &CStr,
    ) -> crate::low::Result<OfxPropertySetHandle> {
        let sys_fn = unsafe { sys_fn!() };

        let mut property_set = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(
                param_set.sys_handle(),
                param_type.as_ptr(),
                name.as_ptr(),
                property_set.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { property_set.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` and `param_set` must be valid.
    ///
    /// ## TODO
    ///
    /// See [`Self::param_define`].
    pub unsafe fn param_define_no_output(
        &self,
        param_set: ParamSetDescriptor,
        param_type: &CStr,
        name: &CStr,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!("paramDefine") };

        crate::low::Status::result_from(unsafe {
            sys_fn(
                param_set.sys_handle(),
                param_type.as_ptr(),
                name.as_ptr(),
                std::ptr::null_mut(),
            )
        })
    }

    /// ## SAFETY
    ///
    /// `self` and `sys_param_handle` must be valid.
    pub(crate) unsafe fn param_delete_all_keys(
        &self,
        sys_param_handle: OfxParamHandle,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(sys_param_handle) })
    }

    /// ## SAFETY
    ///
    /// `self` and `sys_param_handle` must be valid.
    pub(crate) unsafe fn param_delete_key(
        &self,
        sys_param_handle: OfxParamHandle,
        time: OfxTime,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(sys_param_handle, time) })
    }

    /// ## SAFETY
    ///
    /// `self` and `param_set` must be valid.
    pub unsafe fn param_edit_begin(
        &self,
        param_set: ParamSetInstance,
        name: &CStr,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(param_set.sys_handle(), name.as_ptr()) })
    }

    /// ## SAFETY
    ///
    /// `self` and `param_set` must be valid.
    pub unsafe fn param_edit_end(&self, param_set: ParamSetInstance) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(param_set.sys_handle()) })
    }

    /// ## SAFETY
    ///
    /// `self` and `param_set` must be valid.
    pub(crate) unsafe fn param_get_handle(
        &self,
        param_set: ParamSetInstance,
        name: &CStr,
    ) -> crate::low::Result<(OfxParamHandle, OfxPropertySetHandle)> {
        let sys_fn = unsafe { sys_fn!() };

        let mut param = std::mem::MaybeUninit::<OfxParamHandle>::uninit();
        let mut property_set = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(
                param_set.sys_handle(),
                name.as_ptr(),
                param.as_mut_ptr(),
                property_set.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { (param.assume_init(), property_set.assume_init()) })
    }

    /// ## SAFETY
    ///
    /// `self` and `param_set` must be valid.
    #[expect(unused)]
    pub(crate) unsafe fn param_get_param_handle(
        &self,
        param_set: ParamSetInstance,
        name: &CStr,
    ) -> crate::low::Result<OfxParamHandle> {
        let sys_fn = unsafe { sys_fn!("paramGetHandle") };

        let mut param = std::mem::MaybeUninit::<OfxParamHandle>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(
                param_set.sys_handle(),
                name.as_ptr(),
                param.as_mut_ptr(),
                std::ptr::null_mut(),
            )
        })?;

        Ok(unsafe { param.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` and `sys_param_handle` must be valid.
    pub(crate) unsafe fn param_get_key_index(
        &self,
        sys_param_handle: OfxParamHandle,
        time: OfxTime,
        direction: c_int,
    ) -> crate::low::Result<c_int> {
        let sys_fn = unsafe { sys_fn!() };

        let mut index = std::mem::MaybeUninit::<c_int>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(sys_param_handle, time, direction, index.as_mut_ptr())
        })?;

        Ok(unsafe { index.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` and `sys_param_handle` must be valid.
    pub(crate) unsafe fn param_get_key_time(
        &self,
        sys_param_handle: OfxParamHandle,
        nth_key: c_uint,
    ) -> crate::low::Result<OfxTime> {
        let sys_fn = unsafe { sys_fn!() };

        let mut time = std::mem::MaybeUninit::<OfxTime>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(sys_param_handle, nth_key, time.as_mut_ptr())
        })?;

        Ok(unsafe { time.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` and `sys_param_handle` must be valid.
    pub(crate) unsafe fn param_get_num_keys(
        &self,
        sys_param_handle: OfxParamHandle,
    ) -> crate::low::Result<c_uint> {
        let sys_fn = unsafe { sys_fn!() };

        let mut num_keys = std::mem::MaybeUninit::<c_uint>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(sys_param_handle, num_keys.as_mut_ptr())
        })?;

        Ok(unsafe { num_keys.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` and `sys_param` must be valid.
    pub(crate) unsafe fn param_get_property_set(
        &self,
        sys_param: OfxParamHandle,
    ) -> crate::low::Result<OfxPropertySetHandle> {
        let sys_fn = unsafe { sys_fn!() };

        let mut prop_handle = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        crate::low::Status::result_from(unsafe { sys_fn(sys_param, prop_handle.as_mut_ptr()) })?;

        Ok(unsafe { prop_handle.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` and `sys_param_set` must be valid.
    unsafe fn param_set_get_property_set(
        &self,
        sys_param_set: OfxParamSetHandle,
    ) -> crate::low::Result<OfxPropertySetHandle> {
        let sys_fn = unsafe { sys_fn!() };

        let mut prop_handle = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(sys_param_set, prop_handle.as_mut_ptr())
        })?;

        Ok(unsafe { prop_handle.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` and `param_set` must be valid.
    #[no_sys_fn]
    pub unsafe fn param_set_get_descriptor_property_set(
        &self,
        param_set: ParamSetDescriptor,
    ) -> crate::low::Result<ParamSetDescriptorPropertySet> {
        Ok(ParamSetDescriptorPropertySet::from(unsafe {
            self.param_set_get_property_set(param_set.sys_handle())?
        }))
    }

    /// ## SAFETY
    ///
    /// `self` and `param_set` must be valid.
    ///
    /// ## TODO
    ///
    /// Should [`ParamSetDescriptorPropertySet`] be renamed as
    /// `ParamSetPropertySet`?
    #[no_sys_fn]
    pub unsafe fn param_set_get_instance_property_set(
        &self,
        param_set: ParamSetInstance,
    ) -> crate::low::Result<ParamSetDescriptorPropertySet> {
        Ok(ParamSetDescriptorPropertySet::from(unsafe {
            self.param_set_get_property_set(param_set.sys_handle())?
        }))
    }
}

#[openfx_internal_macros::low_impl_suite]
impl ParametricParameterSuiteV1 {
    /// ## SAFETY
    ///
    /// `self` and `sys_param` must be valid.
    pub(crate) unsafe fn parametric_param_add_control_point(
        &self,
        sys_param: OfxParamHandle,
        curve_index: c_int,
        time: OfxTime,
        key: f64,
        value: f64,
        add_animation_key: bool,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(sys_param, curve_index, time, key, value, add_animation_key)
        })?;

        Ok(())
    }

    /// ## SAFETY
    ///
    /// `self` and `sys_param` must be valid.
    pub(crate) unsafe fn parametric_param_delete_all_control_points(
        &self,
        sys_param: OfxParamHandle,
        curve_index: c_int,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(sys_param, curve_index) })?;

        Ok(())
    }

    /// ## SAFETY
    ///
    /// `self` and `sys_param` must be valid.
    pub(crate) unsafe fn parametric_param_delete_control_point(
        &self,
        sys_param: OfxParamHandle,
        curve_index: c_int,
        nth_ctl: c_int,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(sys_param, curve_index, nth_ctl) })?;

        Ok(())
    }

    /// ## SAFETY
    ///
    /// `self` and `sys_param` must be valid.
    pub(crate) unsafe fn parametric_param_get_n_control_points(
        &self,
        sys_param: OfxParamHandle,
        curve_index: c_int,
        time: OfxTime,
    ) -> crate::low::Result<c_int> {
        let sys_fn = unsafe { sys_fn!() };

        let mut return_value = std::mem::MaybeUninit::<c_int>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(sys_param, curve_index, time, return_value.as_mut_ptr())
        })?;

        Ok(unsafe { return_value.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` and `sys_param` must be valid.
    pub(crate) unsafe fn parametric_param_get_nth_control_point(
        &self,
        sys_param: OfxParamHandle,
        curve_index: c_int,
        time: OfxTime,
        nth_ctl: c_int,
    ) -> crate::low::Result<(f64, f64)> {
        let sys_fn = unsafe { sys_fn!() };

        let mut key = std::mem::MaybeUninit::<f64>::uninit();
        let mut value = std::mem::MaybeUninit::<f64>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(
                sys_param,
                curve_index,
                time,
                nth_ctl,
                key.as_mut_ptr(),
                value.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { (key.assume_init(), value.assume_init()) })
    }

    /// ## SAFETY
    ///
    /// `self` and `param` must be valid.
    pub(crate) unsafe fn parametric_param_get_value(
        &self,
        param: OfxParamHandle,
        curve_index: c_int,
        time: OfxTime,
        parametric_position: f64,
    ) -> crate::low::Result<f64> {
        let sys_fn = unsafe { sys_fn!() };

        let mut return_value = std::mem::MaybeUninit::<f64>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(
                param,
                curve_index,
                time,
                parametric_position,
                return_value.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { return_value.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` and `param` must be valid.
    #[expect(clippy::too_many_arguments)]
    pub(crate) unsafe fn parametric_param_set_nth_control_point(
        &self,
        param: OfxParamHandle,
        curve_index: c_int,
        time: OfxTime,
        nth_ctl: c_int,
        key: f64,
        value: f64,
        add_animation_key: bool,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(
                param,
                curve_index,
                time,
                nth_ctl,
                key,
                value,
                add_animation_key,
            )
        })?;

        Ok(())
    }
}

#[openfx_internal_macros::low_impl_suite]
impl ProgressSuiteV1 {
    /// ## SAFETY
    ///
    /// `self` and `effect_instance` must be valid.
    unsafe fn progress_end(&self, effect_instance: &ImageEffectInstance) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(effect_instance.sys_handle() as *mut c_void)
        })
    }

    /// ## SAFETY
    ///
    /// `self` and `effect_instance` must be valid.
    unsafe fn progress_start(
        &self,
        effect_instance: &ImageEffectInstance,
        label: &CStr,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(effect_instance.sys_handle() as *mut c_void, label.as_ptr())
        })
    }

    /// ## SAFETY
    ///
    /// `self` and `effect_instance` must be valid.
    unsafe fn progress_update(
        &self,
        effect_instance: &ImageEffectInstance,
        progress: f64,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(effect_instance.sys_handle() as *mut c_void, progress)
        })
    }
}

#[openfx_internal_macros::low_impl_suite]
impl ProgressSuiteV2 {
    /// ## SAFETY
    ///
    /// `self` and `effect_instance` must be valid.
    unsafe fn progress_end(&self, effect_instance: &ImageEffectInstance) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(effect_instance.sys_handle() as *mut c_void)
        })
    }

    /// ## SAFETY
    ///
    /// `self` and `effect_instance` must be valid.
    unsafe fn progress_start(
        &self,
        effect_instance: &ImageEffectInstance,
        message: &CStr,
        message_id: &CStr,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(
                effect_instance.sys_handle() as *mut c_void,
                message.as_ptr(),
                message_id.as_ptr(),
            )
        })
    }

    /// ## SAFETY
    ///
    /// `self` and `effect_instance` must be valid.
    unsafe fn progress_update(
        &self,
        effect_instance: &ImageEffectInstance,
        progress: f64,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(effect_instance.sys_handle() as *mut c_void, progress)
        })
    }
}

#[openfx_internal_macros::low_impl_suite]
impl PropertySuiteV1 {}

#[openfx_internal_macros::low_impl_suite]
impl TimeLineSuiteV1 {
    /// ## SAFETY
    ///
    /// `self` and `instance` must be valid.
    unsafe fn get_time(&self, instance: &ImageEffectInstance) -> crate::low::Result<OfxTime> {
        let sys_fn = unsafe { sys_fn!() };

        let mut time = std::mem::MaybeUninit::<OfxTime>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(instance.sys_handle() as *mut c_void, time.as_mut_ptr())
        })?;

        Ok(unsafe { time.assume_init() })
    }

    /// ## SAFETY
    ///
    /// `self` and `instance` must be valid.
    unsafe fn get_time_bounds(
        &self,
        instance: &ImageEffectInstance,
    ) -> crate::low::Result<(OfxTime, OfxTime)> {
        let sys_fn = unsafe { sys_fn!() };

        let mut first_time = std::mem::MaybeUninit::<OfxTime>::uninit();
        let mut last_time = std::mem::MaybeUninit::<OfxTime>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(
                instance.sys_handle() as *mut c_void,
                first_time.as_mut_ptr(),
                last_time.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { (first_time.assume_init(), last_time.assume_init()) })
    }

    /// ## SAFETY
    ///
    /// `self` and `instance` must be valid.
    unsafe fn goto_time(
        &self,
        instance: &ImageEffectInstance,
        time: OfxTime,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(instance.sys_handle() as *mut c_void, time)
        })
    }
}
