# jniLibs

The Rust core's shared libraries are placed here at build time, one folder per ABI:

```
jniLibs/
├── arm64-v8a/libgroup_core.so   # real devices
└── x86_64/libgroup_core.so      # emulator
```

They are produced by cross-compiling the `core/` crate with `cargo-ndk` (see the
top-level README, "Producing the .so (Tier 2)"). They are intentionally git-ignored
because they are large build artifacts, not source.
