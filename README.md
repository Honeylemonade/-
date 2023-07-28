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
| **支持基础配置检测** | 点击[提交/测试]按钮，可以校验后端服务与数据库是否能正常访问，判断是 DB 还是后端服务连接失败<br /><img src="https://pic-1306533678.cos.ap-nanjing.myqcloud.com/uPic/image-20230728210925634.png" alt="image-20230728210925634" style="zoom:50%;" /> |
| **支持实例多选**     | 若想全选，使用快捷键 command+a<br /><img src="https://pic-1306533678.cos.ap-nanjing.myqcloud.com/uPic/image-20230728210938402.png" alt="image-20230728210938402" style="zoom:50%;" /> |
| **支持多种数据类型** | 可以根据数据类型进行不同的配置：<br /><img src="https://pic-1306533678.cos.ap-nanjing.myqcloud.com/uPic/image-20230728210952524.png" alt="image-20230728210952524" style="zoom:50%;" /> |
| **支持数据配置**     | **支持固定值和随机值：**string、boolean类型随机值不需要配置上下界int 类型不需要配置小数点位数double 类型支持配置、上界、下界、小数点位数其中 string  类型默认随机值为["a","b","c","d","e"]**string类型随机值也支持random choose：**固定值中填写：["open","close"]选中随机值程序会随机写入["open","close"]中的任一字符串<br /><img src="https://pic-1306533678.cos.ap-nanjing.myqcloud.com/uPic/image-20230728211005207.png" alt="image-20230728211005207" style="zoom:50%;" /> |
| **支持两种运行方式** | 针对于不同场景，可以选择不同运行方式；例如为了模拟设备实时上数，可以选择【实时上数】模式为了导入历史数据或补数，可以选择【历史上数】模式<br /><img src="https://pic-1306533678.cos.ap-nanjing.myqcloud.com/uPic/image-20230728211023740.png" alt="image-20230728211023740" style="zoom:50%;" /> |
| **支持日志监控**     | 支持展示 log，监控程序状态以及数据写入情况<br /><img src="https://pic-1306533678.cos.ap-nanjing.myqcloud.com/uPic/image-20230728211035660.png" alt="image-20230728211035660" style="zoom:50%;" /> |
| **支持配置导入导出** | 支持配置的导入导出。无需每次打开程序时重新配置<br /><img src="https://pic-1306533678.cos.ap-nanjing.myqcloud.com/uPic/image-20230728211045772.png" alt="image-20230728211045772" style="zoom:50%;" /> |

# 数据模拟 V 2.0

新版的工具使用 Vue Tauri Rust进行重构 ，提供更友好的交互、更小的存储容量，以及更快的性能。

并增加了多种新功能和特性（见下更新内容）

<img src="https://pic-1306533678.cos.ap-nanjing.myqcloud.com/uPic/image-20230728210505889.png" alt="image-20230728210505889" style="zoom: 25%;" />

<img src="https://pic-1306533678.cos.ap-nanjing.myqcloud.com/uPic/image-20230728210514343.png" alt="image-20230728210514343" style="zoom: 25%;" />

| **特性**                               | **介绍**                                                     |
| -------------------------------------- | ------------------------------------------------------------ |
| Vue +tauri+rust 进行重构               | 跨平台开发：Tauri是一个跨平台的开发框架，能够一次性编写、构建、部署应用程序到各种操作系统，包括Windows、MacOS和各种Linux版本等。高性能：Rust是一种高性能的系统级编程语言。它具有内存安全、线程安全和高性能的特点。在开发过程中，使用Rust可以使代码更加高效和稳定。工具丰富：Tauri框架附带了一些强大的工具，如Tauri CLI、Tauri APIs、Tauri Bundler等，能够帮助开发者轻松构建高质量的跨平台应用程序。美观易用：Vue是一个流行的JavaScript框架，它采用了MVVM架构，使得界面的编写非常方便。同时，它使用组件化的方式，使得开发者可以创建复杂的用户界面，并且易于维护。 |
| 模型配置界面将增加滚轮                 | 防止模型属性过多时，无法显示提交按钮<br /><img src="https://pic-1306533678.cos.ap-nanjing.myqcloud.com/uPic/image-20230728211057384.png" alt="image-20230728211057384" style="zoom:50%;" /> |
| 模型选择支持模糊搜索                   | <img src="https://pic-1306533678.cos.ap-nanjing.myqcloud.com/uPic/image-20230728211106531.png" alt="image-20230728211106531" style="zoom:50%;" /> |
| 支持编辑模型配置                       | <img src="https://pic-1306533678.cos.ap-nanjing.myqcloud.com/uPic/image-20230728211118929.png" alt="image-20230728211118929" style="zoom:50%;" /> |
| 历史上数模式，增加可交互式时间选择窗口 | <img src="https://pic-1306533678.cos.ap-nanjing.myqcloud.com/uPic/image-20230728211128094.png" alt="image-20230728211128094" style="zoom:50%;" /> |
| 历史上数模式，增加进度条               | <img src="https://pic-1306533678.cos.ap-nanjing.myqcloud.com/uPic/image-20230728211136099.png" alt="image-20230728211136099" style="zoom:50%;" /> |
| 拆分模型发送 slink                     | 尽最大可能针对每一个模型上数，防止由于部分模型被删，导致所有无法模拟所有数据 |
| 支持异常模型上数状态监控               | <img src="https://pic-1306533678.cos.ap-nanjing.myqcloud.com/uPic/image-20230728211146868.png" alt="image-20230728211146868" style="zoom:50%;" /> |
| 支持 Windows 系统                      |                                                              |

﻿