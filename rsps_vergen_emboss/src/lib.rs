#[doc(hidden)]
pub use emboss::emboss_env as __rsps_emboss_env;

#[macro_export]
macro_rules! rsps_vergen_emboss {
    (groups=$($group: ident),+) => {
        $(
            rsps_vergen_emboss!(group=$group);
        )+
    };
    (group=build) => {
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_BUILD_DATE");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_BUILD_TIME");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_BUILD_TIMESTAMP");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "CARGO_PKG_VERSION", key = "VERGEN_BUILD_SEMVER");
    };
    (group=git) => {
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_GIT_BRANCH");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_GIT_COMMIT_DATE");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_GIT_COMMIT_TIME");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_GIT_COMMIT_TIMESTAMP");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_GIT_SEMVER");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_GIT_SEMVER_LIGHTWEIGHT");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_GIT_SHA");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_GIT_SHA_SHORT");
    };
    (group=rustc) => {
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_RUSTC_CHANNEL");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_RUSTC_COMMIT_DATE");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_RUSTC_COMMIT_HASH");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_RUSTC_HOST_TRIPLE");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_RUSTC_LLVM_VERSION");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_RUSTC_SEMVER");
    };
    (group=cargo) => {
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_CARGO_FEATURES");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_CARGO_TARGET_TRIPLE");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_CARGO_OPT_LEVEL", key = "VERGEN_CARGO_PROFILE");
    };
    (group=rust) => {
        rsps_vergen_emboss!(groups=rustc,cargo);
    };
    (group=sysinfo) => {
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_SYSINFO_NAME");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_SYSINFO_OS_VERSION");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_SYSINFO_USER");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_SYSINFO_TOTAL_MEMORY");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_SYSINFO_CPU_VENDOR");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_SYSINFO_CPU_CORE_COUNT");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_SYSINFO_CPU_NAME");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_SYSINFO_CPU_BRAND");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_SYSINFO_CPU_FREQUENCY");
    };
    (group=all) => {
        rsps_vergen_emboss!(groups=build,git,rustc,cargo,sysinfo);
    };
    (group=rsps) => {
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "CARGO_PKG_VERSION", key = "VERGEN_BUILD_SEMVER");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_CARGO_OPT_LEVEL", ke = "VERGEN_CARGO_PROFILE");

        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_BUILD_TIMESTAMP");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_RUSTC_SEMVER");
        rsps_vergen_emboss::__rsps_emboss_env!(env_var = "VERGEN_CARGO_FEATURES");
    };
    () => {
        rsps_vergen_emboss!(group=rsps);
    };
}
