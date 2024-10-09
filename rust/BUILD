load("@rules_rust//rust:defs.bzl", "rust_library")


rust_library(
    name = "rust",
    srcs = glob(["src/**/*.rs"]),
    edition = "2021",
    deps = [
        ":alloc",
        "@crate_index//:android-tzdata",
        "@crate_index//:iana-time-zone",
        "@crate_index//:jiff",
    ],
    visibility = [":__subpackages__"],
)

# C interface exposed for interop
cc_library(
    name = "rust_cc",
    hdrs = ["rust.h"],
    deps = [":rust"],
    visibility = ["//visibility:public"],
)

cc_library(
    name = "alloc",
    srcs = ["alloc.cc"],
)
