package(default_visibility = ["//android:__pkg__"])

android_library(
    name = "rust_java",
    srcs = glob(["*.java"]),
    deps = [":rust_jni"],
)

cc_library(
    name = "rust_jni",
    hdrs = ["logging.h"],
    srcs = [
        "rust_jni.cpp",
        "logging.cpp"
    ],
    deps = ["//rust:rust_cc"],
    defines = ["DEBUG_LOGGING"],
)
