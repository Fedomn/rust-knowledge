Rust Design Philosophy: Safe, Fast, Concurrent

## 版本概念

从不同的维度来定义使用的rust版本:

* 语义化版本: 同Sem Ver(major.minor.patch), 分别是 不兼容API修改.向下兼容功能新增.向下兼容问题修复
* 发行版本: branch->发行版, 分别是 master->Nightly / beta->Beta / stable->Stable
* Edition: rust每3年一次 定义一次edition(截止的SemVar) 来代表当前rust支持的语言特性, 类比书籍的版本(2013版/2020版)

## Code Organization

* Packages: A Cargo feature that lets you build, test, and share crates (cargo new my-project)
* Crates: A tree of modules that produces a library or executable
* Modules and use: Let you control the organization, scope, and privacy of paths
* Paths: A way of naming an item, such as a struct, function, or module

Here, we have a package only contains src/main.rs, meaning it only contains a binary crate named my-project. 

If a package contains src/main.rs and src/lib.rs, it has two crates: a library and a binary, both with the same name as the package. 

A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.

use keyword that brings a path into scope.

`mod custom_name;` tell Rust to load the contents of the module from another file with the same name as the module. 