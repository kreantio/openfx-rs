//! ## Note
//!
//! ### `pub` vs `pub(crate)`
//!
//! Functions implemented on suites whose first parameter (excluding the
//! receiver) is an object from [`crate::low_plugin::objects`] should be
//! `pub(crate)`. They may then be invoked only indirectly through functions
//! implemented on the respective objects.
//!
//! Since the first parameters of functions on [`PropertySuiteV1`] are
//! [`OfxPropertySetHandle`]s rather than the objects mentioned above, those
//! functions remain `pub`.
//!
//! Since the first parameters of functions on [`MessageSuiteV1`] and
//! [`MessageSuiteV2`] are either [`MessageHandle`] or
//! [`PersistentMessageHandle`], both defined in this module, they also don't
//! fall under the rule mentioned above and remain `pub`. Besides, the
//! respective objects don't implement the corresponding functions either,
//! so that there is only a single way to call those functions.

use std::{
    ffi::{CStr, c_char, c_int, c_uint, c_void},
    ptr::NonNull,
};

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
            OpenGLTexturePropertySet,
        },
    },
    sys_umbrella::{
        OfxImageClipHandle, OfxImageEffectHandle, OfxImageMemoryHandle, OfxInteractHandle,
        OfxMutexHandle, OfxParamHandle, OfxParamSetHandle, OfxPointD, OfxPropertySetHandle,
        OfxRGBAColourF, OfxRectD, OfxStatus, OfxTime,
    },
};

mod impl_parameter_suite_methods;
mod impl_property_suite_property_accessor_methods;

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/generated/code_from_c/low_plugin_suites.rs",
));

#[openfx_internal_macros::low_impl_suite]
impl DialogSuiteV1 {
    /// ## Safety
    ///
    /// `self` must be valid.
    pub unsafe fn notify_redraw_pending(&self) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!(pascal) };

        crate::low::Status::result_from(unsafe { sys_fn() })
    }

    /// ## Safety
    ///
    /// `self` must be valid.
    pub unsafe fn request_dialog(&self, user_data: *mut c_void) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!(pascal) };

        crate::low::Status::result_from(unsafe { sys_fn(user_data) })
    }
}

#[openfx_internal_macros::low_impl_suite]
impl DrawSuiteV1 {
    /// ## Safety
    ///
    /// `self` and `context` must be valid.
    pub(crate) unsafe fn draw(
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

    /// ## Safety
    ///
    /// `self` and `context` must be valid.
    pub(crate) unsafe fn draw_text(
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

    /// ## Safety
    ///
    /// `self` and `context` must be valid.
    pub(crate) unsafe fn get_colour(
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

    /// ## Safety
    ///
    /// `self` and `context` must be valid.
    pub(crate) unsafe fn set_colour(
        &self,
        context: &DrawContext,
        colour: OfxRGBAColourF,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(context.sys_handle(), &colour) })
    }

    /// ## Safety
    ///
    /// `self` and `context` must be valid.
    pub(crate) unsafe fn set_line_stipple(
        &self,
        context: &DrawContext,
        pattern: DrawLineStipplePattern,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(context.sys_handle(), pattern.as_sys()) })
    }

    /// ## Safety
    ///
    /// `self` and `context` must be valid.
    pub(crate) unsafe fn set_line_width(
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
    /// ## Safety
    ///
    /// `self` and `texture_handle` must be valid.
    ///
    /// ## TODO
    ///
    /// See [ImageEffectSuiteV1::clip_release_image]'s TODO.
    pub unsafe fn clip_free_texture(
        &self,
        texture_handle: &OpenGLTexturePropertySet,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(texture_handle.sys_handle()) })
    }

    /// ## Safety
    ///
    /// `self` and `clip` must be valid.
    pub(crate) unsafe fn clip_load_texture(
        &self,
        clip: &ImageClipInstance,
        time: OfxTime,
        format: &CStr,
        region: &OfxRectD,
    ) -> crate::low::Result<OpenGLTexturePropertySet> {
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

        Ok(unsafe { OpenGLTexturePropertySet::from(texture_handle.assume_init()) })
    }

    /// ## Safety
    ///
    /// `self` must be valid.
    pub unsafe fn flush_resources(&self) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn() })
    }
}

