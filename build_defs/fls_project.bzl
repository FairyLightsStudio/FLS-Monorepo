"""Project marker used by fls to discover coarse cross-project dependencies."""

def _fls_project_impl(_ctx: AnalysisContext):
    return [DefaultInfo()]

fls_project = rule(
    impl = _fls_project_impl,
    attrs = {
        "deps": attrs.list(attrs.dep(), default = []),
    },
)
