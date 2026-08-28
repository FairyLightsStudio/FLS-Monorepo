"""FLS 自定义 zig C/C++ 工具链。

与 prelude 的 `cxx_zig_toolchain` 的区别：

prelude 用 `cmd_script()` 把 `zig cc` 包装成一个位于 buck-out 的 shell 脚本
（`zig_cc.sh`），脚本内部以**相对路径**（相对 repo root）引用 zig 二进制。
但 rust build script 的 CC shim（`from_any_dir.py`）会在 build script 自行
选择的任意 cwd 下执行该 wrapper，导致脚本内部的相对路径解析失败
（`No such file or directory`, exit 127），例如 aws-lc-rs / ring / zstd-sys
等原生 crate 的 build script 全部编译失败。

这里直接让 compiler/linker/archiver 的 `RunInfo` 指向
`cmd_args(zig, "cc")` 形式的参数列表：CC shim 生成
`from_any_dir.py --cwd=... <zig绝对路径> cc <flags> "$@"`，
`from_any_dir` 会把 zig 路径重写为相对 build script cwd 的可解析路径，
`os.execl` 直接执行 zig 二进制本身（不经过任何 wrapper 脚本），
因此从任意 cwd 都能正常工作。
"""

load("@prelude//cxx:cxx_toolchain_types.bzl", "BinaryUtilitiesInfo", "CCompilerInfo", "CxxCompilerInfo", "CxxInternalTools", "LinkerInfo", "LinkerType", "ShlibInterfacesMode", "StripFlagsInfo", "cxx_toolchain_infos")
load("@prelude//cxx:debug.bzl", "SplitDebugMode")
load("@prelude//cxx:headers.bzl", "HeaderMode")
load("@prelude//cxx:linker.bzl", "is_pdb_generated")
load("@prelude//linking:link_info.bzl", "LinkStyle")
load("@prelude//os_lookup:defs.bzl", "Os", "OsLookup")
load("@prelude//toolchains/cxx/zig:defs.bzl", "ZigDistributionInfo")

def _get_linker_type(os: str) -> LinkerType:
    if os == "linux":
        return LinkerType("gnu")
    elif os == "macos" or os == "freebsd":
        return LinkerType("gnu")
    elif os == "windows":
        return LinkerType("windows")
    else:
        fail("Cannot determine linker type: Unknown OS '{}'".format(os))

