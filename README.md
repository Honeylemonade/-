# aPaaS数字孪生工具

其功能为进行数据模拟，数字孪生系统接入数据，为完成场景演示，满足功能回归等需求。

## 开启调试

```sh
npm run tauri dev
```

## 构建项目

```sh
npm run tauri build
```

## 功能介绍

| **特性**             | **介绍**                                                     |
| -------------------- | ------------------------------------------------------------ |
| **支持基础配置检测** | 点击**[提交/测试]**按钮，可以校验后端服务与数据库是否能正常访问，判断是 DB 还是后端服务连接失败<img src="https://rte.weiyun.baidu.com/wiki/attach/image/api/imageDownloadAddress?attachId=9e404f8affd648dabf53ec8a2aac8f59&docGuid=S1fyDXx0SkX-TP" alt="img" style="zoom:33%;" /> |
| **支持实例多选**     | <img src="https://rte.weiyun.baidu.com/wiki/attach/image/api/imageDownloadAddress?attachId=579b67ed915c4f6790d0e82ab8b7cf20&docGuid=S1fyDXx0SkX-TP" alt="img" style="zoom:33%;" />若想全选，使用快捷键 command+a |
| **支持多种数据类型** | <img src="https://rte.weiyun.baidu.com/wiki/attach/image/api/imageDownloadAddress?attachId=e31d7c507aa24a22b6325f0646dad225&docGuid=S1fyDXx0SkX-TP" alt="img" style="zoom:33%;" />可以根据数据类型进行不同的配置： |
| **支持数据配置**     | **支持固定值和随机值：**string、boolean类型随机值不需要配置上下界int 类型不需要配置小数点位数double 类型支持配置、上界、下界、小数点位数其中 string  类型默认随机值为["a","b","c","d","e"]**string类型随机值也支持random choose：**<img src="https://rte.weiyun.baidu.com/wiki/attach/image/api/imageDownloadAddress?attachId=a2e24c9db2a14a839b62907b5c3b69eb&docGuid=S1fyDXx0SkX-TP" alt="img" style="zoom:33%;" />固定值中填写：["open","close"]选中随机值程序会随机写入["open","close"]中的任一字符串 |
| **支持两种运行方式** | 针对于不同场景，可以选择不同运行方式；例如为了模拟设备实时上数，可以选择【实时上数】模式为了导入历史数据或补数，可以选择【历史上数】模式<img src="https://rte.weiyun.baidu.com/wiki/attach/image/api/imageDownloadAddress?attachId=c18fb2640b354c1e82ef887f3eb0866b&docGuid=S1fyDXx0SkX-TP" alt="img" style="zoom:33%;" /> |
| **支持日志监控**     | <img src="https://rte.weiyun.baidu.com/wiki/attach/image/api/imageDownloadAddress?attachId=bc38b991691743be8d5de10e3f56fc11&docGuid=S1fyDXx0SkX-TP" alt="img" style="zoom:33%;" />支持展示 log，监控程序状态以及数据写入情况 |
| **支持配置导入导出** | 支持配置的导入导出。无需每次打开程序时重新配置<img src="https://rte.weiyun.baidu.com/wiki/attach/image/api/imageDownloadAddress?attachId=a1bc540844c24673863324fe1e811ff5&docGuid=S1fyDXx0SkX-TP" alt="img" style="zoom:33%;" /> |

# 数据模拟 V 2.0

新版的工具使用 Vue Tauri Rust进行重构 ，提供更友好的交互、更小的存储容量，以及更快的性能。

并增加了多种新功能和特性（见下更新内容）

<img src="https://pic-1306533678.cos.ap-nanjing.myqcloud.com/uPic/image-20230728210505889.png" alt="image-20230728210505889" style="zoom:50%;" />

<img src="https://pic-1306533678.cos.ap-nanjing.myqcloud.com/uPic/image-20230728210514343.png" alt="image-20230728210514343" style="zoom:50%;" />

| **特性**                               | **介绍**                                                     |
| -------------------------------------- | ------------------------------------------------------------ |
| Vue +tauri+rust 进行重构               | 跨平台开发：Tauri是一个跨平台的开发框架，能够一次性编写、构建、部署应用程序到各种操作系统，包括Windows、MacOS和各种Linux版本等。高性能：Rust是一种高性能的系统级编程语言。它具有内存安全、线程安全和高性能的特点。在开发过程中，使用Rust可以使代码更加高效和稳定。工具丰富：Tauri框架附带了一些强大的工具，如Tauri CLI、Tauri APIs、Tauri Bundler等，能够帮助开发者轻松构建高质量的跨平台应用程序。美观易用：Vue是一个流行的JavaScript框架，它采用了MVVM架构，使得界面的编写非常方便。同时，它使用组件化的方式，使得开发者可以创建复杂的用户界面，并且易于维护。 |
| 模型配置界面将增加滚轮                 | 防止模型属性过多时，无法显示提交按钮<img src="https://rte.weiyun.baidu.com/wiki/attach/image/api/imageDownloadAddress?attachId=50d27f1bec344bb08e1933dd379adea4&docGuid=S1fyDXx0SkX-TP" alt="img" style="zoom:25%;" /> |
| 模型选择支持模糊搜索                   | <img src="https://rte.weiyun.baidu.com/wiki/attach/image/api/imageDownloadAddress?attachId=7566d71af02b45e68a2f853bcde67d71&docGuid=S1fyDXx0SkX-TP" alt="img" style="zoom:25%;" /> |
| 支持编辑模型配置                       | <img src="https://rte.weiyun.baidu.com/wiki/attach/image/api/imageDownloadAddress?attachId=ce3c15a84e494211bc0cfaad16cfb2ce&docGuid=S1fyDXx0SkX-TP" alt="img" style="zoom:25%;" /> |
| 历史上数模式，增加可交互式时间选择窗口 | <img src="https://rte.weiyun.baidu.com/wiki/attach/image/api/imageDownloadAddress?attachId=d33364c6d45f400da537847ed02a1f1a&docGuid=S1fyDXx0SkX-TP" alt="img" style="zoom:25%;" /> |
| 历史上数模式，增加进度条               | <img src="https://rte.weiyun.baidu.com/wiki/attach/image/api/imageDownloadAddress?attachId=a43705c6388840e0ab7a306b3c661ed5&docGuid=S1fyDXx0SkX-TP" alt="img" style="zoom:25%;" /> |
| 拆分模型发送 slink                     | 尽最大可能针对每一个模型上数，防止由于部分模型被删，导致所有无法模拟所有数据 |
| 支持异常模型上数状态监控               | <img src="https://rte.weiyun.baidu.com/wiki/attach/image/api/imageDownloadAddress?attachId=a79262fda1b64ab9aa1950278d83c11d&docGuid=S1fyDXx0SkX-TP" alt="img" style="zoom:25%;" /> |
| 支持 Windows 系统                      |                                                              |

﻿