# 富邦新一代 API｜程式交易的新武器

## TradeAPI

### SDK 下載

***

本頁重點

* 提供富邦新一代 API（Fubon Neo API）各語言 SDK 下載。
* 最新版本為 **v2.2.8**，並提供[版本遷移資訊](#%E7%89%88%E6%9C%AC%E9%81%B7%E7%A7%BB%E8%B3%87%E8%A8%8A)。
* API Key 登入支援 >= v2.2.7，網頁憑證匯出登入支援 >= v2.2.8。
* 安裝步驟與相容性總覽請見[安裝與版本相容性](https://www.fbs.com.tw/TradeAPI/docs/install-compatibility.txt)。

| 項目         | 說明                                                                       |
| ------------ | -------------------------------------------------------------------------- |
| 最新版本     | v2.2.8                                                                     |
| 支援語言     | Python / C# / JavaScript (Node.js) / C++、Go（僅支援證券交易帳務及條件單） |
| 支援平台     | 依語言提供 Windows / macOS / Linux 版本                                    |
| API Key 登入 | >= v2.2.7                                                                  |
| 憑證匯出登入 | >= v2.2.8                                                                  |

相關資訊

1. 新一代 API 提供測試環境，詳情請見 [快速上手](https://www.fbs.com.tw/TradeAPI/docs/welcome.txt#%E6%B8%AC%E8%A9%A6%E7%92%B0%E5%A2%83) 頁面 <br />
2. [網頁憑證匯出](https://www.fbs.com.tw/TradeAPI/docs/key.txt)可用於API 登入 (版本要求 >= v2.2.8)

API Key 登入 >= v2.2.7

* [API Key 說明](https://www.fbs.com.tw/TradeAPI/docs/trading/api-key-apply.txt) <br />
* [API Key 申請](https://www.fbs.com.tw/TradeAPI/docs/key.txt)

版本注意事項 **>=2.2.4**

1. 新增證券委託單 user\_def 欄位輸入規則檢核提醒 <br />
   a. 僅可使用 ***大小寫英文字母及0~9數字***，最多 10 個字元 (***規則更新 >=2.2.8***) <br />
   b. 若字元合規但長度超過 10 字元，將自動縮減至 10 字元並帶入下單；交易 event callback 發送提醒訊息 <br />
   c. 若字元不合規，則自動將 user\_def 帶入空值；交易 event callback 發送提醒訊息 <br />

（***註：*** user\_def 欄位輸入不符合規則**仍會送單**，但欄位值將被自動調整，調整方式請參考上述點 b 及 c ）

2. Python 行情 Web API 例外情況改用 ***Exception*** 方式回報。詳情請見行情 Web API 中之 Python 程式範例。

版本注意事項 **>=2.2.0**

1. 以下條件單功能目前不支援**期貨夜盤**： a. 分時分量 b. 移動鎖利 c. 時間觸發條件

2. 證券條件單停損停利子單設定物件（TPSLOrder）新增參數 `trigger`，可設定觸發價格參照為 成交價、最佳買價、最價賣價， 若無特別設定則預設值為 成交價，詳請參閱條件單 \[參數對照表]

（***註：*** C# 使用預設值須將此參數填入 ***null***，Python 及 JS 可忽略此參數）<br />（***註 2：*** **期貨條件單** 物件（FutOptTpslOrder）亦新增此欄位，尚無具體功能，C# 請填入 ***null***，Python 及 JS 可忽略此欄位 ）

#### 檔案載點[​](#檔案載點 "Direct link to 檔案載點")

##### Python[​](#python "Direct link to Python")

支援 3.8、3.9、3.10、3.11、3.12 與 3.13 版本，v2.0.1 後不支援 Python 3.7（目前不支援 3.14）

**SDK:**

* Windows 64 位元 [下載](https://www.fbs.com.tw/TradeAPI_SDK/fubon_binary/fubon_neo-2.2.8-cp37-abi3-win_amd64.zip)

* MacOs

  <!-- -->

  * Arm 64 位元 [下載](https://www.fbs.com.tw/TradeAPI_SDK/fubon_binary/fubon_neo-2.2.8-cp37-abi3-macosx_11_0_arm64.zip)
  * X86 64 位元 [下載](https://www.fbs.com.tw/TradeAPI_SDK/fubon_binary/fubon_neo-2.2.8-cp37-abi3-macosx_10_12_x86_64.zip)

* Linux 64 位元 [下載](https://www.fbs.com.tw/TradeAPI_SDK/fubon_binary/fubon_neo-2.2.8-cp37-abi3-manylinux_2_17_x86_64.manylinux2014_x86_64.zip)

info

**範例程式碼：** [Python 範例程式碼](https://www.fbs.com.tw/TradeAPI_SDK/sample_code/python_sample_code.zip) (.ipynb, Jupyter 互動開發格式, 需搭配 SDK 使用)

##### JavaScript[​](#javascript "Direct link to JavaScript")

支援 Node.js 16 以上版本

**SDK:**

* Fubon-Neo.tgz [下載](https://www.fbs.com.tw/TradeAPI_SDK/fubon_binary/fubon-neo-2.2.8.nodejs.zip)

info

**範例程式碼：** [JS 範例程式碼](https://www.fbs.com.tw/TradeAPI_SDK/sample_code/js_sample_code.zip) (.ipynb, Jupyter 互動開發格式, 需搭配 SDK 使用)

##### C#[​](#c "Direct link to C#")

使用 .NET Standard 2.0 開發，建議使用 .netcoreapp 3.1 以上，或 .NETFramework 4.7.2 以上版本

**SDK:**

* nupkg(64 位元) [下載](https://www.fbs.com.tw/TradeAPI_SDK/fubon_binary/FubonNeo.2.2.8.nupkg.zip)

info

**範例程式碼：** [C# 下單範例程式碼](https://www.fbs.com.tw/TradeAPI_SDK/sample_code/fubondotnetsdkgui.zip) (Visual Studio 專案範例 (WPF), 需搭配 SDK 使用)<br />**範例程式碼：** [C# 行情及帳務範例程式碼](https://www.fbs.com.tw/TradeAPI_SDK/sample_code/marketdata_n_accounting.zip) (Visual Studio 專案範例 (Windows Forms), 需搭配 SDK 使用)

##### C++[​](#c-1 "Direct link to C++")

支援 C++ 20 及以上版本

* [SDK及範例程式碼下載](https://www.fbs.com.tw/TradeAPI_SDK/fubon_binary/FubonNeo.2.2.8_CppSDKPackage.zip)

##### Golang[​](#golang "Direct link to Golang")

* [SDK及範例程式碼下載](https://www.fbs.com.tw/TradeAPI_SDK/fubon_binary/fubon-neo-2.2.8.golang.zip)

#### 連線測試[​](#連線測試 "Direct link to 連線測試")

**連線測試小幫手：** [程式下載](https://www.fbs.com.tw/TradeAPI_SDK/sample_code/API_Sign_Test.zip) (Windows平台使用)

#### 版本遷移資訊[​](#版本遷移資訊 "Direct link to 版本遷移資訊")

##### 2.2.8[​](#228 "Direct link to 2.2.8")

* 證券

  <!-- -->

  1. 行情新增**股務事件**資料API
  2. [歷史行情 Candles](https://www.fbs.com.tw/TradeAPI/docs/market-data/http-api/historical/candles.txt)新增**還原股價**選項

* 新增 [LLM 輔助開發](https://www.fbs.com.tw/TradeAPI/docs/welcome/build-with-llm.txt) 頁面

版本注意事項 **>=2.2.8**

1. 新增證券委託單 user\_def 欄位輸入規則檢核提醒<br />a. 僅可使用 ***大小寫英文字母及0~9數字***，最多 10 個字元 (***規則更新 >=2.2.8***)<br />

2. API Key 權限變更強制斷線機制，調整為僅限於受變更影響之 key<br />(例：使用key1、key2同時登入上線，刪除 key1 => key1 session 強制斷線； key2 session 不受影響)<br />

3. [網頁憑證匯出](https://www.fbs.com.tw/TradeAPI/docs/key.txt)可用於 API 登入

##### 2.2.7[​](#227 "Direct link to 2.2.7")

* 新增 使用 APY-KEY 登入及憑證匯出功能

info

* [API Key 說明](https://www.fbs.com.tw/TradeAPI/docs/trading/api-key-apply.txt) <br />
* [API Key 申請](https://www.fbs.com.tw/TradeAPI/docs/key.txt)

##### 2.2.6[​](#226 "Direct link to 2.2.6")

* SDK 新增 Golang 版本 (證券交易及帳務)
* 證券 行情 Web API 新增技術指標

##### 2.2.5[​](#225 "Direct link to 2.2.5")

* 證券 商品漲跌幅報表查詢功能（上市、櫃）
* 證券 現冲券配額查詢新增警示股註記欄位
* 連線管理元件升級

##### 2.2.4[​](#224 "Direct link to 2.2.4")

* 證券 當沖條件單上線
* 證券 支援先進先出帳務查詢
* 證券 委託單 user\_def 欄位新增輸入規範檢核提醒
* SDK 新增 C++ 版本 (證券交易、帳務及條件單功能)

版本注意事項 **>=2.2.4**

1. 新增證券委託單 user\_def 欄位輸入規則檢核提醒 <br />
   a. 僅可使用 ***ASCII 33-126*** 範圍中字元，最多 10 個字元 <br />
   b. 若字元合規但長度超過 10 字元，將自動縮減至 10 字元並帶入下單；交易 event callback 發送提醒訊息 <br />
   c. 若字元不合規，則自動將 user\_def 帶入空值；交易 event callback 發送提醒訊息 <br />

（***註：*** user\_def 欄位輸入不符合規則**仍會送單**，但欄位值將被自動調整，調整方式請參考上述點 b 及 c ）

2. Python 行情 Web API 例外情況改用 ***Exception*** 方式回報。詳情請見行情 Web API 中之 Python 範例程式。

##### 2.2.3[​](#223 "Direct link to 2.2.3")

* 例外處理 ( exception handling ) 元件優化

##### 2.2.2[​](#222 "Direct link to 2.2.2")

* 下單模組速&#x5EA6;***再*** 優化升級

##### 2.2.1[​](#221 "Direct link to 2.2.1")

* 新增連線參數設定 ( [說明](https://www.fbs.com.tw/TradeAPI/docs/trading/guide/advance/ping_pong.txt) )

##### 2.2.0[​](#220 "Direct link to 2.2.0")

* 證券 **條件單**停損停利觸發價格條件設定
* 期貨/選擇權 新增**條件單**夜盤功能選項（FutureNight/OptionNight）

版本注意事項 >=2.2.0

1. 以下條件單功能目前不支援**期貨夜盤**： a. 分時分量 b. 移動鎖利 c. 時間觸發條件

2. 證券條件單停損停利子單設定物件（TPSLOrder）新增參數 `trigger`，可設定觸發價格參照為 成交價、最佳買價、最價賣價， 若無特別設定則預設值為 成交價，詳請參閱條件單 \[參數對照表]

（***註：*** C# 使用預設值須將此參數填入 ***null***，Python 及 JS 可忽略此參數）<br />（***註 2：*** **期貨條件單** 物件（FutOptTpslOrder）亦新增此欄位，尚無具體功能，C# 請填入 ***null***，Python 及 JS 可忽略此欄位 ）

##### 2.1.1[​](#211 "Direct link to 2.1.1")

* 證券 長日期歷史委託單及成交紀錄查詢（單次查詢最大30日區間）
* 期貨/選擇權 新增移動鎖利及分時分量條件單功能

##### 2.1.0[​](#210 "Direct link to 2.1.0")

* 期貨/選擇權 條件單功能上線
* (證券) 委託單歷程查詢新增 狀態 (status) 及 錯誤訊息 (err\_msg) 欄位

##### 2.0.1[​](#201 "Direct link to 2.0.1")

* 期貨/選擇權 交易及帳務功能
* Python SDK 支援 Python 3.12 (**停止支援 Python 3.7**)

##### 1.3.2[​](#132 "Direct link to 1.3.2")

* 下單模組速度優化升級

##### 1.3.1[​](#131 "Direct link to 1.3.1")

* 證券條件單
* 期貨/選擇權 行情
* (C#) 交易 callback function 使用方式變更 (詳請見 交易文件 -> SDK Reference -> [版本升級指南](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/upgrade-guide.txt))

##### 1.0.4[​](#104 "Direct link to 1.0.4")

* OrderResult 物件新增 details 欄位
* 即時行情新增 Normal Mode（詳請見 行情文件 -> WebSocket -> [模式切換](https://www.fbs.com.tw/TradeAPI/docs/market-data/websocket-api/upgrade-guide.txt)）


---

### 安裝與版本相容性

本頁重點

* 支援語言：Python / C# / JavaScript (Node.js) / C++、Go（僅支援證券交易帳務及條件單）。
* 最低版本：Python 3.8–3.13（不含 3.14）、Node.js 16+、.NET Standard 2.0、C++20+、Go 1.19+。
* SDK 下載位置：請見 [SDK 下載](https://www.fbs.com.tw/TradeAPI/docs/download/download-sdk.txt)。
* 本頁提供最小安裝步驟與版本相容性總覽。

#### 支援環境與版本相容性[​](#支援環境與版本相容性 "Direct link to 支援環境與版本相容性")

| 語言                 | 最低版本              | 支援 OS                 | 安裝方式        | 備註                                         |
| -------------------- | --------------------- | ----------------------- | --------------- | -------------------------------------------- |
| Python               | 3.8–3.13（v2.0.1 起） | Windows / macOS / Linux | `.whl`          | v2.0.1 起停止支援 Python 3.7；不支援 3.14    |
| JavaScript (Node.js) | 16+                   | Windows / macOS / Linux | `.tgz` 本地依賴 | 以 Node.js 套件方式安裝                      |
| C#                   | .NET Standard 2.0     | Windows                 | `.nupkg`        | 建議 .NET Core 3.1+ 或 .NET Framework 4.7.2+ |
| C++                  | C++20+                | Windows / macOS / Linux | SDK 檔案        | 僅支援證券交易帳務及條件單                   |
| Go                   | 1.19+                 | Windows / macOS / Linux | SDK 檔案        | 僅支援證券交易帳務及條件單                   |

#### 安裝流程（精簡版）[​](#安裝流程精簡版 "Direct link to 安裝流程（精簡版）")

##### Python[​](#python "Direct link to Python")

1. 下載對應平台的 `.whl`。
2. 安裝：

```bash
pip install fubon_neo-<version>-cp37-abi3-win_amd64.whl

```

##### JavaScript (Node.js)[​](#javascript-nodejs "Direct link to JavaScript (Node.js)")

1. 下載 `.tgz` 並放入專案資料夾。
2. `package.json` 加入：

```json
"dependencies": {
  "fubon-neo": "file://<path-to-js-binary>/fubon-neo-<version>.tgz"
}

```

3. 安裝：

```bash
npm install

```

##### C#[​](#c "Direct link to C#")

1. 下載 `.nupkg`。
2. Visual Studio 以 NuGet Package Manager 安裝，或使用本機 NuGet 來源。

info

詳細安裝指引請見[快速開始](https://www.fbs.com.tw/TradeAPI/docs/trading/quickstart.txt)

##### C++[​](#c-1 "Direct link to C++")

1. 下載 C++ SDK 與範例程式碼。
2. 依範例專案設定 include/lib 路徑。

##### Go[​](#go "Direct link to Go")

1. 下載 Go SDK 與範例程式碼（Go 1.19+）。
2. 依範例專案設定 `go.mod` 及本地模組路徑。

#### 檔名與安裝包說明[​](#檔名與安裝包說明 "Direct link to 檔名與安裝包說明")

* `fubon_neo-<version>-cp37-abi3-win_amd64.whl`
* `fubon-neo-<version>.tgz`
* `FubonNeo.<version>.nupkg`

#### 版本功能對照 / 重大變更[​](#版本功能對照--重大變更 "Direct link to 版本功能對照 / 重大變更")

* API Key 登入：>= v2.2.7
* 網頁憑證匯出登入：>= v2.2.8
* Python 3.7：v2.0.1 起停止支援
* Python 3.14：目前不支援

更多版本資訊請見 [版本遷移資訊](https://www.fbs.com.tw/TradeAPI/docs/download/download-sdk.txt#%E7%89%88%E6%9C%AC%E9%81%B7%E7%A7%BB%E8%B3%87%E8%A8%8A)。

#### 常見問題（FAQ）[​](#常見問題faq "Direct link to 常見問題（FAQ）")

**Q1：安裝後如何確認 SDK 可用？**<br /><!-- -->A：請先完成登入與連線測試，再嘗試呼叫任一 API。

**Q2：Python 3.7 為什麼無法安裝？**<br /><!-- -->A：自 v2.0.1 起停止支援 Python 3.7。

**Q3：Python 3.14 可以使用嗎？**<br /><!-- -->A：目前不支援，請使用 Python 3.8–3.13。

**Q4：C++ / Go 為何不支援所有功能？**<br /><!-- -->A：目前僅提供證券交易帳務及條件單相關功能。

#### 下一步[​](#下一步 "Direct link to 下一步")

* [事前準備](https://www.fbs.com.tw/TradeAPI/docs/trading/prepare.txt)
* [快速開始](https://www.fbs.com.tw/TradeAPI/docs/trading/quickstart.txt)
* [API Key 申請](https://www.fbs.com.tw/TradeAPI/docs/key.txt)


---

### 金鑰申請及管理

版本限制

2.2.7版本新增功能

憑證匯出開放

2.2.8版起可使用本頁登入後匯出之憑證登入API，加強跨平台支援

連線測試

進行第一次登入「連線測試」需用**一般帳密+憑證**，不可使用API-Key登入方式進行連線測試

<!-- -->

金鑰申請及管理

您仍可使用現有的帳號憑證登入並使用 API。<br />**API Key 是一個選擇性功能**，主要提供 **更高的安全性與靈活性**，適合有進階需求的使用者。

##### 為什麼選擇 API Key？[​](#為什麼選擇-api-key "Direct link to 為什麼選擇 API Key？")

* **細緻權限控管**：可限制金鑰僅能使用特定功能（例如僅查詢行情、不允許下單）。
* **IP 白名單**：僅允許指定 IP 存取，降低外部風險。
* **快速撤銷**：隨時停用金鑰，不影響主要帳號。

此功能 **非強制**，但若您希望提升安全性或需要更彈性的整合方式，**建議啟用 API Key**。

注意事項

**請注意！** 當 API Key 列表有變動時（新增或移除），所有以 API Key 方式登入的 session 都會被強制登出

機制調整 >= v2.2.8 (需使用新版SDK)

API Key 權限變更強制斷線機制，調整為僅限於受變更影響之 key<br />(例：使用key1、key2同時登入上線，刪除 key1 => key1 session 強制斷線； key2 session 不受影響)<br />

#### 申請憑證[​](#申請憑證 "Direct link to 申請憑證")

申請 API Key 前，需先申請您的網頁憑證，即可進行 API Key 申請

1. 輸入您的身分證字號、電子平台密碼 ![key\_login\_step2](/TradeAPI/assets/images/key_login_step2-516ea7a99d80eba338fd760b77a2bc93.png)

2. 申請網頁版憑證，收取 OTP ![OTP](/TradeAPI/assets/images/OTP-3771c5ab5c12c603f48994c14e130c77.png)

3. 申請完憑證後，即可進行憑證匯出與新增金鑰 ![key\_login\_step3](/TradeAPI/assets/images/key_login_step3-48c99a5a670c60ce55e5d09a4fb097eb.png)

使用預設密碼憑證

在系統提示輸入憑證密碼時，請使用您的登入 ID

4. 進行金鑰申請

   * 點擊新增金鑰<br />![add\_key](/TradeAPI/assets/images/add_key-ce15b3f601484f2a96499f2a85150398.png)

   * 設置控制權限 ( IP或有效期限若不輸入，則為無限制 ) ![add\_key\_step2](/TradeAPI/assets/images/add_key_step2-ee688b483e6d020bb1ad988c42cd2325.png)

   * 設置成功，顯示 Secret Key ( Secret Key 關閉後即不再顯示 ) ![add\_key\_step3](/TradeAPI/assets/images/add_key_step3-6913c4ffe9e7e563c740c6ae15f80cbe.png)

5. 可檢視先前申請的 Key 內容或停用 ( Key 最多申請 30 把 ) ![key\_list](/TradeAPI/assets/images/key_list-7e65381ef6623e6c61a53a7cef01bf94.png)

本頁重點

* 本頁提供富邦新一代 API（Fubon Neo API）API Key 申請與管理入口。
* API Key 可用於登入與權限控管（依版本支援情況）。
* 完成申請後，可依需求調整權限或管理已建立的 Key。

| 項目     | 說明                            |
| -------- | ------------------------------- |
| 功能     | API Key 申請與管理              |
| 適用產品 | 富邦新一代 API（Fubon Neo API） |
| 主要用途 | 登入驗證、權限控管              |
| 相關頁面 | API Key 說明 / 申請流程         |


---

### 資本變動資料

取得上市櫃股票及 ETF 之資本變動資料（面額變更、減資、分割）

```text
GET /corporate-actions/capital-changes/

```

版本資訊

v2.2.8 起新增功能

#### Parameters[​](#parameters "Direct link to Parameters")

| Name         | Type   | Description                                                  |
| ------------ | ------ | ------------------------------------------------------------ |
| `start_date` | string | 開始日期（格式：`yyyy-MM-dd`）                               |
| `end_date`   | string | 結束日期（格式：`yyyy-MM-dd`）可輸入未來日期取得未來預告資訊 |
| `sort`       | string | 排序方式（`asc`、`desc`）                                    |

#### Response[​](#response "Direct link to Response")

| Name                              | Type      | Description                                                                                                 |
| --------------------------------- | --------- | ----------------------------------------------------------------------------------------------------------- |
| `data`\*                          | object\[] | 資本變動資料                                                                                                |
| `data.symbol`                     | string    | 股票代號                                                                                                    |
| `data.name`                       | string    | 股票名稱                                                                                                    |
| `data.actionType`                 | string    | 事件類型：`etf_split_or_merge` ETF 分割或反分割、`par_value_change` 股票面額變更 、`capital_reduction` 減資 |
| `data.effectiveDate`              | string    | 恢復買賣日期 (YYYY-MM-DD)                                                                                   |
| `data.adjustmentFactor`           | number    | 調整係數                                                                                                    |
| `data.haltDate`                   | string    | 停止買賣日期 (YYYY-MM-DD)                                                                                   |
| `data.resumeDate`                 | string    | 恢復買賣日期 (YYYY-MM-DD)                                                                                   |
| `data.raw`                        | object    | 原始資料                                                                                                    |
| `data.raw.exchangeRatio`          | number    | 換股比例 (面額變更、減資時才有值)                                                                           |
| `data.raw.parValueBefore`         | number    | 變更前面額 (面額變更時才有值)                                                                               |
| `data.raw.parValueAfter`          | number    | 變更後面額 (面額變更時才有值)                                                                               |
| `data.raw.lastClosePrice`         | number    | 停止買賣前收盤價                                                                                            |
| `data.raw.referencePrice`         | number    | 恢復買賣參考價                                                                                              |
| `data.raw.limitUpPrice`           | number    | 漲停價格                                                                                                    |
| `data.raw.limitDownPrice`         | number    | 跌停價格                                                                                                    |
| `data.raw.openingReferencePrice`  | number    | 開盤競價基準                                                                                                |
| `data.raw.splitType`              | string    | 分割類型：`分割`、`反分割`                                                                                  |
| `data.raw.reductionReason`        | string    | 減資原因 (減資時才有值)                                                                                     |
| `data.raw.refundPerShare`         | number    | 每股退還股款 (元) (減資時才有值)                                                                            |
| `data.raw.rightsOfferingRatio`    | number    | 減資後現金增資配股率 (減資時才有值)                                                                         |
| `data.raw.rightsOfferingPrice`    | number    | 現金增資認購價 (元) (減資時才有值)                                                                          |
| `data.raw.exRightsReferencePrice` | number    | 除權參考價 (減資時才有值)                                                                                   |

#### Example[​](#example "Direct link to Example")

* Python
* Node.js
* C#

```python
from fubon_neo.sdk import FubonSDK, Order

sdk = FubonSDK()

accounts = sdk.login("Your ID", "Your password", "Your cert path", "Your cert password")  # 需登入後，才能取得行情權限

sdk.init_realtime() # 建立行情連線

reststock = sdk.marketdata.rest_client.stock

