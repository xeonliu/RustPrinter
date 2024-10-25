# API
## Common API
+ Login
## Web API
+ History
+ Device Availablility
+ PDF / Word Upload
## UPMClient API
+ `POST /api/client/PrintJob/Create`
    ```json
    {
    "dwProperty": 0,
    "szJobName": "屏幕截图 2024-10-02 233633",
    "dwCopies": 1,
    "szAttribe": "single,collate,NUP1,", // "color,vdup,collate,NUP1," // vdup means Vertical Duplex. // "color,hdup,collate,NUP1,"
    "szPaperDetail": "[{\"dwPaperID\":9,\"dwBWPages\":1,\"dwColorPages\":0,\"dwPaperNum\":1}]", // dwPaperID 9 means A4 dwBWPages 5 dwColorPages 0, dwPaperNum 3. // Notice dwPaper Num is calculated by total paper number.
    "szColorMap": "0" // "11101" Color
    }
    ```
    ```json
    {
        "code": 0,
        "message": "",
        "result": {
            "szLogonName": "",
            "dwStatus": 17,
            "dwJobId": 167523,
            "dwOldJobId": 579850,
            "dwProperty": 33554432,
            "dwCreateDate": 20241005,
            "dwCreateTime": 213611,
            "dwPrintDate": 0,
            "dwPrintTime": 0,
            "dwCopies": 1,
            "dwDevSN": 0,
            "dwFee": 0,
            "dwType": 1,
            "dwSpecPrinter": 0,
            "dwFSID": 0,
            "szAttribe": "single,collate,NUP1,",
            "szPaperDetail": "[{\"dwPaperID\":9,\"dwBWPages\":1,\"dwColorPages\":0,\"dwPaperNum\":1}]",
            "szColorMap": "0",
            "szCardNO": "",
            "szTrueName": "",
            "szJobName": " 2024-10-02 233633",
            "szFileName": "20241005/f4a5169164ba403a87fcbb572f7aa6ee",
            "szMemo": null
        }
    }
    ```
+ `POST /api/client/PrintJob/Upload?dwJobId={} HTTP/1.1 `
    ```
    multipart/form-data; boundary=---------------------------Boundaryd1vksiw0aMcjdDd46cs3c
    ```
+ `POST /api/client/PrintJob/UploadPreview?dwJobId={} HTTP/1.1`
  + Upload PVG File 
+ `POST /api/client/PrintJob/Set HTTP/1.1`
    ```
    Object
        Member: dwJobId
            [Path with value: /dwJobId:167523]
            [Member with value: dwJobId:167523]
            Number value: 167523
            Key: dwJobId
            [Path: /dwJobId]
        Member: dwStatus
            [Path with value: /dwStatus:1]
            [Member with value: dwStatus:1]
            Number value: 1
            Key: dwStatus
            [Path: /dwStatus]
        Member: OSESSIONID
            [Path with value: /OSESSIONID:2a50d60c07f44c0aace049be7aff5fcf]
            [Member with value: OSESSIONID:2a50d60c07f44c0aace049be7aff5fcf]
            String value: 2a50d60c07f44c0aace049be7aff5fcf
            Key: OSESSIONID
            [Path: /OSESSIONID]
        ```
## Charge API
### 获取用户余额
+ `GET /uniwx/UserSet.aspx?m=1&id={jAccountId} HTTP/1.1`
    + 无需鉴权（危险）

### 充值

> 注意：该部分流程尚不清晰，胡乱调用可能导致充值给虚空。

+ `GET /uniwx/s.aspx?c=recharge_1_{jAccountId} HTTP/1.1` 
    + 进行一系列OAuth 2鉴权。
+ 跳转到`GET /uniwx/pay.aspx?state=recharge_1_{jAccountId} HTTP/1.1`
    + 该界面下有Token
    + 有一个填写充值金额的框
+ `POST /uniwx/pay.aspx?state=recharge_1_{jAccountId} HTTP/1.1`
    + 提交表单发送POST请求可以充值
    + 包括Token和充值金额
+ 返回一个带微信支付脚本的网页

# “联创打印管理系统”虚拟打印机执行流程

## 安装目录结构
```
C:\upmclient
├─log
├─mondll # 储存端口监视器文件 `upmlocalmon.dll`
│  ├─Win10X32
│  ├─Win10X64
│  ├─Win7X32
│  ├─Win7X64
│  ├─Win8X32
│  ├─Win8X64
│  ├─Win8_1X32
│  ├─Win8_1X64
│  └─WinXPX32
├─temp # 临时文件。包括PJL文件(.tmp后缀)、PVG文件（预览信息？）、RAW图片文件、压缩的PJL文件(.tmp2)后缀
├─X32
├─X64
│  cconnector.dll # 用于JsonRPC，UPMClient调用
│  Compress.dll # 用于`gzip`压缩。用于压缩PJL文件生成`.tmp2`后缀文件，UPMClient调用
│  GdiPlus.dll # Gdi+ 是 Windows 的图形API。
│  opmgoupm.exe # 似乎是一个浏览器。打开会进入eprint主页。
│  Setup.exe # 安装UPMClient驱动（使用`UPMPortMonitor.reg`更新注册表）
│  UnInstall.cmd # 带参数执行`Setup.exe`
│  update.exe
│  UPMClient.exe # 主程序。不加参数启动可以进入eprint主页，被端口监视器使用命令行参数调用会进入打印界面。
│  UPMClient.ini # 配置文件，作为注册表的备选。
│  UPMPortMonitor.reg # 配置端口监视器的注册表脚本
└─ zlib1.dll # 用于`zlib`压缩
```

使用Ghidra进行逆向。

## 驱动部分
+ 驱动是Unidrv，由GPF文件定义GDI指令到PCL指令的转换。内嵌在Setup.exe中，安装时添加到系统驱动目录。

## 端口监视器部分
+ 端口监视器会储存驱动发送到该端口的PJL文件于`temp`目录，命名为`[JobId].tmp`
+ 驱动会使用命令行启动`UMPClient.exe`，参数为`/JOB:[JobId] /PRINTER:联创打印管理系统`

## `UPMClient.exe`部分
+ 使用传入的Job参数和打印机名称使用GetPrinter等Win32API检查打印任务队列
+ 生成预览文件`.pvg`（实现未知, Should be Using GDI+ to render EMF spooler file）
    + `.pvg` is a self-defined format containing base64 encoded png.
+ 向后端发送添加任务的请求，获得新任务的id。
+ 调用`Compress.dll`中的`CompressFile(input, output, "wb")`获得`[JobId].tmp2`（实为`gzip`压缩）
+ 若点击预览，生成`.raw` PNG 图片文件
+ 向后端上传压缩后的文件
+ 向后端上传预览文件？（实现未知）
+ 向后端发送结束请求
+ 清除临时文件，删除打印任务
+ 清除临时文件，删除打印任务
