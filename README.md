<div align="center">
	<h1>CK-567</h1>
<h2>CK-567 强大的Anti-Virus对抗工具</h2>
</div>
<div align="center">
<img src="./doc/ck.jpg"/>
</div>

### 由遮天 项目组指导

shellcode **格式**

- **raw**
### 环境

**安装Rust**

rust环境[https://www.rust-lang.org/learn/get-started]

**C++环境**


build exe需要<br/>
**方案一:VisualStudio**

先安装VisualStudio<br/>
https://visualstudio.microsoft.com/zh-hans/visual-cpp-build-tools/ <br/>
执行下面命令（如果有VS则直接执行）<br/>
```
rustup default nightly
```
缺点 占用磁盘空间大,可以看下面方案<br/>

**方案二:MinGW**<br/>

先安装mingw<br/>
https://osdn.net/projects/mingw/downloads/68260/mingw-get-setup.exe/ <br/>
执行下面命令（如果有MinGW则直接执行）<br/>
```
x84 64位
rustup toolchain install nightly-x86_64-pc-windows-gnu
rustup default nightly-x86_64-pc-windows-gnu
```


### 使用

```

                 ▄████▄   ██ ▄█▀
                ▒██▀ ▀█   ██▄█▒
                ▒▓█    ▄ ▓███▄░
                ▒▓▓▄ ▄██▒▓██ █▄
                ▒ ▓███▀ ░▒██▒ █▄
                ░ ░▒ ▒  ░▒ ▒▒ ▓▒
                  ░  ▒   ░ ░▒ ▒░
                ░        ░ ░░ ░
                ░ ░      ░  ░
                ░

version:0.1
```


**加载器：**

```
                 ▄████▄   ██ ▄█▀
                ▒██▀ ▀█   ██▄█▒
                ▒▓█    ▄ ▓███▄░
                ▒▓▓▄ ▄██▒▓██ █▄
                ▒ ▓███▀ ░▒██▒ █▄
                ░ ░▒ ▒  ░▒ ▒▒ ▓▒
                  ░  ▒   ░ ░▒ ▒░
                ░        ░ ░░ ░
                ░ ░      ░  ░
                ░

version:0.1
error: the following required arguments were not provided:
  -f <file>
  -n <name>

Usage: CK-567.exe shellcode -f <file> -n <name>

For more information, try '--help'.
```

```
CK-567.exe shellcode  -f=C:\Users\10431\Desktop\payload.bin  -n=a1
```

**捆绑木马：**

```

                 ▄████▄   ██ ▄█▀
                ▒██▀ ▀█   ██▄█▒
                ▒▓█    ▄ ▓███▄░
                ▒▓▓▄ ▄██▒▓██ █▄
                ▒ ▓███▀ ░▒██▒ █▄
                ░ ░▒ ▒  ░▒ ▒▒ ▓▒
                  ░  ▒   ░ ░▒ ▒░
                ░        ░ ░░ ░
                ░ ░      ░  ░
                ░

version:0.1
error: the following required arguments were not provided:
  -f <file>
  -i <ico>
  -t <trojan>

Usage: CK-567.exe bind -f <file> -i <ico> -t <trojan>

For more information, try '--help'.

```
