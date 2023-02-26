# BakaSayaka
一个简单的，难以被关闭的循环播放《Decretum（宿命）》（Sayaka处刑曲）的Rust程序。

> Sayaka第二次发现自己是笨蛋十二周年献礼（根据第八集播出时间）

![Sayaka](./Baka_Sayaka.jpg)



## Features
embed-asset: 将音乐捆绑入二进制文件内，否则将会在运行时读取music/Decretum.mp3进行播放

unkill-signal： 启用后会忽略所有可以忽略的信号，防止普通关闭

unkill-advance： Windows下运行时会持续尝试关闭taskmgr.exe，其他OS下无作用（请注意关闭taskmgr需要管理员权限，因此请将程序提升运行，本程序不会主动检查是否已经被提升，请手动尝试）


> 本项目遵循GPL-3.0开源协议