#[openfx_internal_macros::low_impl_suite]
impl ImageEffectSuiteV1 {
    /// ## Safety
    ///
    /// `self` and `image_effect` must be valid.
    pub(crate) unsafe fn abort(&self, image_effect: &ImageEffectInstance) -> bool {
        let sys_fn = unsafe { sys_fn!() };

        (unsafe { sys_fn(image_effect.sys_handle()) }) != 0
    }

    /// ## Safety
    ///
    /// `self` and `image_effect` must be valid.
    pub(crate) unsafe fn clip_define(
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

    /// ## Safety
    ///
    /// `self` and `image_effect` must be valid.
    pub(crate) unsafe fn clip_get_handle(
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

    /// ## Safety
    ///
    /// `self` and `image_effect` must be valid.
    pub(crate) unsafe fn clip_get_clip_handle(
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

    /// ## Safety
    ///
    /// `self` and `clip` must be valid.
    pub(crate) unsafe fn clip_get_image(
        &self,
        clip: &ImageClipInstance,
        time: OfxTime,
        region: Option<&OfxRectD>,
    ) -> crate::low::Result<ImageInstancePropertySet> {
        let sys_fn = unsafe { sys_fn!() };

        let mut image_handle = std::mem::MaybeUninit::<OfxPropertySetHandle>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(
                clip.sys_handle(),
                time,
                region
                    .as_ref()
                    .map_or(std::ptr::null(), |r| *r as *const OfxRectD),
                image_handle.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { ImageInstancePropertySet::from(image_handle.assume_init()) })
    }

    /// ## Safety
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

    /// ## Safety
    ///
    /// `self` and `clip` must be valid.
    #[no_sys_fn]
    pub(crate) unsafe fn clip_get_descriptor_property_set(
        &self,
        clip: &ImageClipDescriptor,
    ) -> crate::low::Result<ImageClipDescriptorPropertySet> {
        Ok(ImageClipDescriptorPropertySet::from(unsafe {
            self.clip_get_property_set(clip.sys_handle())?
        }))
    }

    /// ## Safety
    ///
    /// `self` and `clip` must be valid.
    #[no_sys_fn]
    pub(crate) unsafe fn clip_get_instance_property_set(
        &self,
        clip: &ImageClipInstance,
    ) -> crate::low::Result<ImageClipInstancePropertySet> {
        Ok(ImageClipInstancePropertySet::from(unsafe {
            self.clip_get_property_set(clip.sys_handle())?
        }))
    }

    /// ## Safety
    ///
    /// `self` and `clip` must be valid.
    pub(crate) unsafe fn clip_get_region_of_definition(
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

    /// ## Safety
    ///
    /// `self` and `image_handle` must be valid.
    ///
    /// ## TODO
    ///
    /// Since all operations on it after this will be invalid, should we consume
    /// `image_handle` here? Or is that out of the scope of the low layer?
    pub unsafe fn clip_release_image(
        &self,
        image_handle: &ImageInstancePropertySet,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(image_handle.sys_handle()) })?;

        Ok(())
    }

    /// ## Safety
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

    /// ## Safety
    ///
    /// `self` and `image_effect` must be valid.
    #[no_sys_fn]
    pub(crate) unsafe fn get_param_set_descriptor(
        &self,
        image_effect: &ImageEffectDescriptor,
    ) -> crate::low::Result<ParamSetDescriptor> {
        Ok(ParamSetDescriptor::from_sys_handle(unsafe {
            self.get_param_set(image_effect.sys_handle())?
        }))
    }

    /// ## Safety
    ///
    /// `self` and `image_effect` must be valid.
    #[no_sys_fn]
    pub(crate) unsafe fn get_param_set_instance(
        &self,
        image_effect: &ImageEffectInstance,
    ) -> crate::low::Result<ParamSetInstance> {
        Ok(ParamSetInstance::from_sys_handle(unsafe {
            self.get_param_set(image_effect.sys_handle())?
        }))
    }

    /// ## Safety
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

    /// ## Safety
    ///
    /// `self` and `image_effect` must be valid.
    #[no_sys_fn]
    pub(crate) unsafe fn get_descriptor_property_set(
        &self,
        image_effect: &ImageEffectDescriptor,
    ) -> crate::low::Result<ImageEffectDescriptorPropertySet> {
        Ok(ImageEffectDescriptorPropertySet::from(unsafe {
            self.get_property_set(image_effect.sys_handle())?
        }))
    }

    /// ## Safety
    ///
    /// `self` and `image_effect` must be valid.
    #[no_sys_fn]
    pub(crate) unsafe fn get_instance_property_set(
        &self,
        image_effect: &ImageEffectInstance,
    ) -> crate::low::Result<ImageEffectInstancePropertySet> {
        Ok(ImageEffectInstancePropertySet::from(unsafe {
            self.get_property_set(image_effect.sys_handle())?
        }))
    }

    /// ## Safety
    ///
    /// `self` and `instance_handle` must be valid.
    pub(crate) unsafe fn image_memory_alloc(
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

    /// ## Safety
    ///
    /// `self` and `memory_handle` must be valid.
    pub(crate) unsafe fn image_memory_free(
        &self,
        memory_handle: &ImageMemory,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(memory_handle.sys_handle()) })
    }

    /// ## Safety
    ///
    /// `self` and `memory_handle` must be valid.
    pub(crate) unsafe fn image_memory_lock(
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

    /// ## Safety
    ///
    /// `self` and `memory_handle` must be valid.
    pub(crate) unsafe fn image_memory_unlock(
        &self,
        memory_handle: &ImageMemory,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(memory_handle.sys_handle()) })
    }
}

