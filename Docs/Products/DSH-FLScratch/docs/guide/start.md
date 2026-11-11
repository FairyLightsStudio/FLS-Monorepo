---
title: 快速开始
---

# 快速开始

当前阶段最有价值的体验方式：**用 DSH 加载 `dsh-scratch` 示例插件，确认工具调用链路跑通**，再把你的 Scratch 项目当成普通目录交给 DSH 读写。

## 1. 准备 DSH

参考 [DeepSeek Harness 文档](https://deepseek-harness.github.io/deepseek-harness/) 安装。本仓库用 [mise](https://mise.jdx.dev/) 管理工具链，在仓库根目录 `mise install` 即可装好需要的版本。

## 2. 加载 dsh-scratch 示例插件

在仓库根目录生成插件覆盖层（路径必须是绝对路径，因此生成的文件不提交）：

```sh
./dsh-scratch/scripts/gen-cordis-yml.sh
```

用它启动 Web UI：

```sh
dsh web --patch /绝对路径/dsh-scratch/cordis.yml
```

启动时终端会打印 `[dsh-scratch] loaded`，卸载时打印 `[dsh-scratch] unloaded`。

## 3. 试一次工具调用

在对话里输入：

```text
Use the scratch_echo tool to echo hello
```

看到工具返回，就说明"DSH + 插件工具"的链路是通的。

## 4. 把 Scratch 项目交给 DSH

把你正在做的 Scratch 项目目录（未打包的 `project.json` 或解包后的资源目录）放在一个普通文件夹里，在 DSH 会话里直接让它：

- 阅读并解释项目结构；
- 批量改名、整理资源；
- 生成文档或素材清单；
- 改动前先给出 diff，再让你确认。

项目目录按普通 git 仓库管理，每次改动都能回滚、能 review。

## 下一步

Scratch 专用工具和 `.sb3` 读写还在规划中，见[路线图](/guide/roadmap)。
