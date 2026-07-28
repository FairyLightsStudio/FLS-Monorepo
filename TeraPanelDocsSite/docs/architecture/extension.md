2. 插件安装逻辑 (Architectural Split)
根据架构设计，插件被分为两类安装：
UI Extensions (本地安装)： 如主题 (Themes)、拼写检查、快捷键映射。这些直接在客户端运行。
Workspace Extensions (远程安装)： 如 Python 解释器、Java 语言服务、Debugger。这些必须安装在 VS Code Server 端，因为它们需要直接读取代码文件和调用系统 API。 

