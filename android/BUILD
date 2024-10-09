load("@rules_kotlin//kotlin:android.bzl", "kt_android_library")
load("@rules_kotlin//kotlin:core.bzl", "kt_compiler_plugin")

sh_test(
    name = "test_no_crash",
    srcs = ["test_no_crash.sh"],
    data = [":app"],
)

# Generates a app binary (.apk).
android_binary(
    name = "app",
    manifest = "AndroidManifest.xml",
    manifest_values = {
        "applicationId": "example.rust",
        "versionName": "1",
        "versionCode": "1",
    },
    deps = [":lib"],
)

kt_android_library(
    name = "lib",
    srcs = glob(["kt/*.kt"]),
    custom_package = "example.rust",
    manifest = "LibraryManifest.xml",
    resource_files = glob(["res/**/*"], exclude = ["**/.DS_Store"]),
    deps = [
        "//android/jni:rust_java",
        "@maven//:androidx_activity_activity",
        "@maven//:androidx_annotation_annotation",
        "@maven//:androidx_appcompat_appcompat",
    ],
)

# install the debug app (app.apk) with adb and run it
sh_binary(
    name = "install",
    srcs = ["install.sh"],
    data = [":app"],
)
