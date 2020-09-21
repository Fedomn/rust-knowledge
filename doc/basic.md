Rust Design Philosophy: Safe, Fast, Concurrent

## 版本概念

从不同的维度来定义使用的rust版本:

* 语义化版本: 同Sem Ver(major.minor.patch), 分别是 不兼容API修改.向下兼容功能新增.向下兼容问题修复
* 发行版本: branch->发行版, 分别是 master->Nightly / beta->Beta / stable->Stable
* Edition: rust每3年一次 定义一次edition(截止的SemVar) 来代表当前rust支持的语言特性, 类比书籍的版本(2013版/2020版)