def _cxx_zig_direct_toolchain_impl(ctx: AnalysisContext) -> list[Provider]:
    dist = ctx.attrs.distribution[ZigDistributionInfo]
    zig = ctx.attrs.distribution[RunInfo]
    target = ["-target", ctx.attrs.target] if ctx.attrs.target else []

    # 直接展开为 `zig cc ...`，不经 cmd_script 包装（见模块 docstring）。
    #
    # `-U__clang__` 是刻意的：cc-rs 通过运行 `$CC -E` 并检查输出里的
    # `"clang"` 标记来判定编译器家族。zig cc 底层是 clang，默认输出
    # `"clang"` 标记，导致 cc-rs 把它当作 Clang 家族，从而注入它自己生成
    # 的 `--target=<rustc triple>`（如 `--target=x86_64-unknown-linux-gnu`，
    # 含 `unknown` vendor 段）。zig 0.14 的 target 查询解析器不认 `unknown`
    # vendor（ziglang/zig#7360 至今未修），会直接报
    # `UnknownOperatingSystem` 而失败。
    #
    # 取消 `__clang__` 宏后，`$CC -E` 的输出只含 `"gcc"` 标记，cc-rs 会把
    # zig 判定为 GNU 家族，从而不注入 `--target=`，zig 用默认 native target
    # 编译（与 rustc triple 一致），C build script 正常通过。
    zig_cc = RunInfo(args = cmd_args(zig, "cc", "-U__clang__"))
    zig_cxx = RunInfo(args = cmd_args(zig, "c++", "-U__clang__"))
    zig_ar = RunInfo(args = cmd_args(zig, "ar"))
    zig_ranlib = RunInfo(args = cmd_args(zig, "ranlib"))

    return [ctx.attrs.distribution[DefaultInfo]] + cxx_toolchain_infos(
        internal_tools = ctx.attrs._cxx_internal_tools[CxxInternalTools],
        platform_name = dist.arch,
        c_compiler_info = CCompilerInfo(
            compiler = zig_cc,
            # 故意不用 "clang"：prelude 的 cargo_buildscript.bzl 在
            # compiler_type 以 "clang" 开头时会注入 `--target=<triple>` 与
            # `--ld-path=<shim>` 参数，而 zig cc 不认 clang 风格的
            # `--target=x86_64-unknown-linux-gnu`（只认 `-target x86_64-linux-gnu`）。
            # zig cc 完全兼容 gcc 风格参数，声明为 "gcc" 即可让 build script
            # 的 CC/CXX 以纯 gcc 风格调用，避免这两个参数。
            compiler_type = "gcc",
            compiler_flags = cmd_args(target, ctx.attrs.c_compiler_flags),
            preprocessor_flags = cmd_args(ctx.attrs.c_preprocessor_flags),
        ),
        cxx_compiler_info = CxxCompilerInfo(
            compiler = zig_cxx,
            compiler_type = "gcc",
            compiler_flags = cmd_args(target, ctx.attrs.cxx_compiler_flags),
            preprocessor_flags = cmd_args(ctx.attrs.cxx_preprocessor_flags),
        ),
        linker_info = LinkerInfo(
            archiver = zig_ar,
            archiver_type = "gnu",
            archiver_supports_argfiles = True,
            archive_objects_locally = False,
            binary_extension = "",
            generate_linker_maps = False,
            link_binaries_locally = False,
            link_libraries_locally = False,
            link_style = LinkStyle(ctx.attrs.link_style),
            link_weight = 1,
            linker = zig_cxx,
            linker_flags = cmd_args(target, ctx.attrs.linker_flags),
            object_file_extension = "o",
            shlib_interfaces = ShlibInterfacesMode("disabled"),
            shared_dep_runtime_ld_flags = ctx.attrs.shared_dep_runtime_ld_flags,
            shared_library_name_default_prefix = "lib",
            shared_library_name_format = "{}.so",
            shared_library_versioned_name_format = "{}.so.{}",
            static_dep_runtime_ld_flags = ctx.attrs.static_dep_runtime_ld_flags,
            static_library_extension = "a",
            static_pic_dep_runtime_ld_flags = ctx.attrs.static_pic_dep_runtime_ld_flags,
            independent_shlib_interface_linker_flags = ctx.attrs.shared_library_interface_flags,
            type = _get_linker_type(dist.os),
            use_archiver_flags = True,
            is_pdb_generated = is_pdb_generated(_get_linker_type(dist.os), ctx.attrs.linker_flags),
        ),
        binary_utilities_info = BinaryUtilitiesInfo(
            bolt_msdk = None,
            dwp = None,
            nm = RunInfo(args = ["nm"]),  # 不在 zig 发行包里。
            objcopy = RunInfo(args = ["objcopy"]),  # 不在 zig 发行包里。
            ranlib = zig_ranlib,
            strip = RunInfo(args = ["strip"]),  # 不在 zig 发行包里。
        ),
        header_mode = HeaderMode("symlink_tree_only"),
        strip_flags_info = StripFlagsInfo(
            strip_debug_flags = ctx.attrs.strip_debug_flags,
            strip_non_global_flags = ctx.attrs.strip_non_global_flags,
            strip_all_flags = ctx.attrs.strip_all_flags,
        ),
        split_debug_mode = SplitDebugMode("none"),
    )

cxx_zig_direct_toolchain = rule(
    impl = _cxx_zig_direct_toolchain_impl,
    attrs = {
        "c_compiler_flags": attrs.list(attrs.arg(), default = []),
        "c_preprocessor_flags": attrs.list(attrs.arg(), default = []),
        "cxx_compiler_flags": attrs.list(attrs.arg(), default = []),
        "cxx_preprocessor_flags": attrs.list(attrs.arg(), default = []),
        "distribution": attrs.exec_dep(providers = [RunInfo, ZigDistributionInfo]),
        "link_style": attrs.enum(
            LinkStyle.values(),
            default = "static",
            doc = """
            The default value of the `link_style` attribute for rules that use this toolchain.
            """,
        ),
        "linker_flags": attrs.list(attrs.arg(), default = []),
        "shared_dep_runtime_ld_flags": attrs.list(attrs.arg(), default = []),
        "shared_library_interface_flags": attrs.list(attrs.string(), default = []),
        "static_dep_runtime_ld_flags": attrs.list(attrs.arg(), default = []),
        "static_pic_dep_runtime_ld_flags": attrs.list(attrs.arg(), default = []),
        "strip_all_flags": attrs.option(attrs.list(attrs.arg()), default = None),
        "strip_debug_flags": attrs.option(attrs.list(attrs.arg()), default = None),
        "strip_non_global_flags": attrs.option(attrs.list(attrs.arg()), default = None),
        "target": attrs.option(attrs.string(), default = None),
        "_cxx_internal_tools": attrs.default_only(attrs.dep(providers = [CxxInternalTools], default = "prelude//cxx/tools:internal_tools")),
    },
    is_toolchain_rule = True,
)
