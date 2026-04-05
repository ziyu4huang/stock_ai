### 富邦新一代 API 條件單

本頁重點

* 本頁介紹富邦新一代 API 條件單功能與支援範圍。
* 提供多種條件單類型，適用自動化策略與風險管理。
* 下一步建議先完成[事前準備](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/prepare.txt)。

| 項目     | 說明                                                                       |
| -------- | -------------------------------------------------------------------------- |
| 產品     | 富邦新一代 API 條件單                                                      |
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

Python 版本支援：3.7（~v1.3.2）、3.8–3.13（v2.0.1~，不含 3.14）。 Node.js 版本支援 : 目前支援Node.js 16以上的版本。 C# 使用.NET Standard 2.0開發，建議您使用 .netcoreapp 3.1 以上；如使用.NETFramework 建議您使用.NETFramework 4.7.2以上版本。


---

### 取消條件單

cancel\_condition\_orders

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                      | 說明     |
| ------- | ----------------------------------------------------------------------------------------- | -------- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#account) | 帳號     |
| guid    | string                                                                                    | 條件單號 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別                | 說明                             |
| ----------- | ------------------- | -------------------------------- |
| is\_success | bool                | 是否成功                         |
| data        | Object              | 條件單回傳資訊                   |
| message     | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 CancelResult 欄位[​](#委託資訊-cancelresult--欄位 "Direct link to 委託資訊 CancelResult  欄位")

Return type : Object

| 參數     | 類別   | 說明         |
| -------- | ------ | ------------ |
| advisory | string | 回覆狀態內容 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cpp
auto cancel_condition = sdk->stock->cancel_condition_orders(target_account, "564b7ad6-a332-470c-93ea-cf3fea00d7fa");
if (!cancel_condition.is_success) {
    std::cout << "cancel condition result failed reason: "
        << (cancel_condition.message.has_value() ? cancel_condition.message.value() : "No message")
        << std::endl;
}
else {
    if (cancel_condition.data.has_value()) {
        for (const auto& result : cancel_condition.data.value()) {
            std::cout << result << std::endl;
        }
    }
    else {
        std::cout << "cancel condition success but no data returned." << std::endl;
    }
}


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cpp

Result {
    isSuccess = true,
    message = None,
    data = CancelResult {
             advisory = 成功筆數:1,失敗筆數:0!
            }
 }

```


---

### 當沖條件單查詢

get\_condition\_daytrade\_by\_id

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                      | 說明     |
| ------- | ----------------------------------------------------------------------------------------- | -------- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#account) | 帳號     |
| guid    | string                                                                                    | 條件單號 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別                | 說明                             |
| ----------- | ------------------- | -------------------------------- |
| is\_success | bool                | 是否成功                         |
| data        | List                | 條件單回傳資訊                   |
| message     | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

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
| create\_time                 | string | 建立時間         |
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
| tpsl\_count                  | string | 停損停利筆數     |
| tpsl\_record                 | List   | 停損停利明細資料 |
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

```cpp
auto get_daytrade_conditionid = sdk->stock->get_condition_daytrade_by_id(target_account, "564b7ad6-a332-470c-93ea-cf3fea00d7fa");
if (!get_daytrade_conditionid.is_success) {
    std::cout << "get condition result failed reason: "
        << (get_daytrade_conditionid.message.has_value() ? get_daytrade_conditionid.message.value() : "No message")
        << std::endl;
}
else {
    if (get_daytrade_conditionid.data.has_value()) {
        for (const auto& result : get_daytrade_conditionid.data.value()) {
            std::cout << result << std::endl;
        }
    }
    else {
        std::cout << "Condition result success but no data returned." << std::endl;
    }
}

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cpp

{
    isSuccess = true,
    message = ,
    data = 
            ConditionDetail {
                guid = 8ff3472b-185a-488c-be5a-b478deda080c,
                batchNo = ,
                orderLevel = 1,
                lastTime = 2024-03-14 12:39:02,
                conditionType = 多條件,
                parentGuid = 58e7a51e-9701-4d26-bc16-18a392a840bd,
                symbol = 2330,
                orderAmount = 0,
                childBatchNo = ,
                account = 1307515,
                conditionContent = 當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止,
                action = 下單(次日回補),
                conditionBuySell = 現股賣,
                conditionSymbol = 台積電 現股賣,
                conditionPrice = 580元(ROD),
                conditionVolume = 5張,
                conditionFilledVolume = 0張,
                createTime = 2024-03-14 12:39:22,
                startDate = 2024/03/14,
                status = 未生效(W),
                errorMessage = null,
                detailRecordsCount = 0,
                detailRecords = ,
                tPSLCount = 0,
                tPSLRecord = 
            }
            
}


```


---

### 當沖多條件單

multi\_conditions\_day\_trade

停損利注意事項

* 停損利設定僅為觸發送單，不保證必定回補成功，需視市場狀況自行調整
* 請確認停損利**委託類別**設定需符合當日沖銷交易規則（例如信用交易使用資券互抵）

當沖條件單查詢

* 使用 guid 查詢當沖條件單請使&#x7528;***當沖條件單查詢***&#x529F;能

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                     | 類別                                                                                                                                | 說明                                                                              |
| ------------------------ | ----------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account                  | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#account)                                           | 帳號                                                                              |
| start\_date              | string                                                                                                                              | 條件開始監控時間                                                                  |
| end\_date                | string                                                                                                                              | 條件結束監控時間                                                                  |
| stop\_sign               | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#stopsign)                                 | 條件停止條件 :`FULL` 全部成交為止、`PARTIAL`部分成交為止、`UNTIL_END`效期結束為止 |
| condition                | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#condition-object)                        | 觸發條件                                                                          |
| ConditionOrder Object    | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#conditionorder-object)       | 委託內容                                                                          |
| ConditionDayTrade Object | [ConditionDayTrade Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#conditiondaytrade-object) | 當沖委託內容                                                                      |
| TPSLObject               | [TpslWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#tpslwrapper-object)                    | 停損停利條件                                                                      |
| FixSession               | bool                                                                                                                                | 當沖回補                                                                          |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別                | 說明                             |
| ----------- | ------------------- | -------------------------------- |
| is\_success | bool                | 是否成功                         |
| data        | Object              | 條件單回傳資訊                   |
| message     | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cpp
// 設計條件內容
Condition condition = Condition{
    TradingType::REFERENCE,
    "2881",
    TriggerContent::MATCHED_PRICE,
    "66",
    Operator::LESS_THAN,
};

Condition condition2 = Condition{
    TradingType::REFERENCE,
    "2881",
    TriggerContent::TOTAL_QUANTITY,
    "1000",
    Operator::GREATER_THAN,
};

std::vector<Condition> conditions;
conditions.push_back(condition);
conditions.push_back(condition2);

ConditionOrder order = ConditionOrder{
    BsAction::BUY,
    "2330",
    "1000",
    1000,
    ConditionMarketType::COMMON,
    ConditionPriceType::LIMIT,
    TimeInForce::ROD,
    ConditionOrderType::STOCK,
};

ConditionDayTrade day_order = ConditionDayTrade{
    "131500",  // 收盤前沖銷時間，可設定區間為 130100 ~ 132000
    true,
    "",
    ConditionPriceType::MARKET
};

TpslOrder tp = TpslOrder{
    TimeInForce::ROD,
    ConditionPriceType::LIMIT,
    ConditionOrderType::STOCK,
    "85",
    "85",
};

TpslOrder sl = TpslOrder{
    TimeInForce::ROD,
    ConditionPriceType::LIMIT,
    ConditionOrderType::STOCK,
    "60",
    "60",
};

TPSLWrapper tpsl = TPSLWrapper{
    StopSign::FULL,
    tp,
    sl,
    "20250816",
    true  // ** 設定 true 以啟用當日沖銷洗價
};

auto daytrade_condition_order = sdk->stock->multi_condition_day_trade(target_account, StopSign::FULL, "130000", conditions , order, day_order, tpsl, true);
if (!daytrade_condition_order.is_success) {
    std::cout << "get order result failed reason: "
        << (daytrade_condition_order.message.has_value() ? daytrade_condition_order.message.value() : "No message")
        << std::endl;
}
else {
    if (daytrade_condition_order.data.has_value()) {
        const auto& send = daytrade_condition_order.data.value();
        std::cout << send << std::endl;
    }
    else {
        std::cout << "Order result success but no data returned." << std::endl;
    }
}

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cpp
{
    isSuccess = True,
    message = ,
    data = ConditionOrderResult {
            guid = 44150047-8977-40b1-953c-ce2XXXXXX
        }
}


```


---

### 當沖單一條件單

single\_condition\_day\_trade

停損利注意事項

* 停損利設定僅為觸發送單，不保證必定回補成功，需視市場狀況自行調整
* 請確認停損利**委託類別**設定需符合當日沖銷交易規則（例如信用交易使用資券互抵）

當沖條件單查詢

* 使用 guid 查詢當沖條件單請使&#x7528;***當沖條件單查詢***&#x529F;能

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                     | 類別                                                                                                                                | 說明                                                                              |
| ------------------------ | ----------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account                  | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#account)                                           | 帳號                                                                              |
| start\_date              | string                                                                                                                              | 條件開始監控時間                                                                  |
| end\_date                | string                                                                                                                              | 條件結束監控時間                                                                  |
| stop\_sign               | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#stopsign)                                 | 條件停止條件 :`FULL` 全部成交為止、`PARTIAL`部分成交為止、`UNTIL_END`效期結束為止 |
| condition                | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#condition-object)                        | 觸發條件                                                                          |
| ConditionOrder Object    | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#conditionorder-object)       | 委託內容                                                                          |
| ConditionDayTrade Object | [ConditionDayTrade Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#conditiondaytrade-object) | 當沖委託內容                                                                      |
| TPSLObject               | [TpslWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#tpslwrapper-object)                    | 停損停利條件                                                                      |
| FixSession               | bool                                                                                                                                | 當沖回補                                                                          |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別                | 說明                             |
| ----------- | ------------------- | -------------------------------- |
| is\_success | bool                | 是否成功                         |
| data        | Object              | 條件單回傳資訊                   |
| message     | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cpp
// 設計條件內容
Condition condition = Condition{
    TradingType::REFERENCE,
    "2881",
    TriggerContent::MATCHED_PRICE,
    "66",
    Operator::LESS_THAN,
};

ConditionOrder order = ConditionOrder{
    BsAction::BUY,
    "2330",
    "1000",
    1000,
    ConditionMarketType::COMMON,
    ConditionPriceType::LIMIT,
    TimeInForce::ROD,
    ConditionOrderType::STOCK,
};

ConditionDayTrade day_order = ConditionDayTrade{
    "131500",  // 收盤前沖銷時間，可設定區間為 130100 ~ 132000
    true,
    "",
    ConditionPriceType::MARKET
};

TpslOrder tp = TpslOrder{
    TimeInForce::ROD,
    ConditionPriceType::LIMIT,
    ConditionOrderType::STOCK,
    "85",
    "85",
};

TpslOrder sl = TpslOrder{
    TimeInForce::ROD,
    ConditionPriceType::LIMIT,
    ConditionOrderType::STOCK,
    "60",
    "60",
};

TPSLWrapper tpsl = TPSLWrapper{
    StopSign::FULL,
    tp,
    sl,
    "20250816",
    true  // ** 設定 true 以啟用當日沖銷洗價
};

auto daytrade_condition_order = sdk->stock->single_condition_day_trade(target_account, StopSign::FULL, "130000", condition , order, day_order, tpsl, true);
if (!daytrade_condition_order.is_success) {
    std::cout << "get order result failed reason: "
        << (daytrade_condition_order.message.has_value() ? daytrade_condition_order.message.value() : "No message")
        << std::endl;
}
else {
    if (daytrade_condition_order.data.has_value()) {
        const auto& send = daytrade_condition_order.data.value();
        std::cout << send << std::endl;
    }
    else {
        std::cout << "Order result success but no data returned." << std::endl;
    }
}

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cpp
{
    isSuccess = True,
    message = ,
    data = ConditionOrderResult {
            guid = 44150047-8977-40b1-953c-ce2XXXXXX
        }
}


```


---

### 參數對照表

#### 類別[​](#類別 "Direct link to 類別")

Class

##### Condition Object[​](#condition-object "Direct link to Condition Object")

| Parameter    | Type           | Meaning                                                                                                                                     |
| ------------ | -------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| marketType   | TradingType    | [監控類型](#tradingtype) 可選用參數 : `REFERENCE` 自動參考委託物件、`SCHEDULED` 時間                                                        |
| symbol       | string         | 股票代號                                                                                                                                    |
| trigger      | TriggerContent | [觸發條件](#triggercontent) 可選用參數 : `BID_PRICE` 買進價、`ASK_PRICE` 賣出價、`MATCHED_PRICE` 成交價、`TOTAL_QUANTITY` 總量、`TIME` 時間 |
| triggerValue | string         | 監控條件數值 (若為時間則填入HHMMSS)                                                                                                         |
| comparison   | Operator       | [比對值](#operator) 可選用參數`GREATER_THAN_OR_EQUAL` 大於等於、`LESS_THAN_OR_EQUAL`小於等於 、 `GREATER_THAN` 大於 、 `LESS_THAN` 小於     |

info

1. 選擇 `REFERENCE`時， trigger 可搭配 `BID_PRICE` 、`ASK_PRICE` 、`MATCHED_PRICE` 、`TOTAL_QUANTITY` 、 `TIME`，若 trigger 使用 `TIME`，則時間條件達成，且時間範圍內商品有交易產生才觸發
2. 選擇`SCHEDULED` 時，symbol需填入空值""，trigger需帶入`TIME`，則會使用系統時間判斷，當時間條件達成即觸發

##### ConditionOrder Object[​](#conditionorder-object "Direct link to ConditionOrder Object")

| Parameter   | Type                | Meaning                                                                                                                                                                                               |
| ----------- | ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| buySell     | BsAction            | [買賣別](#bsaction)                                                                                                                                                                                   |
| symbol      | string              | 股票代號                                                                                                                                                                                              |
| price       | string              | 委託價格                                                                                                                                                                                              |
| quantity    | int                 | 委託數量                                                                                                                                                                                              |
| marketType  | ConditionMarketType | [盤別](#conditionmarkettype) 可選用參數`COMMON` 整股、`FIXING`定盤、`INTRADAY_ODD` 盤中零股、`ODD` 盤後零股                                                                                           |
| priceType   | ConditionPriceType  | [價格別](#conditionpricetype) : 可選參數 :`BID_PRICE` 買進價、 `ASK_PRICE` 賣出價、`MATCHED_PRICE`成交價、`LIMIT` 限價、`LIMIT_UP` 漲停、`LIMIT_DOWN` 跌停、`MARKET` 市價、`REFERENCE` 參考價(平盤價) |
| timeInForce | TimeInForce         | [委託條件](#timeinforce) 可選用參數為 `ROD`、`FOK`、`IOC`                                                                                                                                             |
| orderType   | ConditionOrderType  | [委託類別](#conditionordertype) : 可選參數 :`STOCK`現貨、`MARGIN`融資、`SHORT`融券                                                                                                                    |

##### ConditionDayTrade Object[​](#conditiondaytrade-object "Direct link to ConditionDayTrade Object")

| Parameter       | Type               | Meaning                                                                                                                                                                                               |
| --------------- | ------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| dayTradeEndTime | string             | 收盤前沖銷時間，格式 "HHMMSS" (例 "130000")；可設定區間為 130100 ~ 132000                                                                                                                            |
| autoCancel      | bool               | 洗價結束後是否刪除已觸發委託                                                                                                                                                                          |
| price           | string             | 委託價格 (若無指定價格，例如市價單，請填空字串 "")                                                                                                                                                    |
| priceType       | ConditionPriceType | [價格別](#conditionpricetype) : 可選參數 :`BID_PRICE` 買進價、 `ASK_PRICE` 賣出價、`MATCHED_PRICE`成交價、`LIMIT` 限價、`LIMIT_UP` 漲停、`LIMIT_DOWN` 跌停、`MARKET` 市價、`REFERENCE` 參考價(平盤價) |

關於收盤前沖銷時間

可設定區間為 130100 ~ 132000

##### TpslOrder Object[​](#tpslorder-object "Direct link to TpslOrder Object")

| Parameter   | Type               | Meaning                                                                                                                                                                                               |
| ----------- | ------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| timeInForce | TimeInforce        | [委託條件](#timeinforce) : 可選參數 :`ROD`、`IOC`、`FOK`                                                                                                                                              |
| priceType   | ConditionPriceType | [價格別](#conditionpricetype) : 可選參數 :`BID_PRICE` 買進價、 `ASK_PRICE` 賣出價、`MATCHED_PRICE`成交價、`LIMIT` 限價、`LIMIT_UP` 漲停、`LIMIT_DOWN` 跌停、`MARKET` 市價、`REFERENCE` 參考價(平盤價) |
| orderType   | ConditionOrderType | [委託類別](#conditionordertype) : 可選參數 :`STOCK`現貨、`MARGIN`融資、`SHORT`融券                                                                                                                    |
| targetPrice | string             | 停損/停利觸發價                                                                                                                                                                                       |
| price       | string             | 停損/停利委託價                                                                                                                                                                                       |
| trigger     | TriggerContent     | 停損/停利[觸發條件](#triggercontent) （詳見表末說明）                                                                                                                                                 |

info

`priceType` 若為`LIMIT`時，`price`需填入價格；其餘price填入 std::nullopt

停損/停利觸發條件

2.2.0 以後版本新增（***2.1.1及以前版本無此欄位***），可設定停損/停利觸發價格別為 1. MATCHED\_PRICE 2. 最佳買價（BID\_PRICE）；3. 最佳賣價（ASK\_PRICE）

若未設定則此欄位填入 ***null***， 預設使用 成交價（MATCHED\_PRICE）

##### TpslWrapper Object[​](#tpslwrapper-object "Direct link to TpslWrapper Object")

| Parameter | Type                                    | Meaning                                                                                            |
| --------- | --------------------------------------- | -------------------------------------------------------------------------------------------------- |
| stopSign  | StopSign                                | [停止條件](#stopsign) : 可選用 `FULL` 全部成交為止、`PARTIAL`部分成交為止、`UNTIL_END`效期結束為止 |
| tp        | TpslOrder Object (Optional)             | 停利條件內容                                                                                       |
| sl        | TpslOrder Object (Optional)             | 停損條件內容                                                                                       |
| endDate   | string (Optional : 空值同母單StartDate) | 條件停止日期                                                                                       |
| intraday  | bool (Optional)                         | 全部成交後日內當沖 ( \* 配合當沖條件單使用時需設為 `true`)                                         |

日內當沖

使用當沖條件單時，`intraday` 需設定為 `true`

##### TrailOrder[​](#trailorder "Direct link to TrailOrder")

| Parameter   | Type               | Meaning                                                                                                                                                                                    |
| ----------- | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| symbol      | string             | 股票代號                                                                                                                                                                                   |
| price       | string             | 基準價 (Note. 只可輸入至多小數點後兩位，若超出將造成系統洗價失敗)                                                                                                                          |
| direction   | Direction          | [方向](#direction) : 可選用 `UP` 上漲、`DOWN` 下跌                                                                                                                                         |
| percentage  | int                | 上漲 / 下跌 % 數                                                                                                                                                                           |
| buysell     | BsAction           | 買賣別 : 可選用 `BUY` 買進 、`SELL` 賣出                                                                                                                                                   |
| quantity    | int                | 委託股數                                                                                                                                                                                   |
| PriceType   | ConditionPriceType | [價格類別](#conditionpricetype) : 可選用 `BID_PRICE` 買進價、`ASK_PRICE` 賣出價、`MATCHED_PRICE` 成交價、`MARKET` 市價、`LIMIT_UP` 漲停價、`LIMIT_DOWN` 跌停價、`REFERENCE` 參考價(平盤價) |
| diff        | int                | 委託買賣價格檔數 (根據 priceType 加減檔數) ，正值為向上加檔數、負值為向下加檔數                                                                                                            |
| timeInForce | TimeInForce        | [委託條件](#timeinforce) 可選用參數為 `ROD`、`FOK`、`IOC`                                                                                                                                  |
| orderType   | ConditionOrderType | [委託類別](#conditionordertype) 可選用參數為 `STOCK`現股、`MARGIN`融資、`SHORT`融券                                                                                                        |

##### SplitDescription[​](#splitdescription "Direct link to SplitDescription")

| Parameter      | Type              | Meaning                                                                                                                                                                                                                                                                                                                                                                                                      |
| -------------- | ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| method         | SliceType         | [Slice Condition](#slicetype) : 可選用 :<br />`TYPE1` 從 `startTime` 開始，每隔幾秒送一筆，總共`totalQuantity`，每筆送 `singleQuantity`<br />`TYPE2` 從 `startTime` 開始到 `endTime` 結束，每隔 `interval` 秒送一筆，總共 `totalQuantity`，剩餘張數加總至最後一筆<br />`TYPE3` 從 `startTime` 開始到 `endTime` 結束，每隔 `interval` 秒送一筆，總共 `totalQuantity`，剩餘張數從最後一筆往前分配.<br /><br /> |
| interval       | int               | 下單間隔時間 (秒)                                                                                                                                                                                                                                                                                                                                                                                            |
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

| Name          | Value | Meaning      |
| ------------- | ----- | ------------ |
| BUY           | 1     | 買           |
| SELL          | 2     | 賣           |
| UN\_SUPPORTED | 3     | 不支援此型別 |
| UN\_DEFINED   | 4     | 未定義此型別 |

##### ConditionMarketType[​](#conditionmarkettype "Direct link to ConditionMarketType")

盤別

| Name          | Value | Meaning  |
| ------------- | ----- | -------- |
| COMMON        | 1     | 一般盤   |
| FIXING        | 2     | 定盤     |
| INTRADAY\_ODD | 3     | 盤中零股 |
| ODD           | 4     | 盤後零股 |

##### TradingType[​](#tradingtype "Direct link to TradingType")

監控類型

| Name      | Value | Meaning          |
| --------- | ----- | ---------------- |
| REFERENCE | 1     | 自動參考委託物件 |
| SCHEDULED | 3     | 時間             |

##### TriggerContent[​](#triggercontent "Direct link to TriggerContent")

觸發條件

| Name            | Value | Meaning |
| --------------- | ----- | ------- |
| BID\_PRICE      | 1     | 買進價  |
| ASK\_PRICE      | 2     | 賣出價  |
| MATCHED\_PRICE  | 3     | 成交價  |
| TOTAL\_QUANTITY | 4     | 總量    |
| TIME            | 5     | 時間    |

##### Operator[​](#operator "Direct link to Operator")

| Name                     | Value | Meaning  |
| ------------------------ | ----- | -------- |
| GREATER\_THAN\_OR\_EQUAL | 1     | 大於等於 |
| LESS\_THAN\_OR\_EQUAL    | 2     | 小於等於 |
| GREATER\_THAN            | 3     | 大於     |
| LESS\_THAN               | 4     | 小於     |

##### StopSign[​](#stopsign "Direct link to StopSign")

| Name       | Value | Meaning      |
| ---------- | ----- | ------------ |
| FULL       | 1     | 全部成交為止 |
| PARTIAL    | 2     | 部分成交為止 |
| UNTIL\_END | 3     | 效期結束為止 |

##### TimeInForce[​](#timeinforce "Direct link to TimeInForce")

委託條件 (TimeInForce)

| Name          | Value | Meaning                               |
| ------------- | ----- | ------------------------------------- |
| ROD           | 1     | 當日有效(Rest of Day)                 |
| FOK           | 2     | 全部成交否則取消(Fill-or-Kill)        |
| IOC           | 3     | 立即成交否則取消(Immediate-or-Cancel) |
| UN\_SUPPORTED | 4     | 不支援此型別                          |
| UN\_DEFINED   | 5     | 未定義此型別                          |

##### ConditionPriceType[​](#conditionpricetype "Direct link to ConditionPriceType")

價格類型 (ConditionPriceType)

| Name           | Value | Meaning        |
| -------------- | ----- | -------------- |
| LINIT          | 1     | 限價           |
| BID\_PRICE     | 2     | 買進價         |
| ASK\_PRICE     | 3     | 賣出價         |
| MARKET         | 4     | 市價           |
| MATCHED\_PRICE | 5     | 成交價         |
| LIMIT\_UP      | 6     | 漲停價         |
| LIMIT\_DOWN    | 7     | 跌停價         |
| REFERENCE      | 8     | 參考價(平盤價) |

##### ConditionOrderType[​](#conditionordertype "Direct link to ConditionOrderType")

委託類型 (orderType)

| Name       | Value | Meaning  |
| ---------- | ----- | -------- |
| STOCK      | 1     | 現貨     |
| MARGIN     | 2     | 融資     |
| SHORT      | 3     | 融券     |
| DAY\_TRADE | 4     | 現股當沖 |

##### Direction[​](#direction "Direct link to Direction")

移動鎖利上漲/下跌追蹤 (direction)

| Name | Value | Meaning |
| ---- | ----- | ------- |
| Up   | 1     | 上漲    |
| Down | 2     | 下跌    |

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

get\_condition\_order\_by\_id

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                      | 說明     |
| ------- | ----------------------------------------------------------------------------------------- | -------- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#account) | 帳號     |
| guid    | string                                                                                    | 條件單號 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別                | 說明                             |
| ----------- | ------------------- | -------------------------------- |
| is\_success | bool                | 是否成功                         |
| data        | List                | 條件單回傳資訊                   |
| message     | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

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
| create\_time                 | string | 建立時間         |
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
| tpsl\_count                  | string | 停損停利筆數     |
| tpsl\_record                 | List   | 停損停利明細資料 |
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

```cpp
auto get_conditionid = sdk->stock->get_condition_order_by_id(target_account, "564b7ad6-a332-470c-93ea-cf3fea00d7fa");
if (!get_conditionid.is_success) {
    std::cout << "get condition result failed reason: "
        << (get_conditionid.message.has_value() ? get_conditionid.message.value() : "No message")
        << std::endl;
}
else {
    if (get_conditionid.data.has_value()) {
        for (const auto& result : get_conditionid.data.value()) {
            std::cout << result << std::endl;
        }
    }
    else {
        std::cout << "Condition result success but no data returned." << std::endl;
    }
}

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cpp

{
    isSuccess = true,
    message = ,
    data = 
            ConditionDetail {
                guid = 8ff3472b-185a-488c-be5a-b478deda080c,
                batchNo = ,
                orderLevel = 1,
                lastTime = 2024-03-14 12:39:02,
                conditionType = 多條件,
                parentGuid = 58e7a51e-9701-4d26-bc16-18a392a840bd,
                symbol = 2330,
                orderAmount = 0,
                childBatchNo = ,
                account = 1307515,
                conditionContent = 當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止,
                action = 下單(次日回補),
                conditionBuySell = 現股賣,
                conditionSymbol = 台積電 現股賣,
                conditionPrice = 580元(ROD),
                conditionVolume = 5張,
                conditionFilledVolume = 0張,
                createTime = 2024-03-14 12:39:22,
                startDate = 2024/03/14,
                status = 未生效(W),
                errorMessage = null,
                detailRecordsCount = 0,
                detailRecords = ,
                tPSLCount = 0,
                tPSLRecord = 
            }
            
}


```


---

### 歷史條件單查詢

GetConditionHistory

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                   | 類別                                                                                                                      | 說明                                                                                                                                                                                                        |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| account                | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#account)                                 | 帳號                                                                                                                                                                                                        |
| startDate              | string                                                                                                                    | 查詢條件單創建日                                                                                                                                                                                            |
| endDate                | string                                                                                                                    | 查詢截止日                                                                                                                                                                                                  |
| conditionHistoryStatus | [History Status](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#historystatus) (Optional) | 歷史條件單狀態 `Type1` 所有條件單 ( 不包含已刪除、失效) 、 `Type2` 選擇期間內全部成交單 、 `Type3` 選擇期間內部分成交單 、 `Type4` 選擇期間刪除單 、 `Type5` 選擇期間失效單 、 `Type6` 選擇期間內已觸發記錄 |

info

歷史資料日期以條件單創建日期為準

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
| create\_time                 | string | 建立時間         |
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
| tpsl\_count                  | string | 停損停利筆數     |
| tpsl\_record                 | List   | 停損停利明細資料 |
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

```cs
auto get_conditionid = sdk->stock->get_condition_history(target_account, "20250604","20250605", std::nullopt);
if (!get_conditionid.is_success) {
    std::cout << "get order result failed reason: "
        << (get_conditionid.message.has_value() ? get_conditionid.message.value() : "No message")
        << std::endl;
}
else {
    if (get_conditionid.data.has_value()) {
        for (const auto& result : get_conditionid.data.value()) {
            std::cout << result << std::endl;
        }
    }
    else {
        std::cout << "Order result success but no data returned." << std::endl;
    }
}

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cpp

{
    isSuccess = True,
    message = ,
    data = [
            ConditionDetail {
                guid = 8ff3472b-185a-488c-be5a-b478deda080c,
                batchNo = ,
                orderLevel = 1,
                lastTime = 2024-03-14 12:39:02,
                conditionType = 多條件,
                parentGuid = 58e7a51e-9701-4d26-bc16-18a392a840bd,
                symbol = 2330,
                orderAmount = 0,
                childBatchNo = ,
                account = 1307515,
                conditionContent = 當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止,
                action = 下單(次日回補),
                conditionBuySell = 現股賣,
                conditionSymbol = 台積電 現股賣,
                conditionPrice = 580元(ROD),
                conditionVolume = 5張,
                conditionFilledVolume = 0張,
                createTime = 2024-03-14 12:39:22,
                startDate = 2024/03/14,
                status = 未生效(W),
                errorMessage = null,
                detailRecordsCount = 0,
                detailRecords = ,
                tPSLCount = 0,
                tPSLRecord = 
            },
            ConditionDetail {
                guid = 8ff3472b-185a-488c-be5a-b478deda080c,
                batchNo = ,
                orderLevel = 1,
                lastTime = 2024-03-14 12:39:02,
                conditionType = 多條件,
                parentGuid = 58e7a51e-9701-4d26-bc16-18a392a840bd,
                symbol = 2330,
                orderAmount = 0,
                childBatchNo = ,
                account = 1307515,
                conditionContent = 當自2024/03/14至2024/07/04內台積電成交價小於等於570元 全部成交為止,
                action = 下單(次日回補),
                conditionBuySell = 現股賣,
                conditionSymbol = 台積電 現股賣,
                conditionPrice = 市價(ROD),
                conditionVolume = 5張,
                conditionFilledVolume = 0張,
                createTime = 2024-03-14 12:39:22,
                startDate = 2024/03/14,
                status = 未生效(W),
                errorMessage = null,
                detailRecordsCount = 0,
                detailRecords =,
                tPSLCount = 0,
                tPSLRecord = 
            },
            ConditionDetail {
                guid = ec757279-bcb3-46f4-80ac-fccfc786bc8d,
                batchNo = ,
                orderLevel = 0,
                lastTime = 2024-05-21 10:30:00,
                conditionType = 多條件,
                parentGuid = ec757279-bcb3-46f4-80ac-fccfc786bc8d,
                symbol = 2330,
                orderAmount = 0,
                childBatchNo = ,
                account = 1307515,
                conditionContent = 當自2024/03/14至2024/07/04內台積電成交價大於等於575元，且 台積電成交總量大於等於1001張，且 台積電時間大於等於10:30:00 部份成交為止,
                action = 下單(當沖),
                conditionBuySell = 現股買,
                conditionSymbol = 台積電 現股買,
                conditionPrice = 576元(ROD),
                conditionVolume = 5張,
                conditionFilledVolume = 0張,
                createTime = 2024-03-14 12:30:02,
                startDate = 2024/03/14,
                status = 觸發-委託失敗(X),
                errorMessage = 單價輸入錯誤[4385715],
                detailRecordsCount = 0,
                detailRecords = [],
                tPSLCount = 2,
                tPSLRecord = [
                    ParentChildRecord
                    {
                        guid = c94b552a-22da-43e4-be44-f27a9c2026e1,
                        account = 1307515,
                        conditionContent = 當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止,
                        action = 下單(次日回補),
                        conditionBuySell = 現股賣,
                        conditionSymbol = 台積電 現股賣,
                        conditionPrice = 580元(ROD),
                        conditionVolume = 5張,
                        conditionFilledVolume = 0張,
                        startDate = 2024/03/14,
                        status = 未生效(W),
                        errorMessage = 
                    },
                    ParentChildRecord
                    {
                        guid = c94b552a-22da-43e4-be44-f27a9c2026e1,
                        account = 1307515,
                        conditionContent = 當自2024/03/14至2024/07/04內台積電成交價小於等於570元 全部成交為止,
                        action = 下單(次日回補),
                        conditionBuySell = 現股賣,
                        conditionSymbol = 台積電 現股賣,
                        conditionPrice = 市價(ROD),
                        conditionVolume = 5張,
                        conditionFilledVolume = 0張,
                        startDate = 2024/03/14,
                        status = 未生效(W),
                        errorMessage = 
                    }
                ]
            }
    ]
}


```


---

### 條件單查詢

get\_condition\_order

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數            | 類別                                                                                                                         | 說明                                                                                                                                                                                                                               |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| account         | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#account)                                    | 帳號                                                                                                                                                                                                                               |
| conditionStatus | [ConditionStatus](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#conditionstatus) (Optional) | 條件單狀態 : `Type1` 今日相關查詢 、 `Type2` 尚有效單 、 `Type3` 條件比對中 、 `Type4` 委託處理中 、`Type5` 委託成功 、 `Type6` 已通知 、 `Type7` 委託失敗 、 `Type8` 已有成交 、`Type9` 刪除成功 、`Type10` 異常 、 `Type11` 失效 |

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
| create\_time                 | string | 建立時間         |
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
| tpsl\_count                  | string | 停損停利筆數     |
| tpsl\_record                 | List   | 停損停利明細資料 |
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

```cpp
auto get_conditionid = sdk->stock->get_condition_order(target_account, std::nullopt);
if (!get_conditionid.is_success) {
    std::cout << "get order result failed reason: "
        << (get_conditionid.message.has_value() ? get_conditionid.message.value() : "No message")
        << std::endl;
}
else {
    if (get_conditionid.data.has_value()) {
        for (const auto& result : get_conditionid.data.value()) {
            std::cout << result << std::endl;
        }
    }
    else {
        std::cout << "Order result success but no data returned." << std::endl;
    }
}

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cpp

{
    isSuccess = true,
    message = ,
    data = [
            ConditionDetail {
                guid = 8ff3472b-185a-488c-be5a-b478deda080c,
                batchNo = ,
                orderLevel = 1,
                lastTime = 2024-03-14 12:39:02,
                conditionType = 多條件,
                parentGuid = 58e7a51e-9701-4d26-bc16-18a392a840bd,
                symbol = 2330,
                orderAmount = 0,
                childBatchNo = ,
                account = 1307515,
                conditionContent = 當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止,
                action = 下單(次日回補),
                conditionBuySell = 現股賣,
                conditionSymbol = 台積電 現股賣,
                conditionPrice = 580元(ROD),
                conditionVolume = 5張,
                conditionFilledVolume = 0張,
                createTime = 2024-03-14 12:39:22,
                startDate = 2024/03/14,
                status = 未生效(W),
                errorMessage = null,
                detailRecordsCount = 0,
                detailRecords = ,
                tPSLCount = 0,
                tPSLRecord = 
            },
            ConditionDetail {
                guid = 8ff3472b-185a-488c-be5a-b478deda080c,
                batchNo = ,
                orderLevel = 1,
                lastTime = 2024-03-14 12:39:02,
                conditionType = 多條件,
                parentGuid = 58e7a51e-9701-4d26-bc16-18a392a840bd,
                symbol = 2330,
                orderAmount = 0,
                childBatchNo = ,
                account = 1307515,
                conditionContent = 當自2024/03/14至2024/07/04內台積電成交價小於等於570元 全部成交為止,
                action = 下單(次日回補),
                conditionBuySell = 現股賣,
                conditionSymbol = 台積電 現股賣,
                conditionPrice = 市價(ROD),
                conditionVolume = 5張,
                conditionFilledVolume = 0張,
                createTime = 2024-03-14 12:39:22,
                startDate = 2024/03/14,
                status = 未生效(W),
                errorMessage = null,
                detailRecordsCount = 0,
                detailRecords =,
                tPSLCount = 0,
                tPSLRecord = 
            },
            ConditionDetail {
                guid = ec757279-bcb3-46f4-80ac-fccfc786bc8d,
                batchNo = ,
                orderLevel = 0,
                lastTime = 2024-05-21 10:30:00,
                conditionType = 多條件,
                parentGuid = ec757279-bcb3-46f4-80ac-fccfc786bc8d,
                symbol = 2330,
                orderAmount = 0,
                childBatchNo = ,
                account = 1307515,
                conditionContent = 當自2024/03/14至2024/07/04內台積電成交價大於等於575元，且 台積電成交總量大於等於1001張，且 台積電時間大於等於10:30:00 部份成交為止,
                action = 下單(當沖),
                conditionBuySell = 現股買,
                conditionSymbol = 台積電 現股買,
                conditionPrice = 576元(ROD),
                conditionVolume = 5張,
                conditionFilledVolume = 0張,
                createTime = 2024-03-14 12:30:02,
                startDate = 2024/03/14,
                status = 觸發-委託失敗(X),
                errorMessage = 單價輸入錯誤[4385715],
                detailRecordsCount = 0,
                detailRecords = [],
                tPSLCount = 2,
                tPSLRecord = [
                    ParentChildRecord
                    {
                        guid = c94b552a-22da-43e4-be44-f27a9c2026e1,
                        account = 1307515,
                        conditionContent = 當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止,
                        action = 下單(次日回補),
                        conditionBuySell = 現股賣,
                        conditionSymbol = 台積電 現股賣,
                        conditionPrice = 580元(ROD),
                        conditionVolume = 5張,
                        conditionFilledVolume = 0張,
                        startDate = 2024/03/14,
                        status = 未生效(W),
                        errorMessage = 
                    },
                    ParentChildRecord
                    {
                        guid = c94b552a-22da-43e4-be44-f27a9c2026e1,
                        account = 1307515,
                        conditionContent = 當自2024/03/14至2024/07/04內台積電成交價小於等於570元 全部成交為止,
                        action = 下單(次日回補),
                        conditionBuySell = 現股賣,
                        conditionSymbol = 台積電 現股賣,
                        conditionPrice = 市價(ROD),
                        conditionVolume = 5張,
                        conditionFilledVolume = 0張,
                        startDate = 2024/03/14,
                        status = 未生效(W),
                        errorMessage = 
                    }
                ]
            }
    ]
}


```


---

### 多條件單

multi\_condition

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                  | 類別                                                                                                                          | 說明                                                                              |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| start\_date           | string                                                                                                                        | 條件開始監控時間                                                                  |
| end\_date             | string                                                                                                                        | 條件結束監控時間                                                                  |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#stopsign)                           | 條件停止條件 :`FULL` 全部成交為止、`PARTIAL`部分成交為止、`UNTIL_END`效期結束為止 |
| MultiCondition        | [Condition List](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#condition-object)             | 觸發條件                                                                          |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別                | 說明                             |
| ----------- | ------------------- | -------------------------------- |
| is\_success | bool                | 是否成功                         |
| data        | Object              | 條件單回傳資訊                   |
| message     | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cpp
// 設計條件內容
Condition condition = Condition{
    TradingType::REFERENCE,
    "2881",
    TriggerContent::MATCHED_PRICE,
    "66",
    Operator::LESS_THAN,
};

Condition condition2 = Condition{
    TradingType::REFERENCE,
    "2881",
    TriggerContent::TOTAL_QUANTITY,
    "1000",
    Operator::GREATER_THAN,
};

std::vector<Condition> conditions;
conditions.push_back(condition);
conditions.push_back(condition2);


ConditionOrder order = ConditionOrder{
    BsAction::BUY,
    "2330",
    "1000",
    1000,
    ConditionMarketType::COMMON,
    ConditionPriceType::LIMIT,
    TimeInForce::ROD,
    ConditionOrderType::STOCK,
};


auto send_condition_order = sdk->stock->multi_condition(target_account, "20250605", "20250605", StopSign::FULL, conditions, order);

if (!send_condition_order.is_success) {
    std::cout << "get order result failed reason: "
        << (send_condition_order.message.has_value() ? send_condition_order.message.value() : "No message")
        << std::endl;
}
else {
    if (send_condition_order.data.has_value()) {
        const auto& send = send_condition_order.data.value();
        std::cout << send << std::endl;
    }
    else {
        std::cout << "Order result success but no data returned." << std::endl;
    }
}

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cpp

{
    isSuccess = true,
    message = ,
    data = ConditionOrderResult {
            guid = 44150047-8977-40b1-953c-ce2XXXXXX,
        }
}


```


---

### 多條件單包含停損停利

multi\_condition

停損利注意事項

* 停損利設定僅為觸發送單，不保證必定成交，需視市場狀況自行調整
* 請確認停損利**委託類別**設定需符合適合之交易規則（例如信用交易資買資賣等）

info

待主單完全成交後，停損停利部分才會啟動

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                  | 類別                                                                                                                          | 說明                                                                              |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| start\_date           | string                                                                                                                        | 條件開始監控時間                                                                  |
| end\_date             | string                                                                                                                        | 條件結束監控時間                                                                  |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#stopsign)                           | 條件停止條件 :`FULL` 全部成交為止、`PARTIAL`部分成交為止、`UNTIL_END`效期結束為止 |
| MultiCondition        | [Condition List](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#condition-object)             | 觸發條件                                                                          |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |
| TPSLObject            | [TpslWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#tpslwrapper-object)              | 停損停利條件                                                                      |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別                | 說明                             |
| ----------- | ------------------- | -------------------------------- |
| is\_success | bool                | 是否成功                         |
| data        | Object              | 條件單回傳資訊                   |
| message     | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
// 設計條件內容
Condition condition = Condition{
    TradingType::REFERENCE,
    "2881",
    TriggerContent::MATCHED_PRICE,
    "66",
    Operator::LESS_THAN,
};

Condition condition2 = Condition{
    TradingType::REFERENCE,
    "2881",
    TriggerContent::TOTAL_QUANTITY,
    "1000",
    Operator::GREATER_THAN,
};

std::vector<Condition> conditions;
conditions.push_back(condition);
conditions.push_back(condition2);


ConditionOrder order = ConditionOrder{
    BsAction::BUY,
    "2330",
    "1000",
    1000,
    ConditionMarketType::COMMON,
    ConditionPriceType::LIMIT,
    TimeInForce::ROD,
    ConditionOrderType::STOCK,
};


TpslOrder tp = TpslOrder{
    TimeInForce::ROD,
    ConditionPriceType::LIMIT,
    ConditionOrderType::STOCK,
    "85",
    "85",
};

TpslOrder sl = TpslOrder{
    TimeInForce::ROD,
    ConditionPriceType::LIMIT,
    ConditionOrderType::STOCK,
    "60",
    "60",
};

TPSLWrapper tpsl = TPSLWrapper{
    StopSign::FULL,
    tp,
    sl,
    "20250605",
    false
};


auto send_condition_order = sdk->stock->multi_condition(target_account, "20250605", "20250605", StopSign::FULL, conditions, order, tpsl);

if (!send_condition_order.is_success) {
    std::cout << "get order result failed reason: "
        << (send_condition_order.message.has_value() ? send_condition_order.message.value() : "No message")
        << std::endl;
}
else {
    if (send_condition_order.data.has_value()) {
        const auto& send = send_condition_order.data.value();
        std::cout << send << std::endl;
    }
    else {
        std::cout << "Order result success but no data returned." << std::endl;
    }
}

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cpp

{
    isSuccess = true,
    message = ,
    data = ConditionOrderResult {
            guid = 44150047-8977-40b1-953c-ce2XXXXXX
        }
}


```


---

### 單一條件單

single\_condition

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                  | 類別                                                                                                                          | 說明                                                                              |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| start\_date           | string                                                                                                                        | 條件開始監控時間                                                                  |
| end\_date             | string                                                                                                                        | 條件結束監控時間                                                                  |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#stopsign)                           | 條件停止條件 :`FULL` 全部成交為止、`PARTIAL`部分成交為止、`UNTIL_END`效期結束為止 |
| condition             | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#condition-object)                  | 觸發條件                                                                          |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別                | 說明                             |
| ----------- | ------------------- | -------------------------------- |
| is\_success | bool                | 是否成功                         |
| data        | Object              | 條件單回傳資訊                   |
| message     | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cpp
// 設計條件內容
Condition condition = Condition{
    TradingType::REFERENCE,
    "2881",
    TriggerContent::MATCHED_PRICE,
    "66",
    Operator::LESS_THAN,
};

ConditionOrder order = ConditionOrder{
    BsAction::BUY,
    "2881",
    "1000",
    1000,
    ConditionMarketType::COMMON,
    ConditionPriceType::LIMIT,
    TimeInForce::ROD,
    ConditionOrderType::STOCK,
};


auto send_condition_order = sdk->stock->single_condition(target_account,"20250605","20250605", StopSign::FULL, condition, order, std::nullopt);

if (!send_condition_order.is_success) {
    std::cout << "get order result failed reason: "
        << (send_condition_order.message.has_value() ? send_condition_order.message.value() : "No message")
        << std::endl;
}
else {
    if (send_condition_order.data.has_value()) {
        const auto& send = send_condition_order.data.value();
        std::cout << send << std::endl;
    }
    else {
        std::cout << "Order result success but no data returned." << std::endl;
    }
}


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cpp

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

single\_condition

停損利注意事項

* 停損利設定僅為觸發送單，不保證必定成交，需視市場狀況自行調整
* 請確認停損利**委託類別**設定需符合適合之交易規則（例如信用交易資買資賣等）

info

待主單完全成交後，停損停利部分才會啟動

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                  | 類別                                                                                                                          | 說明                                                                              |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| start\_date           | string                                                                                                                        | 條件開始監控時間                                                                  |
| end\_date             | string                                                                                                                        | 條件結束監控時間                                                                  |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#stopsign)                           | 條件停止條件 :`FULL` 全部成交為止、`PARTIAL`部分成交為止、`UNTIL_END`效期結束為止 |
| condition             | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#condition-object)                  | 觸發條件                                                                          |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |
| TPSLObject            | [TpslWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#tpslwrapper-object)              | 停損停利條件                                                                      |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別                | 說明                             |
| ----------- | ------------------- | -------------------------------- |
| is\_success | bool                | 是否成功                         |
| data        | Object              | 條件單回傳資訊                   |
| message     | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cpp
// 設計條件內容
Condition condition = Condition{
    TradingType::REFERENCE,
    "2881",
    TriggerContent::MATCHED_PRICE,
    "66",
    Operator::LESS_THAN,
};

ConditionOrder order = ConditionOrder{
    BsAction::BUY,
    "2330",
    "1000",
    1000,
    ConditionMarketType::COMMON,
    ConditionPriceType::LIMIT,
    TimeInForce::ROD,
    ConditionOrderType::STOCK,
};


TpslOrder tp = TpslOrder{
    TimeInForce::ROD,
    ConditionPriceType::LIMIT,
    ConditionOrderType::STOCK,
    "85",
    "85",
};

TpslOrder sl = TpslOrder{
    TimeInForce::ROD,
    ConditionPriceType::LIMIT,
    ConditionOrderType::STOCK,
    "60",
    "60",
};

TPSLWrapper tpsl = TPSLWrapper{
    StopSign::FULL,
    tp,
    sl,
    "20250605",
    false
};

auto send_condition_order = sdk->stock->single_condition(target_account, "20250605", "20250605", StopSign::FULL, condition, order, tpsl);

if (!send_condition_order.is_success) {
    std::cout << "get order result failed reason: "
        << (send_condition_order.message.has_value() ? send_condition_order.message.value() : "No message")
        << std::endl;
}
else {
    if (send_condition_order.data.has_value()) {
        const auto& send = send_condition_order.data.value();
        std::cout << send << std::endl;
    }
    else {
        std::cout << "Order result success but no data returned." << std::endl;
    }
}


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cpp
{
    isSuccess = True,
    message = ,
    data = ConditionOrderResult {
            guid = 44150047-8977-40b1-953c-ce2XXXXXX,
        }
}


```


---

### 分時分量查詢

get\_time\_slice\_order

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                      | 說明             |
| ------- | ----------------------------------------------------------------------------------------- | ---------------- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#account) | 帳號             |
| batchNo | string                                                                                    | 分時分量條件單號 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別                | 說明                             |
| ----------- | ------------------- | -------------------------------- |
| is\_success | bool                | 是否成功                         |
| data        | List                | 條件單回傳資訊                   |
| message     | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

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
| create\_time                 | string | 建立時間         |
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
| tpsl\_count                  | string | 停損停利筆數     |
| tpsl\_record                 | List   | 停損停利明細資料 |
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

```cpp
auto get_timeslice = sdk->stock->get_time_slice_order(target_account, "25060500000002");
if (!get_timeslice.is_success) {
    std::cout << "get order result failed reason: "
        << (get_timeslice.message.has_value() ? get_timeslice.message.value() : "No message")
        << std::endl;
}
else {
    if (get_timeslice.data.has_value()) {
        for (const auto& result : get_timeslice.data.value()) {
            std::cout << result << std::endl;
        }
    }
    else {
        std::cout << "Order result success but no data returned." << std::endl;
    }
}

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cpp
{
    isSuccess = true,
    message = ,
    data = [
            ConditionDetail{
                Guid = c4dc90c1-4277-42ea-b585-085dc347eac0,
                BatchNo = ,
                OrderLevel = 0,
                LastTime = 2024-07-23 17:30:01,
                ConditionType = 分時分量,
                ParentGuid = ,
                Symbol = 2881,
                OrderAmount = 0,
                ChildBatchNo = ,
                Account = 1307515,
                ConditionContent = 當於2024/07/23 定時單時間大於等於08:40:00 全部成交為止,
                Action = 下單,
                ConditionBuySell = 現股買,
                ConditionSymbol = 富邦金 現股買,
                ConditionPrice = 66元(ROD),
                ConditionVolume = 1張,
                ConditionFilledVolume = 0張,
                CreateTime = 2024-07-22 17:30:03,
                StartDate = 2024/07/23,
                Status = 條件單中止(I),
                ErrorMessage = ,
                DetailRecordsCount = 0,
                DetailRecords = [],
                TpslCount = 0,
                TpslRecord = []
            },

            ConditionDetail
            {
                Guid = 2975702e-f36f-4da4-bab6-1310344ec05d,
                BatchNo = ,
                OrderLevel = 0,
                LastTime = 2024-07-23 17:30:01,
                ConditionType = 分時分量,
                ParentGuid = ,
                Symbol = 2881,
                OrderAmount = 0,
                ChildBatchNo = ,
                Account = 1307515,
                ConditionContent = 當於2024/07/23 定時單時間大於等於09:10:00 全部成交為止,
                Action = 下單,
                ConditionBuySell = 現股買,
                ConditionSymbol = 富邦金 現股買,
                ConditionPrice = 66元(ROD),
                ConditionVolume = 1張,
                ConditionFilledVolume = 0張,
                CreateTime = 2024-07-22 17:30:03,
                StartDate = 2024/07/23,
                Status = 條件單中止(I),
                ErrorMessage = ,
                DetailRecordsCount = 0,
                DetailRecords = [],
                TpslCount = 0,
                TpslRecord = []
            }
    ]
}


```


---

### 分時分量條件單

time\_slice\_order

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                  | 類別                                                                                                                          | 說明                                                                              |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| start\_date           | string                                                                                                                        | 條件開始監控時間                                                                  |
| end\_date             | string                                                                                                                        | 條件結束監控時間                                                                  |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#stopsign)                           | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| splitDescription      | [SplitDescription](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#splitdescription)           | 分單條件                                                                          |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別                | 說明                             |
| ----------- | ------------------- | -------------------------------- |
| is\_success | bool                | 是否成功                         |
| data        | Object              | 條件單回傳資訊                   |
| message     | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數    | 類別   | 說明             |
| ------- | ------ | ---------------- |
| batchId | string | 分時分量條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cpp
// 設計條件內容
SplitDescription split = SplitDescription{
    TimeSliceOrderType::TYPE1,
    30,
    1000,
    10000,
    "083000",
    std::nullopt
};

ConditionOrder sliceorder = ConditionOrder{
    BsAction::BUY,
    "2330",
    "1000",
    1000,
    ConditionMarketType::COMMON,
    ConditionPriceType::LIMIT,
    TimeInForce::ROD,
    ConditionOrderType::STOCK,
};

auto timeslice = sdk->stock->time_slice_order(target_account, "20250605","20250605", StopSign::FULL, split, sliceorder);

if (!timeslice.is_success) {
    std::cout << "send timeslice result failed reason: "
        << (timeslice.message.has_value() ? timeslice.message.value() : "No message")
        << std::endl;
}
else {
    if (timeslice.data.has_value()) {
        const auto& send = timeslice.data.value();
        std::cout << send << std::endl;
    }
    else {
        std::cout << "send timeslice success but no data returned." << std::endl;
    }
}


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cpp

{
    isSuccess = true,
    message = ,
    data = ConditionOrderResult {
            guid = 24080500000002,
        }
}


```


---

### 歷史移動鎖利查詢

get\_trail\_history

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                      | 說明       |
| ----------- | ----------------------------------------------------------------------------------------- | ---------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#account) | 帳號       |
| start\_date | string                                                                                    | 查詢開始日 |
| end\_date   | string                                                                                    | 查詢截止日 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別                | 說明                             |
| ----------- | ------------------- | -------------------------------- |
| is\_success | bool                | 是否成功                         |
| data        | list                | 條件單回傳資訊                   |
| message     | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

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
| create\_time                 | string | 建立時間         |
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
| tpsl\_count                  | string | 停損停利筆數     |
| tpsl\_record                 | List   | 停損停利明細資料 |
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

```cpp
auto get_trail_his = sdk->stock->get_trail_history(target_account, "20250601", "20250606");
if (!get_trail_his.is_success) {
    std::cout << "get order result failed reason: "
        << (get_trail_his.message.has_value() ? get_trail_his.message.value() : "No message")
        << std::endl;
}
else {
    if (get_trail_his.data.has_value()) {
        for (const auto& result : get_trail_his.data.value()) {
            std::cout << result << std::endl;
        }
    }
    else {
        std::cout << "Order result success but no data returned." << std::endl;
    }
}

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cpp

{
    isSuccess = True,
    message = ,
    data = [
            ConditionDetail {
                Guid = 5c154a76-e7ef-4b8f-94b5-80bf08fa4b8e,
                BatchNo = ,
                OrderLevel = 0,
                LastTime = 2024-08-02 16:45:01,
                ConditionType = ,
                ParentGuid = ,
                Symbol = 2889,
                OrderAmount = 0,
                ChildBatchNo = ,
                Account = 1307515,
                ConditionContent = 從2024/08/02到2024/08/02內當國票金從1000(原始基準價)，下跌達10%，持續至預定張數全部成交或期限到達為止,
                Action = 下單,
                ConditionBuySell = 現股買,
                ConditionSymbol = 國票金 (2889),
                ConditionPrice = 成交價(1) 檔(ROD),
                ConditionVolume = 1張,
                ConditionFilledVolume = 0張,
                CreateTime = 2024-08-02 10:07:31,
                StartDate = 2024/08/02,
                Status = 條件單中止(I),
                ErrorMessage = null,
                DetailRecordsCount = 0,
                DetailRecords = [],
                TpslCount = 0,
                TpslRecord = []
            },
            ConditionDetail
            {
                Guid = c71117c8-aa70-4477-9f04-ff4c73a4fad0,
                BatchNo = "",
                OrderLevel = 0,
                LastTime = 2024-07-29 17:30:00,
                ConditionType = ,
                ParentGuid = ,
                Symbol = 2330,
                OrderAmount = 0,
                ChildBatchNo = "",
                Account = 1307515,
                ConditionContent = 從2024/07/29到2024/07/29內當台積電從860(原始基準價)，上漲達5%，持續至預定張數全部成交或期限到達為止,
                Action = 下單,
                ConditionBuySell = 現股買,
                ConditionSymbol = 台積電 (2330),
                ConditionPrice = 成交價(5) 檔(ROD),
                ConditionVolume = 2張,
                ConditionFilledVolume = 0張,
                CreateTime = 2024-07-29 11:01:49,
                StartDate = 2024/07/29,
                Status = 條件單中止(I),
                ErrorMessage = null,
                DetailRecordsCount = 0,
                DetailRecords = [],
                TpslCount = 0,
                TpslRecord = []
            }
    ]
}


```


---

### 移動鎖利查詢

get\_trail\_order

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                      | 說明 |
| ------- | ----------------------------------------------------------------------------------------- | ---- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#account) | 帳號 |

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
| create\_time                 | string | 建立時間         |
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
| tpsl\_count                  | string | 停損停利筆數     |
| tpsl\_record                 | List   | 停損停利明細資料 |
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

```cpp
auto get_trail = sdk->stock->get_trail_order(target_account);

if (!get_trail.is_success) {
    std::cout << "get order result failed reason: "
        << (get_trail.message.has_value() ? get_trail.message.value() : "No message")
        << std::endl;
}
else {
    if (get_trail.data.has_value()) {
        for (const auto& result : get_trail.data.value()) {
            std::cout << result << std::endl;
        }
    }
    else {
        std::cout << "Order result success but no data returned." << std::endl;
    }
}


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cpp

{
    isSuccess = true,
    message = ,
    data = [
            ConditionDetail {
                Guid = 5c154a76-e7ef-4b8f-94b5-80bf08fa4b8e,
                BatchNo = ,
                OrderLevel = 0,
                LastTime = 2024-08-02 16:45:01,
                ConditionType = ,
                ParentGuid = ,
                Symbol = 2889,
                OrderAmount = 0,
                ChildBatchNo = ,
                Account = 1307515,
                ConditionContent = 從2024/08/02到2024/08/02內當國票金從1000(原始基準價)，下跌達10%，持續至預定張數全部成交或期限到達為止,
                Action = 下單,
                ConditionBuySell = 現股買,
                ConditionSymbol = 國票金 (2889),
                ConditionPrice = 成交價(1) 檔(ROD),
                ConditionVolume = 1張,
                ConditionFilledVolume = 0張,
                CreateTime = 2024-08-02 10:07:31,
                StartDate = 2024/08/02,
                Status = 條件單中止(I),
                ErrorMessage = null,
                DetailRecordsCount = 0,
                DetailRecords = [],
                TpslCount = 0,
                TpslRecord = []
            },
            ConditionDetail
            {
                Guid = c71117c8-aa70-4477-9f04-ff4c73a4fad0,
                BatchNo = "",
                OrderLevel = 0,
                LastTime = 2024-07-29 17:30:00,
                ConditionType = ,
                ParentGuid = ,
                Symbol = 2330,
                OrderAmount = 0,
                ChildBatchNo = "",
                Account = 1307515,
                ConditionContent = 從2024/07/29到2024/07/29內當台積電從860(原始基準價)，上漲達5%，持續至預定張數全部成交或期限到達為止,
                Action = 下單,
                ConditionBuySell = 現股買,
                ConditionSymbol = 台積電 (2330),
                ConditionPrice = 成交價(5) 檔(ROD),
                ConditionVolume = 2張,
                ConditionFilledVolume = 0張,
                CreateTime = 2024-07-29 11:01:49,
                StartDate = 2024/07/29,
                Status = 條件單中止(I),
                ErrorMessage = null,
                DetailRecordsCount = 0,
                DetailRecords = [],
                TpslCount = 0,
                TpslRecord = []
            }
    ]
}


```


---

### 移動鎖利條件單

trail\_profit

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                    | 說明                                                                             |
| ----------- | ------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#account)               | 帳號                                                                             |
| start\_date | string                                                                                                  | 條件開始監控時間                                                                 |
| end\_date   | string                                                                                                  | 條件結束監控時間                                                                 |
| stop\_sign  | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#stopsign)     | 條件停止條件 :`Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| TrailOrder  | [TrailOrder](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/cpp/EnumMatrix.txt#trailorder) | 分單條件                                                                         |

caution

TrailOrder 內之 基準價 (price) 只可輸入至多小數點後兩位，若超出將造成系統洗價失敗

info

當前股票價格價觸及觸發價時（以基準價計算之漲跌 % 數） 即下單

**例：** 若初始基準價設 100, 向下 5% 下單停損/利，初始觸發價為 95，若價格無上漲超過 100（即基準價無調整）， 則市場價為 95 時將觸發條件下單

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別                | 說明                             |
| ----------- | ------------------- | -------------------------------- |
| is\_success | bool                | 是否成功                         |
| data        | Object              | 條件單回傳資訊                   |
| message     | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cpp
// 設計條件內容
TrailOrder trail = TrailOrder{
    "2330",
    "1000",
    Direction::UP,
    5,
    BsAction::BUY,
    2000,
    ConditionPriceType::MATCHED_PRICE,
    5,
    TimeInForce::ROD,
    ConditionOrderType::STOCK
};


auto trail_order = sdk->stock->trail_profit(target_account, "20250606","20250606", StopSign::FULL, trail);

if (!trail_order.is_success) {
    std::cout << "send trail_order result failed reason: "
        << (trail_order.message.has_value() ? trail_order.message.value() : "No message")
        << std::endl;
}
else {
    if (trail_order.data.has_value()) {
        const auto& send = trail_order.data.value();
        std::cout << send << std::endl;
    }
    else {
        std::cout << "send trail_order success but no data returned." << std::endl;
    }
}


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cpp

{
    isSuccess = true,
    message = ,
    data = ConditionOrderResult {
            guid = 44150047-8977-40b1-953c-ce2XXXXXX,
        }
}


```


---

### 取消條件單

CancelConditionOrders

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                         | 說明     |
| ------- | -------------------------------------------------------------------------------------------- | -------- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#account) | 帳號     |
| guid    | string                                                                                       | 條件單號 |

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
sdk.Stock.CancelConditionOrders(account, "c9df498a-3b28-4b50-a6f2-f7bd524e96df");


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

Result {
    isSuccess = true,
    message = None,
    data = CancelResult {
             advisory = 成功筆數:1,失敗筆數:0!
            }
 }

```


---

### 當沖條件單查詢

GetConditionDaytradeById

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                         | 說明         |
| ------- | -------------------------------------------------------------------------------------------- | ------------ |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#account) | 帳號         |
| guid    | string                                                                                       | 當沖條件單號 |

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
sdk.Stock.GetConditionDaytradeById(account,"8ff3472b-185a-488c-be5a-b478deda080c");

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = 
            ConditionDetail {
                guid = 8ff3472b-185a-488c-be5a-b478deda080c,
                batchNo = ,
                orderLevel = 1,
                lastTime = 2024-03-14 12:39:02,
                conditionType = 多條件,
                parentGuid = 58e7a51e-9701-4d26-bc16-18a392a840bd,
                symbol = 2330,
                orderAmount = 0,
                childBatchNo = ,
                account = 1307515,
                conditionContent = 當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止,
                action = 下單(次日回補),
                conditionBuySell = 現股賣,
                conditionSymbol = 台積電 現股賣,
                conditionPrice = 580元(ROD),
                conditionVolume = 5張,
                conditionFilledVolume = 0張,
                createTime = 2024-03-14 12:39:22,
                startDate = 2024/03/14,
                status = 未生效(W),
                errorMessage = null,
                detailRecordsCount = 0,
                detailRecords = ,
                tPSLCount = 0,
                tPSLRecord = 
            }
            
}


```


---

### 當沖多條件單

MultiConditionDayTrade

停損利注意事項

* 停損利設定僅為觸發送單，不保證必定回補成功，需視市場狀況自行調整
* 請確認停損利**委託類別**設定需符合當日沖銷交易規則（例如信用交易使用資券互抵）

當沖條件單查詢

* 使用 guid 查詢當沖條件單請使&#x7528;***當沖條件單查詢***&#x529F;能

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                     | 類別                                                                                                                             | 說明                                                                              |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account                  | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| stopSign                 | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#stopsign)                           | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| endTime                  | string                                                                                                                           | 父單洗價結束時間                                                                  |
| multicondition           | [Condition List](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#condition-object)             | 觸發條件                                                                          |
| orderObject              | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |
| ConditionDayTrade Object | [ConditionDayTrade](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#conditiondaytrade-object)  | 當沖回補內容                                                                      |
| TPSLObject               | [TPSLWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#tpslwrapper-object)              | 停損停利條件                                                                      |
| fixSession               | bool                                                                                                                             | 執行定盤回補                                                                      |

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
    "2881",
    TriggerContent.MatchedPrice,
    "66",
    Operator.LessThan
);

var condition2 = new Condition(
    TradingType.Reference,
    "2881",
    TriggerContent.TotalQuantity,
    "8000",
    Operator.LessThan
);

List<Condition> conditions = new List<Condition>();

conditions.Add(condition1);
conditions.Add(condition2);

var order = new ConditionOrder(
    BsAction.Buy,
    "2881",
    "66",
    2000,
    ConditionMarketType.Common,
    ConditionPriceType.Limit,
    TimeInForce.Rod,
    ConditionOrderType.Stock
);

var daytrade_obj = new ConditionDayTrade(
    "131000", // 收盤前沖銷時間，可設定區間為 130100 ~ 132000
    true,        
    "",
    ConditionPriceType.Market 
);

var tp = new TpslOrder(
    TimeInForce.Rod,
    ConditionPriceType.Limit,
    ConditionOrderType.Stock,
    "85",
    "85",
    null
);

var sl = new TpslOrder(
    TimeInForce.Rod,
    ConditionPriceType.Limit,
    ConditionOrderType.Stock,
    "60",
    "60",
    null
);

var tpsl = new TpslWrapper(
    StopSign.Full,
    tp,
    sl,
    "20240517",
    true  // ** 設定 true 以啟用當日沖銷洗價
);


sdk.Stock.MultiConditionDayTrade(account, StopSign.Full, "130000", conditions, order, daytrade_obj, tpsl, true);

// 不設定停損停利
//sdk.Stock.MultiConditionDayTrade(account, StopSign.Full, "130000", conditions, order, daytrade_obj, null, true);

// 不設定定盤回補
//sdk.Stock.MultiConditionDayTrade(account, StopSign.Full, "130000", conditions, order, daytrade_obj, tpsl, false);



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

### 當沖單一條件單

SingleConditionDayTrade

停損利注意事項

* 停損利設定僅為觸發送單，不保證必定回補成功，需視市場狀況自行調整
* 請確認停損利**委託類別**設定需符合當日沖銷交易規則（例如信用交易使用資券互抵）

當沖條件單查詢

* 使用 guid 查詢當沖條件單請使&#x7528;***當沖條件單查詢***&#x529F;能

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                     | 類別                                                                                                                             | 說明                                                                              |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account                  | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| stopSign                 | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#stopsign)                           | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| endTime                  | string                                                                                                                           | 父單洗價結束時間                                                                  |
| condition                | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#condition-object)                  | 觸發條件                                                                          |
| orderObject              | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |
| ConditionDayTrade Object | [ConditionDayTrade](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#conditiondaytrade-object)  | 當沖回補內容                                                                      |
| TPSLObject               | [TPSLWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#tpslwrapper-object)              | 停損停利條件                                                                      |
| fixSession               | bool                                                                                                                             | 執行定盤回補                                                                      |

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
    "2881",
    TriggerContent.MatchedPrice,
    "66",
    Operator.LessThan
);

var order = new ConditionOrder(
    BsAction.Buy,
    "2881",
    "66",
    2000,
    ConditionMarketType.Common,
    ConditionPriceType.Limit,
    TimeInForce.Rod,
    ConditionOrderType.Stock
);

var daytrade_obj = new ConditionDayTrade(
    "131000", // 收盤前沖銷時間，可設定區間為 130100 ~ 132000
    true,        
    "",
    ConditionPriceType.Market 
);

var tp = new TpslOrder(
    TimeInForce.Rod,
    ConditionPriceType.Limit,
    ConditionOrderType.Stock,
    "85",
    "85",
    null
);

var sl = new TpslOrder(
    TimeInForce.Rod,
    ConditionPriceType.Limit,
    ConditionOrderType.Stock,
    "60",
    "60",
    null
);

var tpsl = new TpslWrapper(
    StopSign.Full,
    tp,
    sl,
    "20240517",
    true  // ** 設定 true 以啟用當日沖銷洗價
);


sdk.Stock.SingleConditionDayTrade(account, StopSign.Full, "130000", condition, order, daytrade_obj, tpsl, true);

// 不設定停損停利
//sdk.Stock.SingleConditionDayTrade(account, StopSign.Full, "130000", condition, order, daytrade_obj, null, true);

// 不設定定盤回補
//sdk.Stock.SingleConditionDayTrade(account, StopSign.Full, "130000", condition, order, daytrade_obj, tpsl, false);



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

1. 選擇 `Reference`時， trigger 可搭配 `BidPrice` 、`AskPrice` 、`MatchedPrice` 、`TotalQuantity` 、 `Time`，若 trigger 使用 Time，則時間條件達成，且時間範圍內商品有交易產生才觸發
2. 選擇`Scheduled` 時，symbol需填入空值""，trigger需帶入`Time`，則會使用系統時間判斷，當時間條件達成即觸發

##### ConditionOrder Object[​](#conditionorder-object "Direct link to ConditionOrder Object")

| Parameter   | Type                | Meaning                                                                                                                                                                                          |
| ----------- | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| buySell     | BsAction            | [買賣別](#bsaction)                                                                                                                                                                              |
| symbol      | string              | 股票代號                                                                                                                                                                                         |
| price       | string              | 委託價格                                                                                                                                                                                         |
| quantity    | int                 | 委託數量                                                                                                                                                                                         |
| marketType  | ConditionMarketType | [盤別](#conditionmarkettype) 可選用參數`Common` 整股、`Fixing`定盤、`IntradayOdd` 盤中零股、`Odd` 盤後零股                                                                                       |
| priceType   | ConditionPriceType  | [價格別](#conditionpricetype) : 可選參數 :`BidPrice` 買進價、 `AskPrice` 賣出價、`MatchedPrice`成交價、`Limit` 限價、`LimitUp` 漲停、`LimitDown` 跌停、`Market` 市價、`Reference` 參考價(平盤價) |
| timeInForce | TimeInForce         | [委託條件](#timeinforce) 可選用參數為 `Rod`、`Fok`、`Ioc`                                                                                                                                        |
| orderType   | ConditionOrderType  | [委託類別](#conditionordertype) : 可選參數 :`Stock`現貨、`Margin`融資、`Short`融券                                                                                                               |

##### TpslOrder Object[​](#tpslorder-object "Direct link to TpslOrder Object")

| Parameter   | Type               | Meaning                                                                                                                                                                                          |
| ----------- | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| timeInForce | TimeInforce        | [委託條件](#timeinforce) : 可選參數 :`ROD`、`IOC`、`FOK`                                                                                                                                         |
| priceType   | ConditionPriceType | [價格別](#conditionpricetype) : 可選參數 :`BidPrice` 買進價、 `AskPrice` 賣出價、`MatchedPrice`成交價、`Limit` 限價、`LimitUp` 漲停、`LimitDown` 跌停、`Market` 市價、`Reference` 參考價(平盤價) |
| orderType   | ConditionOrderType | [委託類別](#conditionordertype) : 可選參數 :`Stock`現貨、`Margin`融資、`Short`融券                                                                                                               |
| targetPrice | string             | 停損/停利觸發價                                                                                                                                                                                  |
| price       | string             | 停損/停利委託價                                                                                                                                                                                  |
| trigger     | TriggerContent     | 停損/停利[觸發條件](#triggercontent) （詳見表末說明）                                                                                                                                            |

info

`priceType` 若為`limit`時，`price`需填入價格；其餘price填入空字串 **或** `null` (>= ver. 2.2.4)

停損/停利觸發條件

2.2.0 以後版本新增（***2.1.1及以前版本無此欄位***），可設定停損/停利觸發價格別為 1. 成交價（MatchedPrice）；2. 最佳買價（BidPrice）；3. 最佳賣價（AskPrice）

若未設定則此欄位填入 ***null***， 預設使用 成交價（MatchedPrice）

##### TpslWrapper Object[​](#tpslwrapper-object "Direct link to TpslWrapper Object")

| Parameter | Type                                    | Meaning                                                                                           |
| --------- | --------------------------------------- | ------------------------------------------------------------------------------------------------- |
| stopSign  | StopSign                                | [停止條件](#stopsign) : 可選用 `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| tp        | TpslOrder Object (Optional)             | 停利條件內容                                                                                      |
| sl        | TpslOrder Object (Optional)             | 停損條件內容                                                                                      |
| endDate   | string (Optional : 空值同母單StartDate) | 條件停止日期                                                                                      |
| intraday  | bool (Optional)                         | 全部成交後日內當沖                                                                                |

日內當沖

使用當沖條件單時，`intraday` 需設定為 `true`

##### TrailOrder[​](#trailorder "Direct link to TrailOrder")

| Parameter   | Type               | Meaning                                                                                                                                                                               |
| ----------- | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol      | string             | 股票代號                                                                                                                                                                              |
| price       | string             | 基準價 (Note. 只可輸入至多小數點後兩位，若超出將造成系統洗價失敗)                                                                                                                     |
| direction   | Direction          | [方向](#direction) : 可選用 `Up` 上漲、`Down` 下跌                                                                                                                                    |
| percentage  | int                | 上漲 / 下跌 % 數                                                                                                                                                                      |
| buysell     | BsAction           | 買賣別 : 可選用 `Buy` 買進 、`Sell` 賣出                                                                                                                                              |
| quantity    | int                | 委託股數                                                                                                                                                                              |
| PriceType   | ConditionPriceType | [價格類別](#conditionpricetype) : 可選用 `BidPrice` 買進價、`AskPrice` 賣出價、`MatchedPrice` 成交價、`Market` 市價、`LimitUp` 漲停價、`LimitDown` 跌停價、`Reference` 參考價(平盤價) |
| diff        | int                | 買賣價格檔數 (根據 priceType 加減檔數) ，正值為向上加檔數、負值為向下加檔數                                                                                                           |
| timeInForce | TimeInForce        | [委託條件](#timeinforce) 可選用參數為 `ROD`、`FOK`、`IOC`                                                                                                                             |
| orderType   | ConditionOrderType | [委託類別](#conditionordertype) 可選用參數為 `Stock`現股、`Margin`融資、`Short`融券                                                                                                   |

##### SplitDescription[​](#splitdescription "Direct link to SplitDescription")

| Parameter      | Type              | Meaning                                                                                                                                                                                                                                                                                                                                                                                                      |
| -------------- | ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| method         | SliceType         | [Slice Condition](#slicetype) : 可選用 :<br />`Type1` 從 `startTime` 開始，每隔幾秒送一筆，總共`totalQuantity`，每筆送 `singleQuantity`<br />`Type2` 從 `startTime` 開始到 `endTime` 結束，每隔 `interval` 秒送一筆，總共 `totalQuantity`，剩餘張數加總至最後一筆<br />`Type3` 從 `startTime` 開始到 `endTime` 結束，每隔 `interval` 秒送一筆，總共 `totalQuantity`，剩餘張數從最後一筆往前分配.<br /><br /> |
| interval       | int               | 下單間隔時間 (秒)                                                                                                                                                                                                                                                                                                                                                                                            |
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

##### ConditionDayTrade Object[​](#conditiondaytrade-object "Direct link to ConditionDayTrade Object")

| Parameter       | Type                                      | Meaning                                            |
| --------------- | ----------------------------------------- | -------------------------------------------------- |
| dayTradeEndTime | string                                    | 收盤前沖銷時間，格式 "HHMMSS" (例 "131500")        |
| autoCancel      | bool                                      | 洗價結束後是否刪除已觸發委託                       |
| price           | string                                    | 委託價格 (若無指定價格，例如市價單，請填空字串 "") |
| priceType       | [ConditionPriceType](#conditionpricetype) | 委託單價格類型                                     |

關於收盤前沖銷時間

可設定區間為 130100 ~ 132000

#### Constants ( 欄位對應數值 )[​](#constants--欄位對應數值- "Direct link to Constants ( 欄位對應數值 )")

##### BsAction[​](#bsaction "Direct link to BsAction")

買賣別

| Name | Meaning |
| ---- | ------- |
| Buy  | 買      |
| Sell | 賣      |

##### ConditionMarketType[​](#conditionmarkettype "Direct link to ConditionMarketType")

盤別

| Name        | Meaning  |
| ----------- | -------- |
| Common      | 一般盤   |
| Fixing      | 定盤     |
| IntradayOdd | 盤中零股 |
| Odd         | 盤後零股 |

##### TradingType[​](#tradingtype "Direct link to TradingType")

監控類型

| Name      | Meaning          |
| --------- | ---------------- |
| Reference | 自動參考委託物件 |
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

##### ConditionPriceType[​](#conditionpricetype "Direct link to ConditionPriceType")

價格類型 (ConditionPriceType)

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

##### ConditionOrderType[​](#conditionordertype "Direct link to ConditionOrderType")

委託類型 (orderType)

| Name   | Meaning |
| ------ | ------- |
| Stock  | 現貨    |
| Margin | 融資    |
| Short  | 融券    |

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

GetConditionOrderById

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                         | 說明     |
| ------- | -------------------------------------------------------------------------------------------- | -------- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#account) | 帳號     |
| guid    | string                                                                                       | 條件單號 |

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
sdk.Stock.GetConditionOrderById(account,"8ff3472b-185a-488c-be5a-b478deda080c");

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = 
            ConditionDetail {
                guid = 8ff3472b-185a-488c-be5a-b478deda080c,
                batchNo = ,
                orderLevel = 1,
                lastTime = 2024-03-14 12:39:02,
                conditionType = 多條件,
                parentGuid = 58e7a51e-9701-4d26-bc16-18a392a840bd,
                symbol = 2330,
                orderAmount = 0,
                childBatchNo = ,
                account = 1307515,
                conditionContent = 當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止,
                action = 下單(次日回補),
                conditionBuySell = 現股賣,
                conditionSymbol = 台積電 現股賣,
                conditionPrice = 580元(ROD),
                conditionVolume = 5張,
                conditionFilledVolume = 0張,
                createTime = 2024-03-14 12:39:22,
                startDate = 2024/03/14,
                status = 未生效(W),
                errorMessage = null,
                detailRecordsCount = 0,
                detailRecords = ,
                tPSLCount = 0,
                tPSLRecord = 
            }
            
}


```


---

### 歷史條件單查詢

GetConditionHistory

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                   | 類別                                                                                                                         | 說明             |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------- | ---------------- |
| account                | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#account)                                 | 帳號             |
| startDate              | string                                                                                                                       | 查詢條件單創建日 |
| endDate                | string                                                                                                                       | 查詢截止日       |
| conditionHistoryStatus | [History Status](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#historystatus) (Optional) | 歷史條件單狀態   |

info

歷史資料日期以條件單創建日期為準

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
sdk.Stock.GetConditionHistory(account,"20240301","20240601");

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = [
            ConditionDetail {
                guid = 8ff3472b-185a-488c-be5a-b478deda080c,
                batchNo = ,
                orderLevel = 1,
                lastTime = 2024-03-14 12:39:02,
                conditionType = 多條件,
                parentGuid = 58e7a51e-9701-4d26-bc16-18a392a840bd,
                symbol = 2330,
                orderAmount = 0,
                childBatchNo = ,
                account = 1307515,
                conditionContent = 當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止,
                action = 下單(次日回補),
                conditionBuySell = 現股賣,
                conditionSymbol = 台積電 現股賣,
                conditionPrice = 580元(ROD),
                conditionVolume = 5張,
                conditionFilledVolume = 0張,
                createTime = 2024-03-14 12:39:22,
                startDate = 2024/03/14,
                status = 未生效(W),
                errorMessage = null,
                detailRecordsCount = 0,
                detailRecords = ,
                tPSLCount = 0,
                tPSLRecord = 
            },
            ConditionDetail {
                guid = 8ff3472b-185a-488c-be5a-b478deda080c,
                batchNo = ,
                orderLevel = 1,
                lastTime = 2024-03-14 12:39:02,
                conditionType = 多條件,
                parentGuid = 58e7a51e-9701-4d26-bc16-18a392a840bd,
                symbol = 2330,
                orderAmount = 0,
                childBatchNo = ,
                account = 1307515,
                conditionContent = 當自2024/03/14至2024/07/04內台積電成交價小於等於570元 全部成交為止,
                action = 下單(次日回補),
                conditionBuySell = 現股賣,
                conditionSymbol = 台積電 現股賣,
                conditionPrice = 市價(ROD),
                conditionVolume = 5張,
                conditionFilledVolume = 0張,
                createTime = 2024-03-14 12:39:22,
                startDate = 2024/03/14,
                status = 未生效(W),
                errorMessage = null,
                detailRecordsCount = 0,
                detailRecords =,
                tPSLCount = 0,
                tPSLRecord = 
            },
            ConditionDetail {
                guid = ec757279-bcb3-46f4-80ac-fccfc786bc8d,
                batchNo = ,
                orderLevel = 0,
                lastTime = 2024-05-21 10:30:00,
                conditionType = 多條件,
                parentGuid = ec757279-bcb3-46f4-80ac-fccfc786bc8d,
                symbol = 2330,
                orderAmount = 0,
                childBatchNo = ,
                account = 1307515,
                conditionContent = 當自2024/03/14至2024/07/04內台積電成交價大於等於575元，且 台積電成交總量大於等於1001張，且 台積電時間大於等於10:30:00 部份成交為止,
                action = 下單(當沖),
                conditionBuySell = 現股買,
                conditionSymbol = 台積電 現股買,
                conditionPrice = 576元(ROD),
                conditionVolume = 5張,
                conditionFilledVolume = 0張,
                createTime = 2024-03-14 12:30:02,
                startDate = 2024/03/14,
                status = 觸發-委託失敗(X),
                errorMessage = 單價輸入錯誤[4385715],
                detailRecordsCount = 0,
                detailRecords = [],
                tPSLCount = 2,
                tPSLRecord = [
                    ParentChildRecord
                    {
                        guid = c94b552a-22da-43e4-be44-f27a9c2026e1,
                        account = 1307515,
                        conditionContent = 當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止,
                        action = 下單(次日回補),
                        conditionBuySell = 現股賣,
                        conditionSymbol = 台積電 現股賣,
                        conditionPrice = 580元(ROD),
                        conditionVolume = 5張,
                        conditionFilledVolume = 0張,
                        startDate = 2024/03/14,
                        status = 未生效(W),
                        errorMessage = 
                    },
                    ParentChildRecord
                    {
                        guid = c94b552a-22da-43e4-be44-f27a9c2026e1,
                        account = 1307515,
                        conditionContent = 當自2024/03/14至2024/07/04內台積電成交價小於等於570元 全部成交為止,
                        action = 下單(次日回補),
                        conditionBuySell = 現股賣,
                        conditionSymbol = 台積電 現股賣,
                        conditionPrice = 市價(ROD),
                        conditionVolume = 5張,
                        conditionFilledVolume = 0張,
                        startDate = 2024/03/14,
                        status = 未生效(W),
                        errorMessage = 
                    }
                ]
            }
    ]
}


```


---

### 條件單查詢

GetConditionOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數            | 類別                                                                                                                            | 說明       |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| account         | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#account)                                    | 帳號       |
| conditionStatus | [ConditionStatus](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#conditionstatus) (Optional) | 條件單狀態 |

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
sdk.Stock.GetConditionOrder(account);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = [
            ConditionDetail {
                guid = 8ff3472b-185a-488c-be5a-b478deda080c,
                batchNo = ,
                orderLevel = 1,
                lastTime = 2024-03-14 12:39:02,
                conditionType = 多條件,
                parentGuid = 58e7a51e-9701-4d26-bc16-18a392a840bd,
                symbol = 2330,
                orderAmount = 0,
                childBatchNo = ,
                account = 1307515,
                conditionContent = 當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止,
                action = 下單(次日回補),
                conditionBuySell = 現股賣,
                conditionSymbol = 台積電 現股賣,
                conditionPrice = 580元(ROD),
                conditionVolume = 5張,
                conditionFilledVolume = 0張,
                createTime = 2024-03-14 12:39:22,
                startDate = 2024/03/14,
                status = 未生效(W),
                errorMessage = null,
                detailRecordsCount = 0,
                detailRecords = ,
                tPSLCount = 0,
                tPSLRecord = 
            },
            ConditionDetail {
                guid = 8ff3472b-185a-488c-be5a-b478deda080c,
                batchNo = ,
                orderLevel = 1,
                lastTime = 2024-03-14 12:39:02,
                conditionType = 多條件,
                parentGuid = 58e7a51e-9701-4d26-bc16-18a392a840bd,
                symbol = 2330,
                orderAmount = 0,
                childBatchNo = ,
                account = 1307515,
                conditionContent = 當自2024/03/14至2024/07/04內台積電成交價小於等於570元 全部成交為止,
                action = 下單(次日回補),
                conditionBuySell = 現股賣,
                conditionSymbol = 台積電 現股賣,
                conditionPrice = 市價(ROD),
                conditionVolume = 5張,
                conditionFilledVolume = 0張,
                createTime = 2024-03-14 12:39:22,
                startDate = 2024/03/14,
                status = 未生效(W),
                errorMessage = null,
                detailRecordsCount = 0,
                detailRecords =,
                tPSLCount = 0,
                tPSLRecord = 
            },
            ConditionDetail {
                guid = ec757279-bcb3-46f4-80ac-fccfc786bc8d,
                batchNo = ,
                orderLevel = 0,
                lastTime = 2024-05-21 10:30:00,
                conditionType = 多條件,
                parentGuid = ec757279-bcb3-46f4-80ac-fccfc786bc8d,
                symbol = 2330,
                orderAmount = 0,
                childBatchNo = ,
                account = 1307515,
                conditionContent = 當自2024/03/14至2024/07/04內台積電成交價大於等於575元，且 台積電成交總量大於等於1001張，且 台積電時間大於等於10:30:00 部份成交為止,
                action = 下單(當沖),
                conditionBuySell = 現股買,
                conditionSymbol = 台積電 現股買,
                conditionPrice = 576元(ROD),
                conditionVolume = 5張,
                conditionFilledVolume = 0張,
                createTime = 2024-03-14 12:30:02,
                startDate = 2024/03/14,
                status = 觸發-委託失敗(X),
                errorMessage = 單價輸入錯誤[4385715],
                detailRecordsCount = 0,
                detailRecords = [],
                tPSLCount = 2,
                tPSLRecord = [
                    ParentChildRecord
                    {
                        guid = c94b552a-22da-43e4-be44-f27a9c2026e1,
                        account = 1307515,
                        conditionContent = 當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止,
                        action = 下單(次日回補),
                        conditionBuySell = 現股賣,
                        conditionSymbol = 台積電 現股賣,
                        conditionPrice = 580元(ROD),
                        conditionVolume = 5張,
                        conditionFilledVolume = 0張,
                        startDate = 2024/03/14,
                        status = 未生效(W),
                        errorMessage = 
                    },
                    ParentChildRecord
                    {
                        guid = c94b552a-22da-43e4-be44-f27a9c2026e1,
                        account = 1307515,
                        conditionContent = 當自2024/03/14至2024/07/04內台積電成交價小於等於570元 全部成交為止,
                        action = 下單(次日回補),
                        conditionBuySell = 現股賣,
                        conditionSymbol = 台積電 現股賣,
                        conditionPrice = 市價(ROD),
                        conditionVolume = 5張,
                        conditionFilledVolume = 0張,
                        startDate = 2024/03/14,
                        status = 未生效(W),
                        errorMessage = 
                    }
                ]
            }
    ]
}


```


---

### 多條件單

MultiCondition

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                  | 類別                                                                                                                             | 說明                                                                             |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#account)                                     | 帳號                                                                             |
| start\_date           | string                                                                                                                           | 條件開始監控時間                                                                 |
| end\_date             | string                                                                                                                           | 條件結束監控時間                                                                 |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#stopsign)                           | 條件停止條件 :`Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| MultiCondition        | [Condition List](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#condition-object)             | 觸發條件                                                                         |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                         |

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
    "2881",
    TriggerContent.MatchedPrice,
    "66",
    Operator.LessThan
);

var condition2 = new Condition(
    TradingType.Reference,
    "2881",
    TriggerContent.TotalQuantity,
    "8000",
    Operator.LessThan
);

List<Condition> conditions = new List<Condition>();

conditions.Add(condition1);
conditions.Add(condition2);

var order = new ConditionOrder(
    BsAction.Buy,
    "2881",
    "66",
    2000,
    ConditionMarketType.Common,
    ConditionPriceType.Limit,
    TimeInForce.Rod,
    ConditionOrderType.Stock
);


sdk.Stock.MultiCondition(account, "20240426", "20240430", StopSign.Full, conditions , order);


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

### 多條件單包含停損停利

MultiCondition

停損利注意事項

* 停損利設定僅為觸發送單，不保證必定成交，需視市場狀況自行調整
* 請確認停損利**委託類別**設定需符合適合之交易規則（例如信用交易資買資賣等）

info

待主單完全成交後，停損停利部分才會啟動

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                  | 類別                                                                                                                             | 說明                                                                             |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#account)                                     | 帳號                                                                             |
| start\_date           | string                                                                                                                           | 條件開始監控時間                                                                 |
| end\_date             | string                                                                                                                           | 條件結束監控時間                                                                 |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#stopsign)                           | 條件停止條件 :`Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| MultiCondition        | [Condition List](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#condition-object)             | 觸發條件                                                                         |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                         |
| TPSLObject            | [TpslWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#tpslwrapper-object)              | 停損停利條件                                                                     |

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
    "2881",
    TriggerContent.MatchedPrice,
    "66",
    Operator.LessThan
);

var condition2 = new Condition(
    TradingType.Reference,
    "2881",
    TriggerContent.TotalQuantity,
    "8000",
    Operator.LessThan
);

List<Condition> conditions = new List<Condition>();

conditions.Add(condition1);
conditions.Add(condition2);


var order = new ConditionOrder(
    BsAction.Buy,
    "2881",
    "66",
    2000,
    ConditionMarketType.Common,
    ConditionPriceType.Limit,
    TimeInForce.Rod,
    ConditionOrderType.Stock
);

// 停損停利若為Market , price 則填空值""

var tp = new TpslOrder(
    TimeInForce.Rod,
    ConditionPriceType.Limit,
    ConditionOrderType.Stock,
    "85",
    "85",
    null  // 2.2.0 以後版本使用，詳請見 [參數對照表] -> [TpslOrder Object]
);

var sl = new TpslOrder(
    TimeInForce.Rod,
    ConditionPriceType.Limit,
    ConditionOrderType.Stock,
    "60",
    "60",
    null  // 2.2.0 以後版本使用，詳請見 [參數對照表] -> [TpslOrder Object]
);

var tpsl = new TpslWrapper(
    StopSign.Full,
    tp,
    sl,
    "20240517",
    false
);

sdk.Stock.MultiCondition(account, "20240426", "20240430", StopSign.Full, conditions, order, tpsl);


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

| 參數                  | 類別                                                                                                                             | 說明                                                                              |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| start\_date           | string                                                                                                                           | 條件開始監控時間                                                                  |
| end\_date             | string                                                                                                                           | 條件結束監控時間                                                                  |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#stopsign)                           | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| condition             | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#condition-object)                  | 觸發條件                                                                          |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |

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
    "2881",
    TriggerContent.MatchedPrice,
    "66",
    Operator.LessThan
);

var order = new ConditionOrder(
    BsAction.Buy,
    "2881",
    "66",
    2000,
    ConditionMarketType.Common,
    ConditionPriceType.Limit,
    TimeInForce.Rod,
    ConditionOrderType.Stock
);


sdk.Stock.SingleCondition(account, "20240426", "20240430", StopSign.Full, condition, order);


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
* 請確認停損利**委託類別**設定需符合適合之交易規則（例如信用交易資買資賣等）

info

待主單完全成交後，停損停利部分才會啟動

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                  | 類別                                                                                                                             | 說明                                                                              |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| start\_date           | string                                                                                                                           | 條件開始監控時間                                                                  |
| end\_date             | string                                                                                                                           | 條件結束監控時間                                                                  |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#stopsign)                           | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| condition             | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#condition-object)                  | 觸發條件                                                                          |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |
| TPSLObject            | [TpslWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#tpslwrapper-object)              | 停損停利條件                                                                      |

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
    "2881",
    TriggerContent.MatchedPrice,
    "66",
    Operator.LessThan
);

var order = new ConditionOrder(
    BsAction.Buy,
    "2881",
    "66",
    2000,
    ConditionMarketType.Common,
    ConditionPriceType.Limit,
    TimeInForce.Rod,
    ConditionOrderType.Stock
);

// 停損停利若為Market , price 則填空值""

var tp = new TpslOrder(
    TimeInForce.Rod,
    ConditionPriceType.Limit,
    ConditionOrderType.Stock,
    "85",
    "85",
    null  // 2.2.0 以後版本使用，詳請見 [參數對照表] -> [TpslOrder Object]
);

var sl = new TpslOrder(
    TimeInForce.Rod,
    ConditionPriceType.Limit,
    ConditionOrderType.Stock,
    "60",
    "60",
    null  // 2.2.0 以後版本使用，詳請見 [參數對照表] -> [TpslOrder Object]
);

var tpsl = new TpslWrapper(
    StopSign.Full,
    tp,
    sl,
    "20240517",
    false
);


sdk.Stock.SingleCondition(account, "20240426", "20240430", StopSign.Full, condition, order, tpsl)


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

### 分時分量查詢

GetTimeSliceOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                         | 說明             |
| ------- | -------------------------------------------------------------------------------------------- | ---------------- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#account) | 帳號             |
| batchNo | string                                                                                       | 分時分量條件單號 |

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
sdk.Stock.GetTimeSliceOrder(account,"1234567");

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = [
           ConditionDetail{
                Guid = c4dc90c1-4277-42ea-b585-085dc347eac0,
                BatchNo = ,
                OrderLevel = 0,
                LastTime = 2024-07-23 17:30:01,
                ConditionType = 分時分量,
                ParentGuid = ,
                Symbol = 2881,
                OrderAmount = 0,
                ChildBatchNo = ,
                Account = 1307515,
                ConditionContent = 當於2024/07/23 定時單時間大於等於08:40:00 全部成交為止,
                Action = 下單,
                ConditionBuySell = 現股買,
                ConditionSymbol = 富邦金 現股買,
                ConditionPrice = 66元(ROD),
                ConditionVolume = "1張,
                ConditionFilledVolume = 0張,
                CreateTime = 2024-07-22 17:30:03,
                StartDate = 2024/07/23,
                Status = 條件單中止(I),
                ErrorMessage = ,
                DetailRecordsCount = 0,
                DetailRecords = [],
                TpslCount = 0,
                TpslRecord = []
            },
            ConditionDetail
            {
                Guid = 2975702e-f36f-4da4-bab6-1310344ec05d,
                BatchNo = ,
                OrderLevel = 0,
                LastTime = 2024-07-23 17:30:01,
                ConditionType = 分時分量,
                ParentGuid = ,
                Symbol = 2881,
                OrderAmount = 0,
                ChildBatchNo = ,
                Account = 1307515,
                ConditionContent = 當於2024/07/23 定時單時間大於等於09:10:00 全部成交為止,
                Action = 下單,
                ConditionBuySell = 現股買,
                ConditionSymbol = 富邦金 現股買,
                ConditionPrice = 66元(ROD),
                ConditionVolume = 1張,
                ConditionFilledVolume = 0張,
                CreateTime = 2024-07-22 17:30:03,
                StartDate = 2024/07/23,
                Status = 條件單中止(I),
                ErrorMessage = ,
                DetailRecordsCount = 0,
                DetailRecords = [],
                TpslCount = 0,
                TpslRecord = []
            }
    ]
}


```


---

### 分時分量條件單

TimeSliceOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                  | 類別                                                                                                                             | 說明                                                                              |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| start\_date           | string                                                                                                                           | 條件開始監控時間                                                                  |
| end\_date             | string                                                                                                                           | 條件結束監控時間                                                                  |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#stopsign)                           | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| splitDescription      | [SplitDescription](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#splitdescription)           | 分單條件                                                                          |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 條件單回傳資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數    | 類別   | 說明             |
| ------- | ------ | ---------------- |
| batchId | string | 分時分量條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
// 設計條件內容
var split = new SplitDescription(
    TimeSliceOrderType.Type1,
     300,
     1000,
     10000,
     "083000",
     null
);


var order = new ConditionOrder(
    BsAction.Buy,
    "2881",
    "66",
    2000,
    ConditionMarketType.Common,
    ConditionPriceType.Limit,
    TimeInForce.Rod,
    ConditionOrderType.Stock
);

sdk.Stock.TimeSliceOrder(target_account, "20240427","20240516", StopSign.Full, split, order);


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = ConditionOrderResult {
            guid = 24080500000002,
        }
}


```


---

### 歷史移動鎖利查詢

GetTrailHistory

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                         | 說明       |
| ----------- | -------------------------------------------------------------------------------------------- | ---------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#account) | 帳號       |
| start\_date | string                                                                                       | 查詢開始日 |
| end\_date   | string                                                                                       | 查詢截止日 |

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
sdk.Stock.GetTrailHistory(account,"20240301","20240601");

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = [
            ConditionDetail {
                Guid = 5c154a76-e7ef-4b8f-94b5-80bf08fa4b8e,
                BatchNo = ,
                OrderLevel = 0,
                LastTime = 2024-08-02 16:45:01,
                ConditionType = ,
                ParentGuid = ,
                Symbol = 2889,
                OrderAmount = 0,
                ChildBatchNo = ,
                Account = 1307515,
                ConditionContent = 從2024/08/02到2024/08/02內當國票金從1000(原始基準價)，下跌達10%，持續至預定張數全部成交或期限到達為止,
                Action = 下單,
                ConditionBuySell = 現股買,
                ConditionSymbol = 國票金 (2889),
                ConditionPrice = 成交價(1) 檔(ROD),
                ConditionVolume = 1張,
                ConditionFilledVolume = 0張,
                CreateTime = 2024-08-02 10:07:31,
                StartDate = 2024/08/02,
                Status = 條件單中止(I),
                ErrorMessage = null,
                DetailRecordsCount = 0,
                DetailRecords = [],
                TpslCount = 0,
                TpslRecord = []
            },
            ConditionDetail
            {
                Guid = c71117c8-aa70-4477-9f04-ff4c73a4fad0,
                BatchNo = "",
                OrderLevel = 0,
                LastTime = 2024-07-29 17:30:00,
                ConditionType = ,
                ParentGuid = ,
                Symbol = 2330,
                OrderAmount = 0,
                ChildBatchNo = "",
                Account = 1307515,
                ConditionContent = 從2024/07/29到2024/07/29內當台積電從860(原始基準價)，上漲達5%，持續至預定張數全部成交或期限到達為止,
                Action = 下單,
                ConditionBuySell = 現股買,
                ConditionSymbol = 台積電 (2330),
                ConditionPrice = 成交價(5) 檔(ROD),
                ConditionVolume = 2張,
                ConditionFilledVolume = 0張,
                CreateTime = 2024-07-29 11:01:49,
                StartDate = 2024/07/29,
                Status = 條件單中止(I),
                ErrorMessage = null,
                DetailRecordsCount = 0,
                DetailRecords = [],
                TpslCount = 0,
                TpslRecord = []
            }
    ]
}


```


---

### 移動鎖利查詢

GetTrailOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                         | 說明 |
| ------- | -------------------------------------------------------------------------------------------- | ---- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#account) | 帳號 |

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
sdk.Stock.GetTrailOrder(account);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

{
    isSuccess = True,
    message = ,
    data = [
            ConditionDetail {
                Guid = 5c154a76-e7ef-4b8f-94b5-80bf08fa4b8e,
                BatchNo = ,
                OrderLevel = 0,
                LastTime = 2024-08-02 16:45:01,
                ConditionType = ,
                ParentGuid = ,
                Symbol = 2889,
                OrderAmount = 0,
                ChildBatchNo = ,
                Account = 1307515,
                ConditionContent = 從2024/08/02到2024/08/02內當國票金從1000(原始基準價)，下跌達10%，持續至預定張數全部成交或期限到達為止,
                Action = 下單,
                ConditionBuySell = 現股買,
                ConditionSymbol = 國票金 (2889),
                ConditionPrice = 成交價(1) 檔(ROD),
                ConditionVolume = 1張,
                ConditionFilledVolume = 0張,
                CreateTime = 2024-08-02 10:07:31,
                StartDate = 2024/08/02,
                Status = 條件單中止(I),
                ErrorMessage = null,
                DetailRecordsCount = 0,
                DetailRecords = [],
                TpslCount = 0,
                TpslRecord = []
            },
            ConditionDetail
            {
                Guid = c71117c8-aa70-4477-9f04-ff4c73a4fad0,
                BatchNo = "",
                OrderLevel = 0,
                LastTime = 2024-07-29 17:30:00,
                ConditionType = ,
                ParentGuid = ,
                Symbol = 2330,
                OrderAmount = 0,
                ChildBatchNo = "",
                Account = 1307515,
                ConditionContent = 從2024/07/29到2024/07/29內當台積電從860(原始基準價)，上漲達5%，持續至預定張數全部成交或期限到達為止,
                Action = 下單,
                ConditionBuySell = 現股買,
                ConditionSymbol = 台積電 (2330),
                ConditionPrice = 成交價(5) 檔(ROD),
                ConditionVolume = 2張,
                ConditionFilledVolume = 0張,
                CreateTime = 2024-07-29 11:01:49,
                StartDate = 2024/07/29,
                Status = 條件單中止(I),
                ErrorMessage = null,
                DetailRecordsCount = 0,
                DetailRecords = [],
                TpslCount = 0,
                TpslRecord = []
            }
    ]
}


```


---

### 移動鎖利條件單

TrailProfit

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                       | 說明                                                                             |
| ----------- | ---------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#account)               | 帳號                                                                             |
| start\_date | string                                                                                                     | 條件開始監控時間                                                                 |
| end\_date   | string                                                                                                     | 條件結束監控時間                                                                 |
| stop\_sign  | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#stopsign)     | 條件停止條件 :`Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| TrailOrder  | [TrailOrder](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/csharp/EnumMatrix.txt#trailorder) | 分單條件                                                                         |

caution

TrailOrder 內之 基準價 (price) 只可輸入至多小數點後兩位，若超出將造成系統洗價失敗

info

當前股票價格價觸及觸發價時（以基準價計算之漲跌 % 數） 即下單

**例：** 若初始基準價設 100, 向下 5% 下單停損/利，初始觸發價為 95，若價格無上漲超過 100（即基準價無調整）， 則市場價為 95 時將觸發條件下單

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
var trail = new TrailOrder(
    "2330",
    "860",
    Direction.Up ,
    5,  // 漲跌 % 數
    BsAction.Buy,
    2000,
    ConditionPriceType.MatchedPrice,
    5, // 向上 or 向下追買 tick數 (向下為負值)
    TimeInForce.Rod,
    ConditionOrderType.Stock
);


sdk.Stock.TrailProfit(target_account, "20240427","20240516", StopSign.Full, trail);


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

| 參數    | 類別                                                                                         | 說明     |
| ------- | -------------------------------------------------------------------------------------------- | -------- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#account) | 帳號     |
| guid    | string                                                                                       | 條件單號 |

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
sdk.stock.cancelConditionOrders(account, "c9df498a-3b28-4b50-a6f2-f7bd524e96df")


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

### 當沖條件單查詢

getConditionDaytradeById

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                         | 說明     |
| ------- | -------------------------------------------------------------------------------------------- | -------- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#account) | 帳號     |
| guid    | string                                                                                       | 條件單號 |

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
sdk.stock.getConditionDaytradeById(account,"8ff3472b-185a-488c-be5a-b478deda080c")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data :
            {
                guid : "8ff3472b-185a-488c-be5a-b478deda080c",
                batchNo : "",
                orderLevel : "1",
                lastTime : "2024-03-14 12:39:02",
                conditionType : "多條件",
                parentGuid : "58e7a51e-9701-4d26-bc16-18a392a840bd",
                symbol : "2330",
                orderAmount : "0",
                childBatchNo : "",
                account : "1307515",
                conditionContent : "當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止",
                action : "下單(次日回補)",
                conditionBuySell : "現股賣",
                conditionSymbol : "台積電 現股賣",
                conditionPrice : "580元(ROD)",
                conditionVolume : "5張",
                conditionFilledVolume : "0張",
                createTime : "2024-03-14 12:39:22",
                startDate : "2024/03/14",
                status : "未生效(W)",
                detailRecordsCount : "0",
                tPSLCount : "0"
            }
}

```


---

### 當沖多條件單

multiConditionDayTrade

停損利注意事項

* 停損利設定僅為觸發送單，不保證必定回補成功，需視市場狀況自行調整
* 請確認停損利**委託類別**設定需符合當日沖銷交易規則（例如信用交易使用資券互抵）

當沖條件單查詢

* 使用 guid 查詢當沖條件單請使&#x7528;***當沖條件單查詢***&#x529F;能

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                     | 類別                                                                                                                             | 說明                                                                              |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account                  | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| stopSign                 | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#stopsign)                           | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| endTime                  | string                                                                                                                           | 父單洗價結束時間                                                                  |
| multicondition           | [Condition List](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#condition-object)             | 觸發條件                                                                          |
| orderObject              | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |
| ConditionDayTrade Object | [ConditionDayTrade](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#conditiondaytrade-object)  | 當沖回補內容                                                                      |
| TPSLObject               | [TPSLWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#tpslwrapper-object)              | 停損停利條件                                                                      |
| fixSession               | bool                                                                                                                             | 執行定盤回補                                                                      |

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
    symbol: "2881",
    trigger: TriggerContent.MatchedPrice,
    triggerValue: "80",
    comparison: Operator.LessThan
}

const condition2 = {
    marketType: TradingType.Reference,
    symbol: "2881",
    trigger: TriggerContent.TotalQuantity,
    triggerValue: "8000",
    comparison: Operator.GreaterThan
}

const order = {
    buySell: BSAction.Buy,
    symbol: "2881",
    price: "66",
    quantity: 2000,
    marketType: ConditionMarketType.Common,
    priceType: ConditionPriceType.Limit,
    timeInForce: TimeInForce.ROD,
    orderType: ConditionOrderType.Stock
}

const daytrade_obj = {
    dayTradeEndTime : "131000", // 收盤前沖銷時間，可設定區間為 130100 ~ 132000
    autoCancel : true,        
    price : "",
    priceType : ConditionPriceType.Market 
}

const tp = {
    timeInForce: TimeInForce.ROD,
    priceType: ConditionPriceType.Limit,
    orderType: ConditionOrderType.Stock,
    targetPrice: "85",
    price: "85",
}

const sl  = {
    timeInForce: TimeInForce.ROD,
    priceType: ConditionPriceType.Limit,
    orderType: ConditionOrderType.Stock,
    targetPrice: "60",
    price: "60",
}

const tpsl = {
    stopSign: StopSign.Full,
    endDate: "20240517", 
    tp: tp,          
    sl: sl,           
    intraday: true  // ** 設定 true 以啟用當日沖銷洗價
}


sdk.stock.multiConditionDayTrade(account, StopSign.Full, "130000", [condition, condition2], order, daytrade_obj, tpsl, true)

// 不設定停損停利
// sdk.stock.multiConditionDayTrade(account, StopSign.Full, "130000", [condition, condition2], order, daytrade_obj, null, true)

// 不設定定盤回補
// sdk.stock.multiConditionDayTrade(account, StopSign.Full, "130000", [condition, condition2], order, daytrade_obj, null, false)

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

### 當沖單一條件單

singleConditionDayTrade

停損利注意事項

* 停損利設定僅為觸發送單，不保證必定回補成功，需視市場狀況自行調整
* 請確認停損利**委託類別**設定需符合當日沖銷交易規則（例如信用交易使用資券互抵）

當沖條件單查詢

* 使用 guid 查詢當沖條件單請使&#x7528;***當沖條件單查詢***&#x529F;能

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                     | 類別                                                                                                                             | 說明                                                                              |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account                  | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| stopSign                 | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#stopsign)                           | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| endTime                  | string                                                                                                                           | 父單洗價結束時間                                                                  |
| condition                | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#condition-object)                  | 觸發條件                                                                          |
| orderObject              | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |
| ConditionDayTrade Object | [ConditionDayTrade](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#conditiondaytrade-object)  | 當沖回補內容                                                                      |
| TPSLObject               | [TPSLWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#tpslwrapper-object)              | 停損停利條件                                                                      |
| fixSession               | bool                                                                                                                             | 執行定盤回補                                                                      |

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
    symbol: "2881",
    trigger: TriggerContent.MatchedPrice,
    triggerValue: "80",
    comparison: Operator.LessThan
}

const order = {
    buySell: BSAction.Buy,
    symbol: "2881",
    price: "66",
    quantity: 2000,
    marketType: ConditionMarketType.Common,
    priceType: ConditionPriceType.Limit,
    timeInForce: TimeInForce.ROD,
    orderType: ConditionOrderType.Stock,
};

const daytrade_obj = {
    dayTradeEndTime : "131000", // 收盤前沖銷時間，可設定區間為 130100 ~ 132000
    autoCancel : true,        
    price : "",
    priceType : ConditionPriceType.Market 
}

const tp = {
    timeInForce: TimeInForce.ROD,
    priceType: ConditionPriceType.Limit,
    orderType: ConditionOrderType.Stock,
    targetPrice: "85",
    price: "85",
}

const sl  = {
    timeInForce: TimeInForce.ROD,
    priceType: ConditionPriceType.Limit,
    orderType: ConditionOrderType.Stock,
    targetPrice: "60",
    price: "60",
}

const tpsl = {
    stopSign: StopSign.Full,
    endDate: "20240517", 
    tp: tp,           
    sl: sl,           
    intraday: true  // ** 設定 true 以啟用當日沖銷洗價
}


sdk.stock.singleConditionDayTrade(account, StopSign.Full, "130000", condition, order, daytrade_obj, tpsl, true)

// 不設定停損停利
// sdk.stock.singleConditionDayTrade(account, StopSign.Full, "130000", condition, order, daytrade_obj, null, true)

// 不設定定盤回補
// sdk.stock.singleConditionDayTrade(account, StopSign.Full, "130000", condition, order, daytrade_obj, null, false)

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

1. 選擇 `Reference`時， trigger 可搭配 `BidPrice` 、`AskPrice` 、`MatchedPrice` 、`TotalQuantity` 、 `Time`，若 trigger 使用 Time，則時間條件達成，且時間範圍內商品有交易產生才觸發
2. 選擇`Scheduled` 時，symbol需填入空值""，trigger需帶入`Time`，則會使用系統時間判斷，當時間條件達成即觸發

##### ConditionOrder Object[​](#conditionorder-object "Direct link to ConditionOrder Object")

| Parameter   | Type                | Meaning                                                                                                                                                                                          |
| ----------- | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| buySell     | BSAction            | [買賣別](#bsaction)                                                                                                                                                                              |
| symbol      | string              | 股票代號                                                                                                                                                                                         |
| price       | string              | 委託價格                                                                                                                                                                                         |
| quantity    | int                 | 委託數量                                                                                                                                                                                         |
| marketType  | ConditionMarketType | [盤別](#conditionmarkettype) 可選用參數`Common` 整股、`Fixing`定盤、`IntradayOdd` 盤中零股、`Odd` 盤後零股                                                                                       |
| priceType   | ConditionPriceType  | [價格別](#conditionpricetype) : 可選參數 :`BidPrice` 買進價、 `AskPrice` 賣出價、`MatchedPrice`成交價、`Limit` 限價、`LimitUp` 漲停、`LimitDown` 跌停、`Market` 市價、`Reference` 參考價(平盤價) |
| timeInForce | TimeInForce         | [委託條件](#timeinforce) 可選用參數為 `ROD`、`FOK`、`IOC`                                                                                                                                        |
| orderType   | ConditionOrderType  | [委託類別](#conditionordertype) : 可選參數 :`Stock`現貨、`Margin`融資、`Short`融券                                                                                                               |

##### TPSLOrder Object[​](#tpslorder-object "Direct link to TPSLOrder Object")

| Parameter          | Type               | Meaning                                                                                                                                                                                          |
| ------------------ | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| timeInForce        | TimeInforce        | [委託條件](#timeinforce) : 可選參數 :`ROD`、`IOC`、`FOK`                                                                                                                                         |
| priceType          | ConditionPriceType | [價格別](#conditionpricetype) : 可選參數 :`BidPrice` 買進價、 `AskPrice` 賣出價、`MatchedPrice`成交價、`Limit` 限價、`LimitUp` 漲停、`LimitDown` 跌停、`Market` 市價、`Reference` 參考價(平盤價) |
| orderType          | ConditionOrderType | [委託類別](#conditionordertype) : 可選參數 :`Stock`現貨、`Margin`融資、`Short`融券                                                                                                               |
| targetPrice        | string             | 停損/停利觸發價                                                                                                                                                                                  |
| price              | string             | 停損/停利委託價                                                                                                                                                                                  |
| trigger (optional) | TriggerContent     | 停損/停利[觸發條件](#triggercontent) （詳見表末說明）                                                                                                                                            |

info

`priceType` 若為`limit`時，`price`需填入價格；其餘price填入空字串 **或** 不使用price欄位 (>= ver. 2.2.4)

停損/停利觸發條件

2.2.0 以後版本新增（***2.1.1及以前版本無此欄位***），可設定停損/停利觸發價格別為 1. 成交價（MatchedPrice）；2. 最佳買價（BidPrice）；3. 最佳賣價（AskPrice）

若未設定則預設使用 成交價（MatchedPrice）

##### TPSLWrapper Object[​](#tpslwrapper-object "Direct link to TPSLWrapper Object")

| Parameter | Type                                    | Meaning                                                                                           |
| --------- | --------------------------------------- | ------------------------------------------------------------------------------------------------- |
| stopSign  | StopSign                                | [停止條件](#stopsign) : 可選用 `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| tp        | TPSLOrder Object (Optional)             | 停利條件內容                                                                                      |
| sl        | TPSLOrder Object (Optional)             | 停損條件內容                                                                                      |
| endDate   | string (Optional : 空值同母單StartDate) | 條件停止日期                                                                                      |
| intraday  | bool (Optional)                         | 全部成交後日內當沖                                                                                |

日內當沖

使用當沖條件單時，`intraday` 需設定為 `true`

##### TrailOrder[​](#trailorder "Direct link to TrailOrder")

| Parameter   | Type               | Meaning                                                                                                                                                                               |
| ----------- | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol      | string             | 股票代號                                                                                                                                                                              |
| price       | string             | 基準價 (Note. 只可輸入至多小數點後兩位，若超出將造成系統洗價失敗)                                                                                                                     |
| direction   | Direction          | [方向](#direction) : 可選用 `Up` 上漲、`Down` 下跌                                                                                                                                    |
| percentage  | int                | 上漲 / 下跌 % 數                                                                                                                                                                      |
| buysell     | BSAction           | 買賣別 : 可選用 `Buy` 買進 、`Sell` 賣出                                                                                                                                              |
| quantity    | int                | 委託股數                                                                                                                                                                              |
| PriceType   | ConditionPriceType | [價格類別](#conditionpricetype) : 可選用 `BidPrice` 買進價、`AskPrice` 賣出價、`MatchedPrice` 成交價、`Market` 市價、`LimitUp` 漲停價、`LimitDown` 跌停價、`Reference` 參考價(平盤價) |
| diff        | int                | 買賣價格檔數 (根據 priceType 加減檔數) ，正值為向上加檔數、負值為向下加檔數                                                                                                           |
| timeInForce | TimeInForce        | [委託條件](#timeinforce) 可選用參數為 `ROD`、`FOK`、`IOC`                                                                                                                             |
| orderType   | ConditionOrderType | [委託類別](#conditionordertype) 可選用參數為 `Stock`現股、`Margin`融資、`Short`融券                                                                                                   |

##### SplitDescription[​](#splitdescription "Direct link to SplitDescription")

| Parameter      | Type              | Meaning                                                                                                                                                                                                                                                                                                                                                                                                      |
| -------------- | ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| method         | SliceType         | [Slice Condition](#slicetype) : 可選用 :<br />`Type1` 從 `startTime` 開始，每隔幾秒送一筆，總共`totalQuantity`，每筆送 `singleQuantity`<br />`Type2` 從 `startTime` 開始到 `endTime` 結束，每隔 `interval` 秒送一筆，總共 `totalQuantity`，剩餘張數加總至最後一筆<br />`Type3` 從 `startTime` 開始到 `endTime` 結束，每隔 `interval` 秒送一筆，總共 `totalQuantity`，剩餘張數從最後一筆往前分配.<br /><br /> |
| interval       | int               | 下單間隔時間 (秒)                                                                                                                                                                                                                                                                                                                                                                                            |
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

##### ConditionDayTrade Object[​](#conditiondaytrade-object "Direct link to ConditionDayTrade Object")

| Parameter       | Type                                      | Meaning                                            |
| --------------- | ----------------------------------------- | -------------------------------------------------- |
| dayTradeEndTime | string                                    | 收盤前沖銷時間，格式 "HHMMSS" (例 "131500")        |
| autoCancel      | bool                                      | 洗價結束後是否刪除已觸發委託                       |
| price           | string                                    | 委託價格 (若無指定價格，例如市價單，請填空字串 "") |
| priceType       | [ConditionPriceType](#conditionpricetype) | 下單價格類型                                       |

關於收盤前沖銷時間

可設定區間為 130100 ~ 132000

#### Constants ( 欄位對應數值 )[​](#constants--欄位對應數值- "Direct link to Constants ( 欄位對應數值 )")

##### BsAction[​](#bsaction "Direct link to BsAction")

買賣別

| Name | Meaning |
| ---- | ------- |
| Buy  | 買      |
| Sell | 賣      |

##### ConditionMarketType[​](#conditionmarkettype "Direct link to ConditionMarketType")

盤別

| Name        | Meaning  |
| ----------- | -------- |
| Common      | 一般盤   |
| Fixing      | 定盤     |
| IntradayOdd | 盤中零股 |
| Odd         | 盤後零股 |

##### TradingType[​](#tradingtype "Direct link to TradingType")

監控類型

| Name      | Meaning          |
| --------- | ---------------- |
| Reference | 自動參考委託物件 |
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

##### ConditionPriceType[​](#conditionpricetype "Direct link to ConditionPriceType")

價格類型 (ConditionPriceType)

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

##### ConditionOrderType[​](#conditionordertype "Direct link to ConditionOrderType")

委託類型 (orderType)

| Name   | Meaning |
| ------ | ------- |
| Stock  | 現貨    |
| Margin | 融資    |
| Short  | 融券    |

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

| 參數    | 類別                                                                                         | 說明     |
| ------- | -------------------------------------------------------------------------------------------- | -------- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#account) | 帳號     |
| guid    | string                                                                                       | 條件單號 |

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
sdk.stock.getConditionOrderById(account,"8ff3472b-185a-488c-be5a-b478deda080c")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data :
            {
                guid : "8ff3472b-185a-488c-be5a-b478deda080c",
                batchNo : "",
                orderLevel : "1",
                lastTime : "2024-03-14 12:39:02",
                conditionType : "多條件",
                parentGuid : "58e7a51e-9701-4d26-bc16-18a392a840bd",
                symbol : "2330",
                orderAmount : "0",
                childBatchNo : "",
                account : "1307515",
                conditionContent : "當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止",
                action : "下單(次日回補)",
                conditionBuySell : "現股賣",
                conditionSymbol : "台積電 現股賣",
                conditionPrice : "580元(ROD)",
                conditionVolume : "5張",
                conditionFilledVolume : "0張",
                createTime : "2024-03-14 12:39:22",
                startDate : "2024/03/14",
                status : "未生效(W)",
                detailRecordsCount : "0",
                tPSLCount : "0"
            }
}

```


---

### 歷史條件單查詢

getConditionHistory

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                       | 類別                                                                                                                         | 說明           |
| -------------------------- | ---------------------------------------------------------------------------------------------------------------------------- | -------------- |
| account                    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#account)                                 | 帳號           |
| start\_date                | string                                                                                                                       | 查詢開始日     |
| end\_date                  | string                                                                                                                       | 查詢截止日     |
| condition\_history\_status | [History Status](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#historystatus) (Optional) | 歷史條件單狀態 |

info

歷史資料日期以條件單創建日期為準

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
sdk.stock.getConditionHistory(account,"20240301","20240601")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data :[
            {
                guid : "8ff3472b-185a-488c-be5a-b478deda080c",
                batchNo : "",
                orderLevel : "1",
                lastTime : "2024-03-14 12:39:02",
                conditionType : "多條件",
                parentGuid : "58e7a51e-9701-4d26-bc16-18a392a840bd",
                symbol : "2330",
                orderAmount : "0",
                childBatchNo : "",
                account : "1307515",
                conditionContent : "當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止",
                action : "下單(次日回補)",
                conditionBuySell : "現股賣",
                conditionSymbol : "台積電 現股賣",
                conditionPrice : "580元(ROD)",
                conditionVolume : "5張",
                conditionFilledVolume : "0張",
                createTime : "2024-03-14 12:39:22",
                startDate : "2024/03/14",
                status : "未生效(W)",
                detailRecordsCount : "0",
                tPSLCount : "0"
            },
            {
                guid : "ec757279-bcb3-46f4-80ac-fccfc786bc8d",
                batchNo : ,
                orderLevel : "0",
                lastTime : "2024-05-21 10:30:00",
                conditionType : "多條件",
                parentGuid : "ec757279-bcb3-46f4-80ac-fccfc786bc8d",
                symbol : "2330",
                orderAmount : "0",
                childBatchNo : ,
                account : "1307515",
                conditionContent : "當自2024/03/14至2024/07/04內台積電成交價大於等於575元，且 台積電成交總量大於等於1001張，且 台積電時間大於等於10:30:00 部份成交為止",
                action : "下單(當沖)",
                conditionBuySell : "現股買",
                conditionSymbol : "台積電 現股買",
                conditionPrice :" 576元(ROD)",
                conditionVolume : "5張",
                conditionFilledVolume : "0張",
                createTime : "2024-03-14 12:30:02",
                startDate : "2024/03/14",
                status : "觸發-委託失敗(X)",
                errorMessage : "單價輸入錯誤[4385715]",
                detailRecordsCount : "0",
                tPSLCount : "2",
                tPSLRecord : [
                    {
                        guid : "c94b552a-22da-43e4-be44-f27a9c2026e1",
                        account : "1307515",
                        conditionContent : "當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止",
                        action : "下單(次日回補)",
                        conditionBuySell : "現股賣",
                        conditionSymbol : "台積電 現股賣",
                        conditionPrice : "580元(ROD)",
                        conditionVolume : "5張",
                        conditionFilledVolume : "0張",
                        startDate : "2024/03/14",
                        status : "未生效(W)",
                    },
                    {
                        guid : "c94b552a-22da-43e4-be44-f27a9c2026e1",
                        account : "1307515",
                        conditionContent : "當自2024/03/14至2024/07/04內台積電成交價小於等於570元 全部成交為止",
                        action : "下單(次日回補)",
                        conditionBuySell : "現股賣",
                        conditionSymbol : "台積電 現股賣",
                        conditionPrice : "市價(ROD)",
                        conditionVolume : "5張",
                        conditionFilledVolume : "0張",
                        startDate : "2024/03/14",
                        status : "未生效(W)",
                    }
                ]
            }
        ]
    }


```


---

### 條件單查詢

getConditionOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數            | 類別                                                                                                                            | 說明       |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| account         | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#account)                                    | 帳號       |
| conditionStatus | [ConditionStatus](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#conditionstatus) (Optional) | 條件單狀態 |

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
sdk.stock.getConditionOrder(account)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data :[
            {
                guid : "8ff3472b-185a-488c-be5a-b478deda080c",
                batchNo : "",
                orderLevel : "1",
                lastTime : "2024-03-14 12:39:02",
                conditionType : "多條件",
                parentGuid : "58e7a51e-9701-4d26-bc16-18a392a840bd",
                symbol : "2330",
                orderAmount : "0",
                childBatchNo : "",
                account : "1307515",
                conditionContent : "當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止",
                action : "下單(次日回補)",
                conditionBuySell : "現股賣",
                conditionSymbol : "台積電 現股賣",
                conditionPrice : "580元(ROD)",
                conditionVolume : "5張",
                conditionFilledVolume : "0張",
                createTime : "2024-03-14 12:39:22",
                startDate : "2024/03/14",
                status : "未生效(W)",
                detailRecordsCount : "0",
                tPSLCount : "0"
            },
            {
                guid : "ec757279-bcb3-46f4-80ac-fccfc786bc8d",
                batchNo : ,
                orderLevel : "0",
                lastTime : "2024-05-21 10:30:00",
                conditionType : "多條件",
                parentGuid : "ec757279-bcb3-46f4-80ac-fccfc786bc8d",
                symbol : "2330",
                orderAmount : "0",
                childBatchNo : ,
                account : "1307515",
                conditionContent : "當自2024/03/14至2024/07/04內台積電成交價大於等於575元，且 台積電成交總量大於等於1001張，且 台積電時間大於等於10:30:00 部份成交為止",
                action : "下單(當沖)",
                conditionBuySell : "現股買",
                conditionSymbol : "台積電 現股買",
                conditionPrice :" 576元(ROD)",
                conditionVolume : "5張",
                conditionFilledVolume : "0張",
                createTime : "2024-03-14 12:30:02",
                startDate : "2024/03/14",
                status : "觸發-委託失敗(X)",
                errorMessage : "單價輸入錯誤[4385715]",
                detailRecordsCount : "0",
                detailRecords : [],
                tPSLCount : "2",
                tPSLRecord : [
                    {
                        guid : "c94b552a-22da-43e4-be44-f27a9c2026e1",
                        account : "1307515",
                        conditionContent : "當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止",
                        action : "下單(次日回補)",
                        conditionBuySell : "現股賣",
                        conditionSymbol : "台積電 現股賣",
                        conditionPrice : "580元(ROD)",
                        conditionVolume : "5張",
                        conditionFilledVolume : "0張",
                        startDate : "2024/03/14",
                        status : "未生效(W)",
                        errorMessage : 
                    },
                    {
                        guid : "c94b552a-22da-43e4-be44-f27a9c2026e1",
                        account : "1307515",
                        conditionContent : "當自2024/03/14至2024/07/04內台積電成交價小於等於570元 全部成交為止",
                        action : "下單(次日回補)",
                        conditionBuySell : "現股賣",
                        conditionSymbol : "台積電 現股賣",
                        conditionPrice : "市價(ROD)",
                        conditionVolume : "5張",
                        conditionFilledVolume : "0張",
                        startDate : "2024/03/14",
                        status : "未生效(W)",
                        errorMessage : 
                    }
                ]
            }
        ]
}

```


---

### 多條件單

multiCondition

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                  | 類別                                                                                                                             | 說明                                                                             |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#account)                                     | 帳號                                                                             |
| start\_date           | string                                                                                                                           | 條件開始監控時間                                                                 |
| end\_date             | string                                                                                                                           | 條件結束監控時間                                                                 |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#stopsign)                           | 條件停止條件 :`Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| MultiCondition        | [Condition List](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#condition-object)             | 觸發條件                                                                         |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                         |

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
    symbol: "2881",
    trigger: TriggerContent.MatchedPrice,
    triggerValue: "80",
    comparison: Operator.LessThan
}

const condition2 = {
    marketType: TradingType.Reference,
    symbol: "2881",
    trigger: TriggerContent.TotalQuantity,
    triggerValue: "8000",
    comparison: Operator.GreaterThan
}

const order = {
  buySell: BSAction.Buy,
  symbol: "2881",
  price: "66",
  quantity: 2000,
  marketType: ConditionMarketType.Common,
  priceType: ConditionPriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: ConditionOrderType.Stock
};

sdk.stock.multiCondition(account, "20240426", "20240430", StopSign.Full, [condition,condition2], order)


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
* 請確認停損利**委託類別**設定需符合適合之交易規則（例如信用交易資買資賣等）

info

待主單完全成交後，停損停利部分才會啟動

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                  | 類別                                                                                                                             | 說明                                                                              |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| start\_date           | string                                                                                                                           | 條件開始監控時間                                                                  |
| end\_date             | string                                                                                                                           | 條件結束監控時間                                                                  |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#stopsign)                           | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| MultiCondition        | [Condition List](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#condition-object)             | 觸發條件                                                                          |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |
| TPSLObject            | [TPSLWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#tpslwrapper-object)              | 停損停利條件                                                                      |

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
    symbol: "2881",
    trigger: TriggerContent.MatchedPrice,
    triggerValue: "80",
    comparison: Operator.LessThan
}

const condition2 = {
    marketType: TradingType.Reference,
    symbol: "2881",
    trigger: TriggerContent.TotalQuantity,
    triggerValue: "8000",
    comparison: Operator.GreaterThan
}

const order = {
  buySell: BSAction.Buy,
  symbol: "2881",
  price: "66",
  quantity: 2000,
  marketType: ConditionMarketType.Common,
  priceType: ConditionPriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: ConditionOrderType.Stock
};

// 停損停利若為Market , price 則填空值""

const tp = {
    timeInForce: TimeInForce.ROD,
    priceType: ConditionPriceType.Limit,
    orderType: ConditionOrderType.Stock,
    targetPrice: "85",
    price: "85",
    // trigger: TriggerContent.MatchPrice  // v2.2.0 新增
}

const sl  = {
    timeInForce: TimeInForce.ROD,
    priceType: ConditionPriceType.Limit,
    orderType: ConditionOrderType.Stock,
    targetPrice: "60",
    price: "60",
    // trigger: TriggerContent.MatchPrice  // v2.2.0 新增
}

const tpsl = {
    stopSign: StopSign.Full,
    tp: tp,             // optional field 
    sl: sl,             // optional field
    endDate: "20240517", // optional field
    intraday: false     // optional field
}


sdk.stock.multiCondition(account, "20240426", "20240430", StopSign.Full, [condition,condition2], order, tpsl)

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

| 參數                  | 類別                                                                                                                             | 說明                                                                              |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| start\_date           | string                                                                                                                           | 條件開始監控時間                                                                  |
| end\_date             | string                                                                                                                           | 條件結束監控時間                                                                  |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#stopsign)                           | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| condition             | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#condition-object)                  | 觸發條件                                                                          |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |

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
    symbol: "2881",
    trigger: TriggerContent.MatchedPrice,
    triggerValue: "80",
    comparison: Operator.LessThan
}

const order = {
  buySell: BSAction.Buy,
  symbol: "2881",
  price: "66",
  quantity: 2000,
  marketType: ConditionMarketType.Common,
  priceType: ConditionPriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: ConditionOrderType.Stock
};


sdk.stock.singleCondition(account, "20240426", "20240430", StopSign.Full, condition, order)

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
* 請確認停損利**委託類別**設定需符合適合之交易規則（例如信用交易資買資賣等）

info

待主單完全成交後，停損停利部分才會啟動

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                  | 類別                                                                                                                             | 說明                                                                              |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| start\_date           | string                                                                                                                           | 條件開始監控時間                                                                  |
| end\_date             | string                                                                                                                           | 條件結束監控時間                                                                  |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#stopsign)                           | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| condition             | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#condition-object)                  | 觸發條件                                                                          |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |
| TPSLObject            | [TPSLWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#tpslwrapper-object)              | 停損停利條件                                                                      |

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
    symbol: "2881",
    trigger: TriggerContent.MatchedPrice,
    triggerValue: "80",
    comparison: Operator.LessThan
}

const order = {
  buySell: BSAction.Buy,
  symbol: "2881",
  price: "66",
  quantity: 2000,
  marketType: ConditionMarketType.Common,
  priceType: ConditionPriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: ConditionOrderType.Stock
};

// 停損停利若為Market , price 則填空值""

const tp = {
    timeInForce: TimeInForce.ROD,
    priceType: ConditionPriceType.Limit,
    orderType: ConditionOrderType.Stock,
    targetPrice: "85",
    price: "85",
    // trigger: TriggerContent.MatchPrice  // v2.2.0 新增
}

const sl  = {
    timeInForce: TimeInForce.ROD,
    priceType: ConditionPriceType.Limit,
    orderType: ConditionOrderType.Stock,
    targetPrice: "60",
    price: "60",
    // trigger: TriggerContent.MatchPrice  // v2.2.0 新增
}

const tpsl = {
    stopSign: StopSign.Full,
    tp: tp,              // optional field 
    sl: sl,              // optional field
    endDate: "20240517", // optional field
    intraday: false      // optional field
}




sdk.stock.singleCondition(account, "20240426", "20240430", StopSign.Full, condition, order, tpsl)

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

| 參數    | 類別                                                                                         | 說明             |
| ------- | -------------------------------------------------------------------------------------------- | ---------------- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#account) | 帳號             |
| batchNo | string                                                                                       | 分時分量條件單號 |

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
sdk.stock.getTimeSliceOrder(account,"1234567")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data :
            {
                guid: "c4dc90c1-4277-42ea-b585-085dc347eac0",
                batchNo: "",
                orderLevel: "0",
                lastTime: "2024-07-23 17:30:01",
                conditionType: "分時分量",
                parentGuid: "",
                symbol: "2881",
                orderAmount: "0",
                childBatchNo: "",
                account: "1307515",
                conditionContent: "當於2024/07/23 定時單時間大於等於08:40:00 全部成交為止",
                action: "下單",
                conditionBuySell: "現股買",
                conditionSymbol: "富邦金 現股買",
                conditionPrice: "66元(ROD)",
                conditionVolume: "1張",
                conditionFilledVolume: "0張",
                createTime: "2024-07-22 17:30:03",
                startDate: "2024/07/23",
                status: "條件單中止(I)",
                detailRecordsCount: "0",
                detailRecords: [],
                tpslCount: "0",
                tpslRecord: [],
            },
            {
                guid: "2975702e-f36f-4da4-bab6-1310344ec05d",
                batchNo: "",
                orderLevel: "0",
                lastTime: "2024-07-23 17:30:01",
                conditionType: "分時分量",
                parentGuid: "",
                symbol: "2881",
                orderAmount: "0",
                childBatchNo: "",
                account: "1307515",
                conditionContent: "當於2024/07/23 定時單時間大於等於09:10:00 全部成交為止",
                action: "下單",
                conditionBuySell: "現股買",
                conditionSymbol: "富邦金 現股買",
                conditionPrice: "66元(ROD)",
                conditionVolume: "1張",
                conditionFilledVolume: "0張",
                createTime: "2024-07-22 17:30:03",
                startDate: "2024/07/23",
                status: "條件單中止(I)",
                detailRecordsCount: "0",
                detailRecords: [],
                tpslCount: "0",
                tpslRecord: [],
            }
}

```


---

### 分時分量條件單

timeSliceOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                  | 類別                                                                                                                             | 說明                                                                              |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| start\_date           | string                                                                                                                           | 條件開始監控時間                                                                  |
| end\_date             | string                                                                                                                           | 條件結束監控時間                                                                  |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#stopsign)                           | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| splitDescription      | [SplitDescription](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#splitdescription)           | 觸發條件                                                                          |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |

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
    singleQuantity: 1000,
    totalQuantity: 10000,
    startTime: '083000',
};

const order = {
  buySell: BSAction.Buy,
  symbol: "2881",
  price: "66",
  quantity: 2000,
  marketType: ConditionMarketType.Common,
  priceType: ConditionPriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: ConditionOrderType.Stock
};

sdk.stock.timeSliceOrder(target_account, "20240427","20240516", StopSign.Full, split, order)


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data : {
            guid : "24080500000002"
    }
}


```


---

### 歷史移動鎖利查詢

getTrailHistory

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                         | 說明       |
| ----------- | -------------------------------------------------------------------------------------------- | ---------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#account) | 帳號       |
| start\_date | string                                                                                       | 查詢開始日 |
| end\_date   | string                                                                                       | 查詢截止日 |

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
sdk.stock.getTrailHistory(account,"20240301","20240601")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data :[
        {
            guid: "5c154a76-e7ef-4b8f-94b5-80bf08fa4b8e",
            batchNo: "",
            orderLevel: "0",
            lastTime: "2024-08-02 16:45:01",
            symbol: "2889",
            orderAmount: "0",
            childBatchNo: "",
            account: "1307515",
            conditionContent: "從2024/08/02到2024/08/02內當國票金從1000(原始基準價)，下跌達10%，持續至預定張數全部成交或期限到達為止",
            action: "下單",
            conditionBuySell: "現股買",
            conditionSymbol: "國票金 (2889)",
            conditionPrice: "成交價(1) 檔(ROD)",
            conditionVolume: "1張",
            conditionFilledVolume: "0張",
            createTime: "2024-08-02 10:07:31",
            startDate: "2024/08/02",
            status: "條件單中止(I)",
            detailRecordsCount: "0",
            detailRecords: [],
            tpslCount: "0",
            tpslRecord: [],
        },
        {
            guid: "c71117c8-aa70-4477-9f04-ff4c73a4fad0",
            batchNo: "",
            orderLevel: "0",
            lastTime: "2024-07-29 17:30:00",
            symbol: "2330",
            orderAmount: "0",
            childBatchNo: "",
            account: "1307515",
            conditionContent: "從2024/07/29到2024/07/29內當台積電從860(原始基準價)，上漲達5%，持續至預定張數全部成交或期限到達為止",
            action: "下單",
            conditionBuySell: "現股買",
            conditionSymbol: "台積電 (2330)",
            conditionPrice: "成交價(5) 檔(ROD)",
            conditionVolume: "2張",
            conditionFilledVolume: "0張",
            createTime: "2024-07-29 11:01:49",
            startDate: "2024/07/29",
            status: "條件單中止(I)",
            detailRecordsCount: "0",
            detailRecords: [],
            tpslCount: "0",
            tpslRecord: [],
        }
    ]
    }


```


---

### 移動鎖利查詢

getTrailOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                         | 說明 |
| ------- | -------------------------------------------------------------------------------------------- | ---- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#account) | 帳號 |

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
sdk.stock.getTrailOrder(account)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

{
    isSuccess: true,
    data :[
        {
            guid: "5c154a76-e7ef-4b8f-94b5-80bf08fa4b8e",
            batchNo: "",
            orderLevel: "0",
            lastTime: "2024-08-02 16:45:01",
            symbol: "2889",
            orderAmount: "0",
            childBatchNo: "",
            account: "1307515",
            conditionContent: "從2024/08/02到2024/08/02內當國票金從1000(原始基準價)，下跌達10%，持續至預定張數全部成交或期限到達為止",
            action: "下單",
            conditionBuySell: "現股買",
            conditionSymbol: "國票金 (2889)",
            conditionPrice: "成交價(1) 檔(ROD)",
            conditionVolume: "1張",
            conditionFilledVolume: "0張",
            createTime: "2024-08-02 10:07:31",
            startDate: "2024/08/02",
            status: "條件單中止(I)",
            detailRecordsCount: "0",
            detailRecords: [],
            tpslCount: "0",
            tpslRecord: [],
        },
        {
            guid: "c71117c8-aa70-4477-9f04-ff4c73a4fad0",
            batchNo: "",
            orderLevel: "0",
            lastTime: "2024-07-29 17:30:00",
            symbol: "2330",
            orderAmount: "0",
            childBatchNo: "",
            account: "1307515",
            conditionContent: "從2024/07/29到2024/07/29內當台積電從860(原始基準價)，上漲達5%，持續至預定張數全部成交或期限到達為止",
            action: "下單",
            conditionBuySell: "現股買",
            conditionSymbol: "台積電 (2330)",
            conditionPrice: "成交價(5) 檔(ROD)",
            conditionVolume: "2張",
            conditionFilledVolume: "0張",
            createTime: "2024-07-29 11:01:49",
            startDate: "2024/07/29",
            status: "條件單中止(I)",
            detailRecordsCount: "0",
            detailRecords: [],
            tpslCount: "0",
            tpslRecord: [],
        }
    ]
}

```


---

### 移動鎖利條件單

trailProfit

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                       | 說明                                                                              |
| ----------- | ---------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#account)               | 帳號                                                                              |
| start\_date | string                                                                                                     | 條件開始監控時間                                                                  |
| end\_date   | string                                                                                                     | 條件結束監控時間                                                                  |
| stop\_sign  | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#stopsign)     | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| trailOrder  | [TrailOrder](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/nodejs/EnumMatrix.txt#trailorder) | 移動鎖利條件                                                                      |

caution

TrailOrder 內之 基準價 (price) 只可輸入至多小數點後兩位，若超出將造成系統洗價失敗

info

當前股票價格價觸及觸發價時（以基準價計算之漲跌 % 數） 即下單

**例：** 若初始基準價設 100, 向下 5% 下單停損/利，初始觸發價為 95，若價格無上漲超過 100（即基準價無調整）， 則市場價為 95 時將觸發條件下單

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
    symbol: "2330",
    price: "860",
    direction: Direction.Down,
    percentage: 5,  
    buySell: BSAction.Sell,
    quantity: 2000,
    priceType: ConditionPriceType.MatchedPrice,
    diff: 5,
    timeInForce: TimeInForce.ROD,
    orderType: ConditionOrderType.Stock
    
};

sdk.stock.trailProfit(target_account, "20240427","20240516", StopSign.Full, trail)


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

| 參數    | 類別                                                                                         | 說明     |
| ------- | -------------------------------------------------------------------------------------------- | -------- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#account) | 帳號     |
| guid    | string                                                                                       | 條件單號 |

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
sdk.stock.cancel_condition_orders(account, "c9df498a-3b28-4b50-a6f2-f7bd524e96df")


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

### 當沖條件單查詢

get\_condition\_daytade\_by\_id

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                         | 說明     |
| ------- | -------------------------------------------------------------------------------------------- | -------- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#account) | 帳號     |
| guid    | string                                                                                       | 條件單號 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別              | 說明                             |
| --------- | ----------------- | -------------------------------- |
| isSuccess | bool              | 是否成功                         |
| data      | List              | 條件單回傳資訊                   |
| message   | string (optional) | 當isSuccess = false 回傳錯誤訊息 |

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
sdk.stock.get_condition_daytrade_by_id(account,"8ff3472b-185a-488c-be5a-b478deda080c")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data :
        ConditionDetail {
                guid: "8ff3472b-185a-488c-be5a-b478deda080c",
                batch_no: "",
                order_level: "1",
                last_time: "2024-03-14 12:39:02",
                condition_type: "多條件",
                parent_guid: "58e7a51e-9701-4d26-bc16-18a392a840bd",
                symbol: "2330",
                order_amount: "0",
                child_batch_no: "",
                account: "1307515",
                condition_content: "當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止",
                action: "下單(次日回補) ",
                condition_buy_sell: "現股賣",
                condition_symbol: "台積電 現股賣",
                condition_price: "580元(ROD)",
                condition_volume: "5張",
                condition_filled_volume: "0張",
                creat_time: "2024-03-14 12:39:22",
                start_date: "2024/03/14",
                status: "未生效(W)",
                error_message: None,
                detail_records_count: "0",
                detail_records: [],
                tpslCount: "0",
                tpslRecord: [],
            }
}

```


---

### 當沖多條件單

multi\_condition\_day\_trade

停損利注意事項

* 停損利設定僅為觸發送單，不保證必定回補成功，需視市場狀況自行調整
* 請確認停損利**委託類別**設定需符合當日沖銷交易規則（例如信用交易使用資券互抵）

當沖條件單查詢

* 使用 guid 查詢當沖條件單請使&#x7528;***當沖條件單查詢***&#x529F;能

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                     | 類別                                                                                                                                   | 說明                                                                              |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account                  | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#account)                                           | 帳號                                                                              |
| stop\_sign               | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#stopsign)                                 | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| end\_time                | string                                                                                                                                 | 父單洗價結束時間                                                                  |
| multicondition           | [Condition List](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#condition-object)                   | 觸發條件                                                                          |
| orderObject              | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#conditionorder-object)       | 委託內容                                                                          |
| ConditionDayTrade Object | [ConditionDayTrade Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#conditiondaytrade-object) | 當沖回補內容                                                                      |
| TPSLObject               | [TPSLWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#tpslwrapper-object)                    | 停損停利條件                                                                      |
| fixSession               | bool                                                                                                                                   | 執行定盤回補                                                                      |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別              | 說明                             |
| --------- | ----------------- | -------------------------------- |
| isSuccess | bool              | 是否成功                         |
| data      | Object            | 條件單回傳資訊                   |
| message   | string (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
#from fubon_neo.sdk import ConditionDayTrade

# 設計條件內容
condition = Condition(
    market_type = TradingType.Reference,        
    symbol = "2881",
    trigger = TriggerContent.MatchedPrice,
    trigger_value = "66",
    comparison = Operator.LessThan
)

condition2 = Condition(
    market_type = TradingType.Reference,        
    symbol = "2881",
    trigger = TriggerContent.TotalQuantity,
    trigger_value = "8000",
    comparison = Operator.LessThan
)

order = ConditionOrder(
    buy_sell = BSAction.Buy,
    symbol = "2881",
    price =  "66",
    quantity =  1000,
    market_type = ConditionMarketType.Common,
    price_type = ConditionPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = ConditionOrderType.Stock
)

daytrade_obj = ConditionDayTrade(
    day_trade_end_time = "131500",  # 收盤前沖銷時間，可設定區間為 130100 ~ 132000
    auto_cancel= True,
    price = "",
    price_type=ConditionPriceType.Market
)

tp = TPSLOrder(
    time_in_force=TimeInForce.ROD,
    price_type=ConditionPriceType.Limit,
    order_type=ConditionOrderType.Stock,
    target_price="85",
    price="85",
)

sl = TPSLOrder(
    time_in_force=TimeInForce.ROD,
    price_type=ConditionPriceType.Limit,
    order_type=ConditionOrderType.Stock,
    target_price="60",
    price="60",
)

tpsl = TPSLWrapper(
    stop_sign= StopSign.Full,
    end_date="20240517", 
    tp=tp,           
    sl=sl,           
    intraday=True  # ** 設定 True 以啟用當日沖銷洗價
)


sdk.stock.multi_condition_day_trade(account, StopSign.Full, "130000", [condition ,condition2], order, daytrade_obj, tpsl, True)

# 不設定停損停利
# sdk.stock.multi_condition_day_trade(account, StopSign.Full, "130000", [condition ,condition2], order, daytrade_obj, None, True)

# 不設定定盤回補
# sdk.stock.multi_condition_day_trade(account, StopSign.Full, "130000", [condition ,condition2], order, daytrade_obj, tpsl, False)


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

### 當沖單一條件單

single\_condition\_day\_trade

停損利注意事項

* 停損利設定僅為觸發送單，不保證必定回補成功，需視市場狀況自行調整
* 請確認停損利**委託類別**設定需符合當日沖銷交易規則（例如信用交易使用資券互抵）

當沖條件單查詢

* 使用 guid 查詢當沖條件單請使&#x7528;***當沖條件單查詢***&#x529F;能

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                     | 類別                                                                                                                             | 說明                                                                              |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account                  | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| stop\_sign               | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#stopsign)                           | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| end\_time                | string                                                                                                                           | 父單洗價結束時間                                                                  |
| condition                | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#condition-object)                  | 觸發條件                                                                          |
| orderObject              | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |
| ConditionDayTrade Object | [ConditionDayTrade](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#conditiondaytrade-object)  | 當沖回補內容                                                                      |
| TPSLObject               | [TPSLWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#tpslwrapper-object)              | 停損停利條件                                                                      |
| fixSession               | bool                                                                                                                             | 執行定盤回補                                                                      |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別              | 說明                             |
| --------- | ----------------- | -------------------------------- |
| isSuccess | bool              | 是否成功                         |
| data      | Object            | 條件單回傳資訊                   |
| message   | string (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 ConditionOrderResult 欄位[​](#委託資訊-conditionorderresult-欄位 "Direct link to 委託資訊 ConditionOrderResult 欄位")

Return type : Object

| 參數 | 類別   | 說明     |
| ---- | ------ | -------- |
| guid | string | 條件單號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
# from fubon_neo.sdk import ConditionDayTrade

# 設計條件內容
condition = Condition(
    market_type = TradingType.Reference,        
    symbol = "2881",
    trigger = TriggerContent.MatchedPrice,
    trigger_value = "66",
    comparison = Operator.LessThan
)

order = ConditionOrder(
    buy_sell = BSAction.Buy,
    symbol = "2881",
    price =  "66",
    quantity =  1000,
    market_type = ConditionMarketType.Common,
    price_type = ConditionPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = ConditionOrderType.Stock
)

daytrade_obj = ConditionDayTrade(
    day_trade_end_time = "131500",  # 收盤前沖銷時間，可設定區間為 130100 ~ 132000
    auto_cancel= True,
    price = "",
    price_type=ConditionPriceType.Market
)

tp = TPSLOrder(
    time_in_force=TimeInForce.ROD,
    price_type=ConditionPriceType.Limit,
    order_type=ConditionOrderType.Stock,
    target_price="85",
    price="85",
)

sl = TPSLOrder(
    time_in_force=TimeInForce.ROD,
    price_type=ConditionPriceType.Limit,
    order_type=ConditionOrderType.Stock,
    target_price="60",
    price="60",
)

tpsl = TPSLWrapper(
    stop_sign= StopSign.Full,
    end_date="20240517", 
    tp=tp,           
    sl=sl,           
    intraday =True  # ** 設定 true 以啟用當日沖銷洗價
)


sdk.stock.single_condition_day_trade(account, StopSign.Full, "130000", condition, order, daytrade_obj, tpsl, True)

# 不設定停損停利
# sdk.stock.single_condition_day_trade(account, StopSign.Full, "130000", condition, order, daytrade_obj, None, True)

# 不設定定盤回補
# sdk.stock.single_condition_day_trade(account, StopSign.Full, "130000", condition, order, daytrade_obj, tpsl, False)


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

1. 選擇 `Reference`時， trigger 可搭配 `BidPrice` 、`AskPrice` 、`MatchedPrice` 、`TotalQuantity` 、 `Time`，若 trigger 使用 Time，則時間條件達成，且時間範圍內商品有交易產生才觸發
2. 選擇`Scheduled` 時，symbol需填入空值""，trigger需帶入`Time`，則會使用系統時間判斷，當時間條件達成即觸發

##### ConditionOrder Object[​](#conditionorder-object "Direct link to ConditionOrder Object")

| Parameter       | Type                | Meaning                                                                                                                                                                                          |
| --------------- | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| buy\_sell       | BSAction            | [買賣別](#bsaction)                                                                                                                                                                              |
| symbol          | string              | 股票代號                                                                                                                                                                                         |
| price           | string              | 委託價格                                                                                                                                                                                         |
| quantity        | int                 | 委託數量                                                                                                                                                                                         |
| market\_type    | ConditionMarketType | [盤別](#conditionmarkettype) 可選用參數`Common` 整股、`Fixing`定盤、`IntradayOdd` 盤中零股、`Odd` 盤後零股                                                                                       |
| price\_type     | ConditionPriceType  | [價格別](#conditionpricetype) : 可選參數 :`BidPrice` 買進價、 `AskPrice` 賣出價、`MatchedPrice`成交價、`Limit` 限價、`LimitUp` 漲停、`LimitDown` 跌停、`Market` 市價、`Reference` 參考價(平盤價) |
| time\_in\_force | TimeInforce         | [委託條件](#timeinforce) : 可選參數 :`ROD`、`IOC`、`FOK`                                                                                                                                         |
| order\_type     | ConditionOrderType  | [委託類別](#conditionordertype) : 可選參數 :`Stock`現貨、`Margin`融資、`Short`融券                                                                                                               |

##### TPSLOrder Object[​](#tpslorder-object "Direct link to TPSLOrder Object")

| Parameter          | Type               | Meaning                                                                                                                                                                                          |
| ------------------ | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| time\_in\_force    | TimeInforce        | [委託條件](#timeinforce) : 可選參數 :`ROD`、`IOC`、`FOK`                                                                                                                                         |
| price\_type        | ConditionPriceType | [價格別](#conditionpricetype) : 可選參數 :`BidPrice` 買進價、 `AskPrice` 賣出價、`MatchedPrice`成交價、`Limit` 限價、`LimitUp` 漲停、`LimitDown` 跌停、`Market` 市價、`Reference` 參考價(平盤價) |
| order\_type        | ConditionOrderType | [委託類別](#conditionordertype) : 可選參數 :`Stock`現貨、`Margin`融資、`Short`融券                                                                                                               |
| target\_price      | string             | 停損/停利觸發價                                                                                                                                                                                  |
| price              | string             | 停損/停利委託價                                                                                                                                                                                  |
| trigger (optional) | TriggerContent     | 停損/停利[觸發條件](#triggercontent) （詳見表末說明）                                                                                                                                            |

info

`priceType` 若為`limit`時，`price`需填入價格；其餘price填入空字串 **或** `None` (>= ver. 2.2.4)

停損/停利觸發條件

2.2.0 以後版本新增（***2.1.1及以前版本無此欄位***），可設定停損/停利觸發價格別為 1. 成交價（MatchedPrice）；2. 最佳買價（BidPrice）；3. 最佳賣價（AskPrice）

若未設定則預設使用 成交價（MatchedPrice）

##### TPSLWrapper Object[​](#tpslwrapper-object "Direct link to TPSLWrapper Object")

| Parameter  | Type                                    | Meaning                                                                                           |
| ---------- | --------------------------------------- | ------------------------------------------------------------------------------------------------- |
| stop\_sign | StopSign                                | [停止條件](#stopsign) : 可選用 `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| tp         | TPSLOrder Object (Optional)             | 停利條件內容                                                                                      |
| sl         | TPSLOrder Object (Optional)             | 停損條件內容                                                                                      |
| end\_date  | string (Optional : 空值同母單StartDate) | 條件停止日期                                                                                      |
| intraday   | bool (Optional)                         | 全部成交後日內當沖                                                                                |

日內當沖

使用當沖條件單時，`intraday` 需設定為 `True`

##### TrailOrder[​](#trailorder "Direct link to TrailOrder")

| Parameter       | Type               | Meaning                                                                                                                                                                               |
| --------------- | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol          | string             | 股票代號                                                                                                                                                                              |
| price           | string             | 基準價 (Note. 只可輸入至多小數點後兩位，若超出將造成系統洗價失敗)                                                                                                                     |
| direction       | Direction          | [方向](#direction) : 可選用 `Up` 上漲、`Down` 下跌                                                                                                                                    |
| percentage      | int                | 上漲 / 下跌 % 數                                                                                                                                                                      |
| buysell         | BSAction           | 買賣別 : 可選用 `Buy` 買進 、`Sell` 賣出                                                                                                                                              |
| quantity        | int                | 委託股數                                                                                                                                                                              |
| price\_type     | ConditionPriceType | [價格類別](#conditionpricetype) : 可選用 `BidPrice` 買進價、`AskPrice` 賣出價、`MatchedPrice` 成交價、`Market` 市價、`LimitUp` 漲停價、`LimitDown` 跌停價、`Reference` 參考價(平盤價) |
| diff            | int                | 買賣價格檔數 (根據 priceType 加減檔數) ，正值為向上加檔數、負值為向下加檔數                                                                                                           |
| time\_in\_force | TimeInForce        | [委託條件](#timeinforce) 可選用參數為 `ROD`、`FOK`、`IOC`                                                                                                                             |
| order\_type     | ConditionOrderType | [委託類別](#conditionordertype) 可選用參數為 `Stock`現股、`Margin`融資、`Short`融券                                                                                                   |

##### SplitDescription[​](#splitdescription "Direct link to SplitDescription")

| Parameter        | Type              | Meaning                                                                                                                                                                                                                                                                                                                                                                                                               |
| ---------------- | ----------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| method           | SliceType         | [Slice Condition](#slicetype) : 可選用 :<br />`Type1` 從 `start_time` 開始，每隔幾秒送一筆，總共`total_quantity`，每筆送 `single_quantity`<br />`Type2` 從 `start_time` 開始到 `end_time` 結束，每隔 `interval` 秒送一筆，總共 `total_quantity`，剩餘張數加總至最後一筆<br />`Type3` 從 `start_time` 開始到 `end_time` 結束，每隔 `interval` 秒送一筆，總共 `total_quantity`，剩餘張數從最後一筆往前分配.<br /><br /> |
| interval         | int               | 下單間隔時間 (秒)                                                                                                                                                                                                                                                                                                                                                                                                     |
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

##### ConditionDayTrade Object[​](#conditiondaytrade-object "Direct link to ConditionDayTrade Object")

| Parameter             | Type                                      | Meaning                                            |
| --------------------- | ----------------------------------------- | -------------------------------------------------- |
| day\_trade\_end\_time | string                                    | 收盤前沖銷時間，格式 "HHMMSS" (例 "131500")        |
| auto\_cancel          | bool                                      | 洗價結束後是否刪除已觸發委託                       |
| price                 | string                                    | 委託價格 (若無指定價格，例如市價單，請填空字串 "") |
| price\_type           | [ConditionPriceType](#conditionpricetype) | 委託價格類型                                       |

關於收盤前沖銷時間

可設定區間為 130100 ~ 132000

#### Constants ( 欄位對應數值 )[​](#constants--欄位對應數值- "Direct link to Constants ( 欄位對應數值 )")

##### BsAction[​](#bsaction "Direct link to BsAction")

買賣別

| Name | Meaning |
| ---- | ------- |
| Buy  | 買      |
| Sell | 賣      |

##### ConditionMarketType[​](#conditionmarkettype "Direct link to ConditionMarketType")

盤別

| Name        | Meaning  |
| ----------- | -------- |
| Common      | 一般盤   |
| Fixing      | 定盤     |
| IntradayOdd | 盤中零股 |
| Odd         | 盤後零股 |

##### TradingType[​](#tradingtype "Direct link to TradingType")

監控類型

| Name      | Meaning          |
| --------- | ---------------- |
| Reference | 自動參考委託物件 |
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

##### ConditionPriceType[​](#conditionpricetype "Direct link to ConditionPriceType")

價格類型 (ConditionPriceType)

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

##### ConditionOrderType[​](#conditionordertype "Direct link to ConditionOrderType")

委託類型 (ConditionOrderType)

| Name   | Meaning |
| ------ | ------- |
| Stock  | 現貨    |
| Margin | 融資    |
| Short  | 融券    |

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

| 參數    | 類別                                                                                         | 說明     |
| ------- | -------------------------------------------------------------------------------------------- | -------- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#account) | 帳號     |
| guid    | string                                                                                       | 條件單號 |

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
sdk.stock.get_condition_order_by_id(account,"8ff3472b-185a-488c-be5a-b478deda080c")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data :
        ConditionDetail {
                guid: "8ff3472b-185a-488c-be5a-b478deda080c",
                batch_no: "",
                order_level: "1",
                last_time: "2024-03-14 12:39:02",
                condition_type: "多條件",
                parent_guid: "58e7a51e-9701-4d26-bc16-18a392a840bd",
                symbol: "2330",
                order_amount: "0",
                child_batch_no: "",
                account: "1307515",
                condition_content: "當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止",
                action: "下單(次日回補) ",
                condition_buy_sell: "現股賣",
                condition_symbol: "台積電 現股賣",
                condition_price: "580元(ROD)",
                condition_volume: "5張",
                condition_filled_volume: "0張",
                creat_time: "2024-03-14 12:39:22",
                start_date: "2024/03/14",
                status: "未生效(W)",
                error_message: None,
                detail_records_count: "0",
                detail_records: [],
                tpslCount: "0",
                tpslRecord: [],
            }
}

```


---

### 歷史條件單查詢

get\_condition\_history

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                       | 類別                                                                                                                         | 說明           |
| -------------------------- | ---------------------------------------------------------------------------------------------------------------------------- | -------------- |
| account                    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#account)                                 | 帳號           |
| start\_date                | string                                                                                                                       | 查詢開始日     |
| end\_date                  | string                                                                                                                       | 查詢截止日     |
| condition\_history\_status | [History Status](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#historystatus) (Optional) | 歷史條件單狀態 |

info

歷史資料日期以條件單創建日期為準

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
sdk.stock.get_condition_history(account,"20240310","20240601")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data : [
        ConditionDetail {
                guid: "8ff3472b-185a-488c-be5a-b478deda080c",
                batch_no: "",
                order_level: "1",
                last_time: "2024-03-14 12:39:02",
                condition_type: "多條件",
                parent_guid: "58e7a51e-9701-4d26-bc16-18a392a840bd",
                symbol: "2330",
                order_amount: "0",
                child_batch_no: "",
                account: "1307515",
                condition_content: "當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止",
                action: "下單(次日回補) ",
                condition_buy_sell: "現股賣",
                condition_symbol: "台積電 現股賣",
                condition_price: "580元(ROD)",
                condition_volume: "5張",
                condition_filled_volume: "0張",
                creat_time: "2024-03-14 12:39:22",
                start_date: "2024/03/14",
                status: "未生效(W)",
                error_message: None,
                detail_records_count: "0",
                detail_records: [],
                tpslCount: "0",
                tpslRecord: [],
            },
            ConditionDetail {
                guid: "8ff3472b-185a-488c-be5a-b478deda080c",
                batch_no: "",
                order_level: "1",
                last_time: "2024-03-14 12:39:02",
                condition_type: "多條件",
                parent_guid: "58e7a51e-9701-4d26-bc16-18a392a840bd",
                symbol: "2330",
                order_amount: "0",
                child_batch_no: "",
                account: "1307515",
                condition_content: "當自2024/03/14至2024/07/04內台積電成交價小於等於570元 全部成交為止",
                action: "下單(次日回補) ",
                condition_buy_sell: "現股賣",
                condition_symbol: "台積電 現股賣",
                condition_price: "市價(ROD)",
                condition_volume: "5張",
                condition_filled_volume: "0張",
                creat_time: "2024-03-14 12:39:22",
                start_date: "2024/03/14",
                status: "未生效(W)",
                error_message: None,
                detail_records_count: "0",
                detail_records: [],
                tpslCount: "0",
                tpslRecord: [],
            },
            ConditionDetail {
                guid: "ec757279-bcb3-46f4-80ac-fccfc786bc8d",
                batch_no: "",
                order_level: "0",
                last_time: "2024-05-21 10:30:00",
                condition_type: "多條件",
                parent_guid: "ec757279-bcb3-46f4-80ac-fccfc786bc8d",
                symbol: "2330",
                order_amount: "0",
                child_batch_no: "",
                account: "1307515",
                condition_content: "當自2024/03/14至2024/07/04內台積電成交價大於等於575元，且 台積電成交總量大於等於1001張，且 台積電時間大於等於10:30:00 部份成交為止",
                action: "下單(當沖) ",
                condition_buy_sell: "現股買",
                condition_symbol: "台積電 現股買",
                condition_price: "576元(ROD)",
                condition_volume: "5張",
                condition_filled_volume: "0張",
                creat_time: "2024-03-14 12:30:02",
                start_date: "2024/03/14",
                status: "觸發-委託失敗(X)",
                error_message: "單價輸入錯誤[4385715]",
                detail_records_count: "0",
                detail_records: [],
                tpslCount: "2",
                tpslRecord: [
                    ParentChildRecord {
                        guid: "c94b552a-22da-43e4-be44-f27a9c2026e1",
                        account: "1307515",
                        condition_content: "當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止",
                        action: "下單(次日回補) ",
                        condition_buy_sell: "現股賣",
                        condition_symbol: "台積電 現股賣",
                        condition_price: "580元(ROD)",
                        condition_volume: "5張",
                        condition_filled_volume: "0張",
                        start_date: "2024/03/14",
                        status: "未生效(W)",
                        error_message: "",
                    },
                    ParentChildRecord {
                        guid: "c94b552a-22da-43e4-be44-f27a9c2026e1",
                        account: "1307515",
                        condition_content: "當自2024/03/14至2024/07/04內台積電成交價小於等於570元 全部成交為止",
                        action: "下單(次日回補) ",
                        condition_buy_sell: "現股賣",
                        condition_symbol: "台積電 現股賣",
                        condition_price: "市價(ROD)",
                        condition_volume: "5張",
                        condition_filled_volume: "0張",
                        start_date: "2024/03/14",
                        status: "未生效(W)",
                        error_message: "",
                    },
                ],
            }
        ]       
    }

```


---

### 條件單查詢

get\_condition\_order

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數              | 類別                                                                                                                            | 說明       |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| account           | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#account)                                    | 帳號       |
| condition\_status | [ConditionStatus](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#conditionstatus) (Optional) | 條件單狀態 |

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
sdk.stock.get_condition_order(account)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data : [
        ConditionDetail {
                guid: "8ff3472b-185a-488c-be5a-b478deda080c",
                batch_no: "",
                order_level: "1",
                last_time: "2024-03-14 12:39:02",
                condition_type: "多條件",
                parent_guid: "58e7a51e-9701-4d26-bc16-18a392a840bd",
                symbol: "2330",
                order_amount: "0",
                child_batch_no: "",
                account: "1307515",
                condition_content: "當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止",
                action: "下單(次日回補) ",
                condition_buy_sell: "現股賣",
                condition_symbol: "台積電 現股賣",
                condition_price: "580元(ROD)",
                condition_volume: "5張",
                condition_filled_volume: "0張",
                creat_time: "2024-03-14 12:39:22",
                start_date: "2024/03/14",
                status: "未生效(W)",
                error_message: None,
                detail_records_count: "0",
                detail_records: [],
                tpslCount: "0",
                tpslRecord: [],
            },
            ConditionDetail {
                guid: "8ff3472b-185a-488c-be5a-b478deda080c",
                batch_no: "",
                order_level: "1",
                last_time: "2024-03-14 12:39:02",
                condition_type: "多條件",
                parent_guid: "58e7a51e-9701-4d26-bc16-18a392a840bd",
                symbol: "2330",
                order_amount: "0",
                child_batch_no: "",
                account: "1307515",
                condition_content: "當自2024/03/14至2024/07/04內台積電成交價小於等於570元 全部成交為止",
                action: "下單(次日回補) ",
                condition_buy_sell: "現股賣",
                condition_symbol: "台積電 現股賣",
                condition_price: "市價(ROD)",
                condition_volume: "5張",
                condition_filled_volume: "0張",
                creat_time: "2024-03-14 12:39:22",
                start_date: "2024/03/14",
                status: "未生效(W)",
                error_message: None,
                detail_records_count: "0",
                detail_records: [],
                tpslCount: "0",
                tpslRecord: [],
            },
            ConditionDetail {
                guid: "ec757279-bcb3-46f4-80ac-fccfc786bc8d",
                batch_no: "",
                order_level: "0",
                last_time: "2024-05-21 10:30:00",
                condition_type: "多條件",
                parent_guid: "ec757279-bcb3-46f4-80ac-fccfc786bc8d",
                symbol: "2330",
                order_amount: "0",
                child_batch_no: "",
                account: "1307515",
                condition_content: "當自2024/03/14至2024/07/04內台積電成交價大於等於575元，且 台積電成交總量大於等於1001張，且 台積電時間大於等於10:30:00 部份成交為止",
                action: "下單(當沖) ",
                condition_buy_sell: "現股買",
                condition_symbol: "台積電 現股買",
                condition_price: "576元(ROD)",
                condition_volume: "5張",
                condition_filled_volume: "0張",
                creat_time: "2024-03-14 12:30:02",
                start_date: "2024/03/14",
                status: "觸發-委託失敗(X)",
                error_message: "單價輸入錯誤[4385715]",
                detail_records_count: "0",
                detail_records: [],
                tpslCount: "2",
                tpslRecord: [
                    ParentChildRecord {
                        guid: "c94b552a-22da-43e4-be44-f27a9c2026e1",
                        account: "1307515",
                        condition_content: "當自2024/03/14至2024/07/04內台積電成交價大於等於580元 全部成交為止",
                        action: "下單(次日回補) ",
                        condition_buy_sell: "現股賣",
                        condition_symbol: "台積電 現股賣",
                        condition_price: "580元(ROD)",
                        condition_volume: "5張",
                        condition_filled_volume: "0張",
                        start_date: "2024/03/14",
                        status: "未生效(W)",
                        error_message: "",
                    },
                    ParentChildRecord {
                        guid: "c94b552a-22da-43e4-be44-f27a9c2026e1",
                        account: "1307515",
                        condition_content: "當自2024/03/14至2024/07/04內台積電成交價小於等於570元 全部成交為止",
                        action: "下單(次日回補) ",
                        condition_buy_sell: "現股賣",
                        condition_symbol: "台積電 現股賣",
                        condition_price: "市價(ROD)",
                        condition_volume: "5張",
                        condition_filled_volume: "0張",
                        start_date: "2024/03/14",
                        status: "未生效(W)",
                        error_message: "",
                    },
                ],
            }
        ]
}

```


---

### 多條件單

multi\_condition

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                  | 類別                                                                                                                             | 說明                                                                              |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| start\_date           | string                                                                                                                           | 條件開始監控時間                                                                  |
| end\_date             | string                                                                                                                           | 條件結束監控時間                                                                  |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#stopsign)                           | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| MultiCondition        | [Condition Array](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#condition-object)            | 觸發條件                                                                          |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |

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
    symbol = "2881",
    trigger = TriggerContent.MatchedPrice,
    trigger_value = "80",
    comparison = Operator.LessThan
)

condition2 = Condition(
    market_type = TradingType.Reference,        
    symbol = "2881",
    trigger = TriggerContent.TotalQuantity,
    trigger_value = "8000",
    comparison = Operator.LessThan
)

order = ConditionOrder(
    buy_sell = BSAction.Buy,
    symbol = "2881",
    price =  "66",
    quantity =  1000,
    market_type = ConditionMarketType.Common,
    price_type = ConditionPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = ConditionOrderType.Stock
)


sdk.stock.multi_condition(account, "20240426", "20240430", StopSign.Full, [condition, condition2], order)


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
* 請確認停損利**委託類別**設定需符合適合之交易規則（例如信用交易資買資賣等）

info

待主單完全成交後，停損停利部分才會啟動

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                  | 類別                                                                                                                             | 說明                                                                             |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#account)                                     | 帳號                                                                             |
| start\_date           | string                                                                                                                           | 條件開始監控時間                                                                 |
| end\_date             | string                                                                                                                           | 條件結束監控時間                                                                 |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#stopsign)                           | 條件停止條件 :`Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| MultiCondition        | [Condition List](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#condition-object)             | 觸發條件                                                                         |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                         |
| TPSLObject            | [TPSLWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#tpslwrapper-object)              | 停損停利條件                                                                     |

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
    symbol = "2881",
    trigger = TriggerContent.MatchedPrice,
    trigger_value = "66",
    comparison = Operator.LessThan
)

condition2 = Condition(
    market_type = TradingType.Reference,        
    symbol = "2881",
    trigger = TriggerContent.TotalQuantity,
    trigger_value = "8000",
    comparison = Operator.LessThan
)

order = ConditionOrder(
    buy_sell = BSAction.Buy,
    symbol = "2881",
    price =  "66",
    quantity =  1000,
    market_type = ConditionMarketType.Common,
    price_type = ConditionPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = ConditionOrderType.Stock,
)

# 停損停利若為Market , price 則填空值""

tp = TPSLOrder(
    time_in_force=TimeInForce.ROD,
    price_type=ConditionPriceType.Limit,
    order_type=ConditionOrderType.Stock,
    target_price="85",
    price="85",
    # trigger=TriggerContent.MatchedPrice  # v2.2.0 新增
)


sl = TPSLOrder(
    time_in_force=TimeInForce.ROD,
    price_type=ConditionPriceType.Limit,
    order_type=ConditionOrderType.Stock,
    target_price="60",
    price="60",
    # trigger=TriggerContent.MatchedPrice  # v2.2.0 新增
)

tpsl = TPSLWrapper(
    stop_sign= StopSign.Full,
    tp=tp,              # optional field 
    sl=sl,              # optional field
    end_date="20240517", # optional field
    intraday =False     # optional field
)


sdk.stock.multi_condition(account, "20240426", "20240430", StopSign.Full, [condition,condition2], order, tpsl)


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

| 參數                  | 類別                                                                                                                             | 說明                                                                              |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| start\_date           | string                                                                                                                           | 條件開始監控時間                                                                  |
| end\_date             | string                                                                                                                           | 條件結束監控時間                                                                  |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#stopsign)                           | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| condition             | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#condition-object)                  | 觸發條件                                                                          |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |

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
    symbol = "2881",
    trigger = TriggerContent.MatchedPrice,
    trigger_value = "80",
    comparison = Operator.LessThan
)

order = ConditionOrder(
    buy_sell= BSAction.Sell,
    symbol = "2881",
    quantity = 1000,
    price = "60",
    market_type = ConditionMarketType.Common,
    price_type = ConditionPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = ConditionOrderType.Stock
)

res = sdk.stock.single_condition(account, "20240427","20240516", StopSign.Full , condition, order)


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
* 請確認停損利**委託類別**設定需符合適合之交易規則（例如信用交易資買資賣等）

info

待主單完全成交後，停損停利部分才會啟動

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                  | 類別                                                                                                                             | 說明                                                                              |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| start\_date           | string                                                                                                                           | 條件開始監控時間                                                                  |
| end\_date             | string                                                                                                                           | 條件結束監控時間                                                                  |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#stopsign)                           | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| condition             | [Condition](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#condition-object)                  | 觸發條件                                                                          |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |
| TPSLObject            | [TPSLWrapper](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#tpslwrapper-object)              | 停損停利條件                                                                      |

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
    symbol = "2881",
    trigger = TriggerContent.MatchedPrice,
    trigger_value = "66",
    comparison = Operator.LessThan
)

order = ConditionOrder(
    buy_sell = BSAction.Buy,
    symbol = "2881",
    price =  "66",
    quantity =  1000,
    market_type = ConditionMarketType.Common,
    price_type = ConditionPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = ConditionOrderType.Stock
)

# 停損停利若為Market , price 則填空值""

tp = TPSLOrder(
    time_in_force=TimeInForce.ROD,
    price_type=ConditionPriceType.Limit,
    order_type=ConditionOrderType.Stock,
    target_price="85",
    price="85",
    # trigger=TriggerContent.MatchedPrice  # v2.2.0 新增
)


sl = TPSLOrder(
    time_in_force=TimeInForce.ROD,
    price_type=ConditionPriceType.Limit,
    order_type=ConditionOrderType.Stock,
    target_price="60",
    price="60",
    # trigger=TriggerContent.MatchedPrice  # v2.2.0 新增
)

tpsl = TPSLWrapper(
    stop_sign= StopSign.Full,
    tp=tp,               # optional field 
    sl=sl,               # optional field
    end_date="20240517", # optional field
    intraday =False       # optional field
)


sdk.stock.single_condition(account, "20240426", "20240430", StopSign.Full, condition, order, tpsl)


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

| 參數      | 類別                                                                                         | 說明             |
| --------- | -------------------------------------------------------------------------------------------- | ---------------- |
| account   | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#account) | 帳號             |
| batch\_no | string                                                                                       | 分時分量條件單號 |

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
sdk.stock.get_time_slice_order(account,"123456")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data :
        ConditionDetail {
            guid: "c4dc90c1-4277-42ea-b585-085dc347eac0",
            batch_no: "",
            order_level: "0",
            last_time: "2024-07-23 17:30:01",
            condition_type: "分時分量",
            parent_guid: "",
            symbol: "2881",
            order_amount: "0",
            child_batch_no: "",
            account: "1307515",
            condition_content: "當於2024/07/23 定時單時間大於等於08:40:00 全部成交為止",
            action: "下單",
            condition_buy_sell: "現股買",
            condition_symbol: "富邦金 現股買",
            condition_price: "66元(ROD)",
            condition_volume: "1張",
            condition_filled_volume: "0張",
            create_time: "2024-07-22 17:30:03",
            start_date: "2024/07/23",
            status: "條件單中止(I)",
            error_message: None,
            detail_records_count: "0",
            detail_records: [],
            TPSLCount: "0",
            TPSLRecord: [],
        }, ConditionDetail {
            guid: "2975702e-f36f-4da4-bab6-1310344ec05d",
            batch_no: "",
            order_level: "0",
            last_time: "2024-07-23 17:30:01",
            condition_type: "分時分量",
            parent_guid: "",
            symbol: "2881",
            order_amount: "0",
            child_batch_no: "",
            account: "1307515",
            condition_content: "當於2024/07/23 定時單時間大於等於09:10:00 全部成交為止",
            action: "下單",
            condition_buy_sell: "現股買",
            condition_symbol: "富邦金 現股買",
            condition_price: "66元(ROD)",
            condition_volume: "1張",
            condition_filled_volume: "0張",
            create_time: "2024-07-22 17:30:03",
            start_date: "2024/07/23",
            status: "條件單中止(I)",
            error_message: None,
            detail_records_count: "0",
            detail_records: [],
            TPSLCount: "0",
            TPSLRecord: [],
        }
}

```


---

### 分時分量條件單

time\_slice\_order

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數                  | 類別                                                                                                                             | 說明                                                                              |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| account               | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#account)                                     | 帳號                                                                              |
| start\_date           | string                                                                                                                           | 條件開始監控時間                                                                  |
| end\_date             | string                                                                                                                           | 條件結束監控時間                                                                  |
| stop\_sign            | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#stopsign)                           | 條件停止條件 : `Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| split\_description    | [SplitDescription](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#splitdescription)           | 觸發條件                                                                          |
| ConditionOrder Object | [ConditionOrder Object](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#conditionorder-object) | 委託內容                                                                          |

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
    single_quantity= 1000,
    total_quantity= 10000,
    start_time='083000',
)

order = ConditionOrder(
    buy_sell = BSAction.Buy,
    symbol = "2881",
    price =  "66",
    quantity =  1000,
    market_type = ConditionMarketType.Common,
    price_type = ConditionPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = ConditionOrderType.Stock
)


sdk.stock.time_slice_order(target_account, "20240427","20240516", StopSign.Full, split, order)


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data : ConditionOrderResult {
            guid : "24080500000002"
        }
}


```


---

### 歷史移動鎖利查詢

get\_trail\_history

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                         | 說明       |
| ----------- | -------------------------------------------------------------------------------------------- | ---------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#account) | 帳號       |
| start\_date | string                                                                                       | 查詢開始日 |
| end\_date   | string                                                                                       | 查詢截止日 |

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
sdk.stock.get_trail_history(account,"20240310","20240601")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data : [
       ConditionDetail {
            guid: "5c154a76-e7ef-4b8f-94b5-80bf08fa4b8e",
            batch_no: "",
            order_level: "0",
            last_time: "2024-08-02 16:45:01",
            condition_type: None,
            parent_guid: None,
            symbol: "2889",
            order_amount: "0",
            child_batch_no: "",
            account: "1307515",
            condition_content: "從2024/08/02到2024/08/02內當國票金從1000(原始基準價)，下跌達10%，持續至預定張數全部成交或期限到達為止",
            action: "下單",
            condition_buy_sell: "現股買",
            condition_symbol: "國票金 (2889)",
            condition_price: "成交價(1) 檔(ROD)",
            condition_volume: "1張",
            condition_filled_volume: "0張",
            create_time: "2024-08-02 10:07:31",
            start_date: "2024/08/02",
            status: "條件單中止(I)",
            error_message: None,
            detail_records_count: "0",
            detail_records: [],
            TPSLCount: "0",
            TPSLRecord: [],
        }, ConditionDetail {
            guid: "c71117c8-aa70-4477-9f04-ff4c73a4fad0",
            batch_no: "",
            order_level: "0",
            last_time: "2024-07-29 17:30:00",
            condition_type: None,
            parent_guid: None,
            symbol: "2330",
            order_amount: "0",
            child_batch_no: "",
            account: "1307515",
            condition_content: "從2024/07/29到2024/07/29內當台積電從860(原始基準價)，上漲達5%，持續至預定張數全部成交或期限到達為止",
            action: "下單",
            condition_buy_sell: "現股買",
            condition_symbol: "台積電 (2330)",
            condition_price: "成交價(5) 檔(ROD)",
            condition_volume: "2張",
            condition_filled_volume: "0張",
            create_time: "2024-07-29 11:01:49",
            start_date: "2024/07/29",
            status: "條件單中止(I)",
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

### 有效移動鎖利查詢

get\_trail\_order

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                         | 說明 |
| ------- | -------------------------------------------------------------------------------------------- | ---- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#account) | 帳號 |

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
sdk.stock.get_trail_order(account)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

Result {
    is_success: True,
    message: None,
    data : [
        ConditionDetail {
            guid: "5c154a76-e7ef-4b8f-94b5-80bf08fa4b8e",
            batch_no: "",
            order_level: "0",
            last_time: "2024-08-02 16:45:01",
            condition_type: None,
            parent_guid: None,
            symbol: "2889",
            order_amount: "0",
            child_batch_no: "",
            account: "1307515",
            condition_content: "從2024/08/02到2024/08/02內當國票金從1000(原始基準價)，下跌達10%，持續至預定張數全部成交或期限到達為止",
            action: "下單",
            condition_buy_sell: "現股買",
            condition_symbol: "國票金 (2889)",
            condition_price: "成交價(1) 檔(ROD)",
            condition_volume: "1張",
            condition_filled_volume: "0張",
            create_time: "2024-08-02 10:07:31",
            start_date: "2024/08/02",
            status: "條件單中止(I)",
            error_message: None,
            detail_records_count: "0",
            detail_records: [],
            TPSLCount: "0",
            TPSLRecord: [],
        }, ConditionDetail {
            guid: "c71117c8-aa70-4477-9f04-ff4c73a4fad0",
            batch_no: "",
            order_level: "0",
            last_time: "2024-07-29 17:30:00",
            condition_type: None,
            parent_guid: None,
            symbol: "2330",
            order_amount: "0",
            child_batch_no: "",
            account: "1307515",
            condition_content: "從2024/07/29到2024/07/29內當台積電從860(原始基準價)，上漲達5%，持續至預定張數全部成交或期限到達為止",
            action: "下單",
            condition_buy_sell: "現股買",
            condition_symbol: "台積電 (2330)",
            condition_price: "成交價(5) 檔(ROD)",
            condition_volume: "2張",
            condition_filled_volume: "0張",
            create_time: "2024-07-29 11:01:49",
            start_date: "2024/07/29",
            status: "條件單中止(I)",
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

### 移動鎖利條件單

trail\_profit

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數         | 類別                                                                                                       | 說明                                                                             |
| ------------ | ---------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| account      | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#account)               | 帳號                                                                             |
| start\_date  | string                                                                                                     | 條件開始監控時間                                                                 |
| end\_date    | string                                                                                                     | 條件結束監控時間                                                                 |
| stop\_sign   | [StopSign](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#stopsign)     | 條件停止條件 :`Full` 全部成交為止、`Partial`部分成交為止、`UntilEnd`效期結束為止 |
| trail\_order | [TrailOrder](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/library/python/EnumMatrix.txt#trailorder) | 移動鎖利條件                                                                     |

caution

TrailOrder 內之 基準價 (price) 只可輸入至多小數點後兩位，若超出將造成系統洗價失敗

info

當前股票價格價觸及觸發價時（以基準價計算之漲跌 % 數） 即下單

**例：** 若初始基準價設 100, 向下 5% 下單停損/利，初始觸發價為 95，若價格無上漲超過 100（即基準價無調整）， 則市場價為 95 時將觸發條件下單

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
trail = TrailOrder(
    symbol = "2330",
    price = "860",
    direction = Direction.Up ,
    percentage = 5,  # 漲跌 % 數
    buy_sell = BSAction.Buy,
    quantity = 2000,
    price_type = ConditionPriceType.MatchedPrice,
    diff = 5,     # 向上 or 向下追買 tick數 (向下為負值)
    time_in_force = TimeInForce.ROD,
    order_type = ConditionOrderType.Stock
)

sdk.stock.trail_profit(target_account, "20240427","20240516", StopSign.Full, trail)


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

### 事前準備

本頁重點

* 使用條件單前，需先閱讀使用說明並完成簽署。
* SDK 版本需升級至 v1.3 以上。
* 支援 Python / Node.js 條件單套件引入。

| 項目     | 說明                                     |
| -------- | ---------------------------------------- |
| 產品     | 富邦新一代 API 條件單                    |
| 必要前置 | 閱讀使用說明、簽署條件單同意書、升級 SDK |
| 支援語言 | Python / C# / JavaScript (Node.js)       |
| 套件引入 | 僅 Python / JS 需額外引入套件            |

caution

在開始富邦新一代 API 條件單前，請先詳細閱讀 [**使用說明**](https://www.fbs.com.tw/TradeAPI/docs/smart-condition/Agreements.txt) 並完成簽署條件單使用同意書

#### 簽署條件單使用同意書[​](#簽署條件單使用同意書 "Direct link to 簽署條件單使用同意書")

使用您的富邦證券帳戶

1. 開啟您的富邦Online手機APP ，點選更多<br />![線上簽署1](/TradeAPI/assets/images/signagree1-344762a81c78861fa652f3bd8d11d735.png)

2. 點選 `M櫃台`<br />![線上簽署2](/TradeAPI/assets/images/signagree2-f228860054133c73f3f12e482545cc55.png)

3. 點選條件單同意書<br />![線上簽署3](/TradeAPI/assets/images/signagree3-ea03b7b52731323e65fe28d393dd2432.png)

#### 升級您的SDK 版本至v1.3以上[​](#升級您的sdk-版本至v13以上 "Direct link to 升級您的SDK 版本至v1.3以上")

SDK 最新版本 : [下載連結](https://www.fbs.com.tw/TradeAPI/docs/download/download-sdk.txt)

#### 引入套件 ( 僅Python、JS需要，C#不需額外操作)[​](#引入套件--僅pythonjs需要c不需額外操作 "Direct link to 引入套件 ( 僅Python、JS需要，C#不需額外操作)")

引入條件單相關套件

* Python
* Node.js

```py
from fubon_neo.sdk import FubonSDK, Condition, ConditionDayTrade, ConditionOrder
from fubon_neo.constant import ( 
    TriggerContent, TradingType, Operator, TPSLOrder, TPSLWrapper, SplitDescription,
    StopSign, TimeSliceOrderType, ConditionMarketType, ConditionPriceType, ConditionOrderType, TrailOrder, Direction, ConditionStatus, HistoryStatus
)


```

```js
const { FubonSDK, TriggerContent, TradingType, Operator, TPSLOrder, TPSLWrapper, SplitDescription,
    StopSign, TimeSliceOrderType, ConditionMarketType, ConditionPriceType, ConditionOrderType, TrailOrder, Direction, ConditionStatus, HistoryStatus } = require('fubon-neo');


```

info

條件單目前不支援期權商品與現貨商品混用


---

### 緊急應變措施

若有因不可抗力之因素或系統異常，而造成無法下單或確認委託成交狀況時，請使用其他平台作為備援

條件單替代平台:

* HTS2
* Online APP
* 營業員

更多訊息請洽富邦證券官網[網路下單系統無法正常使用之應變措施](https://www.fbs.com.tw/Beginner/TradingNote/)


---

### 停損停利單

富邦新一代 API 支援條件下單後的停損停利單，

停損停利運作機制 : 當觸發的條件單，觸發後，並委託成交，就會啟動停損停利的監控機制，而當停利條件達成時，停損就會失效；反之亦然 (同一般市面上常見OCO)

![TPSL](/TradeAPI/assets/images/TPSL-c929bfde158264f3327c48f316c32bde.png)

#### 程式範例[​](#程式範例 "Direct link to 程式範例")

* Python
* Node.js
* C#

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
    endDate: "20240517"
}


sdk.futopt.singleCondition(account, "20240426", "20240430", StopSign.Full, condition, order, tpsl)

```

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
    BSAction.Buy,
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


---

### 移動鎖利條件單

富邦新一代 API 支援移動鎖利單，

移動鎖利運作機制 : 當基準條件觸發後，若設定的回檔幅度未達到，則不會觸發，當後續股價持續往上，則會使用新的價格作為基準，若此時回檔幅度到達設定值，則送出委託

![trailProfit](/TradeAPI/assets/images/TrailProfit-62f309ff8f8104114c69c2fc69bb96d1.png)

##### 程式範例[​](#程式範例 "Direct link to 程式範例")

* Python
* Node.js
* C#

```py
# 設計條件內容
trail = FutOptTrailOrder(
    symbol = "TXFL4",
    price = "22000",
    direction = Direction.Down ,
    tick_num = 5,  # 漲跌 tick 數
    buy_sell = BSAction.Sell,
    lot = 2,
    price_type = FutOptConditionPriceType.MatchedPrice,
    diff = 1,     # 向上 or 向下追買 tick數 (向下為負值)
    time_in_force = TimeInForce.ROD,
    order_type = FutOptConditionOrderType.Close
)


sdk.futopt.trail_profit(target_account, "20240427","20240516", StopSign.Full, trail)


```

```js
// 設計條件內容
const trail = {
    symbol: "TXO20000E4",
    price: "1050",
    direction: Direction.Down,
    tickNum: 5,  
    buysell: BSAction.Sell,
    lot: 2,
    priceType: FutOptConditionPriceType.MatchedPrice,
    diff: 1,
    timeInForce: TimeInForce.ROD,
    orderType: FutOptConditionOrderType.Close
    
};


sdk.futopt.trailProfit(target_account, "20240427","20240516", StopSign.Full, trail)


```

```cs
// 設計條件內容
var trail = new FutOptTrailOrder(
    "TXO20000E4",
    "1050",
    Direction.Up ,
    5,  // 漲跌 tick 數
    BSAction.Buy,
    2,
    FutOptConditionPriceType.MatchedPrice,
    1, // 向上 or 向下追買 tick數 (向下為負值)
    TimeInForce.Rod,
    FutOptConditionOrderType.Close
);


sdk.FutOpt.TrailProfit(target_account, "20240427","20240516", StopSign.Full, trail);


```


---

### 條件單風險聲明

本公司為服務投資人，提供網路下單客戶得預先設定下單條件之條件下單功能服務（以下簡稱本功能或本服務）。本服務不收取任何費用，但本公司保留得隨時調整、修正、暫停或終止此項服務之權利；為確保使用者權益及網路交易安全秩序，請投資人於啟用本功能前，詳細閱讀以下條款，以保障投資人的權利，在投資人點選本使用同意書下方之同意鈕，即視為已經詳閱並接受本同意書之內容。<br /><!-- -->一、條件下單功能服務具有以下（但不限於此）之風險，投資人應自承以下風險所衍生之損害或損失：。<br /><!-- -->1 網際網路之傳輸通訊，可能因不可抗力或不可歸責於本公司之事由，包括（但不在此限）斷電、斷線、網路壅塞等因素，導致資料傳輸延遲、停滯、無法發出或無法接收。<br /><!-- -->2 本功能或服務有可能因病毒、系統或機件發生故障、錯誤、執行障礙，駭客入侵等因素，導致資料或資訊傳送發生遺漏、錯誤、或遭竄改。<br /><!-- -->3 其他網路交易之風險。<br /><!-- -->二、條件下單功能服務，係依據投資人設定之個股或期貨商品種類及下單條件，提供即時有效之資訊，並結合既有之網路下單功能，允許投資人在同一網頁操作證券或期貨下單之委託指示。本公司目前所提供之條件下單功能服務包括觸價下單、觸量下單、定時下單、長效停損/停利下單及多條件下單共五種（簡稱為條件下單），不同種類的條件下單，其設定方式、效力有可能不同，或受有特別限制，投資人於設定每一筆條件下單前應詳細閱讀注意事項，審慎為之。<br /><!-- -->三、經觸發條件下單功能服務「直接下單」功能鍵後，具有與口頭、電話、傳真、網路下單相同之效力，如經成交即不得取消，投資人負有交割義務，以及證券、期貨相關法令所規定之各項責任。<br /><!-- -->四、投資人得於成交前以網路或電話方式變更或取消委託，投資人於下單前應充分瞭解有關變更或取消委託之操作。因不可抗力或不可歸責於本公司之事由，包括（但不在此限）斷電、斷線、網路壅塞、傳輸干擾等因素，致變更或取消意思表示無法即時送達之風險，由投資人自行負擔。<br /><!-- -->五、基於網路維運、符合資訊法規及網路規範、確保網路交易安全，或服從主管機關之命令等因素，對於有礙系統正常運作、或可能造成系統資訊不穩定、或其他異常狀況之條件下單，本公司保留刪除之權利。如經決定刪除，本公司將另以電子平台公告、行動簡訊或電子郵件通知投資人，並請投資人至富邦證券網站、HTS快易點、HTS2多易點及Online查詢條件下單訊息公告。<br /><!-- -->六、經公告於本公司網站上之其他有關電子下單、客戶資料使用、網站管理、保密措施等規範，於條件下單功能服務均有其適用。如有未盡事宜，悉照中華民國相關法令以及經公告之網際網路交易規範為準據。<br /><!-- -->七、請投資人依本公司條件下單之下單規則，於完成有效期間條件下單之設定後， 請於該條件下單之有效期間內之每一營業日當日查詢該條件下單狀態；條件下單之詳細狀態僅保留至當日，若投資人對條件下單狀態有疑慮，請於該筆交易當日洽詢所屬分公司營業同仁。如交易當日條件下單有故障或因不可抗力因素導致系統無法送單情形，則該交易視為未下單處理。<br /><!-- -->注意事項：

1. 如使用條件單訂閱功能，請務必確認各項訂閱條件內容，避免產生錯誤。另提醒投資人，此為訂閱功能，待條件符合，才會觸發下單，如欲查詢訂閱狀況可至富邦Online/HTS2多易點/HTS快易點。
2. 就前述訂閱功能，投資人可設定條件單觸發後，系統以e-mail發送觸發通知給投資人，或直接下單。如不熟悉條件單操作流程，建議設定觸發通知，待熟悉後再選擇直接下單。
3. 如使用定時單下最後一盤，建議設定停盤丟單，因條件單主機與交易所主機不同，雖然固定每3分鐘進行1次網路校時，但難免出現秒差（如設定13:25下單，可能會下在13:24:59），請留意。
4. 條件單接受比對A商品下B商品，設定A商品時會自動帶委託A商品，要委託B商品請手動調整。
5. 當投資人設定之行情條件觸發且本系統送出投資人設定之委託單後，本公司會將該委託單送至交易所，該委託單將會轉為一般委託，此時無法取消訂閱該筆條件單，請直接使用下單查詢功能，查找觸發後的一般委託進行改單調整。
6. 條件單訂閱冷門商品，有可能會碰到開盤沒報價或價格穩定措施3分鐘無行情，這時條件單會進入等價狀況，待有行情才會開始繼續洗價比對。
7. 請留意，使用條件單訂閱功能時，待觸發送單後才會與本公司後台風控進行比對。
8. 請留意，證期權多條件長效單、移動鎖利條件單、庫存停損利條件單之效期最長為3個月，定時定額條件單效期最長為6個月。
9. 其餘未盡事宜請參閱各條件單介面下方注意事項，確認閱讀內容完畢後始得訂閱條件單。


---

### 錯誤碼與狀態碼對照表

本頁面彙整了富邦新一代 API（期貨條件單）中常見的狀態碼，方便開發者在遇到問題或進行狀態判斷時快速檢索。

#### 條件單狀態 (ConditionStatus)[​](#條件單狀態-conditionstatus "Direct link to 條件單狀態 (ConditionStatus)")

條件單的當前狀態。

| 狀態名稱 | 說明         |
| -------- | ------------ |
| `Type1`  | 今日相關查詢 |
| `Type2`  | 尚有效單     |
| `Type3`  | 條件比對中   |
| `Type4`  | 委託處理中   |
| `Type5`  | 委託成功     |
| `Type6`  | 已通知       |
| `Type7`  | 委託失敗     |
| `Type8`  | 已有成交     |
| `Type9`  | 刪除成功     |
| `Type10` | 異常         |
| `Type11` | 失效         |

#### 歷史條件單狀態 (HistoryStatus)[​](#歷史條件單狀態-historystatus "Direct link to 歷史條件單狀態 (HistoryStatus)")

歷史條件單的狀態。

| 狀態名稱 | 說明                            |
| -------- | ------------------------------- |
| `Type1`  | 所有條件單 (不包含已刪除、失效) |
| `Type2`  | 選擇期間內全部成交單            |
| `Type3`  | 選擇期間內部分成交單            |
| `Type4`  | 選擇期間刪除單                  |
| `Type5`  | 選擇期間失效單                  |
| `Type6`  | 選擇期間內已觸發記錄            |


---