#[openfx_internal_macros::low_impl_suite]
impl InteractSuiteV1 {
    /// ## Safety
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

    /// ## Safety
    ///
    /// `self` and `interact_instance` must be valid.
    ///
    /// ## Note
    ///
    /// See [`Self::interact_get_property_set`] about the weird name of the
    /// first parameter.
    #[no_sys_fn]
    pub(crate) unsafe fn interacte_get_descriptor_property_set(
        &self,
        interact_instance: &InteractDescriptor,
    ) -> crate::low::Result<InteractDescriptorPropertySet> {
        Ok(InteractDescriptorPropertySet::from(unsafe {
            self.interact_get_property_set(interact_instance.sys_handle())?
        }))
    }

    /// ## Safety
    ///
    /// `self` and `interact_instance` must be valid.
    #[no_sys_fn]
    pub(crate) unsafe fn interacte_get_instance_property_set(
        &self,
        interact_instance: &InteractInstance,
    ) -> crate::low::Result<InteractInstancePropertySet> {
        Ok(InteractInstancePropertySet::from(unsafe {
            self.interact_get_property_set(interact_instance.sys_handle())?
        }))
    }

    /// ## Safety
    ///
    /// `self` and `interact_instance` must be valid.
    pub(crate) unsafe fn interact_redraw(
        &self,
        interact_instance: &InteractInstance,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(interact_instance.sys_handle()) })
    }

    /// ## Safety
    ///
    /// `self` and `interact_instance` must be valid.
    pub(crate) unsafe fn interact_swap_buffers(
        &self,
        interact_instance: &InteractInstance,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(interact_instance.sys_handle()) })
    }
}

#[openfx_internal_macros::low_impl_suite]
impl MemorySuiteV1 {
    /// ## Safety
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
    /// ## Safety
    ///
    /// `self` and `allocated_data` must be valid.
    pub unsafe fn memory_free(&self, allocated_data: *mut c_void) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(allocated_data) })
    }
}

#[repr(transparent)]
pub struct MessageHandle(*mut c_void);

impl MessageHandle {
    pub const NULL: Self = Self(std::ptr::null_mut());
}

impl From<&ImageEffectDescriptor> for MessageHandle {
    fn from(descriptor: &ImageEffectDescriptor) -> Self {
        Self(descriptor.sys_handle() as *mut c_void)
    }
}
impl From<&ImageEffectInstance> for MessageHandle {
    fn from(instance: &ImageEffectInstance) -> Self {
        Self(instance.sys_handle() as *mut c_void)
    }
}

