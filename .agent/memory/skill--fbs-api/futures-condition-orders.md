### 富邦新一代 API 期貨條件單

***

本頁重點

* 本頁介紹富邦新一代 API 期貨條件單功能與支援範圍。
* 提供多種條件單類型，適用期貨自動化策略與風險管理。
* 下一步建議先完成[事前準備](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/prepare.txt)。

| 項目     | 說明                                                                       |
| -------- | -------------------------------------------------------------------------- |
| 產品     | 富邦新一代 API 期貨條件單                                                  |
| 主要功能 | 觸價/觸量、停損停利、分時分量                                              |
| 平台     | Windows / macOS / Linux                                                    |
| 語言     | Python / C# / JavaScript (Node.js) / C++、Go（僅支援證券交易帳務及條件單） |
| 前置     | 需簽署條件單使用同意書                                                     |

#### 概述[​](#概述 "Direct link to 概述")

富邦新一代 API 條件單為您的程式交易，提供了更完善的功能，滿足每一位開發者達成條件下單。<br /><!-- -->提供用戶自由選擇主流開發語言( Python、C#、JavaScript)，在創建程式交易的過程中，輕易地取用 API，使用 API 各種功能。

#### 特色[​](#特色 "Direct link to 特色")

支援跨平台：<br /><!-- -->新一代 API 支援 Windows、MacOS、Linux。<br /><!-- -->新一代 API 支援 Python、C#、JavaScript（Node.js）、C++、Go 主流語言（C++、Go 僅支援證券交易帳務及條件單）。

#### 主要功能[​](#主要功能 "Direct link to 主要功能")

直接管理交易：建立單一條件觸價觸量單、多條件觸價觸量單、停損停利單、分時分量單···等。

#### 版本支援[​](#版本支援 "Direct link to 版本支援")

Python 版本支援：3.7（~v1.3.2）、3.8–3.13（v2.0.1~，不含 3.14）。<br /><!-- -->Node.js 版本支援 : 目前支援Node.js 16以上的版本。<br /><!-- -->C# 使用.NET Standard 2.0開發，建議您使用 .netcoreapp 3.1 以上；如使用.NETFramework 建議您使用.NETFramework 4.7.2以上版本。


---

### 取消條件單

CancelConditionOrders

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                                                                      | 說明     |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                                                                       | 帳號     |
| guid       | string                                                                                                                                                                    | 條件單號 |
| marketType | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#futoptconditionmarkettype) (Optional) default : 日盤 | 盤別     |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 CancelResult 欄位[​](#委託資訊-cancelresult--欄位 "Direct link to 委託資訊 CancelResult  欄位")

Return type : Object

| 參數     | 類別   | 說明         |
| -------- | ------ | ------------ |
| advisory | string | 回覆狀態內容 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
sdk.FutOpt.CancelConditionOrders(account, "c9df498a-3b28-4b50-a6f2-f7bd524e96df");


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data : CancelResult {
             advisory: "成功筆數:1,失敗筆數:0!",
            }
 }



```


---

### 參數對照表

#### 類別[​](#類別 "Direct link to 類別")

Class

##### Condition Object[​](#condition-object "Direct link to Condition Object")

| Parameter    | Type           | Meaning                                                                                                                                 |
| ------------ | -------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| marketType   | TradingType    | [監控類型](#tradingtype) 可選用參數 : `Reference` 自動參考委託物件、`Scheduled` 時間                                                    |
| symbol       | string         | 股票代號                                                                                                                                |
| trigger      | TriggerContent | [觸發條件](#triggercontent) 可選用參數 : `BidPrice` 買進價、`AskPrice` 賣出價、`MatchedPrice` 成交價、`TotalQuantity` 總量、`Time` 時間 |
| triggerValue | string         | 監控條件數值 (若為時間則填入HHMMSS)                                                                                                     |
| comparison   | Operator       | [比對值](#operator) 可選用參數`GreaterThanOrEqual` 大於等於、`LessThanOrEqual`小於等於 、 `GreaterThan` 大於 、 `LessThan` 小於         |

info

1. 選擇 `Reference`時， trigger 可搭配 `BidPrice` 、`AskPrice` 、`MatchedPrice` 、`TotalQuantity`
2. 選擇`Scheduled` 時，symbol需填入欲觸發之商品，trigger需帶入`Time`

##### FutOptConditionOrder Object[​](#futoptconditionorder-object "Direct link to FutOptConditionOrder Object")

| Parameter   | Type                      | Meaning                                                                                                                                                                                                                         |
| ----------- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| buySell     | BsAction                  | [買賣別](#bsaction) 可選用參數`Buy` 買 、 `Sell` 賣                                                                                                                                                                             |
| symbol      | string                    | 股票代號                                                                                                                                                                                                                        |
| price       | string                    | 委託價格                                                                                                                                                                                                                        |
| quantity    | int                       | 委託數量                                                                                                                                                                                                                        |
| marketType  | FutOptConditionMarketType | [盤別](#futoptconditionmarkettype) 可選用參數 : `Future` 期貨日盤、`Option` 選擇權日盤、`FutureNight` 期貨夜盤、`OptionNight` 選擇權夜盤 (**IMPORTANT:** 以下功能不支援夜盤： 1. 分時分量, 2. 移動鎖利, 3. 時間觸發條件)        |
| priceType   | FutOptConditionPriceType  | [價格別](#futoptconditionpricetype) : 可選參數 :`BidPrice` 買進價、 `AskPrice` 賣出價、`MatchedPrice`成交價、`Limit` 限價、`LimitUp` 漲停、`LimitDown` 跌停、`Market` 市價、`Reference` 參考價(平盤價) 、`RangeMarket` 範圍市價 |
| timeInForce | TimeInforce               | [委託條件](#timeinforce) : 可選參數 :`Rod`、`Ioc`、`Fok`                                                                                                                                                                        |
| orderType   | FutOptConditionOrderType  | [委託類別](#futoptconditionordertype) : 可選參數 :`New` 新倉、`Close` 平倉                                                                                                                                                      |

caution

以下功能不支援夜盤：

1. 分時分量
2. 移動鎖利
3. 時間觸發條件

##### FutOptTPSLOrder Object[​](#futopttpslorder-object "Direct link to FutOptTPSLOrder Object")

| Parameter              | Type                     | Meaning                                                                                                                                                                                                                         |
| ---------------------- | ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| timeInForce            | TimeInforce              | [委託條件](#timeinforce) : 可選參數 :`Rod`、`Ioc`、`Fok`                                                                                                                                                                        |
| priceType              | FutOptConditionPriceType | [價格別](#futoptconditionpricetype) : 可選參數 :`BidPrice` 買進價、 `AskPrice` 賣出價、`MatchedPrice`成交價、`Limit` 限價、`LimitUp` 漲停、`LimitDown` 跌停、`Market` 市價、`Reference` 參考價(平盤價) 、`RangeMarket` 範圍市價 |
| orderType              | FutOptConditionOrderType | [委託類別](#futoptconditionordertype) : 可選參數 :`New` 新倉、`Close` 平倉                                                                                                                                                      |
| targetPrice            | string                   | 停損/停利觸發價                                                                                                                                                                                                                 |
| price                  | string                   | 停損/停利委託價                                                                                                                                                                                                                 |
| trigger (**保留欄位**) | TriggerContent           | 2.2.0 版本新增欄位，暫無相關功能，請填入 ***null***                                                                                                                                                                             |

info

`priceType` 若為`limit`時，`price`需填入價格；其餘price填入空字串

##### FutOptTpslWrapper Object[​](#futopttpslwrapper-object "Direct link to FutOptTpslWrapper Object")

| Parameter | Type                                    | Meaning                                                                                           |
| --------- | --------------------------------------- | ------------------------------------------------------------------------------------------------- |
| stopSign  | StopSign                                | [停止條件](#stopsign) : 可選用 `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| TP        | FutOptTpslOrder Object (Optional)       | 停利條件內容                                                                                      |
| SL        | FutOptTpslOrder Object (Optional)       | 停損條件內容                                                                                      |
| endDate   | string (Optional : 空值同母單StartDate) | 條件停止日期                                                                                      |
| dayTrade  | bool (Optional)                         | 全部成交後當沖 (期貨固定為True)                                                                   |

##### FutOptTrailOrder[​](#futopttrailorder "Direct link to FutOptTrailOrder")

