pub mod real_c_headers {
    macro_rules! include_c_header {
        ($name:ident, $file_name:literal) => {
            pub(crate) static $name: &str =
                include_str!(concat!("../../../../vendor/openfx/include/", $file_name));
        };
    }

    include_c_header!(OFX_COLOUR, "ofxColour.h");
    include_c_header!(OFX_CORE, "ofxCore.h");
    include_c_header!(OFX_DIALOG, "ofxDialog.h");
    include_c_header!(OFX_DRAW_SUITE, "ofxDrawSuite.h");
    include_c_header!(OFX_GPU_RENDER, "ofxGPURender.h");
    include_c_header!(OFX_IMAGE_EFFECT, "ofxImageEffect.h");
    include_c_header!(OFX_INTERACT, "ofxInteract.h");
    include_c_header!(OFX_KEY_SYMS, "ofxKeySyms.h");
    include_c_header!(OFX_MEMORY, "ofxMemory.h");
    include_c_header!(OFX_MESSAGE, "ofxMessage.h");
    include_c_header!(OFX_MULTI_THREAD, "ofxMultiThread.h");
    include_c_header!(OFX_OLD, "ofxOld.h");
    include_c_header!(OFX_OPENGL_RENDER, "ofxOpenGLRender.h");
    include_c_header!(OFX_PARAM, "ofxParam.h");
    include_c_header!(OFX_PARAMETRIC_PARAM, "ofxParametricParam.h");
    include_c_header!(OFX_PIXELS, "ofxPixels.h");
    include_c_header!(OFX_PROGRESS, "ofxProgress.h");
    include_c_header!(OFX_PROPERTY, "ofxProperty.h");
    include_c_header!(OFX_TIME_LINE, "ofxTimeLine.h");

    macro_rules! make_test_real_file {
        ($tester_name:path, $fn_name: ident, $static_name:ident) => {
            #[test]
            #[expect(non_snake_case)]
            fn $fn_name() {
                $tester_name(crate::test_fixtures::real_c_headers::$static_name);
            }
        };
    }

    macro_rules! make_test_real_files {
        ($tester_name:path) => {
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxColour,
                OFX_COLOUR
            );
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxCore,
                OFX_CORE
            );
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxDialog,
                OFX_DIALOG
            );
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxDrawSuite,
                OFX_DRAW_SUITE
            );
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxGPURender,
                OFX_GPU_RENDER
            );
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxImageEffect,
                OFX_IMAGE_EFFECT
            );
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxInteract,
                OFX_INTERACT
            );
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxKeySyms,
                OFX_KEY_SYMS
            );
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxMemory,
                OFX_MEMORY
            );
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxMessage,
                OFX_MESSAGE
            );
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxMultiThread,
                OFX_MULTI_THREAD
            );
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxOld,
                OFX_OLD
            );
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxOpenGLRender,
                OFX_OPENGL_RENDER
            );
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxParam,
                OFX_PARAM
            );
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxParametricParam,
                OFX_PARAMETRIC_PARAM
            );
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxPixels,
                OFX_PIXELS
            );
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxProgress,
                OFX_PROGRESS
            );
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxProperty,
                OFX_PROPERTY
            );
            crate::test_fixtures::real_c_headers::make_test_real_file!(
                $tester_name,
                test_real_file_ofxTimeLine,
                OFX_TIME_LINE
            );
        };
    }

    pub(crate) use make_test_real_file;
    pub(crate) use make_test_real_files;
}