#[repr(transparent)]
pub struct PersistentMessageHandle(NonNull<c_void>);

impl From<&ImageEffectDescriptor> for PersistentMessageHandle {
    fn from(descriptor: &ImageEffectDescriptor) -> Self {
        // TODO: make objects store non-null pointers so that `.sys_handle()`
        // can return `NonNull` and `new_unchecked` can be removed here?
        Self(unsafe { NonNull::new_unchecked(descriptor.sys_handle() as *mut c_void) })
    }
}
impl From<&ImageEffectInstance> for PersistentMessageHandle {
    fn from(instance: &ImageEffectInstance) -> Self {
        // TODO: make objects store non-null pointers so that `.sys_handle()`
        // can return `NonNull` and `new_unchecked` can be removed here?
        Self(unsafe { NonNull::new_unchecked(instance.sys_handle() as *mut c_void) })
    }
}

#[openfx_internal_macros::low_impl_suite]
impl MessageSuiteV1 {
    /// ## Safety
    ///
    /// `self` and `sys_handle` must be valid.
    ///
    /// ## Note
    ///
    /// This function is `pub` so that callers who need to pass variadic
    /// arguments more flexibly can do so. Regular use should prefer
    /// [`Self::message`].
    ///
    /// ## TODO
    ///
    /// We currently don't have an enum type for `kOfxMessageType*`?
    pub unsafe extern "C" fn sys_message(
        &self,
        sys_handle: *mut c_void,
        message_type: NonNull<c_char>,
        message_id: *const c_char,
        format: NonNull<c_char>,
        args: ...
    ) -> OfxStatus {
        let sys_fn = unsafe { sys_fn!("message") };

        unsafe {
            sys_fn(
                sys_handle,
                message_type.as_ptr(),
                message_id,
                format.as_ptr(),
                args,
            )
        }
    }

    /// ## Safety
    ///
    /// `self` and `handle` must be valid.
    #[no_sys_fn]
    pub unsafe fn message(
        &self,
        handle: MessageHandle,
        message_type: &CStr,
        message_id: Option<&CStr>,
        message: &CStr,
    ) -> crate::low::Result<()> {
        crate::low::Status::result_from(unsafe {
            self.sys_message(
                handle.0,
                NonNull::new_unchecked(message_type.as_ptr() as *mut c_char),
                message_id.map_or(std::ptr::null(), |s| s.as_ptr()),
                NonNull::new_unchecked(c"%s".as_ptr() as *mut c_char),
                message,
            )
        })
    }
}