| Parameter   | Type                     | Meaning                                                                                                                                                                                                     |
| ----------- | ------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol      | string                   | 商品代號                                                                                                                                                                                                    |
| price       | string                   | 基準價                                                                                                                                                                                                      |
| direction   | Direction                | [方向](#direction) : 可選用 `Up` 上漲、`Down` 下跌                                                                                                                                                          |
| tickNum     | int                      | 上漲 / 下跌 ticker 數                                                                                                                                                                                       |
| buysell     | BsAction                 | 買賣別 : 可選用 `Buy` 買進 、`Sell` 賣出                                                                                                                                                                    |
| lot         | int                      | 委託口數                                                                                                                                                                                                    |
| PriceType   | FutOptConditionPriceType | [價格類別](#futoptconditionpricetype) : 可選用 `BidPrice` 買進價、`AskPrice` 賣出價、`MatchedPrice` 成交價、`Market` 市價、`LimitUp` 漲停價、`LimitDown` 跌停價、`Reference` 平盤價、`RangeMarket` 範圍市價 |
| diff        | int                      | 買賣價格檔數 (根據 priceType 加減檔數) ，正值為向上加檔數、負值為向下加檔數                                                                                                                                 |
| timeInForce | TimeInForce              | [委託條件](#timeinforce) 可選用參數為 `ROD`、`FOK`、`IOC`                                                                                                                                                   |
| orderType   | FutOptConditionOrderType | [委託類別](#futoptconditionordertype) 可選用參數為 `New` 新倉、`Close`平倉                                                                                                                                  |

##### SplitDescription[​](#splitdescription "Direct link to SplitDescription")

| Parameter      | Type              | Meaning                                                                                                                                                                                                                                                                                                                                                                                                      |
| -------------- | ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| method         | SliceType         | [Slice Condition](#slicetype) : 可選用 :<br />`Type1` 從 `startTime` 開始，每隔幾秒送一筆，總共`totalQuantity`，每筆送 `singleQuantity`<br />`Type2` 從 `startTime` 開始到 `endTime` 結束，每隔 `interval` 秒送一筆，總共 `totalQuantity`，剩餘張數加總至最後一筆<br />`Type3` 從 `startTime` 開始到 `endTime` 結束，每隔 `interval` 秒送一筆，總共 `totalQuantity`，剩餘張數從最後一筆往前分配.<br /><br /> |
| interval       | int               | 下單間隔時間                                                                                                                                                                                                                                                                                                                                                                                                 |
| singleQuantity | int               | 單筆下單量 (股數)                                                                                                                                                                                                                                                                                                                                                                                            |
| totalQuantity  | int (Optional)    | 總共下單量 (股數)                                                                                                                                                                                                                                                                                                                                                                                            |
| startTime      | string            | 開始時間                                                                                                                                                                                                                                                                                                                                                                                                     |
| endTime        | string (Optional) | 結束時間                                                                                                                                                                                                                                                                                                                                                                                                     |

caution

請注意，總共下單量需大於單筆下單量。

##### ConditionOrderResult[​](#conditionorderresult "Direct link to ConditionOrderResult")

| Parameter | Type   | Meaning  |
| --------- | ------ | -------- |
| guid      | string | 條件單號 |

#### Constants ( 欄位對應數值 )[​](#constants--欄位對應數值- "Direct link to Constants ( 欄位對應數值 )")

##### BSAction[​](#bsaction "Direct link to BSAction")

買賣別 (buySell)

| Name | Meaning |
| ---- | ------- |
| Buy  | 買      |
| Sell | 賣      |

##### TradingType[​](#tradingtype "Direct link to TradingType")

監控類型

| Name      | Meaning          |
| --------- | ---------------- |
| Reference | 自動參考委託物件 |
| Index     | 指數             |
| Scheduled | 時間             |

##### TriggerContent[​](#triggercontent "Direct link to TriggerContent")

觸發條件

| Name          | Meaning |
| ------------- | ------- |
| BidPrice      | 買進價  |
| AskPrice      | 賣出價  |
| MatchedPrice  | 成交價  |
| TotalQuantity | 總量    |
| Time          | 時間    |

##### Operator[​](#operator "Direct link to Operator")

| Name               | Meaning  |
| ------------------ | -------- |
| GreaterThanOrEqual | 大於等於 |
| LessThanOrEqual    | 小於等於 |
| GreaterThan        | 大於     |
| LessThan           | 小於     |

##### StopSign[​](#stopsign "Direct link to StopSign")

| Name     | Meaning      |
| -------- | ------------ |
| Full     | 全部成交為止 |
| Partial  | 部分成交為止 |
| UntilEnd | 效期結束為止 |

##### TimeInForce[​](#timeinforce "Direct link to TimeInForce")

委託條件 (TimeInForce)

| Name | Meaning                               |
| ---- | ------------------------------------- |
| ROD  | 當日有效(Rest of Day)                 |
| FOK  | 全部成交否則取消(Fill-or-Kill)        |
| IOC  | 立即成交否則取消(Immediate-or-Cancel) |

##### FutOptConditionMarketType[​](#futoptconditionmarkettype "Direct link to FutOptConditionMarketType")

盤別 (FutOptConditionMarketType)

| Name   | Meaning    |
| ------ | ---------- |
| Future | 期貨日盤   |
| Option | 選擇權日盤 |

##### FutOptConditionPriceType[​](#futoptconditionpricetype "Direct link to FutOptConditionPriceType")

價格類型 (FutOptConditionPriceType)

| Name         | Meaning        |
| ------------ | -------------- |
| Limit        | 限價           |
| BidPrice     | 買進價         |
| AskPrice     | 賣出價         |
| Market       | 市價           |
| MatchedPrice | 成交價         |
| LimitUp      | 漲停價         |
| LimitDown    | 跌停價         |
| Reference    | 參考價(平盤價) |
| RangeMarket  | 範圍市價       |

##### FutOptConditionOrderType[​](#futoptconditionordertype "Direct link to FutOptConditionOrderType")

委託類型 (FutOptConditionOrderType)

| Name  | Meaning |
| ----- | ------- |
| New   | 新倉    |
| Close | 平倉    |

##### Direction[​](#direction "Direct link to Direction")

移動鎖利上漲/下跌追蹤 (direction)

| Name | Meaning |
| ---- | ------- |
| Up   | 上漲    |
| Down | 下跌    |

##### SliceType[​](#slicetype "Direct link to SliceType")

分單型態 (SliceType)

| Name  | Meaning                                                          |
| ----- | ---------------------------------------------------------------- |
| Type1 | 從開始時間，每隔幾秒送一筆，總共送N筆，每筆送M張                 |
| Type2 | 從開始到結束，每隔X秒送一筆，總共N張，剩餘張數加總至最後一筆     |
| Type3 | 從開始到結束，每隔X秒送一筆，總共N張，剩餘張數從最後一筆往前分配 |

##### Operator[​](#operator-1 "Direct link to Operator")

| Name               | Meaning  |
| ------------------ | -------- |
| GreaterThanOrEqual | 大於等於 |
| LessThanOrEqual    | 小於等於 |
| GreaterThan        | 大於     |
| LessThan           | 小於     |

##### ConditionStatus[​](#conditionstatus "Direct link to ConditionStatus")

條件單狀態 (ConditionStatus)

| Name   | Meaning      |
| ------ | ------------ |
| Type1  | 今日相關查詢 |
| Type2  | 尚有效單     |
| Type3  | 條件比對中   |
| Type4  | 委託處理中   |
| Type5  | 委託成功     |
| Type6  | 已通知       |
| Type7  | 委託失敗     |
| Type8  | 已有成交     |
| Type9  | 刪除成功     |
| Type10 | 異常         |
| Type11 | 失效         |

##### HistoryStatus[​](#historystatus "Direct link to HistoryStatus")

歷史條件單狀態 (ConditionHistoryStatus)

| Name  | Meaning                          |
| ----- | -------------------------------- |
| Type1 | 所有條件單 ( 不包含已刪除、失效) |
| Type2 | 選擇期間內全部成交單             |
| Type3 | 選擇期間內部分成交單             |
| Type4 | 選擇期間刪除單                   |
| Type5 | 選擇期間失效單                   |
| Type6 | 選擇期間內已觸發記錄             |


---

### 條件單查詢 By 條件單號

GetConditionOrderById

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                                                                      | 說明     |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                                                                       | 帳號     |
| guid       | string                                                                                                                                                                    | 條件單號 |
| marketType | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#futoptconditionmarkettype) (Optional) default : 日盤 | 盤別     |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | List                | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionDetail 欄位[​](#委託資訊-conditiondetail--欄位 "Direct link to 委託資訊 ConditionDetail  欄位")

Return type : Object

| 參數                     | 類別   | 說明             |
| ------------------------ | ------ | ---------------- |
| guid                     | string | 條件單號         |
| batchNo                  | string | 分單批號         |
| orderLevel               | string | 條件單層級       |
| lastTime                 | string | 更新時間         |
| parentGuid               | string | 父單單號         |
| symbol                   | string | 商品代號         |
| orderAmount              | string | 委託金額         |
| childBatchNo             | string | 子單分單批號     |
| account                  | string | 帳號             |
| conditionContent         | string | 條件內容         |
| action                   | string | 處理方式         |
| conditionBuySell         | string | 買賣別           |
| conditionSymbol          | string | 商品名稱         |
| conditionPrice           | string | 價格             |
| conditionVolume          | string | 委託數量         |
| conditionFilledVolume    | string | 成交數量         |
| createTime               | string | 建立時間         |
| startDate                | string | 預約開始時間     |
| status                   | string | 目前狀態         |
| errorMessage             | string | 目前狀態異常說明 |
| detailRecordsCount       | string | 查詢明細筆數     |
| detailRecords            | List   | 查詢明細資料     |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |
| tpslCount                | string | 停損停利筆數     |
| tpslRecord               | List   | 停損停利明細資料 |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
sdk.FutOpt.GetConditionOrderById(account,"8ff3472b-185a-488c-be5a-b478deda080c",FutOptConditionMarketType.Future);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = {
            Guid = adada47e-dea3-4a5a-9eff-d36bd7a71e87,
            BatchNo = ,
            OrderLevel = 0,
            LastTime = 2024-11-15 10:38:48,
            ConditionType = 觸價,
            ParentGuid = adada47e-dea3-4a5a-9eff-d36bd7a71e87,
            Symbol = FIMTX202411,
            OrderAmount = 0,
            ChildBatchNo = ,
            Account = 9974825,
            ConditionContent = 當自2024/11/15至2024/11/20內台指期202412成交價大於21000點 全部成交為止,
            Action = 下單(新倉),
            ConditionBuySell = 賣,
            ConditionSymbol = 賣 小型台指202411,
            ConditionPrice = 範圍市價(ROD),
            ConditionVolume = 2,
            ConditionFilledVolume = 0,
            CreateTime = 2024-11-15 10:38:48,
            StartDate = 2024/11/15,
            Status = 洗價中(Y),
            ErrorMessage = null,
            DetailRecordsCount = 0,
            DetailRecords = [],
            TpslCount = 2,
            TpslRecord = 
            [
                {
                    Guid = b79084fe-6fdf-461f-a264-0482072237eb,
                    Account = 9974825,
                    ConditionContent = 當自2024/11/15至2024/11/20內小型台指202411買進價大於等於22340元 全部成交為止,
                    Action = 下單(平倉),
                    ConditionBuySell = 買,
                    ConditionSymbol = 買 小型台指202411,
                    ConditionPrice = 22340(ROD),
                    ConditionVolume = 2,
                    ConditionFilledVolume = 0,
                    StartDate = 2024/11/15,
                    Status = 未生效(W),
                    ErrorMessage = 
                },
                {
                    Guid = b79084fe-6fdf-461f-a264-0482072237eb,
                    Account = 9974825,
                    ConditionContent = 當自2024/11/15至2024/11/20內小型台指202411買進價小於等於22280元 全部成交為止,
                    Action = 下單(平倉),
                    ConditionBuySell = 買,
                    ConditionSymbol = 買 小型台指202411,
                    ConditionPrice = 22280(ROD),
                    ConditionVolume = 2,
                    ConditionFilledVolume = 0,
                    StartDate = 2024/11/15,
                    Status = 未生效(W),
                    ErrorMessage = 
                }
            ]
        }     
}


```


---

### 歷史條件單查詢

GetConditionHistory

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數          | 類別                                                                                                                                                                      | 說明           |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------- |
| account       | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                                                                       | 帳號           |
| startDate     | string                                                                                                                                                                    | 查詢開始日     |
| endDate       | string                                                                                                                                                                    | 查詢截止日     |
| marketType    | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#futoptconditionmarkettype) (Optional) default : 日盤 | 盤別           |
| historyStatus | [ConditionHistoryStatus](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#historystatus) (Optional)                               | 歷史條件單狀態 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | list                | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionDetail 欄位[​](#委託資訊-conditiondetail--欄位 "Direct link to 委託資訊 ConditionDetail  欄位")

Return type : Object

| 參數                     | 類別   | 說明             |
| ------------------------ | ------ | ---------------- |
| guid                     | string | 條件單號         |
| batchNo                  | string | 分單批號         |
| orderLevel               | string | 條件單層級       |
| lastTime                 | string | 更新時間         |
| parentGuid               | string | 父單單號         |
| symbol                   | string | 商品代號         |
| orderAmount              | string | 委託金額         |
| childBatchNo             | string | 子單分單批號     |
| account                  | string | 帳號             |
| conditionContent         | string | 條件內容         |
| action                   | string | 處理方式         |
| conditionBuySell         | string | 買賣別           |
| conditionSymbol          | string | 商品名稱         |
| conditionPrice           | string | 價格             |
| conditionVolume          | string | 委託數量         |
| conditionFilledVolume    | string | 成交數量         |
| createTime               | string | 建立時間         |
| startDate                | string | 預約開始時間     |
| status                   | string | 目前狀態         |
| errorMessage             | string | 目前狀態異常說明 |
| detailRecordsCount       | string | 查詢明細筆數     |
| detailRecords            | List   | 查詢明細資料     |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |
| tpslCount                | string | 停損停利筆數     |
| tpslRecord               | List   | 停損停利明細資料 |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
sdk.FutOpt.GetConditionHistory(account,"20240301","20240601",FutOptConditionMarketType.Future);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = [{
            Guid = adada47e-dea3-4a5a-9eff-d36bd7a71e87,
            BatchNo = ,
            OrderLevel = 0,
            LastTime = 2024-11-15 10:38:48,
            ConditionType = 觸價,
            ParentGuid = adada47e-dea3-4a5a-9eff-d36bd7a71e87,
            Symbol = FIMTX202411,
            OrderAmount = 0,
            ChildBatchNo = ,
            Account = 9974825,
            ConditionContent = 當自2024/11/15至2024/11/20內台指期202412成交價大於21000點 全部成交為止,
            Action = 下單(新倉),
            ConditionBuySell = 賣,
            ConditionSymbol = 賣 小型台指202411,
            ConditionPrice = 範圍市價(ROD),
            ConditionVolume = 2,
            ConditionFilledVolume = 0,
            CreateTime = 2024-11-15 10:38:48,
            StartDate = 2024/11/15,
            Status = 洗價中(Y),
            ErrorMessage = null,
            DetailRecordsCount = 0,
            DetailRecords = [],
            TpslCount = 2,
            TpslRecord = 
            [
                {
                    Guid = b79084fe-6fdf-461f-a264-0482072237eb,
                    Account = 9974825,
                    ConditionContent = 當自2024/11/15至2024/11/20內小型台指202411買進價大於等於22340元 全部成交為止,
                    Action = 下單(平倉),
                    ConditionBuySell = 買,
                    ConditionSymbol = 買 小型台指202411,
                    ConditionPrice = 22340(ROD),
                    ConditionVolume = 2,
                    ConditionFilledVolume = 0,
                    StartDate = 2024/11/15,
                    Status = 未生效(W),
                    ErrorMessage = 
                },
                {
                    Guid = b79084fe-6fdf-461f-a264-0482072237eb,
                    Account = 9974825,
                    ConditionContent = 當自2024/11/15至2024/11/20內小型台指202411買進價小於等於22280元 全部成交為止,
                    Action = 下單(平倉),
                    ConditionBuySell = 買,
                    ConditionSymbol = 買 小型台指202411,
                    ConditionPrice = 22280(ROD),
                    ConditionVolume = 2,
                    ConditionFilledVolume = 0,
                    StartDate = 2024/11/15,
                    Status = 未生效(W),
                    ErrorMessage = 
                }
            ]
        },
        ...
    ]
}


```


---

### 條件單查詢

GetConditionOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數            | 類別                                                                                                                                                                      | 說明       |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| account         | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                                                                       | 帳號       |
| marketType      | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#futoptconditionmarkettype) (Optional) default : 日盤 | 盤別       |
| conditionStatus | [ConditionStatus](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#conditionstatus) (Optional)                                    | 條件單狀態 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | List                | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionDetail 欄位[​](#委託資訊-conditiondetail--欄位 "Direct link to 委託資訊 ConditionDetail  欄位")

Return type : Object

| 參數                     | 類別   | 說明             |
| ------------------------ | ------ | ---------------- |
| guid                     | string | 條件單號         |
| batchNo                  | string | 分單批號         |
| orderLevel               | string | 條件單層級       |
| lastTime                 | string | 更新時間         |
| parentGuid               | string | 父單單號         |
| symbol                   | string | 商品代號         |
| orderAmount              | string | 委託金額         |
| childBatchNo             | string | 子單分單批號     |
| account                  | string | 帳號             |
| conditionContent         | string | 條件內容         |
| action                   | string | 處理方式         |
| conditionBuySell         | string | 買賣別           |
| conditionSymbol          | string | 商品名稱         |
| conditionPrice           | string | 價格             |
| conditionVolume          | string | 委託數量         |
| conditionFilledVolume    | string | 成交數量         |
| createTime               | string | 建立時間         |
| startDate                | string | 預約開始時間     |
| status                   | string | 目前狀態         |
| errorMessage             | string | 目前狀態異常說明 |
| detailRecordsCount       | string | 查詢明細筆數     |
| detailRecords            | List   | 查詢明細資料     |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |
| tpslCount                | string | 停損停利筆數     |
| tpslRecord               | List   | 停損停利明細資料 |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
sdk.FutOpt.GetConditionOrder(account, FutOptConditionMarketType.Future);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = [{
            Guid = adada47e-dea3-4a5a-9eff-d36bd7a71e87,
            BatchNo = ,
            OrderLevel = 0,
            LastTime = 2024-11-15 10:38:48,
            ConditionType = 觸價,
            ParentGuid = adada47e-dea3-4a5a-9eff-d36bd7a71e87,
            Symbol = FIMTX202411,
            OrderAmount = 0,
            ChildBatchNo = ,
            Account = 9974825,
            ConditionContent = 當自2024/11/15至2024/11/20內台指期202412成交價大於21000點 全部成交為止,
            Action = 下單(新倉),
            ConditionBuySell = 賣,
            ConditionSymbol = 賣 小型台指202411,
            ConditionPrice = 範圍市價(ROD),
            ConditionVolume = 2,
            ConditionFilledVolume = 0,
            CreateTime = 2024-11-15 10:38:48,
            StartDate = 2024/11/15,
            Status = 洗價中(Y),
            ErrorMessage = null,
            DetailRecordsCount = 0,
            DetailRecords = [],
            TpslCount = 2,
            TpslRecord = 
            [
                {
                    Guid = b79084fe-6fdf-461f-a264-0482072237eb,
                    Account = 9974825,
                    ConditionContent = 當自2024/11/15至2024/11/20內小型台指202411買進價大於等於22340元 全部成交為止,
                    Action = 下單(平倉),
                    ConditionBuySell = 買,
                    ConditionSymbol = 買 小型台指202411,
                    ConditionPrice = 22340(ROD),
                    ConditionVolume = 2,
                    ConditionFilledVolume = 0,
                    StartDate = 2024/11/15,
                    Status = 未生效(W),
                    ErrorMessage = 
                },
                {
                    Guid = b79084fe-6fdf-461f-a264-0482072237eb,
                    Account = 9974825,
                    ConditionContent = 當自2024/11/15至2024/11/20內小型台指202411買進價小於等於22280元 全部成交為止,
                    Action = 下單(平倉),
                    ConditionBuySell = 買,
                    ConditionSymbol = 買 小型台指202411,
                    ConditionPrice = 22280(ROD),
                    ConditionVolume = 2,
                    ConditionFilledVolume = 0,
                    StartDate = 2024/11/15,
                    Status = 未生效(W),
                    ErrorMessage = 
                }
            ]
        },
        ...
    ]
}


```


---

### 多條件單

MultiCondition

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數           | 類別                                                                                                                                | 說明                                                                             |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| account        | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                                 | 帳號                                                                             |
| start\_date    | string                                                                                                                              | 條件開始監控時間                                                                 |
| end\_date      | string                                                                                                                              | 條件結束監控時間                                                                 |
| stop\_sign     | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#stopsign)                       | 條件停止條件 :`Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| MultiCondition | [Condition List](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#condition-object)         | 觸發條件                                                                         |
| OrderObject    | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#futoptconditionorder-object) | 委託內容                                                                         |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
// 設計條件內容
var condition1 = new Condition(
    TradingType.Reference,
    "TXO20000E4",
    TriggerContent.MatchedPrice,
    "100",
    Operator.LessThan
);

var condition2 = new Condition(
    TradingType.Reference,
    "TXO20000E4",
    TriggerContent.TotalQuantity,
    "30",
    Operator.LessThan
);

List<Condition> conditions = new List<Condition>();

conditions.Add(condition1);
conditions.Add(condition2);

var order = new FutOptConditionOrder(
    BsAction.Buy,
    "TXO20000E4",
    "500",
    1,
    FutOptConditionMarketType.Option,
    FutOptConditionPriceType.Limit,
    TimeInForce.Rod,
    FutOptConditionOrderType.New
);


sdk.FutOpt.MultiCondition(account, "20240426", "20240430", StopSign.Full, conditions, order);


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = ConditionOrderResult {
            guid = 44150047-8977-40b1-953c-ce2XXXXXX,
        }
}


```


---

### 多條件單

MultiCondition

停損利注意事項

* 停損利設定僅為觸發送單，不保證必定成交，需視市場狀況自行調整
* 請確認停損利**委託類別**設定需符合適合之交易規則

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數           | 類別                                                                                                                                   | 說明                                                                             |
| -------------- | -------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| account        | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                                    | 帳號                                                                             |
| start\_date    | string                                                                                                                                 | 條件開始監控時間                                                                 |
| end\_date      | string                                                                                                                                 | 條件結束監控時間                                                                 |
| stop\_sign     | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#stopsign)                          | 條件停止條件 :`Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| MultiCondition | [Condition List](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#condition-object)            | 觸發條件                                                                         |
| OrderObject    | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#futoptconditionorder-object)    | 委託內容                                                                         |
| TPSLObject     | [FutOptTpslWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#futopttpslwrapper-object) | 停損停利條件                                                                     |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
// 設計條件內容
var condition1 = new Condition(
    TradingType.Reference,
    "TXO20000E4",
    TriggerContent.MatchedPrice,
    "100",
    Operator.LessThan
);

var condition2 = new Condition(
    TradingType.Reference,
    "TXO20000E4",
    TriggerContent.TotalQuantity,
    "30",
    Operator.LessThan
);

List<Condition> conditions = new List<Condition>();

conditions.Add(condition1);
conditions.Add(condition2);


var order = new FutOptConditionOrder(
    BsAction.Buy,
    "TXO20000E4",
    "100",
    1,
    FutOptConditionMarketType.Option,
    FutOptConditionPriceType.Limit,
    TimeInForce.Rod,
    FutOptConditionOrderType.New
);

var tp = new FutOptTpslOrder(
    TimeInForce.Rod,
    FutOptConditionPriceType.Limit,
    FutOptConditionOrderType.Close,
    "120",
    "120",
    null  // 2.2.0 版本新增，暫無相關功能
);

var sl = new FutOptTpslOrder(
    TimeInForce.Rod,
    FutOptConditionPriceType.Limit,
    FutOptConditionOrderType.Close,
    "85",
    "85",
    null  // 2.2.0 版本新增，暫無相關功能
);

var tpsl = new FutOptTpslWrapper(
    StopSign.Full,
    tp,
    sl,
    "20240517",
    false
);


sdk.FutOpt.MultiCondition(account, "20240426", "20240430", StopSign.Full, conditions, order, tpsl);


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = ConditionOrderResult {
            guid = 44150047-8977-40b1-953c-ce2XXXXXX
        }
}


```


---

### 單一條件單

SingleCondition

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                                                         | 說明                                                                              |
| ----------- | -------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                                          | 帳號                                                                              |
| start\_date | string                                                                                                                                       | 條件開始監控時間                                                                  |
| end\_date   | string                                                                                                                                       | 條件結束監控時間                                                                  |
| stop\_sign  | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#stopsign)                                | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| condition   | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#condition-object)                       | 觸發條件                                                                          |
| OrderObject | [FutOptConditionOrder](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#futoptconditionorder-object) | 委託內容                                                                          |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
// 設計條件內容
var condition = new Condition(
    TradingType.Reference,
    "TXO20000E4",
    TriggerContent.MatchedPrice,
    "100",
    Operator.LessThan
);

var order = new FutOptConditionOrder(
    BsAction.Buy,
    "TXO20000E4",
    "500",
    1,
    FutOptConditionMarketType.Option,
    FutOptConditionPriceType.Limit,
    TimeInForce.Rod,
    FutOptConditionOrderType.New,
);


sdk.FutOpt.SingleCondition(account, "20240426", "20240430", StopSign.Full, condition, order);


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = ConditionOrderResult {
            guid = 44150047-8977-40b1-953c-ce2XXXXXX
        }
}


```


---

### 單一條件單包含停損停利

SingleCondition

停損利注意事項

* 停損利設定僅為觸發送單，不保證必定成交，需視市場狀況自行調整
* 請確認停損利**委託類別**設定需符合適合之交易規則

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                                                   | 說明                                                                              |
| ----------- | -------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                                    | 帳號                                                                              |
| start\_date | string                                                                                                                                 | 條件開始監控時間                                                                  |
| end\_date   | string                                                                                                                                 | 條件結束監控時間                                                                  |
| stop\_sign  | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#stopsign)                          | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| condition   | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#condition-object)                 | 觸發條件                                                                          |
| OrderObject | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#futoptconditionorder-object)    | 委託內容                                                                          |
| TPSLObject  | [FutOptTpslWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#futopttpslwrapper-object) | 停損停利條件                                                                      |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
// 設計條件內容
var condition = new Condition(
    TradingType.Reference,
    "TXO20000E4",
    TriggerContent.MatchedPrice,
    "100",
    Operator.LessThan
);

var order = new FutOptConditionOrder(
    BsAction.Buy,
    "TXO20000E4",
    "100",
    1,
    FutOptConditionMarketType.Option,
    FutOptConditionPriceType.Limit,
    TimeInForce.Rod,
    FutOptConditionOrderType.New
);
var tp = new FutOptTpslOrder(
    TimeInForce.Rod,
    FutOptConditionPriceType.Limit,
    FutOptConditionOrderType.Close,
    "120",
    "120",
    null  // 2.2.0 版本新增，暫無相關功能
);

var sl = new FutOptTpslOrder(
    TimeInForce.Rod,
    FutOptConditionPriceType.Limit,
    FutOptConditionOrderType.Close,
    "85",
    "85",
    null  // 2.2.0 版本新增，暫無相關功能
);

var tpsl = new FutOptTpslWrapper(
    StopSign.Full,
    tp,
    sl,
    "20240517",
    false
);


sdk.FutOpt.SingleCondition(account, "20240426", "20240430", StopSign.Full, condition, order, tpsl);


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = ConditionOrderResult {
            guid = 44150047-8977-40b1-953c-ce2XXXXXX,
        }
}


```


---

### 分時分量單查詢

GetTimeSliceOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                                                                      | 說明         |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------ |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                                                                       | 帳號         |
| guid       | string                                                                                                                                                                    | 分時分量單號 |
| marketType | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#futoptconditionmarkettype) (Optional) default : 日盤 | 盤別         |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | List                | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionDetail 欄位[​](#委託資訊-conditiondetail--欄位 "Direct link to 委託資訊 ConditionDetail  欄位")

Return type : Object

| 參數                     | 類別   | 說明             |
| ------------------------ | ------ | ---------------- |
| guid                     | string | 條件單號         |
| batchNo                  | string | 分單批號         |
| orderLevel               | string | 條件單層級       |
| lastTime                 | string | 更新時間         |
| parentGuid               | string | 父單單號         |
| symbol                   | string | 商品代號         |
| orderAmount              | string | 委託金額         |
| childBatchNo             | string | 子單分單批號     |
| account                  | string | 帳號             |
| conditionContent         | string | 條件內容         |
| action                   | string | 處理方式         |
| conditionBuySell         | string | 買賣別           |
| conditionSymbol          | string | 商品名稱         |
| conditionPrice           | string | 價格             |
| conditionVolume          | string | 委託數量         |
| conditionFilledVolume    | string | 成交數量         |
| createTime               | string | 建立時間         |
| startDate                | string | 預約開始時間     |
| status                   | string | 目前狀態         |
| errorMessage             | string | 目前狀態異常說明 |
| detailRecordsCount       | string | 查詢明細筆數     |
| detailRecords            | List   | 查詢明細資料     |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |
| tpslCount                | string | 停損停利筆數     |
| tpslRecord               | List   | 停損停利明細資料 |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
sdk.FutOpt.GetTimeSliceOrder(account,"123456",FutOptConditionMarketType.Future);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = [
            ConditionDetail{
                Guid = f76c49ee-e955-41ac-8e11-02d6d2b78918,
                BatchNo = ,
                OrderLevel = 0,
                LastTime = 2024-08-16 17:13:29,
                ConditionType = 定時,
                ParentGuid = ,
                Symbol = TXO20000H4,
                OrderAmount = 0,
                ChildBatchNo = ,
                Account = 9974825,
                ConditionContent = 當於2024/08/16 定時單時間大於等於09:15:00 全部成交為止,
                Action = 下單(新倉),
                ConditionBuySell = 買,
                ConditionSymbol = 買 台指2024年8 月買權履約價：20000,
                ConditionPrice = 500(ROD),
                ConditionVolume = 1,
                ConditionFilledVolume = 0,
                CreateTime = 2024-08-16 17:13:28,
                StartDate = 2024/08/16,
                Status = 預約(N),
                ErrorMessage = null,
                DetailRecordsCount = 0,
                DetailRecords = new List<object>(),
                TpslCount = 0,
                TpslRecord = new List<object>()
            }   
    ]       
}


```


---

### 分時分量條件單

TimeSliceOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數             | 類別                                                                                                                                | 說明                                                                              |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account          | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                                 | 帳號                                                                              |
| start\_date      | string                                                                                                                              | 條件開始監控時間                                                                  |
| end\_date        | string                                                                                                                              | 條件結束監控時間                                                                  |
| stop\_sign       | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#stopsign)                       | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| splitDescription | [SplitDescription](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#splitdescription)       | 分單條件                                                                          |
| OrderObject      | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#futoptconditionorder-object) | 委託內容                                                                          |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
// 設計條件內容
var split = new SplitDescription(
    TimeSliceOrderType.Type1,
     300,
     1,
     10,
     "083000",
     null
);


var order = new FutOptConditionOrder(
    BsAction.Buy,
    "TXO20000E4",
    "100",
    1,
    FutOptConditionMarketType.Option,
    FutOptConditionPriceType.Limit,
    TimeInForce.Rod,
    FutOptConditionOrderType.New
);

sdk.FutOpt.TimeSliceOrder(target_account, "20240427","20240516", StopSign.Full, split, order);


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = ConditionOrderResult {
            guid = 24081200000006
        }
}


```


---

### 歷史移動鎖利查詢

GetTrailHistory

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                                                            | 說明       |
| ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                                             | 帳號       |
| start\_date | string                                                                                                                                          | 查詢開始日 |
| end\_date   | string                                                                                                                                          | 查詢截止日 |
| marketType  | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#futoptconditionmarkettype) | 盤別       |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | list                | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionDetail 欄位[​](#委託資訊-conditiondetail--欄位 "Direct link to 委託資訊 ConditionDetail  欄位")

Return type : Object

| 參數                     | 類別   | 說明             |
| ------------------------ | ------ | ---------------- |
| guid                     | string | 條件單號         |
| batchNo                  | string | 分單批號         |
| orderLevel               | string | 條件單層級       |
| lastTime                 | string | 更新時間         |
| parentGuid               | string | 父單單號         |
| symbol                   | string | 商品代號         |
| orderAmount              | string | 委託金額         |
| childBatchNo             | string | 子單分單批號     |
| account                  | string | 帳號             |
| conditionContent         | string | 條件內容         |
| action                   | string | 處理方式         |
| conditionBuySell         | string | 買賣別           |
| conditionSymbol          | string | 商品名稱         |
| conditionPrice           | string | 價格             |
| conditionVolume          | string | 委託數量         |
| conditionFilledVolume    | string | 成交數量         |
| createTime               | string | 建立時間         |
| startDate                | string | 預約開始時間     |
| status                   | string | 目前狀態         |
| errorMessage             | string | 目前狀態異常說明 |
| detailRecordsCount       | string | 查詢明細筆數     |
| detailRecords            | List   | 查詢明細資料     |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |
| tpslCount                | string | 停損停利筆數     |
| tpslRecord               | List   | 停損停利明細資料 |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
sdk.FutOpt.GetTrailHistory(account,"20240301","20240601",FutOptConditionMarketType.Future);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = [
            {
                Guid = 1d97125b-9847-4b30-a066-2b490be17b2d,
                BatchNo = ,
                OrderLevel = 0,
                LastTime = 2024-09-15 10:33:33,
                ConditionType = 移動鎖利,
                ParentGuid = ,
                Symbol = FITX202411,
                OrderAmount = 0,
                ChildBatchNo = ,
                Account = 9974825,
                ConditionContent = 當自2024/09/15至2024/11/16內台指期202411成交價小於等於20000點 全部成交為止,
                Action = 下單(新倉),
                ConditionBuySell = 買,
                ConditionSymbol = 台指期202411 買,
                ConditionPrice = 賣出價(3) 檔(ROD),
                ConditionVolume = 1,
                ConditionFilledVolume = 0,
                CreateTime = 2024-09-15 10:33:33,
                StartDate = 2024/09/15,
                Status = 洗價中(Y),
                ErrorMessage = null,
                DetailRecordsCount = 0,
                DetailRecords = new List<object>(),
                TpslCount = 0,
                TpslRecord = new List<object>()
            },
            ...
    ]
}



```


---

### 移動鎖利查詢

GetTrailOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                                            | 說明 |
| ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | ---- |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                                             | 帳號 |
| marketType | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#futoptconditionmarkettype) | 盤別 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | List                | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionDetail 欄位[​](#委託資訊-conditiondetail--欄位 "Direct link to 委託資訊 ConditionDetail  欄位")

Return type : Object

| 參數                     | 類別   | 說明             |
| ------------------------ | ------ | ---------------- |
| guid                     | string | 條件單號         |
| batchNo                  | string | 分單批號         |
| orderLevel               | string | 條件單層級       |
| lastTime                 | string | 更新時間         |
| parentGuid               | string | 父單單號         |
| symbol                   | string | 商品代號         |
| orderAmount              | string | 委託金額         |
| childBatchNo             | string | 子單分單批號     |
| account                  | string | 帳號             |
| conditionContent         | string | 條件內容         |
| action                   | string | 處理方式         |
| conditionBuySell         | string | 買賣別           |
| conditionSymbol          | string | 商品名稱         |
| conditionPrice           | string | 價格             |
| conditionVolume          | string | 委託數量         |
| conditionFilledVolume    | string | 成交數量         |
| createTime               | string | 建立時間         |
| startDate                | string | 預約開始時間     |
| status                   | string | 目前狀態         |
| errorMessage             | string | 目前狀態異常說明 |
| detailRecordsCount       | string | 查詢明細筆數     |
| detailRecords            | List   | 查詢明細資料     |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |
| tpslCount                | string | 停損停利筆數     |
| tpslRecord               | List   | 停損停利明細資料 |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
sdk.FutOpt.GetTrailOrder(account, FutOptConditionMarketType.Future);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = [
            {
                Guid = 1d97125b-9847-4b30-a066-2b490be17b2d,
                BatchNo = ,
                OrderLevel = 0,
                LastTime = 2024-09-15 10:33:33,
                ConditionType = 移動鎖利,
                ParentGuid = ,
                Symbol = FITX202411,
                OrderAmount = 0,
                ChildBatchNo = ,
                Account = 9974825,
                ConditionContent = 當自2024/09/15至2024/11/16內台指期202411成交價小於等於20000點 全部成交為止,
                Action = 下單(新倉),
                ConditionBuySell = 買,
                ConditionSymbol = 台指期202411 買,
                ConditionPrice = 賣出價(3) 檔(ROD),
                ConditionVolume = 1,
                ConditionFilledVolume = 0,
                CreateTime = 2024-09-15 10:33:33,
                StartDate = 2024/09/15,
                Status = 洗價中(Y),
                ErrorMessage = null,
                DetailRecordsCount = 0,
                DetailRecords = new List<object>(),
                TpslCount = 0,
                TpslRecord = new List<object>()
            },
            ...
    ]
}


```


---

### 移動鎖利條件單

TrailProfit

caution

僅支援日盤交易

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數             | 類別                                                                                                                          | 說明                                                                             |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| account          | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                           | 帳號                                                                             |
| start\_date      | string                                                                                                                        | 條件開始監控時間                                                                 |
| end\_date        | string                                                                                                                        | 條件結束監控時間                                                                 |
| stop\_sign       | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#stopsign)                 | 條件停止條件 :`Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| FutOptTrailOrder | [FutOptTrailOrder](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#futopttrailorder) | 分單條件                                                                         |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
// 設計條件內容
var trail = new FutOptTrailOrder(
    "TXO20000E4",
    "100",
    Direction.Up ,
    5,  // 漲跌 tick 數
    BsAction.Buy,
    2,
    FutOptConditionPriceType.MatchedPrice,
    5, // 向上 or 向下追買 tick數 (向下為負值)
    TimeInForce.Rod,
    FutOptConditionOrderType.Close
);


sdk.FutOpt.TrailProfit(target_account, "20240427","20240516", StopSign.Full, trail);


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = ConditionOrderResult {
            guid = 44150047-8977-40b1-953c-ce2XXXXXX,
        }
}


```


---

### 取消條件單

cancelConditionOrders

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                                                                      | 說明     |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                                                                       | 帳號     |
| guid       | string                                                                                                                                                                    | 條件單號 |
| marketType | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#futoptconditionmarkettype) (Optional) default : 日盤 | 盤別     |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 CancelResult 欄位[​](#委託資訊-cancelresult--欄位 "Direct link to 委託資訊 CancelResult  欄位")

Return type : Object

| 參數     | 類別   | 說明         |
| -------- | ------ | ------------ |
| advisory | string | 回覆狀態內容 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
sdk.FutOpt.cancelConditionOrders(account, "c9df498a-3b28-4b50-a6f2-f7bd524e96df", )


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data :  {
              advisory: "成功筆數:1,失敗筆數:0!",
            }
 }



```


---

### 參數對照表

#### 類別[​](#類別 "Direct link to 類別")

Class

##### Condition Object[​](#condition-object "Direct link to Condition Object")

| Parameter    | Type           | Meaning                                                                                                                                 |
| ------------ | -------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| marketType   | TradingType    | [監控類型](#tradingtype) 可選用參數 : `Reference` 自動參考委託物件、`Scheduled` 時間                                                    |
| symbol       | string         | 股票代號                                                                                                                                |
| trigger      | TriggerContent | [觸發條件](#triggercontent) 可選用參數 : `BidPrice` 買進價、`AskPrice` 賣出價、`MatchedPrice` 成交價、`TotalQuantity` 總量、`Time` 時間 |
| triggerValue | string         | 監控條件數值 (若為時間則填入HHMMSS)                                                                                                     |
| comparison   | Operator       | [比對值](#operator) 可選用參數`GreaterThanOrEqual` 大於等於、`LessThanOrEqual`小於等於 、 `GreaterThan` 大於 、 `LessThan` 小於         |

info

1. 選擇 `Reference`時， trigger 可搭配 `BidPrice` 、`AskPrice` 、`MatchedPrice` 、`TotalQuantity`
2. 選擇`Scheduled` 時，symbol需填入欲觸發之商品，trigger需帶入`Time`

##### FutOptConditionOrder Object[​](#futoptconditionorder-object "Direct link to FutOptConditionOrder Object")

| Parameter   | Type                      | Meaning                                                                                                                                                                                                                         |
| ----------- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| buySell     | BSAction                  | [買賣別](#bsaction) 可選用參數`Buy` 買 、 `Sell` 賣                                                                                                                                                                             |
| symbol      | string                    | 股票代號                                                                                                                                                                                                                        |
| price       | string                    | 委託價格                                                                                                                                                                                                                        |
| quantity    | int                       | 委託數量                                                                                                                                                                                                                        |
| marketType  | FutOptConditionMarketType | [盤別](#futoptconditionmarkettype) 可選用參數 : `Future` 期貨日盤、`Option` 選擇權日盤、`FutureNight` 期貨夜盤、`OptionNight` 選擇權夜盤 (**IMPORTANT:** 以下功能不支援夜盤： 1. 分時分量, 2. 移動鎖利, 3. 時間觸發條件)        |
| priceType   | FutOptConditionPriceType  | [價格別](#futoptconditionpricetype) : 可選參數 :`BidPrice` 買進價、 `AskPrice` 賣出價、`MatchedPrice`成交價、`Limit` 限價、`LimitUp` 漲停、`LimitDown` 跌停、`Market` 市價、`Reference` 參考價(平盤價) 、`RangeMarket` 範圍市價 |
| timeInForce | TimeInforce               | [委託條件](#timeinforce) : 可選參數 :`ROD`、`IOC`、`FOK`                                                                                                                                                                        |
| orderType   | FutOptConditionOrderType  | [委託類別](#futoptconditionordertype) : 可選參數 :`New` 新倉、`Close` 平倉                                                                                                                                                      |

caution

以下功能不支援夜盤：

1. 分時分量
2. 移動鎖利
3. 時間觸發條件

##### FutOptTPSLOrder Object[​](#futopttpslorder-object "Direct link to FutOptTPSLOrder Object")

| Parameter              | Type                     | Meaning                                                                                                                                                                                                                         |
| ---------------------- | ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| timeInForce            | TimeInforce              | [委託條件](#timeinforce) : 可選參數 :`ROD`、`IOC`、`FOK`                                                                                                                                                                        |
| priceType              | FutOptConditionPriceType | [價格別](#futoptconditionpricetype) : 可選參數 :`BidPrice` 買進價、 `AskPrice` 賣出價、`MatchedPrice`成交價、`Limit` 限價、`LimitUp` 漲停、`LimitDown` 跌停、`Market` 市價、`Reference` 參考價(平盤價) 、`RangeMarket` 範圍市價 |
| orderType              | FutOptConditionOrderType | [委託類別](#futoptconditionordertype) : 可選參數 :`New` 新倉、`Close` 平倉                                                                                                                                                      |
| targetPrice            | string                   | 停損/停利觸發價                                                                                                                                                                                                                 |
| price                  | string                   | 停損/停利委託價                                                                                                                                                                                                                 |
| trigger (**保留欄位**) | TriggerContent           | 2.2.0 版本新增欄位，暫無相關功能，可忽略                                                                                                                                                                                        |

info

`priceType` 若為`limit`時，`price`需填入價格；其餘price填入空字串

##### FutOptTPSLWrapper Object[​](#futopttpslwrapper-object "Direct link to FutOptTPSLWrapper Object")

| Parameter | Type                                    | Meaning                                                                                           |
| --------- | --------------------------------------- | ------------------------------------------------------------------------------------------------- |
| stopSign  | StopSign                                | [停止條件](#stopsign) : 可選用 `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| TP        | FutOptTPSLOrder Object (Optional)       | 停利條件內容                                                                                      |
| SL        | FutOptTPSLOrder Object (Optional)       | 停損條件內容                                                                                      |
| endDate   | string (Optional : 空值同母單StartDate) | 條件停止日期                                                                                      |
| dayTrade  | bool (Optional)                         | 全部成交後當沖 (期貨固定為true)                                                                   |

##### FutOptTrailOrder[​](#futopttrailorder "Direct link to FutOptTrailOrder")

| Parameter   | Type                     | Meaning                                                                                                                                                                                                     |
| ----------- | ------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol      | string                   | 商品代號                                                                                                                                                                                                    |
| price       | string                   | 基準價                                                                                                                                                                                                      |
| direction   | Direction                | [方向](#direction) : 可選用 `Up` 上漲、`Down` 下跌                                                                                                                                                          |
| tickNum     | int                      | 上漲 / 下跌 ticker 數                                                                                                                                                                                       |
| buysell     | BSAction                 | 買賣別 : 可選用 `Buy` 買進 、`Sell` 賣出                                                                                                                                                                    |
| lot         | int                      | 委託口數                                                                                                                                                                                                    |
| PriceType   | FutOptConditionPriceType | [價格類別](#futoptconditionpricetype) : 可選用 `BidPrice` 買進價、`AskPrice` 賣出價、`MatchedPrice` 成交價、`Market` 市價、`LimitUp` 漲停價、`LimitDown` 跌停價、`Reference` 平盤價、`RangeMarket` 範圍市價 |
| diff        | int                      | 買賣價格檔數 (根據 priceType 加減檔數) ，正值為向上加檔數、負值為向下加檔數                                                                                                                                 |
| timeInForce | TimeInForce              | [委託條件](#timeinforce) 可選用參數為 `ROD`、`FOK`、`IOC`                                                                                                                                                   |
| orderType   | FutOptConditionOrderType | [委託類別](#futoptconditionordertype) 可選用參數為 `New` 新倉、`Close`平倉                                                                                                                                  |

##### SplitDescription[​](#splitdescription "Direct link to SplitDescription")

| Parameter      | Type              | Meaning                                                                                                                                                                                                                                                                                                                                                                                                      |
| -------------- | ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| method         | SliceType         | [Slice Condition](#slicetype) : 可選用 :<br />`Type1` 從 `startTime` 開始，每隔幾秒送一筆，總共`totalQuantity`，每筆送 `singleQuantity`<br />`Type2` 從 `startTime` 開始到 `endTime` 結束，每隔 `interval` 秒送一筆，總共 `totalQuantity`，剩餘張數加總至最後一筆<br />`Type3` 從 `startTime` 開始到 `endTime` 結束，每隔 `interval` 秒送一筆，總共 `totalQuantity`，剩餘張數從最後一筆往前分配.<br /><br /> |
| interval       | int               | 下單間隔時間                                                                                                                                                                                                                                                                                                                                                                                                 |
| singleQuantity | int               | 單筆下單量 (股數)                                                                                                                                                                                                                                                                                                                                                                                            |
| totalQuantity  | int (Optional)    | 總共下單量 (股數)                                                                                                                                                                                                                                                                                                                                                                                            |
| startTime      | string            | 開始時間                                                                                                                                                                                                                                                                                                                                                                                                     |
| endTime        | string (Optional) | 結束時間                                                                                                                                                                                                                                                                                                                                                                                                     |

caution

請注意，總共下單量需大於單筆下單量。

##### ConditionOrderResult[​](#conditionorderresult "Direct link to ConditionOrderResult")

| Parameter | Type   | Meaning  |
| --------- | ------ | -------- |
| guid      | string | 條件單號 |

#### Constants ( 欄位對應數值 )[​](#constants--欄位對應數值- "Direct link to Constants ( 欄位對應數值 )")

##### BsAction[​](#bsaction "Direct link to BsAction")

買賣別

| Name | Meaning |
| ---- | ------- |
| Buy  | 買      |
| Sell | 賣      |

##### TradingType[​](#tradingtype "Direct link to TradingType")

監控類型

| Name      | Meaning          |
| --------- | ---------------- |
| Reference | 自動參考委託物件 |
| Index     | 指數             |
| Scheduled | 時間             |

##### TriggerContent[​](#triggercontent "Direct link to TriggerContent")

觸發條件

| Name          | Meaning |
| ------------- | ------- |
| BidPrice      | 買進價  |
| AskPrice      | 賣出價  |
| MatchedPrice  | 成交價  |
| TotalQuantity | 總量    |
| Time          | 時間    |

##### Operator[​](#operator "Direct link to Operator")

| Name               | Meaning  |
| ------------------ | -------- |
| GreaterThanOrEqual | 大於等於 |
| LessThanOrEqual    | 小於等於 |
| GreaterThan        | 大於     |
| LessThan           | 小於     |

##### StopSign[​](#stopsign "Direct link to StopSign")

| Name     | Meaning      |
| -------- | ------------ |
| Full     | 全部成交為止 |
| Partial  | 部分成交為止 |
| UntilEnd | 效期結束為止 |

##### TimeInForce[​](#timeinforce "Direct link to TimeInForce")

委託條件 (TimeInForce)

| Name | Meaning                               |
| ---- | ------------------------------------- |
| ROD  | 當日有效(Rest of Day)                 |
| FOK  | 全部成交否則取消(Fill-or-Kill)        |
| IOC  | 立即成交否則取消(Immediate-or-Cancel) |

##### FutOptConditionMarketType[​](#futoptconditionmarkettype "Direct link to FutOptConditionMarketType")

委託類型 (FutOptConditionMarketType)

| Name   | Meaning    |
| ------ | ---------- |
| Future | 期貨日盤   |
| Option | 選擇權日盤 |

##### FutOptConditionPriceType[​](#futoptconditionpricetype "Direct link to FutOptConditionPriceType")

價格類型 (FutOptConditionPriceType)

| Name         | Meaning        |
| ------------ | -------------- |
| Limit        | 限價           |
| BidPrice     | 買進價         |
| AskPrice     | 賣出價         |
| Market       | 市價           |
| MatchedPrice | 成交價         |
| LimitUp      | 漲停價         |
| LimitDown    | 跌停價         |
| Reference    | 參考價(平盤價) |
| RangeMarket  | 範圍市價       |

##### FutOptConditionOrderType[​](#futoptconditionordertype "Direct link to FutOptConditionOrderType")

委託類型 (FutOptConditionOrderType)

| Name  | Meaning |
| ----- | ------- |
| New   | 新倉    |
| Close | 平倉    |

##### Direction[​](#direction "Direct link to Direction")

移動鎖利上漲/下跌追蹤 (direction)

| Name | Meaning |
| ---- | ------- |
| Up   | 上漲    |
| Down | 下跌    |

##### SliceType[​](#slicetype "Direct link to SliceType")

分單型態 (SliceType)

| Name  | Meaning                                                          |
| ----- | ---------------------------------------------------------------- |
| Type1 | 從開始時間，每隔幾秒送一筆，總共送N筆，每筆送M張                 |
| Type2 | 從開始到結束，每隔X秒送一筆，總共N張，剩餘張數加總至最後一筆     |
| Type3 | 從開始到結束，每隔X秒送一筆，總共N張，剩餘張數從最後一筆往前分配 |

##### ConditionStatus[​](#conditionstatus "Direct link to ConditionStatus")

條件單狀態 (ConditionStatus)

| Name   | Meaning      |
| ------ | ------------ |
| Type1  | 今日相關查詢 |
| Type2  | 尚有效單     |
| Type3  | 條件比對中   |
| Type4  | 委託處理中   |
| Type5  | 委託成功     |
| Type6  | 已通知       |
| Type7  | 委託失敗     |
| Type8  | 已有成交     |
| Type9  | 刪除成功     |
| Type10 | 異常         |
| Type11 | 失效         |

##### HistoryStatus[​](#historystatus "Direct link to HistoryStatus")

歷史條件單狀態 (HistoryStatus)

| Name  | Meaning                          |
| ----- | -------------------------------- |
| Type1 | 所有條件單 ( 不包含已刪除、失效) |
| Type2 | 選擇期間內全部成交單             |
| Type3 | 選擇期間內部分成交單             |
| Type4 | 選擇期間刪除單                   |
| Type5 | 選擇期間失效單                   |
| Type6 | 選擇期間內已觸發記錄             |


---

### 條件單查詢 By 條件單號

getConditionOrderById

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                                                                      | 說明     |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                                                                       | 帳號     |
| guid       | string                                                                                                                                                                    | 條件單號 |
| marketType | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#futoptconditionmarkettype) (Optional) default : 日盤 | 盤別     |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | List                | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess : false 回傳錯誤訊息 |

##### 委託資訊 ConditionDetail 欄位[​](#委託資訊-conditiondetail--欄位 "Direct link to 委託資訊 ConditionDetail  欄位")

Return type : Object

| 參數                     | 類別   | 說明             |
| ------------------------ | ------ | ---------------- |
| guid                     | string | 條件單號         |
| batchNo                  | string | 分單批號         |
| orderLevel               | string | 條件單層級       |
| lastTime                 | string | 更新時間         |
| parentGuid               | string | 父單單號         |
| symbol                   | string | 商品代號         |
| orderAmount              | string | 委託金額         |
| childBatchNo             | string | 子單分單批號     |
| account                  | string | 帳號             |
| conditionContent         | string | 條件內容         |
| action                   | string | 處理方式         |
| conditionBuySell         | string | 買賣別           |
| conditionSymbol          | string | 商品名稱         |
| conditionPrice           | string | 價格             |
| conditionVolume          | string | 委託數量         |
| conditionFilledVolume    | string | 成交數量         |
| createTime               | string | 建立時間         |
| startDate                | string | 預約開始時間     |
| status                   | string | 目前狀態         |
| errorMessage             | string | 目前狀態異常說明 |
| detailRecordsCount       | string | 查詢明細筆數     |
| detailRecords            | List   | 查詢明細資料     |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |
| tpslCount                | string | 停損停利筆數     |
| tpslRecord               | List   | 停損停利明細資料 |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
sdk.futopt.getConditionOrderById(account,"8ff3472b-185a-488c-be5a-b478deda080c")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data :
            {
                guid: "adada47e-dea3-4a5a-9eff-d36bd7a71e87",
                batchNo: "",
                orderLevel: "0",
                lastTime: "2024-11-15 10:38:48",
                conditionType: "觸價",
                parentGuid: "adada47e-dea3-4a5a-9eff-d36bd7a71e87",
                symbol: "FIMTX202411",
                orderAmount: "0",
                childBatchNo: "",
                account: "9974825",
                conditionContent: "當自2024/11/15至2024/11/20內台指期202412成交價大於21000點 全部成交為止",
                action: "下單(新倉)",
                conditionBuySell: "賣",
                conditionSymbol: "賣 小型台指202411",
                conditionPrice: "範圍市價(ROD)",
                conditionVolume: "2口",
                conditionFilledVolume: "0口",
                createTime: "2024-11-15 10:38:48",
                startDate: "2024/11/15",
                status: "洗價中(Y)",
                detailRecordsCount: "0",
                detailRecords: [],
                tpslCount: "2",
                tpslRecord: [
                    {
                        guid: "b79084fe-6fdf-461f-a264-0482072237eb",
                        account: "9974825",
                        conditionContent: "當自2024/11/15至2024/11/20內小型台指202411買進價大於等於22340元 全部成交為止",
                        action: "下單(平倉)",
                        conditionBuySell: "買",
                        conditionSymbol: "買 小型台指202411",
                        conditionPrice: "22340(ROD)",
                        conditionVolume: "2口",
                        conditionFilledVolume: "0口",
                        startDate: "2024/11/15",
                        status: "未生效(W)",
                        errorMessage: "",
                    },
                    {
                        guid: "b79084fe-6fdf-461f-a264-0482072237eb",
                        account: "9974825",
                        conditionContent: "當自2024/11/15至2024/11/20內小型台指202411買進價小於等於22280元 全部成交為止",
                        action: "下單(平倉)",
                        conditionBuySell: "買",
                        conditionSymbol: "買 小型台指202411",
                        conditionPrice: "22280(ROD)",
                        conditionVolume: "2口",
                        conditionFilledVolume: "0口",
                        startDate: "2024/11/15",
                        status: "未生效(W)",
                        errorMessage: "",
                    },
            ]
        }
    
}

```


---

### 歷史條件單查詢

getConditionHistory

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數            | 類別                                                                                                                                                                      | 說明           |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------- |
| account         | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                                                                       | 帳號           |
| start\_date     | string                                                                                                                                                                    | 查詢開始日     |
| end\_date       | string                                                                                                                                                                    | 查詢截止日     |
| marketType      | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#futoptconditionmarkettype) (Optional) default : 日盤 | 盤別           |
| history\_status | [HistoryStatus](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/csharp/EnumMatrix.txt#historystatus) (Optional)                                        | 歷史條件單狀態 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | list                | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess : false 回傳錯誤訊息 |

##### 委託資訊 ConditionDetail 欄位[​](#委託資訊-conditiondetail--欄位 "Direct link to 委託資訊 ConditionDetail  欄位")

Return type : Object

| 參數                     | 類別   | 說明             |
| ------------------------ | ------ | ---------------- |
| guid                     | string | 條件單號         |
| batchNo                  | string | 分單批號         |
| orderLevel               | string | 條件單層級       |
| lastTime                 | string | 更新時間         |
| parentGuid               | string | 父單單號         |
| symbol                   | string | 商品代號         |
| orderAmount              | string | 委託金額         |
| childBatchNo             | string | 子單分單批號     |
| account                  | string | 帳號             |
| conditionContent         | string | 條件內容         |
| action                   | string | 處理方式         |
| conditionBuySell         | string | 買賣別           |
| conditionSymbol          | string | 商品名稱         |
| conditionPrice           | string | 價格             |
| conditionVolume          | string | 委託數量         |
| conditionFilledVolume    | string | 成交數量         |
| createTime               | string | 建立時間         |
| startDate                | string | 預約開始時間     |
| status                   | string | 目前狀態         |
| errorMessage             | string | 目前狀態異常說明 |
| detailRecordsCount       | string | 查詢明細筆數     |
| detailRecords            | List   | 查詢明細資料     |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |
| tpslCount                | string | 停損停利筆數     |
| tpslRecord               | List   | 停損停利明細資料 |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
sdk.futopt.getConditionHistory(account,"20240301","20240601")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data :[
               {
                guid: "adada47e-dea3-4a5a-9eff-d36bd7a71e87",
                batchNo: "",
                orderLevel: "0",
                lastTime: "2024-11-15 10:38:48",
                conditionType: "觸價",
                parentGuid: "adada47e-dea3-4a5a-9eff-d36bd7a71e87",
                symbol: "FIMTX202411",
                orderAmount: "0",
                childBatchNo: "",
                account: "9974825",
                conditionContent: "當自2024/11/15至2024/11/20內台指期202412成交價大於21000點 全部成交為止",
                action: "下單(新倉)",
                conditionBuySell: "賣",
                conditionSymbol: "賣 小型台指202411",
                conditionPrice: "範圍市價(ROD)",
                conditionVolume: "2口",
                conditionFilledVolume: "0口",
                createTime: "2024-11-15 10:38:48",
                startDate: "2024/11/15",
                status: "洗價中(Y)",
                errorMessage: null,
                detailRecordsCount: "0",
                detailRecords: [],
                tpslCount: "2",
                tpslRecord: [
                    {
                        guid: "b79084fe-6fdf-461f-a264-0482072237eb",
                        account: "9974825",
                        conditionContent: "當自2024/11/15至2024/11/20內小型台指202411買進價大於等於22340元 全部成交為止",
                        action: "下單(平倉)",
                        conditionBuySell: "買",
                        conditionSymbol: "買 小型台指202411",
                        conditionPrice: "22340(ROD)",
                        conditionVolume: "2口",
                        conditionFilledVolume: "0口",
                        startDate: "2024/11/15",
                        status: "未生效(W)",
                        errorMessage: "",
                    },
                    {
                        guid: "b79084fe-6fdf-461f-a264-0482072237eb",
                        account: "9974825",
                        conditionContent: "當自2024/11/15至2024/11/20內小型台指202411買進價小於等於22280元 全部成交為止",
                        action: "下單(平倉)",
                        conditionBuySell: "買",
                        conditionSymbol: "買 小型台指202411",
                        conditionPrice: "22280(ROD)",
                        conditionVolume: "2口",
                        conditionFilledVolume: "0口",
                        startDate: "2024/11/15",
                        status: "未生效(W)",
                        errorMessage: "",
                    },
            ],
        },
        ...
    ]   
}


```


---

### 條件單查詢

getConditionOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數            | 類別                                                                                                                                                                      | 說明       |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| account         | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                                                                       | 帳號       |
| marketType      | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#futoptconditionmarkettype) (Optional) default : 日盤 | 盤別       |
| conditionStatus | [ConditionStatus](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#conditionstatus) (Optional)                                           | 條件單狀態 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | List                | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess : false 回傳錯誤訊息 |

##### 委託資訊 ConditionDetail 欄位[​](#委託資訊-conditiondetail--欄位 "Direct link to 委託資訊 ConditionDetail  欄位")

Return type : Object

| 參數                     | 類別   | 說明             |
| ------------------------ | ------ | ---------------- |
| guid                     | string | 條件單號         |
| batchNo                  | string | 分單批號         |
| orderLevel               | string | 條件單層級       |
| lastTime                 | string | 更新時間         |
| parentGuid               | string | 父單單號         |
| symbol                   | string | 商品代號         |
| orderAmount              | string | 委託金額         |
| childBatchNo             | string | 子單分單批號     |
| account                  | string | 帳號             |
| conditionContent         | string | 條件內容         |
| action                   | string | 處理方式         |
| conditionBuySell         | string | 買賣別           |
| conditionSymbol          | string | 商品名稱         |
| conditionPrice           | string | 價格             |
| conditionVolume          | string | 委託數量         |
| conditionFilledVolume    | string | 成交數量         |
| createTime               | string | 建立時間         |
| startDate                | string | 預約開始時間     |
| status                   | string | 目前狀態         |
| errorMessage             | string | 目前狀態異常說明 |
| detailRecordsCount       | string | 查詢明細筆數     |
| detailRecords            | List   | 查詢明細資料     |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |
| tpslCount                | string | 停損停利筆數     |
| tpslRecord               | List   | 停損停利明細資料 |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
sdk.futopt.getConditionOrder(account)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data :[
               {
                guid: "adada47e-dea3-4a5a-9eff-d36bd7a71e87",
                batchNo: "",
                orderLevel: "0",
                lastTime: "2024-11-15 10:38:48",
                conditionType: "觸價",
                parentGuid: "adada47e-dea3-4a5a-9eff-d36bd7a71e87",
                symbol: "FIMTX202411",
                orderAmount: "0",
                childBatchNo: "",
                account: "9974825",
                conditionContent: "當自2024/11/15至2024/11/20內台指期202412成交價大於21000點 全部成交為止",
                action: "下單(新倉)",
                conditionBuySell: "賣",
                conditionSymbol: "賣 小型台指202411",
                conditionPrice: "範圍市價(ROD)",
                conditionVolume: "2口",
                conditionFilledVolume: "0口",
                createTime: "2024-11-15 10:38:48",
                startDate: "2024/11/15",
                status: "洗價中(Y)",
                detailRecordsCount: "0",
                detailRecords: [],
                tpslCount: "2",
                tpslRecord: [
                    {
                        guid: "b79084fe-6fdf-461f-a264-0482072237eb",
                        account: "9974825",
                        conditionContent: "當自2024/11/15至2024/11/20內小型台指202411買進價大於等於22340元 全部成交為止",
                        action: "下單(平倉)",
                        conditionBuySell: "買",
                        conditionSymbol: "買 小型台指202411",
                        conditionPrice: "22340(ROD)",
                        conditionVolume: "2口",
                        conditionFilledVolume: "0口",
                        startDate: "2024/11/15",
                        status: "未生效(W)",
                        errorMessage: "",
                    },
                    {
                        guid: "b79084fe-6fdf-461f-a264-0482072237eb",
                        account: "9974825",
                        conditionContent: "當自2024/11/15至2024/11/20內小型台指202411買進價小於等於22280元 全部成交為止",
                        action: "下單(平倉)",
                        conditionBuySell: "買",
                        conditionSymbol: "買 小型台指202411",
                        conditionPrice: "22280(ROD)",
                        conditionVolume: "2口",
                        conditionFilledVolume: "0口",
                        startDate: "2024/11/15",
                        status: "未生效(W)",
                        errorMessage: "",
                    },
            ],
        },
        ...
    ]
}

```


---

### 多條件單

multiCondition

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數           | 類別                                                                                                                                | 說明                                                                             |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| account        | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                                 | 帳號                                                                             |
| start\_date    | string                                                                                                                              | 條件開始監控時間                                                                 |
| end\_date      | string                                                                                                                              | 條件結束監控時間                                                                 |
| stop\_sign     | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#stopsign)                       | 條件停止條件 :`Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| MultiCondition | [Condition List](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#condition-object)         | 觸發條件                                                                         |
| OrderObject    | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#futoptconditionorder-object) | 委託內容                                                                         |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
// 設計條件內容
const condition = {
    marketType: TradingType.Reference,
    symbol: "TXO20000E4",
    trigger: TriggerContent.MatchedPrice,
    triggerValue: "100",
    comparison: Operator.LessThan
}

const condition2 = {
    marketType: TradingType.Reference,
    symbol: "TXO20000E4",
    trigger: TriggerContent.TotalQuantity,
    triggerValue: "30",
    comparison: Operator.GreaterThan
}


const order = {
  buySell: BSAction.Buy,
  symbol: "TXO20000E4",
  price: "500",
  lot: 1,
  marketType: FutOptConditionMarketType.Option,
  priceType: FutOptConditionPriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: FutOptConditionOrderType.New,

};



sdk.futopt.multiCondition(account, "20240426", "20240430", StopSign.Full, [condition,condition2], order)


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data : {
            guid : "44150047-8977-40b1-953c-ce2XXXXXX"
    }
}


```


---

### 多條件單包含停損停利

multiCondition

停損利注意事項

* 停損利設定僅為觸發送單，不保證必定成交，需視市場狀況自行調整
* 請確認停損利**委託類別**設定需符合適合之交易規則

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數           | 類別                                                                                                                                   | 說明                                                                              |
| -------------- | -------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account        | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                                    | 帳號                                                                              |
| start\_date    | string                                                                                                                                 | 條件開始監控時間                                                                  |
| end\_date      | string                                                                                                                                 | 條件結束監控時間                                                                  |
| stop\_sign     | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#stopsign)                          | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| MultiCondition | [Condition List](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#condition-object)            | 觸發條件                                                                          |
| OrderObject    | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#futoptconditionorder-object)    | 委託內容                                                                          |
| TPSLObject     | [FutOptTPSLWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#futopttpslwrapper-object) | 停損停利條件                                                                      |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
// 設計條件內容
const condition = {
    marketType: TradingType.Reference,
    symbol: "TXO20000E4",
    trigger: TriggerContent.MatchedPrice,
    triggerValue: "100",
    comparison: Operator.LessThan
}

const condition2 = {
    marketType: TradingType.Reference,
    symbol: "TXO20000E4",
    trigger: TriggerContent.TotalQuantity,
    triggerValue: "30",
    comparison: Operator.GreaterThan
}

const order = {
  buySell: BSAction.Buy,
  symbol: "TXO20000E4",
  price: "100",
  lot: 1,
  marketType: FutOptConditionMarketType.Option,
  priceType: FutOptConditionPriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: FutOptConditionOrderType.New

};

const tp = {
    timeInForce: TimeInForce.ROD,
    priceType: FutOptConditionPriceType.Limit,
    orderType: FutOptConditionOrderType.Close,
    targetPrice: "120",
    price: "120"

}

const sl  = {
    timeInForce: TimeInForce.ROD,
    priceType: FutOptConditionPriceType.Limit,
    orderType: FutOptConditionOrderType.Close,
    targetPrice: "60",
    price: "60"
}

const tpsl = {
    stopSign: StopSign.Full,
    tp: tp,           // optional field 
    sl: sl,           // optional field
    endDate: "20240517" // optional field
}


sdk.futopt.multiCondition(account, "20240426", "20240430", StopSign.Full, [condition,condition2], order, tpsl)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data : {
            guid : "44150047-8977-40b1-953c-ce2XXXXXX"
    }
}


```


---

### 單一條件單

singleCondition

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                                                | 說明                                                                              |
| ----------- | ----------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                                 | 帳號                                                                              |
| start\_date | string                                                                                                                              | 條件開始監控時間                                                                  |
| end\_date   | string                                                                                                                              | 條件結束監控時間                                                                  |
| stop\_sign  | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#stopsign)                       | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| condition   | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#condition-object)              | 觸發條件                                                                          |
| OrderObject | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#futoptconditionorder-object) | 委託內容                                                                          |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
const condition = {
    marketType: TradingType.Reference,
    symbol: "TXO20000E4",
    trigger: TriggerContent.MatchedPrice,
    triggerValue: "100",
    comparison: Operator.LessThan
}


const order = {
  buySell: BSAction.Buy,
  symbol: "TXO20000E4",
  price: "100",
  lot: 1,
  marketType: FutOptConditionMarketType.Option,
  priceType: FutOptConditionPriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: FutOptConditionOrderType.New

};


sdk.futopt.singleCondition(account, "20240426", "20240430", StopSign.Full, condition, order)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data : {
            guid : "44150047-8977-40b1-953c-ce2XXXXXX"
    }
}


```


---

### 單一條件單包含停損停利

singleCondition

停損利注意事項

* 停損利設定僅為觸發送單，不保證必定成交，需視市場狀況自行調整
* 請確認停損利**委託類別**設定需符合適合之交易規則

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                                                   | 說明                                                                              |
| ----------- | -------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                                    | 帳號                                                                              |
| start\_date | string                                                                                                                                 | 條件開始監控時間                                                                  |
| end\_date   | string                                                                                                                                 | 條件結束監控時間                                                                  |
| stop\_sign  | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#stopsign)                          | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| condition   | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#condition-object)                 | 觸發條件                                                                          |
| OrderObject | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#futoptconditionorder-object)    | 委託內容                                                                          |
| TPSLObject  | [FutOptTPSLWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#futopttpslwrapper-object) | 停損停利條件                                                                      |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
// 設計條件內容
const condition = {
    marketType: TradingType.Reference,
    symbol: "TXO20000E4",
    trigger: TriggerContent.MatchedPrice,
    triggerValue: "100",
    comparison: Operator.LessThan
}

const order = {
  buySell: BSAction.Buy,
  symbol: "TXO20000E4",
  price: "100",
  lot: 1,
  marketType: FutOptConditionMarketType.Option,
  priceType: FutOptConditionPriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: FutOptConditionOrderType.New

};

const tp = {
    timeInForce: TimeInForce.ROD,
    priceType: FutOptConditionPriceType.Limit,
    orderType: FutOptConditionOrderType.Close,
    targetPrice: "120",
    price: "120"

}

const sl  = {
    timeInForce: TimeInForce.ROD,
    priceType: FutOptConditionPriceType.Limit,
    orderType: FutOptConditionOrderType.Close,
    targetPrice: "60",
    price: "60"
}

const tpsl = {
    stopSign: StopSign.Full,
    tp: tp,           // optional field 
    sl: sl,           // optional field
    endDate: "20240517" // optional field
}


sdk.futopt.singleCondition(account, "20240426", "20240430", StopSign.Full, condition, order, tpsl)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data : {
            guid : "44150047-8977-40b1-953c-ce2XXXXXX"
    }
}


```


---

### 分時分量查詢

getTimeSliceOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                                                                      | 說明             |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                                                                       | 帳號             |
| batchNo    | string                                                                                                                                                                    | 分時分量條件單號 |
| marketType | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#futoptconditionmarkettype) (Optional) default : 日盤 | 盤別             |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | List                | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess : false 回傳錯誤訊息 |

##### 委託資訊 ConditionDetail 欄位[​](#委託資訊-conditiondetail--欄位 "Direct link to 委託資訊 ConditionDetail  欄位")

Return type : Object

| 參數                     | 類別   | 說明             |
| ------------------------ | ------ | ---------------- |
| guid                     | string | 條件單號         |
| batchNo                  | string | 分單批號         |
| orderLevel               | string | 條件單層級       |
| lastTime                 | string | 更新時間         |
| parentGuid               | string | 父單單號         |
| symbol                   | string | 商品代號         |
| orderAmount              | string | 委託金額         |
| childBatchNo             | string | 子單分單批號     |
| account                  | string | 帳號             |
| conditionContent         | string | 條件內容         |
| action                   | string | 處理方式         |
| conditionBuySell         | string | 買賣別           |
| conditionSymbol          | string | 商品名稱         |
| conditionPrice           | string | 價格             |
| conditionVolume          | string | 委託數量         |
| conditionFilledVolume    | string | 成交數量         |
| createTime               | string | 建立時間         |
| startDate                | string | 預約開始時間     |
| status                   | string | 目前狀態         |
| errorMessage             | string | 目前狀態異常說明 |
| detailRecordsCount       | string | 查詢明細筆數     |
| detailRecords            | List   | 查詢明細資料     |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |
| tpslCount                | string | 停損停利筆數     |
| tpslRecord               | List   | 停損停利明細資料 |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
sdk.futopt.getTimeSliceOrder(account,"1234567")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data :
        [
            {
                guid: "f76c49ee-e955-41ac-8e11-02d6d2b78918",
                batchNo: "",
                orderLevel: "0",
                lastTime: "2024-08-16 17:13:29",
                conditionType: "定時",
                parentGuid: "",
                symbol: "TXO20000H4",
                orderAmount: "0",
                childBatchNo: "",
                account: "9974825",
                conditionContent: "當於2024/08/16 定時單時間大於等於09:15:00 全部成交為止",
                action: "下單(新倉)",
                conditionBuySell: "買",
                conditionSymbol: "買 台指2024年8 月買權履約價：20000",
                conditionPrice: "500(ROD)",
                conditionVolume: "1口",
                conditionFilledVolume: "0口",
                createTime: "2024-08-16 17:13:28",
                startDate: "2024/08/16",
                status: "預約(N)",
                detailRecordsCount: "0",
                detailRecords: [],
                tpslCount: "0",
                tpslRecord: []
            }
    ]
}

```


---

### 分時分量條件單

timeSliceOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數             | 類別                                                                                                                                | 說明                                                                              |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account          | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                                 | 帳號                                                                              |
| start\_date      | string                                                                                                                              | 條件開始監控時間                                                                  |
| end\_date        | string                                                                                                                              | 條件結束監控時間                                                                  |
| stop\_sign       | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#stopsign)                       | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| splitDescription | [SplitDescription](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#splitdescription)       | 觸發條件                                                                          |
| OrderObject      | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#futoptconditionorder-object) | 委託內容                                                                          |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
// 設計條件內容
const split = {
    method: TimeSliceOrderType.Type1,
    interval: 300,
    singleQuantity: 1,
    totalQuantity: 10,
    startTime: '083000',
};

const order = {
  buySell: BSAction.Buy,
  symbol: "TXO20000E4",
  price: "500",
  lot: 1,
  marketType: FutOptConditionMarketType.Option,
  priceType: FutOptConditionPriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: FutOptConditionOrderType.New,

};

sdk.futopt.timeSliceOrder(target_account, "20240427","20240516", StopSign.Full, split, order)


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data : {
            guid : "24081200000006"
    }
}


```


---

### 歷史移動鎖利查詢

getTrailHistory

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                                                            | 說明       |
| ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                                             | 帳號       |
| start\_date | string                                                                                                                                          | 查詢開始日 |
| end\_date   | string                                                                                                                                          | 查詢截止日 |
| marketType  | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#futoptconditionmarkettype) | 盤別       |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | list                | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess : false 回傳錯誤訊息 |

##### 委託資訊 ConditionDetail 欄位[​](#委託資訊-conditiondetail--欄位 "Direct link to 委託資訊 ConditionDetail  欄位")

Return type : Object

| 參數                     | 類別   | 說明             |
| ------------------------ | ------ | ---------------- |
| guid                     | string | 條件單號         |
| batchNo                  | string | 分單批號         |
| orderLevel               | string | 條件單層級       |
| lastTime                 | string | 更新時間         |
| parentGuid               | string | 父單單號         |
| symbol                   | string | 商品代號         |
| orderAmount              | string | 委託金額         |
| childBatchNo             | string | 子單分單批號     |
| account                  | string | 帳號             |
| conditionContent         | string | 條件內容         |
| action                   | string | 處理方式         |
| conditionBuySell         | string | 買賣別           |
| conditionSymbol          | string | 商品名稱         |
| conditionPrice           | string | 價格             |
| conditionVolume          | string | 委託數量         |
| conditionFilledVolume    | string | 成交數量         |
| createTime               | string | 建立時間         |
| startDate                | string | 預約開始時間     |
| status                   | string | 目前狀態         |
| errorMessage             | string | 目前狀態異常說明 |
| detailRecordsCount       | string | 查詢明細筆數     |
| detailRecords            | List   | 查詢明細資料     |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |
| tpslCount                | string | 停損停利筆數     |
| tpslRecord               | List   | 停損停利明細資料 |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
sdk.futopt.getTrailHistory(account,"20240301","20240601", FutOptConditioMarketType.Future)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data :[
            {
                guid: "1d97125b-9847-4b30-a066-2b490be17b2d",
                batchNo: "",
                orderLevel: "0",
                lastTime: "2024-09-15 10:33:33",
                conditionType: "移動鎖利",
                parentGuid: "",
                symbol: "FITX202411",
                orderAmount: "0",
                childBatchNo: "",
                account: "9974825",
                conditionContent: "當自2024/09/15至2024/11/16內台指期202411成交價小於等於20000點 全部成交為止",
                action: "下單(新倉)",
                conditionBuySell: "買",
                conditionSymbol: "台指期202411 買",
                conditionPrice: "賣出價(3) 檔(ROD)",
                conditionVolume: "1口",
                conditionFilledVolume: "0口",
                createTime: "2024-09-15 10:33:33",
                startDate: "2024/09/15",
                status: "洗價中(Y)",
                detailRecordsCount: "0",
                detailRecords: [],
                tpslCount: "0",
                tpslRecord: [],
            },
            ...
    ]
}

```


---

### 移動鎖利查詢

getTrailOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                                            | 說明 |
| ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | ---- |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                                             | 帳號 |
| marketType | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#futoptconditionmarkettype) | 盤別 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | List                | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess : false 回傳錯誤訊息 |

##### 委託資訊 ConditionDetail 欄位[​](#委託資訊-conditiondetail--欄位 "Direct link to 委託資訊 ConditionDetail  欄位")

Return type : Object

| 參數                     | 類別   | 說明             |
| ------------------------ | ------ | ---------------- |
| guid                     | string | 條件單號         |
| batchNo                  | string | 分單批號         |
| orderLevel               | string | 條件單層級       |
| lastTime                 | string | 更新時間         |
| parentGuid               | string | 父單單號         |
| symbol                   | string | 商品代號         |
| orderAmount              | string | 委託金額         |
| childBatchNo             | string | 子單分單批號     |
| account                  | string | 帳號             |
| conditionContent         | string | 條件內容         |
| action                   | string | 處理方式         |
| conditionBuySell         | string | 買賣別           |
| conditionSymbol          | string | 商品名稱         |
| conditionPrice           | string | 價格             |
| conditionVolume          | string | 委託數量         |
| conditionFilledVolume    | string | 成交數量         |
| createTime               | string | 建立時間         |
| startDate                | string | 預約開始時間     |
| status                   | string | 目前狀態         |
| errorMessage             | string | 目前狀態異常說明 |
| detailRecordsCount       | string | 查詢明細筆數     |
| detailRecords            | List   | 查詢明細資料     |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |
| tpslCount                | string | 停損停利筆數     |
| tpslRecord               | List   | 停損停利明細資料 |
| >> guid                  | string | 條件單號         |
| >> account               | string | 帳號             |
| >> conditionContent      | string | 條件內容         |
| >> action                | string | 處理方式         |
| >> conditionBuySell      | string | 買賣別           |
| >> conditionSymbol       | string | 商品名稱         |
| >> conditionPrice        | string | 價格             |
| >> conditionVolume       | string | 委託數量         |
| >> conditionFilledVolume | string | 成交數量         |
| >> startDate             | string | 預約時間         |
| >> status                | string | 目前狀態         |
| >> errorMessage          | string | 目前狀態異常說明 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
sdk.futopt.getTrailOrder(account, FutOptConditionMarketType.Future)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data :[
            {
                guid: "1d97125b-9847-4b30-a066-2b490be17b2d",
                batchNo: "",
                orderLevel: "0",
                lastTime: "2024-09-15 10:33:33",
                conditionType: "移動鎖利",
                parentGuid: "",
                symbol: "FITX202411",
                orderAmount: "0",
                childBatchNo: "",
                account: "9974825",
                conditionContent: "當自2024/09/15至2024/11/16內台指期202411成交價小於等於20000點 全部成交為止",
                action: "下單(新倉)",
                conditionBuySell: "買",
                conditionSymbol: "台指期202411 買",
                conditionPrice: "賣出價(3) 檔(ROD)",
                conditionVolume: "1口",
                conditionFilledVolume: "0口",
                createTime: "2024-09-15 10:33:33",
                startDate: "2024/09/15",
                status: "洗價中(Y)",
                detailRecordsCount: "0",
                detailRecords: [],
                tpslCount: "0",
                tpslRecord: [],
            },
            ...
    ]
}

```


---

### 移動鎖利條件單

caution

僅支援日盤交易

trailProfit

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數             | 類別                                                                                                                          | 說明                                                                              |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account          | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                           | 帳號                                                                              |
| startDate        | string                                                                                                                        | 條件開始監控時間                                                                  |
| endDate          | string                                                                                                                        | 條件結束監控時間                                                                  |
| stopSign         | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#stopsign)                 | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| futoptTrailOrder | [FutOptTrailOrder](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/nodejs/EnumMatrix.txt#futopttrailorder) | 移動鎖利條件                                                                      |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數        | 類別   | 說明         |
| ----------- | ------ | ------------ |
| guid        | string | 條件單號     |
| reply\_code | string | 回覆狀態代號 |
| advisory    | string | 回覆狀態內容 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
// 設計條件內容
const trail = {
    symbol: "TXO20000E4",
    price: "100",
    direction: Direction.Down,
    tickNum: 5,  
    buySell: BSAction.Sell,
    lot: 2,
    priceType: FutOptConditionPriceType.MatchedPrice,
    diff: 5,
    timeInForce: TimeInForce.ROD,
    orderType: FutOptConditionOrderType.Close
    
};


sdk.futopt.trailProfit(target_account, "20240427","20240516", StopSign.Full, trail)


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data : {
            guid : "44150047-8977-40b1-953c-ce2XXXXXX"
    }
}


```


---

### 取消條件單

cancel\_condition\_order

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                                                                      | 說明     |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                                                                       | 帳號     |
| guid       | string                                                                                                                                                                    | 條件單號 |
| marketType | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#futoptconditionmarkettype) (Optional) default : 日盤 | 盤別     |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 CancelResult 欄位[​](#委託資訊-cancelresult--欄位 "Direct link to 委託資訊 CancelResult  欄位")

Return type : Object

| 參數     | 類別   | 說明         |
| -------- | ------ | ------------ |
| advisory | string | 回覆狀態內容 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
sdk.futopt.cancel_condition_orders(account, "c9df498a-3b28-4b50-a6f2-f7bd524e96df")


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data : CancelResult {
             advisory: "成功筆數:1,失敗筆數:0!"
            }
    }


```


---

### 參數對照表

#### 類別[​](#類別 "Direct link to 類別")

Class

##### Condition Object[​](#condition-object "Direct link to Condition Object")

| Parameter      | Type           | Meaning                                                                                                                                 |
| -------------- | -------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| market\_type   | TradingType    | [監控類型](#tradingtype) 可選用參數 : `Reference` 自動參考委託物件、`Scheduled` 時間                                                    |
| symbol         | string         | 股票代號                                                                                                                                |
| trigger        | TriggerContent | [觸發條件](#triggercontent) 可選用參數 : `BidPrice` 買進價、`AskPrice` 賣出價、`MatchedPrice` 成交價、`TotalQuantity` 總量、`Time` 時間 |
| trigger\_value | string         | 監控條件數值 (若為時間則填入HHMMSS)                                                                                                     |
| comparison     | Operator       | [比對值](#operator) 可選用參數`GreaterThanOrEqual` 大於等於、`LessThanOrEqual`小於等於 、 `GreaterThan` 大於 、 `LessThan` 小於         |

info

1. 選擇 `Reference`時， trigger 可搭配 `BidPrice` 、`AskPrice` 、`MatchedPrice` 、`TotalQuantity`
2. 選擇`Scheduled` 時，symbol需填入欲觸發之商品，trigger需帶入`Time`

##### FutOptConditionOrder Object[​](#futoptconditionorder-object "Direct link to FutOptConditionOrder Object")

| Parameter       | Type                      | Meaning                                                                                                                                                                                                                         |
| --------------- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| buy\_sell       | BSAction                  | [買賣別](#bsaction) 可選用參數`Buy` 買 、 `Sell` 賣                                                                                                                                                                             |
| symbol          | string                    | 股票代號                                                                                                                                                                                                                        |
| price           | string                    | 委託價格                                                                                                                                                                                                                        |
| quantity        | int                       | 委託數量                                                                                                                                                                                                                        |
| market\_type    | FutOptConditionMarketType | [盤別](#futoptconditionmarkettype) 可選用參數 : `Future` 期貨日盤、`Option` 選擇權日盤、`FutureNight` 期貨夜盤、`OptionNight` 選擇權夜盤 (**IMPORTANT:** 以下功能不支援夜盤： 1. 分時分量, 2. 移動鎖利, 3. 時間觸發條件)        |
| price\_type     | FutOptConditionPriceType  | [價格別](#futoptconditionpricetype) : 可選參數 :`BidPrice` 買進價、 `AskPrice` 賣出價、`MatchedPrice`成交價、`Limit` 限價、`LimitUp` 漲停、`LimitDown` 跌停、`Market` 市價、`Reference` 參考價(平盤價) 、`RangeMarket` 範圍市價 |
| time\_in\_force | TimeInforce               | [委託條件](#timeinforce) : 可選參數 :`ROD`、`IOC`、`FOK`                                                                                                                                                                        |
| order\_type     | FutOptConditionOrderType  | [委託類別](#futoptconditionordertype) : 可選參數 :`New` 新倉、`Close` 平倉                                                                                                                                                      |

caution

以下功能不支援夜盤：

1. 分時分量
2. 移動鎖利
3. 時間觸發條件

##### FutOptTPSLOrder Object[​](#futopttpslorder-object "Direct link to FutOptTPSLOrder Object")

| Parameter              | Type                     | Meaning                                                                                                                                                                                                                         |
| ---------------------- | ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| time\_in\_force        | TimeInforce              | [委託條件](#timeinforce) : 可選參數 :`ROD`、`IOC`、`FOK`                                                                                                                                                                        |
| price\_type            | FutOptConditionPriceType | [價格別](#futoptconditionpricetype) : 可選參數 :`BidPrice` 買進價、 `AskPrice` 賣出價、`MatchedPrice`成交價、`Limit` 限價、`LimitUp` 漲停、`LimitDown` 跌停、`Market` 市價、`Reference` 參考價(平盤價) 、`RangeMarket` 範圍市價 |
| order\_type            | FutOptConditionOrderType | [委託類別](#futoptconditionordertype) : 可選參數 :`New` 新倉、`Close` 平倉                                                                                                                                                      |
| target\_price          | string                   | 停損/停利觸發價                                                                                                                                                                                                                 |
| price                  | string                   | 停損/停利委託價                                                                                                                                                                                                                 |
| trigger (**保留欄位**) | TriggerContent           | 2.2.0 版本新增欄位，暫無相關功能，可忽略                                                                                                                                                                                        |

info

`priceType` 若為`limit`時，`price`需填入價格；其餘price填入空字串

##### FutOptTPSLWrapper Object[​](#futopttpslwrapper-object "Direct link to FutOptTPSLWrapper Object")

| Parameter  | Type                                    | Meaning                                                                                           |
| ---------- | --------------------------------------- | ------------------------------------------------------------------------------------------------- |
| stop\_sign | StopSign                                | [停止條件](#stopsign) : 可選用 `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| TP         | FutOptTPSLOrder Object (Optional)       | 停利條件內容                                                                                      |
| SL         | FutOptTPSLOrder Object (Optional)       | 停損條件內容                                                                                      |
| end\_date  | string (Optional : 空值同母單StartDate) | 條件停止日期                                                                                      |
| day\_trade | bool (Optional)                         | 全部成交後當沖 (期貨固定為True)                                                                   |

##### FutOptTrailOrder[​](#futopttrailorder "Direct link to FutOptTrailOrder")

| Parameter       | Type                     | Meaning                                                                                                                                                                                                                         |
| --------------- | ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol          | string                   | 商品代號                                                                                                                                                                                                                        |
| price           | string                   | 基準價                                                                                                                                                                                                                          |
| direction       | Direction                | [方向](#direction) : 可選用 `Up` 上漲、`Down` 下跌                                                                                                                                                                              |
| tick\_num       | int                      | 上漲 / 下跌 ticker 數                                                                                                                                                                                                           |
| buysell         | BSAction                 | 買賣別 : 可選用 `Buy` 買進 、`Sell` 賣出                                                                                                                                                                                        |
| lot             | int                      | 委託口數                                                                                                                                                                                                                        |
| price\_type     | FutOptConditionPriceType | [價格別](#futoptconditionpricetype) : 可選參數 :`BidPrice` 買進價、 `AskPrice` 賣出價、`MatchedPrice`成交價、`Limit` 限價、`LimitUp` 漲停、`LimitDown` 跌停、`Market` 市價、`Reference` 參考價(平盤價) 、`RangeMarket` 範圍市價 |
| diff            | int                      | 買賣價格檔數 (根據 priceType 加減檔數) ，正值為向上加檔數、負值為向下加檔數                                                                                                                                                     |
| time\_in\_force | TimeInForce              | [委託條件](#timeinforce) 可選用參數為 `ROD`、`FOK`、`IOC`                                                                                                                                                                       |
| order\_type     | FutOptConditionOrderType | [委託類別](#futoptconditionordertype) : 可選參數 :`New` 新倉、`Close` 平倉                                                                                                                                                      |

##### SplitDescription[​](#splitdescription "Direct link to SplitDescription")

| Parameter        | Type              | Meaning                                                                                                                                                                                                                                                                                                                                                                                                               |
| ---------------- | ----------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| method           | SliceType         | [Slice Condition](#slicetype) : 可選用 :<br />`Type1` 從 `start_time` 開始，每隔幾秒送一筆，總共`total_quantity`，每筆送 `single_quantity`<br />`Type2` 從 `start_time` 開始到 `end_time` 結束，每隔 `interval` 秒送一筆，總共 `total_quantity`，剩餘張數加總至最後一筆<br />`Type3` 從 `start_time` 開始到 `end_time` 結束，每隔 `interval` 秒送一筆，總共 `total_quantity`，剩餘張數從最後一筆往前分配.<br /><br /> |
| interval         | int               | 下單間隔時間                                                                                                                                                                                                                                                                                                                                                                                                          |
| single\_quantity | int               | 單筆下單量 (股數)                                                                                                                                                                                                                                                                                                                                                                                                     |
| total\_quantity  | int (Optional)    | 總共下單量 (股數)                                                                                                                                                                                                                                                                                                                                                                                                     |
| start\_time      | string            | 開始時間                                                                                                                                                                                                                                                                                                                                                                                                              |
| end\_time        | string (Optional) | 結束時間                                                                                                                                                                                                                                                                                                                                                                                                              |

caution

請注意，總共下單量需大於單筆下單量。

##### ConditionOrderResult[​](#conditionorderresult "Direct link to ConditionOrderResult")

| Parameter | Type   | Meaning  |
| --------- | ------ | -------- |
| guid      | string | 條件單號 |

#### Constants ( 欄位對應數值 )[​](#constants--欄位對應數值- "Direct link to Constants ( 欄位對應數值 )")

##### BSAction[​](#bsaction "Direct link to BSAction")

買賣別 (buySell)

| Name | Meaning |
| ---- | ------- |
| Buy  | 買      |
| Sell | 賣      |

##### ConditionMarketType[​](#conditionmarkettype "Direct link to ConditionMarketType")

盤別

| Name        | Meaning  |
| ----------- | -------- |
| Common      | 整股     |
| Fixing      | 定盤     |
| IntradayOdd | 盤中零股 |
| Odd         | 盤後零股 |

##### TradingType[​](#tradingtype "Direct link to TradingType")

監控類型

| Name      | Meaning          |
| --------- | ---------------- |
| Reference | 自動參考委託物件 |
| Index     | 指數             |
| Scheduled | 時間             |

##### TriggerContent[​](#triggercontent "Direct link to TriggerContent")

觸發條件

| Name          | Meaning |
| ------------- | ------- |
| BidPrice      | 買進價  |
| AskPrice      | 賣出價  |
| MatchedPrice  | 成交價  |
| TotalQuantity | 總量    |
| Time          | 時間    |

##### Operator[​](#operator "Direct link to Operator")

| Name               | Meaning  |
| ------------------ | -------- |
| GreaterThanOrEqual | 大於等於 |
| LessThanOrEqual    | 小於等於 |
| GreaterThan        | 大於     |
| LessThan           | 小於     |

##### StopSign[​](#stopsign "Direct link to StopSign")

| Name     | Meaning      |
| -------- | ------------ |
| Full     | 全部成交為止 |
| Partial  | 部分成交為止 |
| UntilEnd | 效期結束為止 |

##### TimeInForce[​](#timeinforce "Direct link to TimeInForce")

委託條件 (TimeInForce)

| Name | Meaning                               |
| ---- | ------------------------------------- |
| ROD  | 當日有效(Rest of Day)                 |
| FOK  | 全部成交否則取消(Fill-or-Kill)        |
| IOC  | 立即成交否則取消(Immediate-or-Cancel) |

##### FutOptConditionMarketType[​](#futoptconditionmarkettype "Direct link to FutOptConditionMarketType")

委託類型 (FutOptConditionMarketType)

| Name   | Meaning    |
| ------ | ---------- |
| Future | 期貨日盤   |
| Option | 選擇權日盤 |

##### FutOptConditionPriceType[​](#futoptconditionpricetype "Direct link to FutOptConditionPriceType")

價格類型 (FutOptConditionPriceType)

| Name         | Meaning        |
| ------------ | -------------- |
| Limit        | 限價           |
| BidPrice     | 買進價         |
| AskPrice     | 賣出價         |
| Market       | 市價           |
| MatchedPrice | 成交價         |
| LimitUp      | 漲停價         |
| LimitDown    | 跌停價         |
| Reference    | 參考價(平盤價) |
| RangeMarket  | 範圍市價       |

##### FutOptConditionOrderType[​](#futoptconditionordertype "Direct link to FutOptConditionOrderType")

委託類型 (FutOptConditionOrderType)

| Name  | Meaning |
| ----- | ------- |
| New   | 新倉    |
| Close | 平倉    |

##### Direction[​](#direction "Direct link to Direction")

移動鎖利上漲/下跌追蹤 (direction)

| Name | Meaning |
| ---- | ------- |
| Up   | 上漲    |
| Down | 下跌    |

##### SliceType[​](#slicetype "Direct link to SliceType")

分單型態 (SliceType)

| Name  | Meaning                                                          |
| ----- | ---------------------------------------------------------------- |
| Type1 | 從開始時間，每隔幾秒送一筆，總共送N筆，每筆送M張                 |
| Type2 | 從開始到結束，每隔X秒送一筆，總共N張，剩餘張數加總至最後一筆     |
| Type3 | 從開始到結束，每隔X秒送一筆，總共N張，剩餘張數從最後一筆往前分配 |

##### ConditionStatus[​](#conditionstatus "Direct link to ConditionStatus")

條件單狀態 (ConditionStatus)

| Name   | Meaning      |
| ------ | ------------ |
| Type1  | 今日相關查詢 |
| Type2  | 尚有效單     |
| Type3  | 條件比對中   |
| Type4  | 委託處理中   |
| Type5  | 委託成功     |
| Type6  | 已通知       |
| Type7  | 委託失敗     |
| Type8  | 已有成交     |
| Type9  | 刪除成功     |
| Type10 | 異常         |
| Type11 | 失效         |

##### HistoryStatus[​](#historystatus "Direct link to HistoryStatus")

歷史條件單狀態 (HistoryStatus)

| Name  | Meaning                          |
| ----- | -------------------------------- |
| Type1 | 所有條件單 ( 不包含已刪除、失效) |
| Type2 | 選擇期間內全部成交單             |
| Type3 | 選擇期間內部分成交單             |
| Type4 | 選擇期間刪除單                   |
| Type5 | 選擇期間失效單                   |
| Type6 | 選擇期間內已觸發記錄             |


---

### 條件單查詢By Guid

get\_condition\_order\_by\_id

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                                                                      | 說明     |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                                                                       | 帳號     |
| guid       | string                                                                                                                                                                    | 條件單號 |
| marketType | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#futoptconditionmarkettype) (Optional) default : 日盤 | 盤別     |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | List                | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionDetail 欄位[​](#委託資訊-conditiondetail--欄位 "Direct link to 委託資訊 ConditionDetail  欄位")

Return type : Object

| 參數                         | 類別   | 說明             |
| ---------------------------- | ------ | ---------------- |
| guid                         | string | 條件單號         |
| batch\_no                    | string | 分單批號         |
| order\_level                 | string | 條件單層級       |
| last\_time                   | string | 更新時間         |
| parent\_guid                 | string | 父單單號         |
| symbol                       | string | 商品代號         |
| order\_amount                | string | 委託金額         |
| child\_batch\_no             | string | 子單分單批號     |
| account                      | string | 帳號             |
| condition\_content           | string | 條件內容         |
| action                       | string | 處理方式         |
| condition\_buy\_sell         | string | 買賣別           |
| condition\_symbol            | string | 商品名稱         |
| condition\_price             | string | 價格             |
| condition\_volume            | string | 委託數量         |
| condition\_filled\_volume    | string | 成交數量         |
| creat\_time                  | string | 建立時間         |
| start\_date                  | string | 預約開始時間     |
| status                       | string | 目前狀態         |
| error\_message               | string | 目前狀態異常說明 |
| detail\_records\_count       | string | 明細筆數         |
| detail\_records              | List   | 明細資料         |
| >> guid                      | string | 條件單號         |
| >> account                   | string | 帳號             |
| >> condition\_content        | string | 條件內容         |
| >> action                    | string | 處理方式         |
| >> condition\_buy\_sell      | string | 買賣別           |
| >> condition\_symbol         | string | 商品名稱         |
| >> condition\_price          | string | 價格             |
| >> condition\_volume         | string | 委託數量         |
| >> condition\_filled\_volume | string | 成交數量         |
| >> start\_date               | string | 預約時間         |
| >> status                    | string | 目前狀態         |
| >> error\_message            | string | 目前狀態異常說明 |
| tpslCount                    | string | 停損停利筆數     |
| tpslRecord                   | List   | 停損停利明細資料 |
| >> guid                      | string | 條件單號         |
| >> account                   | string | 帳號             |
| >> condition\_content        | string | 條件內容         |
| >> action                    | string | 處理方式         |
| >> condition\_buy\_sell      | string | 買賣別           |
| >> condition\_symbol         | string | 商品名稱         |
| >> condition\_price          | string | 價格             |
| >> condition\_volume         | string | 委託數量         |
| >> condition\_filled\_volume | string | 成交數量         |
| >> start\_date               | string | 預約時間         |
| >> status                    | string | 目前狀態         |
| >> error\_message            | string | 目前狀態異常說明 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
sdk.futopt.get_condition_order_by_id(account,"8ff3472b-185a-488c-be5a-b478deda080c")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data :[
        ConditionDetail {
            guid: "adada47e-dea3-4a5a-9eff-d36bd7a71e87",
            batch_no: "",
            order_level: "0",
            last_time: "2024-11-15 10:38:48",
            condition_type: "觸價",
            parent_guid: "adada47e-dea3-4a5a-9eff-d36bd7a71e87",
            symbol: "FIMTX202411",
            order_amount: "0",
            child_batch_no: "",
            account: "9974825",
            condition_content: "當自2024/11/15至2024/11/20內台指期202412成交價大於21000點 全部成交為止",
            action: "下單(新倉)",
            condition_buy_sell: "賣",
            condition_symbol: "賣 小型台指202411",
            condition_price: "範圍市價(ROD)",
            condition_volume: "2口",
            condition_filled_volume: "0口",
            create_time: "2024-11-15 10:38:48",
            start_date: "2024/11/15",
            status: "洗價中(Y)",
            error_message: None,
            detail_records_count: "0",
            detail_records: [],
            TPSLCount: "2",
            TPSLRecord: [
                ParentChildRecord {
                    guid: "b79084fe-6fdf-461f-a264-0482072237eb",
                    account: "9974825",
                    condition_content: "當自2024/11/15至2024/11/20內小型台指202411買進價大於等於22340元 全部成交為止",
                    action: "下單(平倉)",
                    condition_buy_sell: "買",
                    condition_symbol: "買 小型台指202411",
                    condition_price: "22340(ROD)",
                    condition_volume: "2口",
                    condition_filled_volume: "0口",
                    start_date: "2024/11/15",
                    status: "未生效(W)",
                    error_message: "",
                },
                ParentChildRecord {
                    guid: "b79084fe-6fdf-461f-a264-0482072237eb",
                    account: "9974825",
                    condition_content: "當自2024/11/15至2024/11/20內小型台指202411買進價小於等於22280元 全部成交為止",
                    action: "下單(平倉)",
                    condition_buy_sell: "買",
                    condition_symbol: "買 小型台指202411",
                    condition_price: "22280(ROD)",
                    condition_volume: "2口",
                    condition_filled_volume: "0口",
                    start_date: "2024/11/15",
                    status: "未生效(W)",
                    error_message: "",
                },
            ],
        }
    ]
}


```


---

### 歷史條件單查詢

get\_condition\_history

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數            | 類別                                                                                                                                                                      | 說明           |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------- |
| account         | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                                                                       | 帳號           |
| start\_date     | string                                                                                                                                                                    | 查詢開始日     |
| end\_date       | string                                                                                                                                                                    | 查詢截止日     |
| marketType      | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#futoptconditionmarkettype) (Optional) default : 日盤 | 盤別           |
| history\_status | [HistoryStatus](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#historystatus) (Optional)                                        | 歷史條件單狀態 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | list                | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionDetail 欄位[​](#委託資訊-conditiondetail--欄位 "Direct link to 委託資訊 ConditionDetail  欄位")

Return type : Object

| 參數                         | 類別   | 說明             |
| ---------------------------- | ------ | ---------------- |
| guid                         | string | 條件單號         |
| batch\_no                    | string | 分單批號         |
| order\_level                 | string | 條件單層級       |
| last\_time                   | string | 更新時間         |
| parent\_guid                 | string | 父單單號         |
| symbol                       | string | 商品代號         |
| order\_amount                | string | 委託金額         |
| child\_batch\_no             | string | 子單分單批號     |
| account                      | string | 帳號             |
| condition\_content           | string | 條件內容         |
| action                       | string | 處理方式         |
| condition\_buy\_sell         | string | 買賣別           |
| condition\_symbol            | string | 商品名稱         |
| condition\_price             | string | 價格             |
| condition\_volume            | string | 委託數量         |
| condition\_filled\_volume    | string | 成交數量         |
| creat\_time                  | string | 建立時間         |
| start\_date                  | string | 預約開始時間     |
| status                       | string | 目前狀態         |
| error\_message               | string | 目前狀態異常說明 |
| detail\_records\_count       | string | 查詢明細筆數     |
| detail\_records              | List   | 查詢明細資料     |
| >> guid                      | string | 條件單號         |
| >> account                   | string | 帳號             |
| >> condition\_content        | string | 條件內容         |
| >> action                    | string | 處理方式         |
| >> condition\_buy\_sell      | string | 買賣別           |
| >> condition\_symbol         | string | 商品名稱         |
| >> condition\_price          | string | 價格             |
| >> condition\_volume         | string | 委託數量         |
| >> condition\_filled\_volume | string | 成交數量         |
| >> start\_date               | string | 預約時間         |
| >> status                    | string | 目前狀態         |
| >> error\_message            | string | 目前狀態異常說明 |
| tpslCount                    | string | 停損停利筆數     |
| tpslRecord                   | List   | 停損停利明細資料 |
| >> guid                      | string | 條件單號         |
| >> account                   | string | 帳號             |
| >> condition\_content        | string | 條件內容         |
| >> action                    | string | 處理方式         |
| >> condition\_buy\_sell      | string | 買賣別           |
| >> condition\_symbol         | string | 商品名稱         |
| >> condition\_price          | string | 價格             |
| >> condition\_volume         | string | 委託數量         |
| >> condition\_filled\_volume | string | 成交數量         |
| >> start\_date               | string | 預約時間         |
| >> status                    | string | 目前狀態         |
| >> error\_message            | string | 目前狀態異常說明 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
sdk.futopt.get_condition_history(account,"20240310","20240601")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data :[
        ConditionDetail {
            guid: "1d97125b-9847-4b30-a066-2b490be17b2d",
            batch_no: "",
            order_level: "0",
            last_time: "2024-09-15 10:33:33",
            condition_type: "移動鎖利",
            parent_guid: "",
            symbol: "FITX202411",
            order_amount: "0",
            child_batch_no: "",
            account: "9974825",
            condition_content: "當自2024/09/15至2024/11/16內台指期202411成交價小於等於20000點 全部成交為止",
            action: "下單(新倉)",
            condition_buy_sell: "買",
            condition_symbol: "台指期202411 買",
            condition_price: "賣出價(3) 檔(ROD)",
            condition_volume: "1口",
            condition_filled_volume: "0口",
            create_time: "2024-09-15 10:33:33",
            start_date: "2024/09/15",
            status: "洗價中(Y)",
            error_message: None,
            detail_records_count: "0",
            detail_records: [],
            TPSLCount: "0",
            TPSLRecord: [],
        }
        ...
    ]
}

```


---

### 條件單查詢

get\_condition\_order

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數              | 類別                                                                                                                                                                      | 說明       |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| account           | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                                                                       | 帳號       |
| marketType        | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#futoptconditionmarkettype) (Optional) default : 日盤 | 盤別       |
| condition\_status | [ConditionStatus](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#conditionstatus) (Optional)                                           | 條件單狀態 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | List                | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionDetail 欄位[​](#委託資訊-conditiondetail--欄位 "Direct link to 委託資訊 ConditionDetail  欄位")

Return type : Object

| 參數                         | 類別   | 說明             |
| ---------------------------- | ------ | ---------------- |
| guid                         | string | 條件單號         |
| batch\_no                    | string | 分單批號         |
| order\_level                 | string | 條件單層級       |
| last\_time                   | string | 更新時間         |
| parent\_guid                 | string | 父單單號         |
| symbol                       | string | 商品代號         |
| order\_amount                | string | 委託金額         |
| child\_batch\_no             | string | 子單分單批號     |
| account                      | string | 帳號             |
| condition\_content           | string | 條件內容         |
| action                       | string | 處理方式         |
| condition\_buy\_sell         | string | 買賣別           |
| condition\_symbol            | string | 商品名稱         |
| condition\_price             | string | 價格             |
| condition\_volume            | string | 委託數量         |
| condition\_filled\_volume    | string | 成交數量         |
| creat\_time                  | string | 建立時間         |
| start\_date                  | string | 預約開始時間     |
| status                       | string | 目前狀態         |
| error\_message               | string | 目前狀態異常說明 |
| detail\_records\_count       | string | 明細筆數         |
| detail\_records              | List   | 明細資料         |
| >> guid                      | string | 條件單號         |
| >> account                   | string | 帳號             |
| >> condition\_content        | string | 條件內容         |
| >> action                    | string | 處理方式         |
| >> condition\_buy\_sell      | string | 買賣別           |
| >> condition\_symbol         | string | 商品名稱         |
| >> condition\_price          | string | 價格             |
| >> condition\_volume         | string | 委託數量         |
| >> condition\_filled\_volume | string | 成交數量         |
| >> start\_date               | string | 預約時間         |
| >> status                    | string | 目前狀態         |
| >> error\_message            | string | 目前狀態異常說明 |
| tpslCount                    | string | 停損停利筆數     |
| tpslRecord                   | List   | 停損停利明細資料 |
| >> guid                      | string | 條件單號         |
| >> account                   | string | 帳號             |
| >> condition\_content        | string | 條件內容         |
| >> action                    | string | 處理方式         |
| >> condition\_buy\_sell      | string | 買賣別           |
| >> condition\_symbol         | string | 商品名稱         |
| >> condition\_price          | string | 價格             |
| >> condition\_volume         | string | 委託數量         |
| >> condition\_filled\_volume | string | 成交數量         |
| >> start\_date               | string | 預約時間         |
| >> status                    | string | 目前狀態         |
| >> error\_message            | string | 目前狀態異常說明 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
sdk.futopt.get_condition_order(account)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data : [
         ConditionDetail {
            guid: "1d97125b-9847-4b30-a066-2b490be17b2d",
            batch_no: "",
            order_level: "0",
            last_time: "2024-09-15 10:33:33",
            condition_type: "移動鎖利",
            parent_guid: "",
            symbol: "FITX202411",
            order_amount: "0",
            child_batch_no: "",
            account: "9974825",
            condition_content: "當自2024/09/15至2024/11/16內台指期202411成交價小於等於20000點 全部成交為止",
            action: "下單(新倉)",
            condition_buy_sell: "買",
            condition_symbol: "台指期202411 買",
            condition_price: "賣出價(3) 檔(ROD)",
            condition_volume: "1口",
            condition_filled_volume: "0口",
            create_time: "2024-09-15 10:33:33",
            start_date: "2024/09/15",
            status: "洗價中(Y)",
            error_message: None,
            detail_records_count: "0",
            detail_records: [],
            TPSLCount: "0",
            TPSLRecord: [],
        }, 
        ...
    ]
}

```


---

### 多條件單

multi\_condition

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數           | 類別                                                                                                                                | 說明                                                                              |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account        | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                                 | 帳號                                                                              |
| start\_date    | string                                                                                                                              | 條件開始監控時間                                                                  |
| end\_date      | string                                                                                                                              | 條件結束監控時間                                                                  |
| stop\_sign     | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#stopsign)                       | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| MultiCondition | [Condition Array](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#condition-object)        | 觸發條件                                                                          |
| OrderObject    | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#futoptconditionorder-object) | 委託內容                                                                          |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
# 設計條件內容
condition = Condition(
    market_type = TradingType.Reference,        
    symbol = "TXO20000E4",
    trigger = TriggerContent.MatchedPrice,
    trigger_value = "100",
    comparison = Operator.LessThan
)

condition2 = Condition(
    market_type = TradingType.Reference,        
    symbol = "TXO20000E4",
    trigger = TriggerContent.TotalQuantity,
    trigger_value = "30",
    comparison = Operator.LessThan
)

order = FutOptConditionOrder(
    buy_sell = BSAction.Buy,
    symbol = "TXO20000E4",
    price =  "500",
    lot =  1,
    market_type = FutOptConditionMarketType.Option,
    price_type = FutOptConditionPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = FutOptConditionOrderType.New
)


sdk.futopt.multi_condition(account, "20240426", "20240430", StopSign.Full, [condition, condition2], order)


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data : ConditionOrderResult {
            guid : "44150047-8977-40b1-953c-ce270f0000"
        }
}


```


---

### 多條件單包含停損停利

multiCondition

停損利注意事項

* 停損利設定僅為觸發送單，不保證必定成交，需視市場狀況自行調整
* 請確認停損利**委託類別**設定需符合適合之交易規則

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數           | 類別                                                                                                                                   | 說明                                                                             |
| -------------- | -------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| account        | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                                    | 帳號                                                                             |
| start\_date    | string                                                                                                                                 | 條件開始監控時間                                                                 |
| end\_date      | string                                                                                                                                 | 條件結束監控時間                                                                 |
| stop\_sign     | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#stopsign)                          | 條件停止條件 :`Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| MultiCondition | [Condition List](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#condition-object)            | 觸發條件                                                                         |
| OrderObject    | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#futoptconditionorder-object)    | 委託內容                                                                         |
| TPSLObject     | [FutOptTPSLWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#futopttpslwrapper-object) | 停損停利條件                                                                     |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py

# 設計條件內容
condition = Condition(
    market_type = TradingType.Reference,        
    symbol = "TXO20000E4",
    trigger = TriggerContent.MatchedPrice,
    trigger_value = "100",
    comparison = Operator.LessThan
)

condition2 = Condition(
    market_type = TradingType.Reference,        
    symbol = "TXO20000E4",
    trigger = TriggerContent.TotalQuantity,
    trigger_value = "30",
    comparison = Operator.LessThan
)

order = FutOptConditionOrder(
    buy_sell = BSAction.Buy,
    symbol = "TXO20000E4",
    price =  "100",
    lot =  1,
    market_type = FutOptConditionMarketType.Option,
    price_type = FutOptConditionPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = FutOptConditionOrderType.New
)

tp = FutOptTPSLOrder(
    time_in_force=TimeInForce.ROD,
    price_type=FutOptConditionPriceType.Limit,
    order_type=FutOptConditionOrderType.Close,
    target_price="120",
    price="120"
)


sl = FutOptTPSLOrder(
    time_in_force=TimeInForce.ROD,
    price_type=FutOptConditionPriceType.Limit,
    order_type=FutOptConditionOrderType.Close,
    target_price="60",
    price="60"
)

tpsl = FutOptTPSLWrapper(
    stop_sign= StopSign.Full,
    tp=tp,           # optional field 
    sl=sl,           # optional field
    end_date="20240517" # optional field
)


sdk.futopt.multi_condition(account, "20240426", "20240430", StopSign.Full, [condition,condition2], order, tpsl)


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data : ConditionOrderResult {
            guid : "44150047-8977-40b1-953c-ce270f0000"
        }
}


```


---

### 單一條件單

single\_condition

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                                                | 說明                                                                              |
| ----------- | ----------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                                 | 帳號                                                                              |
| start\_date | string                                                                                                                              | 條件開始監控時間                                                                  |
| end\_date   | string                                                                                                                              | 條件結束監控時間                                                                  |
| stop\_sign  | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#stopsign)                       | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| condition   | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#condition-object)              | 觸發條件                                                                          |
| OrderObject | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#futoptconditionorder-object) | 委託內容                                                                          |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
# 設計條件內容
condition = Condition(
    market_type = TradingType.Reference,        
    symbol = "TXO20000E4",
    trigger = TriggerContent.MatchedPrice,
    trigger_value = "100",
    comparison = Operator.LessThan
)

order = FutOptConditionOrder(
    buy_sell = BSAction.Buy,
    symbol = "TXO20000E4",
    price =  "500",
    lot =  1,
    market_type = FutOptConditionMarketType.Option,
    price_type = FutOptConditionPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = FutOptConditionOrderType.New
)

res = sdk.futopt.single_condition(account, "20240427","20240516", StopSign.Full , condition, order)


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data : ConditionOrderResult {
            guid : "44150047-8977-40b1-953c-ce270f36150"
        }
}


```


---

### 單一條件單包含停損停利

single\_condition

停損利注意事項

* 停損利設定僅為觸發送單，不保證必定成交，需視市場狀況自行調整
* 請確認停損利**委託類別**設定需符合適合之交易規則

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                                                   | 說明                                                                              |
| ----------- | -------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                                    | 帳號                                                                              |
| start\_date | string                                                                                                                                 | 條件開始監控時間                                                                  |
| end\_date   | string                                                                                                                                 | 條件結束監控時間                                                                  |
| stop\_sign  | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#stopsign)                          | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| condition   | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#condition-object)                 | 觸發條件                                                                          |
| OrderObject | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#futoptconditionorder-object)    | 委託內容                                                                          |
| TPSLObject  | [FutOptTPSLWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#futopttpslwrapper-object) | 停損停利條件                                                                      |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
# 設計條件內容
condition = Condition(
    market_type = TradingType.Reference,        
    symbol = "TXO20000E4",
    trigger = TriggerContent.MatchedPrice,
    trigger_value = "100",
    comparison = Operator.LessThan
)

order = FutOptConditionOrder(
    buy_sell = BSAction.Buy,
    symbol = "TXO20000E4",
    price =  "100",
    lot =  1,
    market_type = FutOptConditionMarketType.Option,
    price_type = FutOptConditionPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = FutOptConditionOrderType.New
)

tp = FutOptTPSLOrder(
    time_in_force=TimeInForce.ROD,
    price_type=FutOptConditionPriceType.Limit,
    order_type=FutOptConditionOrderType.Close,
    target_price="85",
    price="85"
)


sl = FutOptTPSLOrder(
    time_in_force=TimeInForce.ROD,
    price_type=FutOptConditionPriceType.Limit,
    order_type=FutOptConditionOrderType.Close,
    target_price="60",
    price="60"
)

tpsl = FutOptTPSLWrapper(
    stop_sign= StopSign.Full,
    tp=tp,              # optional field 
    sl=sl,              # optional field
    end_date="20240517" # optional field
)


sdk.futopt.single_condition(account, "20240426", "20240430", StopSign.Full, condition, order, tpsl)


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data : ConditionOrderResult {
            guid : "44150047-8977-40b1-953c-ce2XXXXXX"
        }
}


```


---

### 分時分量查詢

get\_time\_slice\_order

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                                                                      | 說明             |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                                                                       | 帳號             |
| batch\_no  | string                                                                                                                                                                    | 分時分量條件單號 |
| marketType | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#futoptconditionmarkettype) (Optional) default : 日盤 | 盤別             |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | List                | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionDetail 欄位[​](#委託資訊-conditiondetail--欄位 "Direct link to 委託資訊 ConditionDetail  欄位")

Return type : Object

| 參數                         | 類別   | 說明             |
| ---------------------------- | ------ | ---------------- |
| guid                         | string | 條件單號         |
| batch\_no                    | string | 分單批號         |
| order\_level                 | string | 條件單層級       |
| last\_time                   | string | 更新時間         |
| parent\_guid                 | string | 父單單號         |
| symbol                       | string | 商品代號         |
| order\_amount                | string | 委託金額         |
| child\_batch\_no             | string | 子單分單批號     |
| account                      | string | 帳號             |
| condition\_content           | string | 條件內容         |
| action                       | string | 處理方式         |
| condition\_buy\_sell         | string | 買賣別           |
| condition\_symbol            | string | 商品名稱         |
| condition\_price             | string | 價格             |
| condition\_volume            | string | 委託數量         |
| condition\_filled\_volume    | string | 成交數量         |
| creat\_time                  | string | 建立時間         |
| start\_date                  | string | 預約開始時間     |
| status                       | string | 目前狀態         |
| error\_message               | string | 目前狀態異常說明 |
| detail\_records\_count       | string | 明細筆數         |
| detail\_records              | List   | 明細資料         |
| >> guid                      | string | 條件單號         |
| >> account                   | string | 帳號             |
| >> condition\_content        | string | 條件內容         |
| >> action                    | string | 處理方式         |
| >> condition\_buy\_sell      | string | 買賣別           |
| >> condition\_symbol         | string | 商品名稱         |
| >> condition\_price          | string | 價格             |
| >> condition\_volume         | string | 委託數量         |
| >> condition\_filled\_volume | string | 成交數量         |
| >> start\_date               | string | 預約時間         |
| >> status                    | string | 目前狀態         |
| >> error\_message            | string | 目前狀態異常說明 |
| tpslCount                    | string | 停損停利筆數     |
| tpslRecord                   | List   | 停損停利明細資料 |
| >> guid                      | string | 條件單號         |
| >> account                   | string | 帳號             |
| >> condition\_content        | string | 條件內容         |
| >> action                    | string | 處理方式         |
| >> condition\_buy\_sell      | string | 買賣別           |
| >> condition\_symbol         | string | 商品名稱         |
| >> condition\_price          | string | 價格             |
| >> condition\_volume         | string | 委託數量         |
| >> condition\_filled\_volume | string | 成交數量         |
| >> start\_date               | string | 預約時間         |
| >> status                    | string | 目前狀態         |
| >> error\_message            | string | 目前狀態異常說明 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
sdk.futopt.get_time_slice_order(account,"123456")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data :[
        ConditionDetail {
            guid: "f76c49ee-e955-41ac-8e11-02d6d2b78918",
            batch_no: "",
            order_level: "0",
            last_time: "2024-08-16 17:13:29",
            condition_type: "定時",
            parent_guid: "",
            symbol: "TXO20000H4",
            order_amount: "0",
            child_batch_no: "",
            account: "9974825",
            condition_content: "當於2024/08/16 定時單時間大於等於09:15:00 全部成交為止",
            action: "下單(新倉)",
            condition_buy_sell: "買",
            condition_symbol: "買 台指2024年8 月買權履約價：20000",
            condition_price: "500(ROD)",
            condition_volume: "1口",
            condition_filled_volume: "0口",
            create_time: "2024-08-16 17:13:28",
            start_date: "2024/08/16",
            status: "預約(N)",
            error_message: None,
            detail_records_count: "0",
            detail_records: [],
            TPSLCount: "0",
            TPSLRecord: [],
        }
    ]
}

```


---

### 分時分量條件單

time\_slice\_order

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數               | 類別                                                                                                                                | 說明                                                                              |
| ------------------ | ----------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account            | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                                 | 帳號                                                                              |
| start\_date        | string                                                                                                                              | 條件開始監控時間                                                                  |
| end\_date          | string                                                                                                                              | 條件結束監控時間                                                                  |
| stop\_sign         | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#stopsign)                       | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| split\_description | [SplitDescription](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#splitdescription)       | 觸發條件                                                                          |
| OrderObject        | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#futoptconditionorder-object) | 委託內容                                                                          |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
# 設計條件內容
split = SplitDescription(
    method=TimeSliceOrderType.Type1,
    interval= 300,
    single_quantity= 1,
    total_quantity= 10,
    start_time='083000'
)

order = FutOptConditionOrder(
    buy_sell = BSAction.Buy,
    symbol = "TXO20000E4",
    price =  "500",
    lot =  1,
    market_type = FutOptConditionMarketType.Option,
    price_type = FutOptConditionPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = FutOptConditionOrderType.New,
)


sdk.futopt.time_slice_order(target_account, "20240427","20240516", StopSign.Full, split, order)


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data : ConditionOrderResult {
            guid : "24081200000006"
        }
}


```


---

### 歷史移動鎖利查詢

get\_trail\_history

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                                                            | 說明       |
| ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                                             | 帳號       |
| start\_date | string                                                                                                                                          | 查詢開始日 |
| end\_date   | string                                                                                                                                          | 查詢截止日 |
| marketType  | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#futoptconditionmarkettype) | 盤別       |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | list                | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionDetail 欄位[​](#委託資訊-conditiondetail--欄位 "Direct link to 委託資訊 ConditionDetail  欄位")

Return type : Object

| 參數                         | 類別   | 說明             |
| ---------------------------- | ------ | ---------------- |
| guid                         | string | 條件單號         |
| batch\_no                    | string | 分單批號         |
| order\_level                 | string | 條件單層級       |
| last\_time                   | string | 更新時間         |
| parent\_guid                 | string | 父單單號         |
| symbol                       | string | 商品代號         |
| order\_amount                | string | 委託金額         |
| child\_batch\_no             | string | 子單分單批號     |
| account                      | string | 帳號             |
| condition\_content           | string | 條件內容         |
| action                       | string | 處理方式         |
| condition\_buy\_sell         | string | 買賣別           |
| condition\_symbol            | string | 商品名稱         |
| condition\_price             | string | 價格             |
| condition\_volume            | string | 委託數量         |
| condition\_filled\_volume    | string | 成交數量         |
| creat\_time                  | string | 建立時間         |
| start\_date                  | string | 預約開始時間     |
| status                       | string | 目前狀態         |
| error\_message               | string | 目前狀態異常說明 |
| detail\_records\_count       | string | 查詢明細筆數     |
| detail\_records              | List   | 查詢明細資料     |
| >> guid                      | string | 條件單號         |
| >> account                   | string | 帳號             |
| >> condition\_content        | string | 條件內容         |
| >> action                    | string | 處理方式         |
| >> condition\_buy\_sell      | string | 買賣別           |
| >> condition\_symbol         | string | 商品名稱         |
| >> condition\_price          | string | 價格             |
| >> condition\_volume         | string | 委託數量         |
| >> condition\_filled\_volume | string | 成交數量         |
| >> start\_date               | string | 預約時間         |
| >> status                    | string | 目前狀態         |
| >> error\_message            | string | 目前狀態異常說明 |
| tpslCount                    | string | 停損停利筆數     |
| tpslRecord                   | List   | 停損停利明細資料 |
| >> guid                      | string | 條件單號         |
| >> account                   | string | 帳號             |
| >> condition\_content        | string | 條件內容         |
| >> action                    | string | 處理方式         |
| >> condition\_buy\_sell      | string | 買賣別           |
| >> condition\_symbol         | string | 商品名稱         |
| >> condition\_price          | string | 價格             |
| >> condition\_volume         | string | 委託數量         |
| >> condition\_filled\_volume | string | 成交數量         |
| >> start\_date               | string | 預約時間         |
| >> status                    | string | 目前狀態         |
| >> error\_message            | string | 目前狀態異常說明 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
sdk.futopt.get_trail_history(account,"20240310","20240601")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data : [
       ConditionDetail {
            guid: "1d97125b-9847-4b30-a066-2b490be17b2d",
            batch_no: "",
            order_level: "0",
            last_time: "2024-09-15 10:33:33",
            condition_type: "移動鎖利",
            parent_guid: "",
            symbol: "FITX202411",
            order_amount: "0",
            child_batch_no: "",
            account: "9974825",
            condition_content: "當自2024/09/15至2024/11/16內台指期202411成交價小於等於20000點 全部成交為止",
            action: "下單(新倉)",
            condition_buy_sell: "買",
            condition_symbol: "台指期202411 買",
            condition_price: "賣出價(3) 檔(ROD)",
            condition_volume: "1口",
            condition_filled_volume: "0口",
            create_time: "2024-09-15 10:33:33",
            start_date: "2024/09/15",
            status: "洗價中(Y)",
            error_message: None,
            detail_records_count: "0",
            detail_records: [],
            TPSLCount: "0",
            TPSLRecord: [],
        },
        ...
    ]
    }

```


---

### 移動鎖利查詢

get\_trail\_order

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                                            | 說明 |
| ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | ---- |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                                             | 帳號 |
| marketType | [FutOptConditionMarketType](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#futoptconditionmarkettype) | 盤別 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | List                | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionDetail 欄位[​](#委託資訊-conditiondetail--欄位 "Direct link to 委託資訊 ConditionDetail  欄位")

Return type : Object

| 參數                         | 類別   | 說明             |
| ---------------------------- | ------ | ---------------- |
| guid                         | string | 條件單號         |
| batch\_no                    | string | 分單批號         |
| order\_level                 | string | 條件單層級       |
| last\_time                   | string | 更新時間         |
| parent\_guid                 | string | 父單單號         |
| symbol                       | string | 商品代號         |
| order\_amount                | string | 委託金額         |
| child\_batch\_no             | string | 子單分單批號     |
| account                      | string | 帳號             |
| condition\_content           | string | 條件內容         |
| action                       | string | 處理方式         |
| condition\_buy\_sell         | string | 買賣別           |
| condition\_symbol            | string | 商品名稱         |
| condition\_price             | string | 價格             |
| condition\_volume            | string | 委託數量         |
| condition\_filled\_volume    | string | 成交數量         |
| creat\_time                  | string | 建立時間         |
| start\_date                  | string | 預約開始時間     |
| status                       | string | 目前狀態         |
| error\_message               | string | 目前狀態異常說明 |
| detail\_records\_count       | string | 明細筆數         |
| detail\_records              | List   | 明細資料         |
| >> guid                      | string | 條件單號         |
| >> account                   | string | 帳號             |
| >> condition\_content        | string | 條件內容         |
| >> action                    | string | 處理方式         |
| >> condition\_buy\_sell      | string | 買賣別           |
| >> condition\_symbol         | string | 商品名稱         |
| >> condition\_price          | string | 價格             |
| >> condition\_volume         | string | 委託數量         |
| >> condition\_filled\_volume | string | 成交數量         |
| >> start\_date               | string | 預約時間         |
| >> status                    | string | 目前狀態         |
| >> error\_message            | string | 目前狀態異常說明 |
| tpslCount                    | string | 停損停利筆數     |
| tpslRecord                   | List   | 停損停利明細資料 |
| >> guid                      | string | 條件單號         |
| >> account                   | string | 帳號             |
| >> condition\_content        | string | 條件內容         |
| >> action                    | string | 處理方式         |
| >> condition\_buy\_sell      | string | 買賣別           |
| >> condition\_symbol         | string | 商品名稱         |
| >> condition\_price          | string | 價格             |
| >> condition\_volume         | string | 委託數量         |
| >> condition\_filled\_volume | string | 成交數量         |
| >> start\_date               | string | 預約時間         |
| >> status                    | string | 目前狀態         |
| >> error\_message            | string | 目前狀態異常說明 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
sdk.futopt.get_trail_order(account)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data : [
         ConditionDetail {
            guid: "1d97125b-9847-4b30-a066-2b490be17b2d",
            batch_no: "",
            order_level: "0",
            last_time: "2024-09-15 10:33:33",
            condition_type: "移動鎖利",
            parent_guid: "",
            symbol: "FITX202411",
            order_amount: "0",
            child_batch_no: "",
            account: "9974825",
            condition_content: "當自2024/09/15至2024/11/16內台指期202411成交價小於等於20000點 全部成交為止",
            action: "下單(新倉)",
            condition_buy_sell: "買",
            condition_symbol: "台指期202411 買",
            condition_price: "賣出價(3) 檔(ROD)",
            condition_volume: "1口",
            condition_filled_volume: "0口",
            create_time: "2024-09-15 10:33:33",
            start_date: "2024/09/15",
            status: "洗價中(Y)",
            error_message: None,
            detail_records_count: "0",
            detail_records: [],
            TPSLCount: "0",
            TPSLRecord: [],
        }
        ...
    ]
    
}

```


---

### 移動鎖利條件單

trail\_profit

caution

僅支援期貨日盤交易

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                   | 類別                                                                                                                          | 說明                                                                             |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| account                | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                           | 帳號                                                                             |
| start\_date            | string                                                                                                                        | 條件開始監控時間                                                                 |
| end\_date              | string                                                                                                                        | 條件結束監控時間                                                                 |
| stop\_sign             | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#stopsign)                 | 條件停止條件 :`Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| fut\_opt\_trail\_order | [FutOptTrailOrder](https://www.fbs.com.tw/TradeAPI/docs/smart-condition-future/library/python/EnumMatrix.txt#futopttrailorder) | 移動鎖利條件                                                                     |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
# 設計條件內容
trail = FutOptTrailOrder(
    symbol = "TXFL4",
    price = "20000",
    direction = Direction.Up ,
    tick_num = 5,  # 漲跌 tick 數
    buy_sell = BSAction.Buy,
    lot = 2,
    price_type = FutOptConditionPriceType.MatchedPrice,
    diff = 5,     # 向上 or 向下追買 tick數 (向下為負值)
    time_in_force = TimeInForce.ROD,
    order_type = FutOptConditionOrderType.Close
)


sdk.futopt.trail_profit(target_account, "20240427","20240516", StopSign.Full, trail)


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data : ConditionOrderResult {
            guid : "44150047-8977-40b1-953c-ce270f0000"
        }
}


```


---

