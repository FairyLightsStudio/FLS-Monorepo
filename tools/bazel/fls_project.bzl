"""Project marker used by fls to discover coarse cross-project dependencies.

Bazel port of //build_defs:fls_project.bzl (Buck2, untouched). The Buck2
rule takes only `deps`; this port keeps the same attribute set. The
Buck2 `AnalysisContext` impl becomes a Bazel `ctx` impl returning a
DefaultInfo that forwards transitive files so
`bazel query "deps(//...)"` surfaces the coarse project graph.

Usage (mirrors BUCK):

    load("//tools/bazel:fls_project.bzl", "fls_project")

    fls_project(
        name = "workspace",
        deps = [...],  # optional cross-project edges
    )
"""

def _fls_project_impl(ctx):
    return [
        DefaultInfo(
            files = depset(
                transitive = [
                    dep[DefaultInfo].files
                    for dep in ctx.attr.deps
                ],
            ),
        ),
    ]

fls_project = rule(
    implementation = _fls_project_impl,
    attrs = {
        "deps": attr.label_list(default = []),
    },
)
