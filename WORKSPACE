load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")


# rules_rust

http_archive(
    name = "rules_rust",
    integrity = "sha256-GuRaQT0LlDOYcyDfKtQQ22oV+vtsiM8P0b87qsvoJts=",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.39.0/rules_rust-v0.39.0.tar.gz"],
)

load(
    "@rules_rust//rust:repositories.bzl",
    "rules_rust_dependencies",
    "rust_register_toolchains",
)

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    versions = ["1.74.0"],
    extra_target_triples = [
        "aarch64-linux-android",
        "armv7-linux-androideabi",
        "i686-linux-android",
        "x86_64-linux-android",
    ],
)


# crate universe
# manage external Rust dependencies using the Crate Universe rules
# docs: https://bazelbuild.github.io/rules_rust/crate_universe.html
# example: https://github.com/bazelbuild/rules_rust/tree/main/examples/crate_universe/no_cargo_manifests

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository", "render_config", "splicing_config")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//rust/crates:Cargo.lock",
    lockfile = "//rust/crates:Cargo.Bazel.lock",
    isolated = False,  # cache results of the previous invocation to
                       # ${HOME}/.cargo so using it is fast
    packages = {
        "android-tzdata": crate.spec(version = "0.1.1"),
        "iana-time-zone": crate.spec(version = "0.1.61"),
        "jiff": crate.spec(version = "0.1.13"),
    },
    splicing_config = splicing_config(resolver_version = "2"),
    render_config = render_config(default_package_name = ""),
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()


# === Android === #

http_archive(
    name = "platforms",
    urls = [
        "https://github.com/bazelbuild/platforms/releases/download/0.0.8/platforms-0.0.8.tar.gz",
    ],
    sha256 = "8150406605389ececb6da07cbcb509d5637a3ab9a24bc69b1101531367d89d74",
)

## Android

http_archive(
    name = "rules_android",
    sha256 = "cd06d15dd8bb59926e4d65f9003bfc20f9da4b2519985c27e190cddc8b7a7806",
    strip_prefix = "rules_android-0.1.1",
    urls = ["https://github.com/bazelbuild/rules_android/archive/v0.1.1.zip"],
)

load("@rules_android//android:rules.bzl", "android_sdk_repository")

android_sdk_repository(name = "androidsdk")

http_archive(
    name = "rules_android_ndk",
    sha256 = "b1a5ddd784e6ed915c2035c0db536a278b5f50c64412128c06877115991391ef",
    strip_prefix = "rules_android_ndk-877c68ef34c9f3353028bf490d269230c1990483",
    url = "https://github.com/bazelbuild/rules_android_ndk/archive/877c68ef34c9f3353028bf490d269230c1990483.zip",
)

load("@rules_android_ndk//:rules.bzl", "android_ndk_repository")

android_ndk_repository(name = "androidndk")

## Kotlin

http_archive(
    name = "rules_kotlin",
    sha256 = "15afe2d727f0dba572e0ce58f1dac20aec1441422ca65f7c3f7671b47fd483bf",
    url = "https://github.com/bazelbuild/rules_kotlin/releases/download/v1.7.0/rules_kotlin_release.tgz",
)

load("@rules_kotlin//kotlin:repositories.bzl", "kotlin_repositories", "kotlinc_version")

_KOTLIN_COMPILER_VERSION = "1.7.20"
_KOTLIN_COMPILER_SHA = "5e3c8d0f965410ff12e90d6f8dc5df2fc09fd595a684d514616851ce7e94ae7d"

kotlin_repositories(
    compiler_release = kotlinc_version(
        release = _KOTLIN_COMPILER_VERSION,
        sha256 = _KOTLIN_COMPILER_SHA,
    ),
)

load("@rules_kotlin//kotlin:core.bzl", "kt_register_toolchains")

kt_register_toolchains()

## JVM External

RULES_JVM_EXTERNAL_TAG = "4.5"
RULES_JVM_EXTERNAL_SHA = "b17d7388feb9bfa7f2fa09031b32707df529f26c91ab9e5d909eb1676badd9a6"

http_archive(
    name = "rules_jvm_external",
    strip_prefix = "rules_jvm_external-%s" % RULES_JVM_EXTERNAL_TAG,
    sha256 = RULES_JVM_EXTERNAL_SHA,
    url = "https://github.com/bazelbuild/rules_jvm_external/archive/%s.zip" % RULES_JVM_EXTERNAL_TAG,
)

load("@rules_jvm_external//:repositories.bzl", "rules_jvm_external_deps")

rules_jvm_external_deps()

load("@rules_jvm_external//:setup.bzl", "rules_jvm_external_setup")

rules_jvm_external_setup()

load("@rules_jvm_external//:defs.bzl", "maven_install")

_LIFECYCLE_VERSION = "2.7.0"

maven_install(
    artifacts = [
        "androidx.activity:activity:1.8.2",
        "androidx.annotation:annotation:1.7.1",
        "androidx.appcompat:appcompat:1.6.1",
        "androidx.core:core-ktx:1.12.0",
        "androidx.lifecycle:lifecycle-livedata-ktx:{}".format(_LIFECYCLE_VERSION),
        "androidx.lifecycle:lifecycle-process:{}".format(_LIFECYCLE_VERSION),
        "androidx.lifecycle:lifecycle-runtime-ktx:{}".format(_LIFECYCLE_VERSION),
        "androidx.lifecycle:lifecycle-service:{}".format(_LIFECYCLE_VERSION),
        "androidx.lifecycle:lifecycle-viewmodel-ktx:{}".format(_LIFECYCLE_VERSION),
        "androidx.lifecycle:lifecycle-viewmodel-savedstate:{}".format(_LIFECYCLE_VERSION),
    ],
    repositories = [
        "https://maven.google.com",
        "https://repo1.maven.org/maven2",
    ],
)
