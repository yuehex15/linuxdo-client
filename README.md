# Linux.Do 轻量 Windows 客户端

基于 **Tauri v2** 的极轻量 Linux.Do 论坛客户端。

## 📥 下载

从 [Releases](https://github.com/yuehex15/linuxdo-client/releases) 下载 `LinuxDo-Windows-x64.zip`。

## 🔧 修改域名

域名默认值在 `src-tauri/src/lib.rs` 的 `get_default_url()` 中，修改后推送自动重新构建。

## ⚙️ 运行时配置

`settings.ini` 可自定义代理模式（system/direct/custom）、禁用 GPU、JS 堆内存限制。修改后重启生效。

## 🖥 最低配置：Windows 10/11 x64, 1GB+ RAM