#[openfx_internal_macros::low_impl_suite]
impl MessageSuiteV2 {
    /// ## Safety
    ///
    /// `self` and `sys_handle` must be valid.
    ///
    /// ## Note
    ///
    /// For `clear_persistent_message_with_*`, don't use
    /// `impl Into<OfxImageEffectHandle>`, because that would allow NULL.
    pub unsafe fn clear_persistent_message(&self, handle: MessageHandle) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(handle.0) })
    }

    /// ## Safety
    ///
    /// `self` and `sys_handle` must be valid.
    ///
    /// ## Note
    ///
    /// See [`MessageSuiteV1::sys_message`].
    ///
    /// ## TODO
    ///
    /// See [`MessageSuiteV1::sys_message`].
    pub unsafe extern "C" fn sys_message(
        &self,
        sys_handle: *mut c_void,
        message_type: NonNull<c_char>,
        message_id: *const c_char,
        format: NonNull<c_char>,
        args: ...
    ) -> OfxStatus {
        let sys_fn = unsafe { sys_fn!("message") };

        unsafe {
            sys_fn(
                sys_handle,
                message_type.as_ptr(),
                message_id,
                format.as_ptr(),
                args,
            )
        }
    }

    /// ## Safety
    ///
    /// `self` and `handle` must be valid.
    #[no_sys_fn]
    pub unsafe fn message(
        &self,
        handle: MessageHandle,
        message_type: &CStr,
        message_id: Option<&CStr>,
        message: &CStr,
    ) -> crate::low::Result<()> {
        crate::low::Status::result_from(unsafe {
            self.sys_message(
                handle.0,
                NonNull::new_unchecked(message_type.as_ptr() as *mut c_char),
                message_id.map_or(std::ptr::null(), |s| s.as_ptr()),
                NonNull::new_unchecked(c"%s".as_ptr() as *mut c_char),
                message,
            )
        })
    }

    /// ## Safety
    ///
    /// `self` and `sys_handle` must be valid.
    ///
    /// ## Note
    ///
    /// This function is `pub` so that callers who need to pass variadic
    /// arguments more flexibly can do so. Regular use should prefer
    /// [`Self::set_persistent_message`].
    ///
    /// ## TODO
    ///
    /// See [`MessageSuiteV1::sys_message`] for more.
    pub unsafe extern "C" fn sys_set_persistent_message(
        &self,
        sys_handle: NonNull<c_void>,
        message_type: NonNull<c_char>,
        message_id: *const c_char,
        format: NonNull<c_char>,
        args: ...
    ) -> OfxStatus {
        let sys_fn = unsafe { sys_fn!("setPersistentMessage") };

        unsafe {
            sys_fn(
                sys_handle.as_ptr(),
                message_type.as_ptr(),
                message_id,
                format.as_ptr(),
                args,
            )
        }
    }

    /// ## Safety
    ///
    /// `self` and `handle` must be valid.
    #[no_sys_fn]
    pub unsafe fn set_persistent_message(
        &self,
        handle: PersistentMessageHandle,
        message_type: &CStr,
        message_id: Option<&CStr>,
        message: &CStr,
    ) -> crate::low::Result<()> {
        unsafe {
            crate::low::Status::result_from(self.sys_set_persistent_message(
                handle.0,
                NonNull::new_unchecked(message_type.as_ptr() as *mut c_char),
                message_id.map_or(std::ptr::null(), |s| s.as_ptr()),
                NonNull::new_unchecked(c"%s".as_ptr() as *mut c_char),
                message,
            ))
        }
    }
}

#[openfx_internal_macros::low_impl_suite]
impl MultiThreadSuiteV1 {
    /// ## Safety
    ///
    /// `self` must be valid.
    unsafe fn sys_multi_thread(
        &self,
        sys_func: unsafe extern "C" fn(
            thread_index: c_uint,
            thread_max: c_uint,
            custom_arg: *mut c_void,
        ),
        n_threads: c_uint,
        custom_arg: *mut c_void,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!("multiThread") };

        crate::low::Status::result_from(unsafe { sys_fn(Some(sys_func), n_threads, custom_arg) })
    }

    /// ## Safety
    ///
    /// `self` must be valid.
    ///
    /// ## TODO
    ///
    /// Find a better name for this function?
    #[no_sys_fn]
    pub unsafe fn multi_thread<T: Send + Sync>(
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

        unsafe { self.sys_multi_thread(func, n_threads, custom_arg as *const T as *mut c_void) }
    }

    /// ## Safety
    ///
    /// `self` must be valid.
    pub unsafe fn multi_thread_index(&self) -> crate::low::Result<c_uint> {
        let sys_fn = unsafe { sys_fn!() };

        let mut thread_index = std::mem::MaybeUninit::<c_uint>::uninit();

        crate::low::Status::result_from(unsafe { sys_fn(thread_index.as_mut_ptr()) })?;

        Ok(unsafe { thread_index.assume_init() })
    }

    /// ## Safety
    ///
    /// `self` must be valid.
    pub unsafe fn multi_thread_is_spawned_thread(&self) -> bool {
        let sys_fn = unsafe { sys_fn!() };

        (unsafe { sys_fn() }) != 0
    }

    /// ## Safety
    ///
    /// `self` must be valid.
    pub unsafe fn multi_thread_num_cpus(&self) -> crate::low::Result<c_uint> {
        let sys_fn = unsafe { sys_fn!("multiThreadNumCPUs") };

        let mut num_cpus = std::mem::MaybeUninit::<c_uint>::uninit();

        crate::low::Status::result_from(unsafe { sys_fn(num_cpus.as_mut_ptr()) })?;

        Ok(unsafe { num_cpus.assume_init() })
    }

    /// ## Safety
    ///
    /// `self` must be valid.
    pub unsafe fn mutex_create(&self, lock_count: c_int) -> crate::low::Result<Mutex> {
        let sys_fn = unsafe { sys_fn!() };

        let mut mutex = std::mem::MaybeUninit::<OfxMutexHandle>::uninit();

        crate::low::Status::result_from(unsafe { sys_fn(mutex.as_mut_ptr(), lock_count) })?;

        Ok(unsafe { Mutex::from_sys_handle(mutex.assume_init()) })
    }

    /// ## Safety
    ///
    /// `self` and `mutex` must be valid.
    ///
    /// ## TODO
    ///
    /// Should we consume `mutex` here? Or is that out of the scope of the low
    /// layer?
    pub(crate) unsafe fn mutex_destroy(&self, mutex: &Mutex) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(mutex.sys_handle()) })
    }

    /// ## Safety
    ///
    /// `self` and `mutex` must be valid.
    pub(crate) unsafe fn mutex_lock(&self, mutex: &Mutex) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(mutex.sys_handle()) })
    }

    /// ## Safety
    ///
    /// `self` and `mutex` must be valid.
    pub(crate) unsafe fn mutex_try_lock(&self, mutex: &Mutex) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe { sys_fn(mutex.sys_handle()) })
    }

    /// ## Safety
    ///
    /// `self` and `mutex` must be valid.
    pub(crate) unsafe fn mutex_unlock(&self, mutex: &Mutex) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!("mutexUnLock") };

        crate::low::Status::result_from(unsafe { sys_fn(mutex.sys_handle()) })
    }
}

