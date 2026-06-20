const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const lib = b.addStaticLibrary(.{
        .name = "phenotype-core",
        .root_source_file = b.path("packages/phenotype-core/src/main.zig"),
        .target = target,
        .optimize = optimize,
    });
    b.installArtifact(lib);

    const main = b.addExecutable(.{
        .name = "phenotype-sdk",
        .root_source_file = b.path("packages/phenotype-core/src/main.zig"),
        .target = target,
        .optimize = optimize,
    });
    b.installArtifact(main);
}
