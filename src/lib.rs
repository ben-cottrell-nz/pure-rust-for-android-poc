use std::env;
use egl::*;
use jni::*;
use jni::errors::Error::JavaException;
use jni::objects::{JObject, JValue};
use jni::sys::{jint, jobject};

enum AndroidToastDuration {
    LengthShort,
    LengthLong
}

pub fn main() {
    println!("other version main");
}

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn android_main() {
    env::set_var("RUST_BACKTRACE","full");
    // let mut egl_major: EGLint = 3;
    // let mut egl_minor: EGLint = 0;
    // let egl_disp = egl::get_current_display();
    // egl::initialize(egl_disp.unwrap(), &mut egl_major, &mut egl_minor);
    // println!("initializing egl, version {}.{}", egl_major, egl_minor);
    // egl_disp.expect("no display");
    let android_ctx = ndk_context::android_context();
    let jvm = unsafe { jni::JavaVM::from_raw(android_ctx.vm().cast())}.unwrap();
    let env = jvm.attach_current_thread().unwrap();
    let notify_class = env.find_class("android/widget/Toast").unwrap();
    //let test_class = env.find_class("this/class/does/not/exist").expect("failed to find non-existent Java class");
    // let make_toast_result = env.call_static_method("android/widget/Toast",
    //                                                                          "makeText",
    //                                                                          "(Landroid/content/Context;Ljava/lang/String;I)Landroid/widget/Toast;",
    //                                                                          &[(android_ctx.context() as jobject).into(),
    //                                                                              env.new_string("hello rust android poc").unwrap().into(),
    //                                                                              jni::objects::JValue::Int(AndroidToastDuration::LengthLong as jint)]);
    //make_toast_result.i().unwrap();
    //panic!("did something bad");

    //let toast_result_inner = make_toast_result.unwrap().l();
    // if toast_result_inner.is_err() {
    //     println!(toast_result_inner.unwrap_err())
    // }
    // env.call_method(make_toast_result.unwrap().l().unwrap(), "show", "()V", &[]);
    println!("running on android");
}