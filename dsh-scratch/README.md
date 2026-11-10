# dsh-scratch

最小示例 DeepSeek Harness 插件：注册一个 `scratch_echo` 工具，并用 `ctx.effect` 演示卸载清理。

## 加载

1. 生成本地 `cordis.yml`（插件路径必须是绝对路径，因此不提交，已被 `.gitignore` 忽略）：

   ```sh
   ./dsh-scratch/scripts/gen-cordis-yml.sh
   ```

2. 用该覆盖层启动 Web UI：

   ```sh
   dsh web --patch /绝对路径/dsh-scratch/cordis.yml
   ```

   在 DSH 源码检出中则为 `pnpm dsh web --patch <路径>/cordis.yml`。启动时终端打印 `[dsh-scratch] loaded`，卸载时打印 `[dsh-scratch] unloaded`。

3. 在对话中输入：`Use the scratch_echo tool to echo hello`。

参考：[第一个插件](https://deepseek-harness.github.io/deepseek-harness/develop/basic/)、[开发一个 Tool](https://deepseek-harness.github.io/deepseek-harness/develop/basic/tool)。