#[openfx_internal_macros::low_impl_suite]
impl OpenCLProgramSuiteV1 {}

#[openfx_internal_macros::low_impl_suite]
impl ParametricParameterSuiteV1 {
    /// ## Safety
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

    /// ## Safety
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

    /// ## Safety
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

    /// ## Safety
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

    /// ## Safety
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

    /// ## Safety
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

    /// ## Safety
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
    /// ## Safety
    ///
    /// `self` and `effect_instance` must be valid.
    pub(crate) unsafe fn progress_end(
        &self,
        effect_instance: &ImageEffectInstance,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(effect_instance.sys_handle() as *mut c_void)
        })
    }

    /// ## Safety
    ///
    /// `self` and `effect_instance` must be valid.
    pub(crate) unsafe fn progress_start(
        &self,
        effect_instance: &ImageEffectInstance,
        label: &CStr,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(effect_instance.sys_handle() as *mut c_void, label.as_ptr())
        })
    }

    /// ## Safety
    ///
    /// `self` and `effect_instance` must be valid.
    pub(crate) unsafe fn progress_update(
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
    /// ## Safety
    ///
    /// `self` and `effect_instance` must be valid.
    pub(crate) unsafe fn progress_end(
        &self,
        effect_instance: &ImageEffectInstance,
    ) -> crate::low::Result<()> {
        let sys_fn = unsafe { sys_fn!() };

        crate::low::Status::result_from(unsafe {
            sys_fn(effect_instance.sys_handle() as *mut c_void)
        })
    }

    /// ## Safety
    ///
    /// `self` and `effect_instance` must be valid.
    pub(crate) unsafe fn progress_start(
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

    /// ## Safety
    ///
    /// `self` and `effect_instance` must be valid.
    pub(crate) unsafe fn progress_update(
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
impl TimeLineSuiteV1 {
    /// ## Safety
    ///
    /// `self` and `instance` must be valid.
    pub(crate) unsafe fn get_time(
        &self,
        instance: &ImageEffectInstance,
    ) -> crate::low::Result<OfxTime> {
        let sys_fn = unsafe { sys_fn!() };

        let mut time = std::mem::MaybeUninit::<OfxTime>::uninit();

        crate::low::Status::result_from(unsafe {
            sys_fn(instance.sys_handle() as *mut c_void, time.as_mut_ptr())
        })?;

        Ok(unsafe { time.assume_init() })
    }

    /// ## Safety
    ///
    /// `self` and `instance` must be valid.
    pub(crate) unsafe fn get_time_bounds(
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

    /// ## Safety
    ///
    /// `self` and `instance` must be valid.
    pub(crate) unsafe fn goto_time(
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
