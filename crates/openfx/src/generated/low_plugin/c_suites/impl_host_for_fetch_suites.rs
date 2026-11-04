use crate::{low_plugin::suites::*, sys_umbrella as su};
impl crate::low_plugin::Host {
    #[doc = "## Safety\n\n`self` must be valid."]
    pub unsafe fn fetch_dialog_suite_v1(&self) -> Option<DialogSuiteV1> {
        let sys_suite = unsafe { self.sys_fetch_suite(su::kOfxDialogSuite, 1) };
        DialogSuiteV1::try_from(sys_suite)
    }
    #[doc = "## Safety\n\n`self` must be valid."]
    pub unsafe fn fetch_draw_suite_v1(&self) -> Option<DrawSuiteV1> {
        let sys_suite = unsafe { self.sys_fetch_suite(su::kOfxDrawSuite, 1) };
        DrawSuiteV1::try_from(sys_suite)
    }
    #[doc = "## Safety\n\n`self` must be valid."]
    pub unsafe fn fetch_image_effect_open_gl_render_suite_v1(
        &self,
    ) -> Option<ImageEffectOpenGLRenderSuiteV1> {
        let sys_suite = unsafe { self.sys_fetch_suite(su::kOfxOpenGLRenderSuite, 1) };
        ImageEffectOpenGLRenderSuiteV1::try_from(sys_suite)
    }
    #[doc = "## Safety\n\n`self` must be valid."]
    pub unsafe fn fetch_image_effect_suite_v1(&self) -> Option<ImageEffectSuiteV1> {
        let sys_suite = unsafe { self.sys_fetch_suite(su::kOfxImageEffectSuite, 1) };
        ImageEffectSuiteV1::try_from(sys_suite)
    }
    #[doc = "## Safety\n\n`self` must be valid."]
    pub unsafe fn fetch_interact_suite_v1(&self) -> Option<InteractSuiteV1> {
        let sys_suite = unsafe { self.sys_fetch_suite(su::kOfxInteractSuite, 1) };
        InteractSuiteV1::try_from(sys_suite)
    }
    #[doc = "## Safety\n\n`self` must be valid."]
    pub unsafe fn fetch_memory_suite_v1(&self) -> Option<MemorySuiteV1> {
        let sys_suite = unsafe { self.sys_fetch_suite(su::kOfxMemorySuite, 1) };
        MemorySuiteV1::try_from(sys_suite)
    }
    #[doc = "## Safety\n\n`self` must be valid."]
    pub unsafe fn fetch_message_suite_v1(&self) -> Option<MessageSuiteV1> {
        let sys_suite = unsafe { self.sys_fetch_suite(su::kOfxMessageSuite, 1) };
        MessageSuiteV1::try_from(sys_suite)
    }
    #[doc = "## Safety\n\n`self` must be valid."]
    pub unsafe fn fetch_message_suite_v2(&self) -> Option<MessageSuiteV2> {
        let sys_suite = unsafe { self.sys_fetch_suite(su::kOfxMessageSuite, 2) };
        MessageSuiteV2::try_from(sys_suite)
    }
    #[doc = "## Safety\n\n`self` must be valid."]
    pub unsafe fn fetch_multi_thread_suite_v1(&self) -> Option<MultiThreadSuiteV1> {
        let sys_suite = unsafe { self.sys_fetch_suite(su::kOfxMultiThreadSuite, 1) };
        MultiThreadSuiteV1::try_from(sys_suite)
    }
    #[doc = "## Safety\n\n`self` must be valid."]
    pub unsafe fn fetch_open_cl_program_suite_v1(&self) -> Option<OpenCLProgramSuiteV1> {
        let sys_suite = unsafe { self.sys_fetch_suite(su::kOfxOpenCLProgramSuite, 1) };
        OpenCLProgramSuiteV1::try_from(sys_suite)
    }
    #[doc = "## Safety\n\n`self` must be valid."]
    pub unsafe fn fetch_parameter_suite_v1(&self) -> Option<ParameterSuiteV1> {
        let sys_suite = unsafe { self.sys_fetch_suite(su::kOfxParameterSuite, 1) };
        ParameterSuiteV1::try_from(sys_suite)
    }
    #[doc = "## Safety\n\n`self` must be valid."]
    pub unsafe fn fetch_parametric_parameter_suite_v1(&self) -> Option<ParametricParameterSuiteV1> {
        let sys_suite = unsafe { self.sys_fetch_suite(su::kOfxParametricParameterSuite, 1) };
        ParametricParameterSuiteV1::try_from(sys_suite)
    }
    #[doc = "## Safety\n\n`self` must be valid."]
    pub unsafe fn fetch_progress_suite_v1(&self) -> Option<ProgressSuiteV1> {
        let sys_suite = unsafe { self.sys_fetch_suite(su::kOfxProgressSuite, 1) };
        ProgressSuiteV1::try_from(sys_suite)
    }
    #[doc = "## Safety\n\n`self` must be valid."]
    pub unsafe fn fetch_progress_suite_v2(&self) -> Option<ProgressSuiteV2> {
        let sys_suite = unsafe { self.sys_fetch_suite(su::kOfxProgressSuite, 2) };
        ProgressSuiteV2::try_from(sys_suite)
    }
    #[doc = "## Safety\n\n`self` must be valid."]
    pub unsafe fn fetch_property_suite_v1(&self) -> Option<PropertySuiteV1> {
        let sys_suite = unsafe { self.sys_fetch_suite(su::kOfxPropertySuite, 1) };
        PropertySuiteV1::try_from(sys_suite)
    }
    #[doc = "## Safety\n\n`self` must be valid."]
    pub unsafe fn fetch_time_line_suite_v1(&self) -> Option<TimeLineSuiteV1> {
        let sys_suite = unsafe { self.sys_fetch_suite(su::kOfxTimeLineSuite, 1) };
        TimeLineSuiteV1::try_from(sys_suite)
    }
}
