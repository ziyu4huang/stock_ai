#選擇權
sdk.futopt.convert_symbol("TXO","202404",20000,CallPut.Call)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py

#期貨
"TXFD4"

#選擇權
"TXO20000D4"

```


---

### 商品保證金查詢

query\_estimate\_margin

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數          | 類別                                                                                                        | 說明     |
| ------------- | ----------------------------------------------------------------------------------------------------------- | -------- |
| account       | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)         | 帳號     |
| order\_object | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#orderobject) | 委託物件 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別           | 說明                               |
| ----------- | -------------- | ---------------------------------- |
| is\_success | bool           | 是否成功                           |
| data        | EstimateMargin | 回傳保證金資訊                     |
| message     | string         | 當is\_success = false 回傳錯誤訊息 |

##### EstimateMargin 欄位[​](#estimatemargin-欄位 "Direct link to EstimateMargin 欄位")

Return type : Object

| 參數             | 類別   | 說明       |
| ---------------- | ------ | ---------- |
| date             | string | 查詢日期   |
| currency         | string | 幣別       |
| estimate\_margin | float  | 預估保證金 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
order = FutOptOrder(
    buy_sell = BSAction.Buy,
    symbol = "TXFE4",
    price = "20890",
    lot = 1,
    market_type = FutOptMarketType.Future,
    price_type = FutOptPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = FutOptOrderType.New,
    user_def = "FromPy" # optional field
);

sdk.futopt.query_estimate_margin(accounts.data[0], order)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
  is_success: True,
  message: None,
  data: EstimateMargin {
    date: "2024/04/10",           # 查詢日期 (string)
    currency: "TWD",              # 幣別 (string)
    estimate_margin: 179000       # 預估保證金 (float)
  }
}

```


---

### 查詢歷史成交

filled\_history

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數         | 類別                                                                                                                  | 說明                                                                                            |
| ------------ | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| account      | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                   | 帳號                                                                                            |
| market\_type | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptmarkettype) | 市場別 : `Future` 期貨日盤、 `Option` 選擇權 、`FutureNight` 期貨夜盤、`OptionNight` 選擇權夜盤 |
| start\_date  | string                                                                                                                | 查詢開始日                                                                                      |
| end\_date    | string (optional) (不帶預設與開始日相同)                                                                              | 查詢終止日                                                                                      |

info

可查詢最近兩日之歷史資料

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別   | 說明                               |
| ----------- | ------ | ---------------------------------- |
| is\_success | bool   | 是否成功                           |
| data        | List   | 回傳成交資訊                       |
| message     | string | 當is\_success = false 回傳錯誤訊息 |

##### 成交資訊 FutOptFilledData 欄位[​](#成交資訊-futoptfilleddata-欄位 "Direct link to 成交資訊 FutOptFilledData 欄位")

Return type : Object

| 參數                | 類別                                                                                                                | 說明                                                        |
| ------------------- | ------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
| date                | string                                                                                                              | 成交日期                                                    |
| branch\_no          | string                                                                                                              | 分公司代號                                                  |
| account             | string                                                                                                              | 帳號                                                        |
| seq\_no             | string                                                                                                              | 委託單流水序號 (只有主動回報才回傳此欄位)                   |
| order\_no           | string                                                                                                              | 委託書號                                                    |
| symbol              | string                                                                                                              | 商品代號                                                    |
| expiry\_date        | string                                                                                                              | 到期日                                                      |
| strike\_price       | float                                                                                                               | 履約價                                                      |
| call\_put           | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput)                 | 買賣權 : `Call` 買權、 `Put` 賣權                           |
| buy\_sell           | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#bsaction)               | 買賣別 : `Buy` 買 、 `Sell` 賣                              |
| symbol\_leg2        | string                                                                                                              | 商品代號 - 複式第二隻腳                                     |
| expiry\_date\_leg2  | string                                                                                                              | 到期日 - 複式第二隻腳                                       |
| strike\_price\_leg2 | float                                                                                                               | 履約價 - 複式第二隻腳                                       |
| call\_put\_leg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput)                 | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權            |
| buy\_sell\_leg2     | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#bsaction)               | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                |
| filled\_no          | string                                                                                                              | 成交流水號                                                  |
| filled\_avg\_price  | float                                                                                                               | 成交均價                                                    |
| filled\_lots        | int                                                                                                                 | 成交口數                                                    |
| filled\_price       | float                                                                                                               | 成交單價                                                    |
| order\_type         | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptordertype) | 委託單類型 : `New` 新單 、 `Close` 平倉 、 `FdayTrade` 當沖 |
| filled\_time        | string                                                                                                              | 成交時間                                                    |
| user\_def           | string                                                                                                              | 用戶自定義 (只有主動回報才回傳此欄位)                       |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
sdk.futopt.filled_history(account,FutOptMarketType.Future,"20230921","20230922")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
    is_success: True,
    message: None,
    data :  [ FutOptFilledData{
                date: "2023/09/15",          # 成交日期 (string)
                branch_no: "6460",           # 分公司代號 (string)
                account: "26",               # 帳號 (string)
                order_no: "bA422",           # 委託書號 (string)
                seq_no: None,                # 委託單流水序號 (string)
                symbol: "FITX",              # 商品代號 (string)
                expiry_date: "202404",       # 履約日 (string)
                strike_price: None,          # 履約價 (float)
                call_put: None,              # 買賣權 (CallPut)
                buy_sell: Buy,               # 買賣別 (BSAction)
                symbol_leg2: None,           # 商品代號 - 複式第二隻腳(string)
                expiry_date_leg2: None,      # 履約日 - 複式第二隻腳(string)
                strike_price_leg2: None,     # 履約價 - 複式第二隻腳(float)
                call_put_leg2: None,         # 買賣權 - 複式第二隻腳(CallPut)
                buy_sell_leg2: None,         # 買賣別 - 複式第二隻腳(BSAction)
                filled_no: "00000000001",    # 成交流水號 (string)
                filled_avg_price: 20890.0,   # 成交均價 (float)
                filled_lots: 1,              # 成交股數 (int)
                filled_price: 20890.0,       # 成交單價 (float)
                order_type: New,             # 委託單類型 (FutOptOrderType)
                filled_time: "10:31:00.931"  # 成交時間 (string)
                user_def: None               # 用戶自定義 (string)
    },
    ...
    ]
}


```


---

### 取得委託單結果

get\_order\_results

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數         | 類別                                                                                                                                      | 說明                                                                                                      |
| ------------ | ----------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| account      | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                                       | 帳號                                                                                                      |
| market\_type | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptmarkettype) (Optional : 不帶為) | 盤別種類 : `Future` 期貨日盤 、 `FutureNight` 期貨夜盤 、 `Option` 選擇權日盤 、 `OptionNight` 選擇權夜盤 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別   | 說明                               |
| ----------- | ------ | ---------------------------------- |
| is\_success | bool   | 是否成功                           |
| data        | List   | 回傳委託資訊                       |
| message     | string | 當is\_success = false 回傳錯誤訊息 |

##### 委託資訊 FutOptOrderResult 欄位[​](#委託資訊-futoptorderresult-欄位 "Direct link to 委託資訊 FutOptOrderResult 欄位")

Return type : List

| 參數                | 類別                                                                                                                  | 說明                                                                                                                                            |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| function\_type      | int (Optional)                                                                                                        | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date                | string                                                                                                                | 交易日期                                                                                                                                        |
| seq\_no             | string                                                                                                                | 委託單流水序號                                                                                                                                  |
| branch\_no          | string                                                                                                                | 分公司代號                                                                                                                                      |
| account             | string                                                                                                                | 帳號                                                                                                                                            |
| order\_no           | string                                                                                                                | 委託書號                                                                                                                                        |
| asset\_type         | int                                                                                                                   | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market              | string                                                                                                                | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| market\_type        | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit                | int                                                                                                                   | 單位數                                                                                                                                          |
| currency            | string                                                                                                                | 幣別                                                                                                                                            |
| symbol              | string                                                                                                                | 商品代號                                                                                                                                        |
| expiry\_date        | string                                                                                                                | 到期日                                                                                                                                          |
| strike\_price       | float                                                                                                                 | 履約價                                                                                                                                          |
| call\_put           | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buy\_sell           | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbol\_leg2        | string                                                                                                                | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiry\_date\_leg2  | string                                                                                                                | 到期日 - 複式第二隻腳                                                                                                                           |
| strike\_price\_leg2 | float                                                                                                                 | 履約價 - 複式第二隻腳                                                                                                                           |
| call\_put\_leg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput)                   | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                                                |
| buy\_sell\_leg2     | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| price\_type         | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price               | float                                                                                                                 | 價格                                                                                                                                            |
| lot                 | int                                                                                                                   | 原始委託股口數                                                                                                                                  |
| time\_in\_force     | [TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#timeinforce)           | 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| order\_type         | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| is\_pre\_order      | bool                                                                                                                  | 是否為預約單                                                                                                                                    |
| status              | int                                                                                                                   | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| after\_price\_type  | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| after\_price        | float                                                                                                                 | 有效委託價格                                                                                                                                    |
| after\_lot          | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| filled\_lot         | int                                                                                                                   | 成交口數                                                                                                                                        |
| filled\_money       | float                                                                                                                 | 成交價金                                                                                                                                        |
| before\_lot         | int                                                                                                                   | 改單前有效口數                                                                                                                                  |
| before\_price       | float                                                                                                                 | 改單前有效價                                                                                                                                    |
| user\_def           | string                                                                                                                | 自訂欄位                                                                                                                                        |
| last\_time          | string                                                                                                                | 最後異動時間                                                                                                                                    |
| detail              | list                                                                                                                  | 委託歷程 (查詢order\_result\_detail or order\_history才有值)                                                                                    |
| error\_message      | string                                                                                                                | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
sdk.futopt.get_order_results(accounts.data[0])

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
    is_success: True,
    message: None,
    data : [FutOptOrderResult{
                function_type: None,                 # 功能別 (int)
                date: "2024/03/25",                  # 交易日期 (string)
                seq_no: "00110212608",               # 委託單流水序號 (string)
                branch_no: "15901",                  # 分公司代號 (string)
                account: "1234567",                  # 帳號 (string)
                order_no: "C0001",                   # 委託書號 (string)
                asset_type: 1,                       # 資產類別 (int)
                market: "TAIMEX",                    # 市場類型 (string)
                market_type: Future,                 # 盤別種類 (FutOptMarketType)
                unit: 1,                             # 單位數 (int)
                currency: "TWD",                     # 幣別 (string)
                symbol: "FITF",                      # 商品代號 (string)
                expiry_date: "202404",               # 到期日 (string)
                strike_price: None,                  # 履約價 (float)
                call_put: None,                      # 買賣權 (CallPut)
                buy_sell: Buy,                       # 買賣別 (BSAction)
                symbol_leg2: None,                   # 商品代號 - 複式第二隻腳 (string)
                expiry_date_leg2: None,              # 到期日 - 複式第二隻腳 (string)
                strike_price_leg2: None,             # 履約價 - 複式第二隻腳 (float)
                call_put_leg2: None,                 # 買賣權 - 複式第二隻腳 (CallPut)
                buy_sell_leg2: None,                 # 買賣別 - 複式第二隻腳 (BSAction)
                price_type: Limit,                   # 原始委託價格別 (PriceType)
                price: 1822.6,                       # 價格 (float)
                lot: 2,                              # 原始委託股口數 (int)
                time_in_force: ROD,                  # 委託條件別 (TimeInforce)
                order_type: Auto,                    # 委託單類型 (FutOptOrderType)
                is_pre_order: False,                 # 是否為預約單 (bool)
                status: 10,                          # 委託單狀態 (int)
                after_price_type: None,              # 有效委託價格別 (PriceType)
                after_price: 1822.6,                 # 有效委託價格 (float)
                after_lot: 2,                        # 有效委託股口數 (int)
                filled_lot: 0,                       # 成交股口數 (int)
                filled_money: 0,                     # 成交價金 (int)
                before_lot: None,                    # 改單前有效股口數 (int)
                before_price: None,                  # 改單前有效價格 (float)
                user_def: None,                      # 自訂欄位 (string)
                last_time: "10:20:27",               # 最後異動時間 (string)
                details: None,                        # 委託歷程 (list)
                error_message: None,                 # 錯誤訊息 (string)
    },
    ...
    ]
}

```


---

### 取得委託單結果 (含歷程)

get\_order\_results\_detail

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數         | 類別                                                                                                                             | 說明                                                                                                      |
| ------------ | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| account      | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                              | 帳號                                                                                                      |
| market\_type | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptmarkettype) (Optional) | 盤別種類 : `Future` 期貨日盤 、 `FutureNight` 期貨夜盤 、 `Option` 選擇權日盤 、 `OptionNight` 選擇權夜盤 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別   | 說明                               |
| ----------- | ------ | ---------------------------------- |
| is\_success | bool   | 是否成功                           |
| data        | List   | 回傳委託資訊                       |
| message     | string | 當is\_success = false 回傳錯誤訊息 |

##### 委託資訊 FutOptOrderResult 欄位[​](#委託資訊-futoptorderresult-欄位 "Direct link to 委託資訊 FutOptOrderResult 欄位")

Return type : List

| 參數                | 類別                                                                                                                  | 說明                                                                                                                                            |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| function\_type      | int (Optional)                                                                                                        | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date                | string                                                                                                                | 交易日期                                                                                                                                        |
| seq\_no             | string                                                                                                                | 委託單流水序號                                                                                                                                  |
| branch\_no          | string                                                                                                                | 分公司代號                                                                                                                                      |
| account             | string                                                                                                                | 帳號                                                                                                                                            |
| order\_no           | string                                                                                                                | 委託書號                                                                                                                                        |
| asset\_type         | int                                                                                                                   | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market              | string                                                                                                                | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| market\_type        | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit                | int                                                                                                                   | 單位數                                                                                                                                          |
| currency            | string                                                                                                                | 幣別                                                                                                                                            |
| symbol              | string                                                                                                                | 商品代號                                                                                                                                        |
| expiry\_date        | string                                                                                                                | 到期日                                                                                                                                          |
| strike\_price       | float                                                                                                                 | 履約價                                                                                                                                          |
| call\_put           | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buy\_sell           | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbol\_leg2        | string                                                                                                                | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiry\_date\_leg2  | string                                                                                                                | 到期日 - 複式第二隻腳                                                                                                                           |
| strike\_price\_leg2 | float                                                                                                                 | 履約價 - 複式第二隻腳                                                                                                                           |
| call\_put\_leg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput)                   | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                                                |
| buy\_sell\_leg2     | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| price\_type         | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price               | float                                                                                                                 | 價格                                                                                                                                            |
| lot                 | int                                                                                                                   | 原始委託股口數                                                                                                                                  |
| time\_in\_force     | [TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#timeinforce)           | 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| order\_type         | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| is\_pre\_order      | bool                                                                                                                  | 是否為預約單                                                                                                                                    |
| status              | int                                                                                                                   | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| after\_price\_type  | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| after\_price        | float                                                                                                                 | 有效委託價格                                                                                                                                    |
| after\_lot          | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| filled\_lot         | int                                                                                                                   | 成交口數                                                                                                                                        |
| filled\_money       | float                                                                                                                 | 成交價金                                                                                                                                        |
| before\_lot         | int                                                                                                                   | 改單前有效口數                                                                                                                                  |
| before\_price       | float                                                                                                                 | 改單前有效價                                                                                                                                    |
| user\_def           | string                                                                                                                | 自訂欄位                                                                                                                                        |
| last\_time          | string                                                                                                                | 最後異動時間                                                                                                                                    |
| error\_message      | string                                                                                                                | 錯誤訊息                                                                                                                                        |
| detail              | list                                                                                                                  | 委託歷程                                                                                                                                        |
| >> function\_type   | int                                                                                                                   | 功能別 : `10` 新單、`15` 改價、 `20` 改量、`30`刪單、`50` 成交 、`90 `失敗                                                                      |
| >> modified\_time   | string                                                                                                                | 修改時間                                                                                                                                        |
| >> before\_lot      | int                                                                                                                   | 原始委託口數                                                                                                                                    |
| >> after\_lot       | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| >> before\_price    | float                                                                                                                 | 原始委託價                                                                                                                                      |
| >> after\_price     | float                                                                                                                 | 有效委託價                                                                                                                                      |
| >> error\_message   | string                                                                                                                | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
sdk.futopt.get_order_results_detail(accounts.data[0])

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
    is_success: True,
    message: None,
    data : [FutOptOrderResult{
                function_type: None,                 # 功能別 (int)
                date: "2024/03/25",                  # 交易日期 (string)
                seq_no: "00110212608",               # 委託單流水序號 (string)
                branch_no: "15901",                  # 分公司代號 (string)
                account: "1234567",                  # 帳號 (string)
                order_no: "C0001",                   # 委託書號 (string)
                asset_type: 1,                       # 資產類別 (int)
                market: "TAIMEX",                    # 市場類型 (string)
                market_type: Future,                 # 盤別種類 (FutOptMarketType)
                unit: 1,                             # 單位數 (int)
                currency: "TWD",                     # 幣別 (string)
                symbol: "FITF",                      # 商品代號 (string)
                expiry_date: "202404",               # 到期日 (string)
                strike_price: None,                  # 履約價 (float)
                call_put: None,                      # 買賣權 (CallPut)
                buy_sell: Buy,                       # 買賣別 (BSAction)
                symbol_leg2: None,                   # 商品代號 - 複式第二隻腳 (string)
                expiry_date_leg2: None,              # 到期日 - 複式第二隻腳 (string)
                strike_price_leg2: None,             # 履約價 - 複式第二隻腳 (float)
                call_put_leg2: None,                 # 買賣權 - 複式第二隻腳 (CallPut)
                buy_sell_leg2: None,                 # 買賣別 - 複式第二隻腳 (BSAction)
                price_type: Limit,                   # 原始委託價格別 (PriceType)
                price: 1822.6,                       # 價格 (float)
                lot: 2,                              # 原始委託股口數 (int)
                time_in_force: ROD,                  # 委託條件別 (TimeInforce)
                order_type: Auto,                    # 委託單類型 (FutOptOrderType)
                is_pre_order: False,                 # 是否為預約單 (bool)
                status: 10,                          # 委託單狀態 (int)
                after_price_type: None,              # 有效委託價格別 (PriceType)
                after_price: 1822.6,                 # 有效委託價格 (float)
                after_lot: 2,                        # 有效委託股口數 (int)
                filled_lot: 0,                       # 成交股口數 (int)
                filled_money: 0,                     # 成交價金 (int)
                before_lot: None,                    # 改單前有效股口數 (int)
                before_price: None,                  # 改單前有效價格 (float)
                user_def: None,                      # 自訂欄位 (string)
                last_time: "10:20:27",               # 最後異動時間 (string)
                error_message: None,                 # 錯誤訊息 (string)
                details:[                            # 委託歷程 (list)
                    OrderDetail{
                        function_type: 10,          # 功能別 (int)
                        modified_time: "10:20:27",  # 修改時間 (string)
                        before_lot: 0,              # 原始委託口數 (int)
                        after_lot: 2,               # 有效委託口數 (int)
                        before_price: 1822.6,       # 原始委託價格 (float)
                        after_price: 1822.6,        # 有效委託價格 (float)
                        error_message: None         # 錯誤訊息 (string)
                    }
                ]
    },
    ...
    ]
}

```


---

### 修改委託價格

modify\_price

##### 先使用make\_modify\_price\_obj 建立 FutOptModifyPrice 物件[​](#先使用make_modify_price_obj-建立-futoptmodifyprice-物件 "Direct link to 先使用make_modify_price_obj 建立 FutOptModifyPrice 物件")

| 參數        | 類別                                                                                                                    | 說明             |
| ----------- | ----------------------------------------------------------------------------------------------------------------------- | ---------------- |
| orderResult | [FutOptOrderResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptorderresult) | 預計修改的委託單 |
| price       | string                                                                                                                  | 修改後的價格     |
| priceType   | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptpricetype)     | 修改後的價格旗標 |

caution

當 price 欄位有填入值時，priceType 欄位為空值或為None ； 當 priceType 欄位有填入值時，price 欄位為空值或為None

將回傳的物件放入 modify\_price 的方法中

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數           | 類別                                                                                                                    | 說明           |
| -------------- | ----------------------------------------------------------------------------------------------------------------------- | -------------- |
| accounts       | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                     | 帳號           |
| ModifyPriceObj | [FutOptModifyPrice](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptmodifyprice) | 修改價格的物件 |
| unblock        | bool (optional) (default = False)                                                                                       | 是否採用非阻塞 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別              | 說明                               |
| ----------- | ----------------- | ---------------------------------- |
| is\_success | bool              | 是否成功                           |
| data        | FutOptOrderResult | 回傳委託單修改資訊                 |
| message     | string            | 當is\_success = false 回傳錯誤訊息 |

##### 修改資訊 FutOptOrderResult 欄位[​](#修改資訊-futoptorderresult-欄位 "Direct link to 修改資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數                | 類別                                                                                                                  | 說明                                                                                                                                            |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| function\_type      | int                                                                                                                   | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date                | string                                                                                                                | 交易日期                                                                                                                                        |
| seq\_no             | string                                                                                                                | 委託單流水序號                                                                                                                                  |
| branch\_no          | string                                                                                                                | 分公司代號                                                                                                                                      |
| account             | string                                                                                                                | 帳號                                                                                                                                            |
| order\_no           | string                                                                                                                | 委託書號                                                                                                                                        |
| asset\_type         | int                                                                                                                   | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market              | string                                                                                                                | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| market\_type        | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit                | int                                                                                                                   | 單位數                                                                                                                                          |
| currency            | string                                                                                                                | 幣別                                                                                                                                            |
| symbol              | string                                                                                                                | 商品代號                                                                                                                                        |
| expiry\_date        | string                                                                                                                | 到期日                                                                                                                                          |
| strike\_price       | float                                                                                                                 | 履約價                                                                                                                                          |
| call\_put           | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buy\_sell           | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbol\_leg2        | string                                                                                                                | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiry\_date\_leg2  | string                                                                                                                | 到期日 - 複式第二隻腳                                                                                                                           |
| strike\_price\_leg2 | float                                                                                                                 | 履約價 - 複式第二隻腳                                                                                                                           |
| call\_put\_leg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput)                   | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                                                |
| buy\_sell\_leg2     | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| price\_type         | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price               | float                                                                                                                 | 價格                                                                                                                                            |
| lot                 | int                                                                                                                   | 原始委託股口數                                                                                                                                  |
| time\_in\_force     | [TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#timeinforce)           | 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| order\_type         | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| is\_pre\_order      | bool                                                                                                                  | 是否為預約單                                                                                                                                    |
| status              | int                                                                                                                   | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| after\_price\_type  | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| after\_price        | float                                                                                                                 | 有效委託價格                                                                                                                                    |
| after\_lot          | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| filled\_lot         | int                                                                                                                   | 成交口數                                                                                                                                        |
| filled\_money       | float                                                                                                                 | 成交價金                                                                                                                                        |
| before\_lot         | int                                                                                                                   | 改單前有效口數                                                                                                                                  |
| before\_price       | float                                                                                                                 | 改單前有效價                                                                                                                                    |
| user\_def           | string                                                                                                                | 自訂欄位                                                                                                                                        |
| last\_time          | string                                                                                                                | 最後異動時間                                                                                                                                    |
| detail              | list                                                                                                                  | 委託歷程 (查詢order\_result\_detail or order\_history才有值)                                                                                    |
| error\_message      | string                                                                                                                | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
modify_price_obj = sdk.futopt.make_modify_price_obj(order_result, "19900")
sdk.futopt.modify_price(account, modify_price_obj)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回�傳範例")

```py
Result {
    is_success: True,
    message: None,
    data : FutOptOrderResult {
            function_type: 15,                   # 功能別 (int)
            date: "2024/03/25",                  # 交易日期 (string)
            seq_no: "00110212663",               # 委託單流水序號 (string)
            branch_no: "15901",                  # 分公司代號 (string)
            account: "1234567",                  # 帳號 (string)
            order_no: "C0005",                   # 委託書號 (string)
            asset_type: 1,                       # 資產類別 (int)
            market: "TAIMEX",                    # 市場類型 (string)
            market_type: Future,                 # 盤別種類 (FutOptMarketType)
            unit: None,                          # 單位數 (int)
            currency: "TWD",                     # 幣別 (string)
            symbol: "FITX",                      # 商品代號 (string)
            expiry_date: "0",                    # 到期日 (string)
            strike_price: 0,                     # 履約價 (float)
            call_put: None,                      # 買賣權 (CallPut)
            buy_sell: Buy,                       # 買賣別 (BSAction)
            symbol_leg2: None,                   # 商品代號 - 複式第二隻腳 (string)
            expiry_date_leg2: None,              # 到期日 - 複式第二隻腳 (string)
            strike_price_leg2: None,             # 履約價 - 複式第二隻腳 (float)
            call_put_leg2: None,                 # 買賣權 - 複式第二隻腳 (CallPut)
            buy_sell_leg2: None,                 # 買賣別 - 複式第二隻腳 (BSAction)
            price_type: Limit,                   # 原始委託價格別 (FutOptPriceType)
            price: 20000,                        # 價格 (float)
            lot: 1,                              # 原始委託股口數 (int)
            time_in_force: ROD,                  # 委託條件別 (TimeInforce)
            order_type: New,                     # 委託單類型 (FutOptOrderType)
            is_pre_order: False,                 # 是否為預約單 (bool)
            status: 10,                          # 委託單狀態 (int)
            after_price_type: Limit,             # 有效委託價格別 (FutOptPriceType)
            after_price: 19900,                  # 有效委託價格 (float)
            after_lot: 1,                        # 有效委託股口數 (int)
            filled_lot: 0,                       # 成交股口數 (int)
            filled_money: 0,                     # 成交價金 (int)
            before_lot: 0,                       # 改單前有效股口數 (int)
            before_price: 20000,                 # 改單前有效價格 (float)
            user_def: None,                      # 自訂欄位 (string)
            last_time: "13:39:05",               # 最後異動時間 (string)
            details: None,                        # 委託歷程 (list)
            error_message: None,                 # 錯誤訊息 (string)
}}

```


---

### 修改委託單數量

modify\_lot

##### 先使用make\_modify\_lot\_obj 建立 FutOptModifyLot 物件[​](#先使用make_modify_lot_obj-建立-futoptmodifylot-物件 "Direct link to 先使用make_modify_lot_obj 建立 FutOptModifyLot 物件")

| 參數        | 類別                                                                                                                    | 說明                                                |
| ----------- | ----------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| orderResult | [FutOptOrderResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptorderresult) | 預計修改的委託單                                    |
| lot         | int                                                                                                                     | 修改後的委託量 ( 修改後數量包含此委託單已成交部份 ) |

將回傳的物件放入 modify\_lot 的方法中

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數            | 類別                                                                                                                | 說明           |
| --------------- | ------------------------------------------------------------------------------------------------------------------- | -------------- |
| account         | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                 | 帳號           |
| futOptModifyLot | [FutOptModifyLot](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptmodifylot) | 修改的委託單   |
| unblock         | bool (optional) (default = False)                                                                                   | 是否採用非阻塞 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別              | 說明                               |
| ----------- | ----------------- | ---------------------------------- |
| is\_success | bool              | 是否成功                           |
| data        | FutOptOrderResult | 回傳委託單修改資訊                 |
| message     | string            | 當is\_success = false 回傳錯誤訊息 |

##### 修改資訊 FutOptOrderResult 欄位[​](#修改資訊-futoptorderresult-欄位 "Direct link to 修改資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數                | 類別                                                                                                                  | 說明                                                                                                                                            |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| function\_type      | int                                                                                                                   | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date                | string                                                                                                                | 交易日期                                                                                                                                        |
| seq\_no             | string                                                                                                                | 委託單流水序號                                                                                                                                  |
| branch\_no          | string                                                                                                                | 分公司代號                                                                                                                                      |
| account             | string                                                                                                                | 帳號                                                                                                                                            |
| order\_no           | string                                                                                                                | 委託書號                                                                                                                                        |
| asset\_type         | int                                                                                                                   | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market              | string                                                                                                                | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| market\_type        | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit                | int                                                                                                                   | 單位數                                                                                                                                          |
| currency            | string                                                                                                                | 幣別                                                                                                                                            |
| symbol              | string                                                                                                                | 商品代號                                                                                                                                        |
| expiry\_date        | string                                                                                                                | 到期日                                                                                                                                          |
| strike\_price       | float                                                                                                                 | 履約價                                                                                                                                          |
| call\_put           | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buy\_sell           | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbol\_leg2        | string                                                                                                                | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiry\_date\_leg2  | string                                                                                                                | 到期日 - 複式第二隻腳                                                                                                                           |
| strike\_price\_leg2 | float                                                                                                                 | 履約價 - 複式第二隻腳                                                                                                                           |
| call\_put\_leg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput)                   | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                                                |
| buy\_sell\_leg2     | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| price\_type         | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price               | float                                                                                                                 | 價格                                                                                                                                            |
| lot                 | int                                                                                                                   | 原始委託股口數                                                                                                                                  |
| time\_in\_force     | [TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#timeinforce)           | 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| order\_type         | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| is\_pre\_order      | bool                                                                                                                  | 是否為預約單                                                                                                                                    |
| status              | int                                                                                                                   | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| after\_price\_type  | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| after\_price        | float                                                                                                                 | 有效委託價格                                                                                                                                    |
| after\_lot          | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| filled\_lot         | int                                                                                                                   | 成交口數                                                                                                                                        |
| filled\_money       | float                                                                                                                 | 成交價金                                                                                                                                        |
| before\_lot         | int                                                                                                                   | 改單前有效口數                                                                                                                                  |
| before\_price       | float                                                                                                                 | 改單前有效價                                                                                                                                    |
| user\_def           | string                                                                                                                | 自訂欄位                                                                                                                                        |
| last\_time          | string                                                                                                                | 最後異動時間                                                                                                                                    |
| detail              | list                                                                                                                  | 委託歷程 (查詢order\_result\_detail or order\_history才有值)                                                                                    |
| error\_message      | string                                                                                                                | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
modify_lot_obj = sdk.futopt.make_modify_lot_obj(target_order, 2)
sdk.futopt.modify_lot(account, modify_lot_obj)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
    is_success: True,
    message: None,
    data : FutOptOrderResult{
            function_type: 20,                  # 功能別 (int)
            date: "2024/03/25",                 # 交易日期 (string)
            seq_no: "03100161319",              # 委託單流水序號 (string)
            branch_no: "15901",                 # 分公司代號 (string)
            account: "1234567",                 # 帳號 (string)
            order_no: "l001D",                  # 委託書號 (string)
            asset_type: 1,                      # 資產類別 (int)
            market: "TAIMEX",                   # 市場類型 (string)
            market_type: FutureNight,           # 盤別種類 (FutOptMarketType)
            unit: None,                         # 單位數 (int)
            currency: "TWD",                    # 幣別 (string)
            symbol: "FIMTX",                    # 商品代號 (string)
            expiry_date: "202404",                  # 到期日 (string)
            strike_price: None,                 # 履約價 (float)
            call_put: None,                     # 買賣權 (CallPut)
            buy_sell: Buy,                      # 買賣別 (BSAction)
            symbol_leg2: None,                  # 商品代號 - 複式第二隻腳 (string)
            expiry_date_leg2: None,             # 到期日 - 複式第二隻腳 (string)
            strike_price_leg2: None,            # 履約價 - 複式第二隻腳 (float)
            call_put_leg2: None,                # 買賣權 - 複式第二隻腳 (CallPut)
            buy_sell_leg2: None,                # 買賣別 - 複式第二隻腳 (BSAction)
            price_type: Limit,                  # 原始委託價格別 (FutOptPriceType)
            price: 20000,                       # 價格 (float)
            lot: 3,                             # 原始委託股口數 (int)
            time_in_force: ROD,                 # 委託條件別 (TimeInforce)
            order_type: New,                    # 委託單類型 (FutOptOrderType)
            is_pre_order: False,                # 是否為預約單 (bool)
            status: 10,                         # 委託單狀態 (int)
            after_price_type: Limit,            # 有效委託價格別 (FutOptPriceType)
            after_price: 20000,                 # 有效委託價格 (float)
            after_lot: 2,                       # 有效委託股口數 (int)
            filled_lot: 0,                      # 成交股口數 (int)
            filled_money: 0,                    # 成交價金 (int)
            before_lot: 0,                      # 改單前有效股口數 (int)
            before_price: 20000,                # 改單前有效價格 (float)
            user_def: None,                     # 自訂欄位 (string)
            last_time: "18:24:40",              # 最後異動時間 (string)
            details: None,                        # 委託歷程 (list)
            error_message: None,                # 錯誤訊息 (string)
    }
}

```


---

### 查詢歷史委託

order\_history

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數         | 類別                                                                                                                                          | 說明                                                                                                      |
| ------------ | --------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| account      | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#account)                                                  | 帳號                                                                                                      |
| start\_date  | string                                                                                                                                        | 查詢開始日                                                                                                |
| end\_date    | string                                                                                                                                        | 查詢終止日                                                                                                |
| market\_type | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptmarkettype) (Optional : 不帶全盤別) | 盤別種類 : `Future` 期貨日盤 、 `FutureNight` 期貨夜盤 、 `Option` 選擇權日盤 、 `OptionNight` 選擇權夜盤 |

info

可查詢最近兩日之歷史資料

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別   | 說明                               |
| ----------- | ------ | ---------------------------------- |
| is\_success | bool   | 是否成功                           |
| data        | List   | 回傳委託資訊                       |
| message     | string | 當is\_success = false 回傳錯誤訊息 |

##### 委託資訊 FutOptOrderResult 欄位[​](#委託資訊-futoptorderresult-欄位 "Direct link to 委託資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數                | 類別                                                                                                                  | 說明                                                                                                                                            |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| function\_type      | int (Optional)                                                                                                        | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date                | string                                                                                                                | 交易日期                                                                                                                                        |
| seq\_no             | string                                                                                                                | 委託單流水序號                                                                                                                                  |
| branch\_no          | string                                                                                                                | 分公司代號                                                                                                                                      |
| account             | string                                                                                                                | 帳號                                                                                                                                            |
| order\_no           | string                                                                                                                | 委託書號                                                                                                                                        |
| asset\_type         | int                                                                                                                   | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market              | string                                                                                                                | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| market\_type        | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit                | int                                                                                                                   | 單位數                                                                                                                                          |
| currency            | string                                                                                                                | 幣別                                                                                                                                            |
| symbol              | string                                                                                                                | 商品代號                                                                                                                                        |
| expiry\_date        | string                                                                                                                | 到期日                                                                                                                                          |
| strike\_price       | float                                                                                                                 | 履約價                                                                                                                                          |
| call\_put           | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buy\_sell           | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbol\_leg2        | string                                                                                                                | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiry\_date\_leg2  | string                                                                                                                | 到期日 - 複式第二隻腳                                                                                                                           |
| strike\_price\_leg2 | float                                                                                                                 | 履約價 - 複式第二隻腳                                                                                                                           |
| call\_put\_leg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput)                   | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                                                |
| buy\_sell\_leg2     | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| price\_type         | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price               | float                                                                                                                 | 價格                                                                                                                                            |
| lot                 | int                                                                                                                   | 原始委託股口數                                                                                                                                  |
| time\_in\_force     | [TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#timeinforce)           | 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| order\_type         | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| is\_pre\_order      | bool                                                                                                                  | 是否為預約單                                                                                                                                    |
| status              | int                                                                                                                   | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| after\_price\_type  | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| after\_price        | float                                                                                                                 | 有效委託價格                                                                                                                                    |
| after\_lot          | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| filled\_lot         | int                                                                                                                   | 成交口數                                                                                                                                        |
| filled\_money       | float                                                                                                                 | 成交價金                                                                                                                                        |
| before\_lot         | int                                                                                                                   | 改單前有效口數                                                                                                                                  |
| before\_price       | float                                                                                                                 | 改單前有效價                                                                                                                                    |
| user\_def           | string                                                                                                                | 自訂欄位                                                                                                                                        |
| last\_time          | string                                                                                                                | 最後異動時間                                                                                                                                    |
| detail              | list                                                                                                                  | 委託歷程                                                                                                                                        |
| >> function\_type   | int                                                                                                                   | 功能別 : `10` 新單、`15` 改價、 `20` 改量、`30`刪單、`50` 成交 、`90 `失敗                                                                      |
| >> modified\_time   | string                                                                                                                | 修改時間                                                                                                                                        |
| >> before\_lot      | int                                                                                                                   | 原始委託口數                                                                                                                                    |
| >> after\_lot       | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| >> before\_price    | float                                                                                                                 | 原始委託價                                                                                                                                      |
| >> after\_price     | float                                                                                                                 | 有效委託價                                                                                                                                      |
| >> error\_message   | string                                                                                                                | 錯誤訊息                                                                                                                                        |
| error\_message      | string                                                                                                                | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
sdk.futopt.order_history(accounts,"20240410","20240411")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
    is_success: True,
    message: None,
    data : [FutOptOrderResult {
                function_type: None,                    # 功能別 (int)
                date: "2024/04/11",                     # 交易日期 (string)
                seq_no: "00230177314",                  # 委託單流水序號 (string)
                branch_no: "15000",                      # 分公司代號 (string)
                account: "9974825",                      # 帳號 (string)
                order_no: "C0020",                       # 委託書號 (string)
                asset_type: 2,                           # 資產類別 :  `1` 期貨 、`2` 選擇權 (int)
                market: "TAIMEX",                        # 市場類型 :  `TAIMEX` 期貨、選擇權 (string)
                market_type: Option,                     # 盤別種類  :  `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤 (FutOptMarketType)
                unit: None,                              # 單位數 (int)
                currency: None,                          # 幣別 (string)
                symbol: "TXO",                           # 商品代號 (string)
                expiry_date: "202404",                   # 到期日 (string)
                strike_price: 18600,                     # 履約價 (float)
                call_put: Call,                          # 買賣權 :  `Call` 買權、 `Put` 賣權 (string)
                buy_sell: Buy,                           # 買賣別 :  `Buy` 買 、 `Sell` 賣 (BSAction)
                symbol_leg2: None,                       # 商品代號 - 複式第二隻腳 (string)
                expiry_date_leg2: None,                  # 到期日 -  複式第二隻腳 (string)
                strike_price_leg2: None,                 # 履約價 - 複式第二隻腳 (float)
                call_put_leg2: None,                     # 買賣權 - 複式第二隻腳 :  `Call` 買權、 `Put` 賣權 (string)
                buy_sell_leg2: None,                     # 買賣別 - 複式第二隻腳:  `Buy` 買 、 `Sell` 賣 (BSAction)
                price_type: Limit,                       # 原始委託價格別  : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價 (FutOptPriceType)
                price: 2100,                             # 價格 (float)
                lot: 1,                                  # 原始委託股口數 (int)
                time_in_force: ROD,                      # 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC (TimeInforce)
                order_type: New,                         # 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖 (FutOptOrderType)
                is_pre_order: False,                     # 是否為預約單 (bool)
                status: 50,                              # 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、`9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功  、 `50` 完全成交 、 `90` 失敗 (int)
                after_price_type: None,                   # 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價 (FutOptPriceType)
                after_price: 2100,                       # 有效委託價格 (float)
                after_lot: 1,                            # 有效委託口數 (int)
                filled_lot: 1,                           # 成交口數 (int)
                filled_money: 2100,                     # 成交價金 (float)
                before_lot: None,                        # 改單前有效口數 (int)
                before_price: None,                      # 改單前有效價 (float)
                user_def: None,                          # 自訂欄位 (string)
                last_time: "10:41:46.760",               # 最後異動時間 (string)
                details:[                             # 委託歷程 (list)
                    OrderDetail{
                        function_type: 10,          # 功能別 (int)
                        modified_time: "10:20:27",  # 修改時間 (string)
                        before_lot: 0,              # 原始委託口數 (int)
                        after_lot: 1,               # 有效委託口數 (int)
                        before_price: 2100,       # 原始委託價格 (float)
                        after_price: 2100         # 有效委託價格 (float)
                        error_message: None         # 錯誤訊息 (string)
                    }
                    ...
                ]
                error_message: None                      # 錯誤訊息 (string)
            },
            FutOptOrderResult {
                date: "2024/04/11",                     # 交易日期 (string)
                seq_no: "00230177315",                  # 委託單流水序號 (string)
                branch_no: "15000",                      # 分公司代號 (string)
                account: "9974825",                      # 帳號 (string)
                order_no: "C0021",                       # 委託書號 (string)
                asset_type: 2,                           # 資產類別 :  `1` 期貨 、`2` 選擇權 (int)
                market: "TAIMEX",                        # 市場類型 :  `TAIMEX` 期貨、選擇權 (string)
                market_type: Option,                     # 盤別種類  :  `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤 (FutOptMarketType)
                unit: None,                              # 單位數 (int)
                currency: None,                          # 幣別 (string)
                symbol: "TXO",                           # 商品代號 (string)
                expiry_date: "202404",                   # 到期日 (string)
                strike_price: 18500,                     # 履約價 (float)
                call_put: Call,                          # 買賣權 :  `Call` 買權、 `Put` 賣權 (string)
                buy_sell: Sell,                          # 買賣別 :  `Buy` 買 、 `Sell` 賣 (BSAction)
                symbol_leg2: None,                       # 商品代號 - 複式第二隻腳 (string)
                expiry_date_leg2: None,                  # 到期日 -  複式第二隻腳 (string)
                strike_price_leg2: None,                 # 履約價 - 複式第二隻腳 (float)
                call_put_leg2: None,                     # 買賣權 - 複式第二隻腳 :  `Call` 買權、 `Put` 賣權 (string)
                buy_sell_leg2: None,                     # 買賣別 - 複式第二隻腳:  `Buy` 買 、 `Sell` 賣 (BSAction)
                price_type: Limit,                       # 原始委託價格別  : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價 (FutOptPriceType)
                price: 2230,                             # 價格 (float)
                lot: 1,                                  # 原始委託股口數 (int)
                time_in_force: ROD,                      # 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC (TimeInforce)
                order_type: New,                         # 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖 (FutOptOrderType)
                is_pre_order: False,                     # 是否為預約單 (bool)
                status: 50,                              # 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、`9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功  、 `50` 完全成交 、 `90` 失敗 (int)
                after_price_type: None,                   # 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價 (FutOptPriceType)
                after_price: 2230,                       # 有效委託價格 (float)
                after_lot: 1,                            # 有效委託口數 (int)
                filled_lot: 1,                           # 成交口數 (int)
                filled_money: 2230,                     # 成交價金 (float)
                before_lot: None,                        # 改單前有效口數 (int)
                before_price: None,                      # 改單前有效價 (float)
                user_def: None,                          # 自訂欄位 (string)
                last_time: "10:41:46.760",               # 最後異動時間 (string)
                error_message: None                      # 錯誤訊息 (string)
            },
    ... #更多的Result回傳
]} 

```


---

### 建立委託單

place\_order

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數          | 類別                                                                                                        | 說明           |
| ------------- | ----------------------------------------------------------------------------------------------------------- | -------------- |
| account       | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)         | 帳號           |
| order\_object | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#orderobject) | 委託內容       |
| unblock       | bool (optional) (default = False)                                                                           | 是否採用非阻塞 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別              | 說明                               |
| ----------- | ----------------- | ---------------------------------- |
| is\_success | bool              | 是否成功                           |
| data        | FutOptOrderResult | 回傳委託資訊                       |
| message     | string            | 當is\_success = false 回傳錯誤訊息 |

##### 委託資訊 FutOptOrderResult 欄位[​](#委託資訊-futoptorderresult-欄位 "Direct link to 委託資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數                | 類別                                                                                                                  | 說明                                                                                                                                            |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| function\_type      | int                                                                                                                   | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date                | string                                                                                                                | 交易日期                                                                                                                                        |
| seq\_no             | string                                                                                                                | 委託單流水序號                                                                                                                                  |
| branch\_no          | string                                                                                                                | 分公司代號                                                                                                                                      |
| account             | string                                                                                                                | 帳號                                                                                                                                            |
| order\_no           | string                                                                                                                | 委託書號                                                                                                                                        |
| asset\_type         | int                                                                                                                   | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market              | string                                                                                                                | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| market\_type        | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit                | int                                                                                                                   | 單位數                                                                                                                                          |
| currency            | string                                                                                                                | 幣別                                                                                                                                            |
| symbol              | string                                                                                                                | 商品代號                                                                                                                                        |
| expiry\_date        | string                                                                                                                | 到期日                                                                                                                                          |
| strike\_price       | float                                                                                                                 | 履約價                                                                                                                                          |
| call\_put           | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buy\_sell           | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbol\_leg2        | string                                                                                                                | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiry\_date\_leg2  | string                                                                                                                | 到期日 - 複式第二隻腳                                                                                                                           |
| strike\_price\_leg2 | float                                                                                                                 | 履約價 - 複式第二隻腳                                                                                                                           |
| call\_put\_leg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput)                   | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                                                |
| buy\_sell\_leg2     | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| price\_type         | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price               | float                                                                                                                 | 價格                                                                                                                                            |
| lot                 | int                                                                                                                   | 原始委託股口數                                                                                                                                  |
| time\_in\_force     | [TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#timeinforce)           | 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| order\_type         | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| is\_pre\_order      | bool                                                                                                                  | 是否為預約單                                                                                                                                    |
| status              | int                                                                                                                   | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| after\_price\_type  | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| after\_price        | float                                                                                                                 | 有效委託價格                                                                                                                                    |
| after\_lot          | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| filled\_lot         | int                                                                                                                   | 成交口數                                                                                                                                        |
| filled\_money       | float                                                                                                                 | 成交價金                                                                                                                                        |
| before\_lot         | int                                                                                                                   | 改單前有效口數                                                                                                                                  |
| before\_price       | float                                                                                                                 | 改單前有效價                                                                                                                                    |
| user\_def           | string                                                                                                                | 自訂欄位                                                                                                                                        |
| last\_time          | string                                                                                                                | 最後異動時間                                                                                                                                    |
| detail              | list                                                                                                                  | 委託歷程 (查詢order\_result\_detail or order\_history才有值)                                                                                    |
| error\_message      | string                                                                                                                | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py

# 單式
order = FutOptOrder(
    buy_sell = BSAction.Buy,
    symbol = "TXO20000E4",
    price =  "530",
    lot =  1,
    market_type = FutOptMarketType.Option,
    price_type = FutOptPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = FutOptOrderType.Auto,
    user_def = "FromPy" # optional field
)

sdk.futopt.place_order(account, order)

# 複式單
order = FutOptOrder(
    buy_sell = BSAction.Sell,
    symbol = "TXO20000E4",
    buy_sell2 = BSAction.Buy,
    symbol2 = "TXO19900E4",
    price =  "90",
    lot =  1,
    market_type = FutOptMarketType.Option,
    price_type = FutOptPriceType.Limit,
    time_in_force = TimeInForce.IOC,
    order_type = FutOptOrderType.Auto,
    user_def = "FromPy" # optional field
)


sdk.futopt.place_order(account, order)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {   #單式回覆
    is_success: True,
    message: None,
    data : FutOptOrderResult {
        function_type: 0,                   # 功能別 (int)
        date: "2024/03/25",                 # 交易日期 (string)
        seq_no: "00230177010",              # 委託單流水序號 (string)
        branch_no: "15901",                 # 分公司代號 (string)
        account: "1234567",                 # 帳號 (string)
        order_no: "C0002",                  # 委託書號 (string)
        asset_type: 2,                      # 資產類別 (int)
        market: "TAIMEX",                   # 市場類型 (string)
        market_type: Option,                # 盤別種類 (FutOptMarketType)
        unit: 1,                            # 單位數 (int)
        currency: "TWD",                    # 幣別 (string)
        symbol: "TXO",                      # 商品代號 (string)
        expiry_date: "202404",              # 到期日 (string)
        strike_price: 20000,               # 履約價 (float)
        call_put: Call,                     # 買賣權 (CallPut)
        buy_sell: Buy,                      # 買賣別 (BSAction)
        symbol_leg2: None,                  # 商品代號 - 複式第二隻腳 (string)
        expiry_date_leg2: None,             # 到期日 - 複式第二隻腳 (string)
        strike_price_leg2: None,            # 履約價 - 複式第二隻腳 (float)
        call_put_leg2: None,                # 買賣權 - 複式第二隻腳 (CallPut)
        buy_sell_leg2: None,                # 買賣別 - 複式第二隻腳 (BSAction)
        price_type: Limit,                  # 原始委託價格別 (FutOptPriceType)
        price: 530,                         # 價格 (float)
        lot: 1,                             # 原始委託股口數 (int)
        time_in_force: ROD,                 # 委託條件別 (TimeInforce)
        order_type: Auto,                   # 委託單類型 (FutOptOrderType)
        is_pre_order: False,                # 是否為預約單 (bool)
        status: 10,                         # 委託單狀態 (int)
        after_price_type: Limit,            # 有效委託價格別 (FutOptPriceType)
        after_price: 530,                   # 有效委託價格 (float)
        after_lot: 1,                       # 有效委託股口數 (int)
        filled_lot: 0,                      # 成交股口數 (int)
        filled_money: 0,                    # 成交價金 (int)
        before_lot: 0,                      # 改單前有效股口數 (int)
        before_price: 530,                  # 改單前有效價格 (float)
        user_def: "FromPy",            # 自訂欄位 (string)
        last_time: "11:50:08",              # 最後異動時間 (string)
        details: None,                        # 委託歷程 (list)
        error_message: None                 # 錯誤訊息 (string)
}}



Result {   #複式回覆
    is_success: True,
    message: None,
    data : FutOptOrderResult {
        function_type: 0,                   # 功能別 (int)
        date: "2024/03/25",                 # 交易日期 (string)
        seq_no: "00230177020",              # 委託單流水序號 (string)
        branch_no: "15901",                 # 分公司代號 (string)
        account: "1234567",                 # 帳號 (string)
        order_no: "C0004",                  # 委託書號 (string)
        asset_type: 2,                      # 資產類別 (int)
        market: "TAIMEX",                   # 市場類型 (string)
        market_type: Option,                # 盤別種類 (FutOptMarketType)
        unit: 1,                            # 單位數 (int)
        currency: "TWD",                    # 幣別 (string)
        symbol: "TXO",                      # 商品代號 (string)
        expiry_date: "202405",              # 到期日 (string)
        strike_price: 20000,               # 履約價 (float)
        call_put: Call,                     # 買賣權 (CallPut)
        buy_sell: Sell,                     # 買賣別 (BSAction)
        symbol_leg2: "TXO",                 # 商品代號 - 複式第二隻腳 (string)
        expiry_date_leg2: "202405",         # 到期日 - 複式第二隻腳 (string)
        strike_price_leg2: 19900,          # 履約價 - 複式第二隻腳 (float)
        call_put_leg2: Call,                # 買賣權 - 複式第二隻腳 (CallPut)
        buy_sell_leg2: Buy,                 # 買賣別 - 複式第二隻腳 (BSAction)
        price_type: Limit,                  # 原始委託價格別 (FutOptPriceType)
        price: 90,                          # 價格 (float)
        lot: 1,                             # 原始委託股口數 (int)
        time_in_force: IOC,                 # 委託條件別 (TimeInforce)
        order_type: New,                    # 委託單類型 (FutOptOrderType)
        is_pre_order: False,                # 是否為預約單 (bool)
        status: 10,                         # 委託單狀態 (int)
        after_price_type: Limit,            # 有效委託價格別 (FutOptPriceType)
        after_price: 90,                    # 有效委託價格 (float)
        after_lot: 1,                       # 有效委託股口數 (int)
        filled_lot: 0,                      # 成交股口數 (int)
        filled_money: 0,                    # 成交價金 (int)
        before_lot: 0,                      # 改單前有效股口數 (int)
        before_price: 90,                   # 改單前有效價格 (float)
        user_def: "FromPy",            # 自訂欄位 (string)
        last_time: "11:57:41",              # 最後異動時間 (string)
        error_message: None                 # 錯誤訊息 (string)
}}


```


---

### 事前準備

caution

在開始富邦新一代 API 前，您必須完成以下步驟

1. 準備好您的富邦期貨帳戶 (若您還未擁有富邦期貨帳戶，請點連結[線上開戶](https://www.fubon.com/futures/home/account/e_open/e_open.html))
2. 簽署 API 風險使用聲明書
3. 申請憑證

#### 準備好富邦期貨帳戶[​](#準備好富邦期貨帳戶 "Direct link to 準備好富邦期貨帳戶")

使用您的富邦期貨帳戶

![富邦期貨帳號](/TradeAPI/assets/images/prepare_account-792b5d9a17005ebde0e7d7050b37bb5c.png)

#### 簽署 API 使用風險暨聲明書[​](#簽署-api-使用風險暨聲明書 "Direct link to 簽署 API 使用風險暨聲明書")

請使用手機`富邦e點通` 或 使用電腦版 `E+`進行API 線上簽署

###### 完成簽署[​](#完成簽署 "Direct link to 完成簽署")

恭喜您，已簽署完成使用風險暨聲明書

#### 申請憑證[​](#申請憑證 "Direct link to 申請憑證")

至[憑證申請、展期](https://www.fbs.com.tw/Certificate/Management/)點選立即執行下載富邦證券憑證e總管(TCEM.exe)申請憑證

![Certificate apply](/TradeAPI/assets/images/ca_navigator-8ddb522e5c70f747eb5a534d53864247.png)

登入完成並且認證後，輸入手機或電子信箱收取OTP驗證

![OTP](/TradeAPI/assets/images/ca_cover-3de617d54b6dfac697be7cae07ea3260.png)

完成申請，憑證將存放於 C:\CAFubon\\(您的身分證字號)，並以您的身份證字號為檔名

![CertLocation](/TradeAPI/assets/images/ca_repo-829d506640c176e4a8fc1b84c13b2908.png)

#### 安裝與版本相容性[​](#安裝與版本相容性 "Direct link to 安裝與版本相容性")

安裝步驟與最低版本需求請見[安裝與版本相容性](https://www.fbs.com.tw/TradeAPI/docs/install-compatibility.txt)，下載 SDK 請至[SDK 下載頁面](https://www.fbs.com.tw/TradeAPI/docs/download/download-sdk.txt)。

#### 開始拓展程式交易之旅[​](#開始拓展程式交易之旅 "Direct link to 開始拓展程式交易之旅")

* Python
* Node.js
* C#

```python
from fubon_neo.sdk import FubonSDK, FutOptOrder
from fubon_neo.constant import TimeInForce, FutOptOrderType, FutOptPriceType, FutOptMarketType, CallPut, BSAction

#載入設定檔與登入
sdk = FubonSDK()

accounts = sdk.login("您的身分證號", "您的登入密碼", "您的憑證路徑位置" , "您的憑證密碼")
## accounts = sdk.login("您的身分證號", "您的登入密碼", "您的憑證路徑位置")  # 若憑證選用＂預設密碼＂, SDK v1.3.2與較新版本適用

acc  = accounts.data[0]

#建立委託物件
order = FutOptOrder(
  buy_sell = BSAction.Sell,
  symbol = "TXFD4",
  price = "20800",
  lot = 3,
  market_type = FutOptMarketType.Future,
  price_type = FutOptPriceType.Limit,
  time_in_force = TimeInForce.ROD,
  order_type = FutOptOrderType.Auto,
  user_def = "Python" # optional field
)

# 下單
order_res = sdk.futopt.place_order(acc, order)
print(order_res)

```

恭喜您<!-- -->🎊<!-- -->，完成下單，即可看到系統回覆的結果

```python
Result {  is_success: True,   message: None,  data : FutOptOrderResult{ function_type: 0, date :  "2023/10/13", seq_no :  "00000000016", branch_no :  "6460", account :  "26", order_no : "bA627", asset_type : 1, market : "TAIMEX", market_type : Future, symbol : "FITX", unit : 1 , currency : "TWD", expiry_date : "202404",  strike_price : , call_put : , buy_sell : Buy , price_type : Limit, price : 20800, lot : 3, time_in_force : ROD, order_type : Auto, is_pre_order : False, status : 10, after_price_type : Limit, afterPrice : 20800,  after_lot : 2000, filled_lot : 0, filled_money : 0, ... , user_def : "FromPyhton", last_time : "12:19:06.048", error_message :  } }

```

```js
const { FubonSDK, TimeInForce, FutOptOrderType, FutOptPriceType, FutOptMarketType, CallPut, BSAction } = require('fubon-neo');

// 載入設定檔與登入
const sdk = new FubonSDK();

const accounts = sdk.login("您的身分證號", "您的登入密碼", "您的憑證路徑", "您的憑證密碼");
// const accounts = sdk.login("您的身分證號", "您的登入密碼", "您的憑證路徑位置");  // 若憑證選用＂預設密碼＂, SDK v1.3.2與較新版本適用

const acc = accounts.data[0];

// 建立委託物件
const order = {
buySell: BSAction.Buy,
symbol: TXFD4,
price: "20800",
lot: 3,
marketType: FutOptMarketType.Future,
priceType: FutOptPriceType.Limit,
timeInForce: TimeInForce.ROD,
orderType: FutOptOrderType.Auto,
userDef: "fromJs"
};

// 下單
sdk.futopt.placeOrder(acc,order);

```

恭喜您<!-- -->🎊<!-- --> ，完成下單，即可看到系統回覆的結果

```js
{ isSuccess: true,  data:{ functionType :  0, date :  '2023/10/13', seqNo :  '00000000016', branchNo :  '6460', account :  '26', orderNo :  'bA627', assetType :  1, market :  'TAIMEX', marketType :  Future, symbol :  'FITX', unit :  1 , currency :  'TWD', expiryDate :  '202404'  , buySell :  Buy , priceType :  Limit, price :  20800, lot :  3, timeInForce :  ROD, orderType :  Auto, isPreOrder :  False, status :  10, afterPriceType :  Limit, afterPrice :  20800,  afterLot :  3, filledLot :  0, filledMoney :  0, ... , userDef :  'FromJS', lastTime :  '12:19:06.048', errorMessage :   } }

```

```cs
using FubonNeo.Sdk;

//載入設定檔與登入
var sdk = new FubonSDK();

var acc_obj = sdk.Login("您的身分證號", "您的登入密碼", "您的憑證路徑" ,"您的憑證密碼");
/*
// 若憑證選用＂預設密碼＂, SDK v1.3.2　與較新版本適用
var acc_obj = sdk.Login("您的身分證號", "您的登入密碼", "您的憑證路徑");
*/

var acc = acc_obj.data[0];

//建立委託物件
var order = new FutOptOrder(
  BsAction.Buy,
  "TXFD4",
  "20800",
  3,
  FutOptMarketType.Future,
  FutOptPriceType.Limit,
  TimeInForce.Rod,
  FutOptOrderType.Auto,
  "FromCS"
);

// 下單
var order_res = sdk.FutOpt.PlaceOrder(acc, order);
Console.WriteLine(order_res);

```

恭喜您<!-- -->🎊<!-- -->，完成下單，即可看到系統回覆的結果

```cs
{  isSuccess = True,  message = , data = FutOptOrderResult{functionType: 0, date = 2023/10/13, seqNo = 00000000016, branchNo = 6460, account = 26, orderNo = bA627, assetType = 1, market = TAIMEX, marketType = Future, symbol = FITX, unit = 1 , currency = TWD, expiryDate = 202404,  strikePrice = , callPut = , buySell = Buy , priceType = Limit, price = 20800, lot = 3, timeInForce = ROD, orderType = Auto, isPreOrder = False, status = 10, afterPriceType = Limit, afterPrice = 66,  afterLot = 2000, filledLot = 0, filledMoney = 0, ... , userDef = FromC#, lastTime = 12:19:06.048, errorMessage =  } }

```


---

### 快速開始

請先參考[事前準備](https://www.fbs.com.tw/TradeAPI/docs/trading-future/prepare.txt)完成風險預告書申請與憑證下載。

#### 安裝與版本相容性[​](#安裝與版本相容性 "Direct link to 安裝與版本相容性")

安裝步驟與最低版本需求請見[安裝與版本相容性](https://www.fbs.com.tw/TradeAPI/docs/install-compatibility.txt)，下載 SDK 請至[SDK 下載頁面](https://www.fbs.com.tw/TradeAPI/docs/download/download-sdk.txt)。

#####

並將憑證放置您的資料夾結構應該像是：

```text
.
└── XXXXXXXXXX.pfx

```

##### [​](#-1 "Direct link to -1")

* Python
* Node.js
* C#

若您未曾使用 SDK 進行過登入，或更換了 SDK 執行環境，請在資料夾新增一個 `index.py` 檔案，貼上以下內容並執行：

```python
from fubon_neo.sdk import FubonSDK, FutOptOrder
from fubon_neo.constant import TimeInForce, FutOptOrderType, FutOptPriceType, FutOptMarketType, CallPut, BSAction

sdk = FubonSDK()
   
accounts = sdk.login("您的身分證字號", "您的登入密碼", "您的憑證位置", "您的憑證密碼") #若有歸戶，則會回傳多筆帳號資訊
## accounts = sdk.login("您的身分證號", "您的登入密碼", "您的憑證路徑位置")  # 若憑證選用＂預設密碼＂, SDK v1.3.2與較新版本適用

acc = accounts.data[0]

```

若您未曾使用 SDK 進行過登入，或更換了 SDK 執行環境，請在資料夾新增一個 `index.js` 檔案，貼上以下內容並執行：

```js
const { FubonSDK, TimeInForce, FutOptOrderType, FutOptPriceType, FutOptMarketType, CallPut, BSAction } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("您的身分證字號", "您的登入密碼", "您的憑證路徑" ,"您的憑證密碼");
// const accounts = sdk.login("您的身分證號", "您的登入密碼", "您的憑證路徑位置");  // 若憑證選用＂預設密碼＂, SDK v1.3.2與較新版本適用

const acc = accounts.data[0];

```

若您未曾使用 SDK 進行過登入，或更換了 SDK 執行環境，請在資料夾新增一個 `index.cs` 檔案，貼上以下內容並執行：

```cs
using FubonNeo.Sdk;

var sdk = new FubonSDK();
var accounts = sdk.Login("您的身分證字號", "您的登入密碼", "您的憑證路徑", "您的憑證密碼"); // 若有歸戶，則會回傳多筆帳號資訊
/*
// 若憑證選用＂預設密碼＂, SDK v1.3.2　與較新版本適用
var acc_obj = sdk.Login("您的身分證號", "您的登入密碼", "您的憑證路徑");
*/

var acc = accounts.data[0];

```

#### 送出委託，買入股票[​](#送出委託買入股票 "Direct link to 送出委託，買入股票")

再將以下範例程式碼補上， 將示範如何使用富邦新一代 API 送出一張以跌停價買入 富邦金(2881)的限價委託：

* Python
* Node.js
* C#

```py
order = FutOptOrder(
    buy_sell = BSAction.Sell,
    symbol = "TXFD4",
    price = "20800",
    lot = 3,
    market_type = FutOptMarketType.Future,
    price_type = FutOptPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = FutOptOrderType.Auto,
    user_def = "Python" # optional field
)
sdk.futopt.place_order(acc, order)

```

```js
const order = {
  buySell: BSAction.Buy,
  symbol: "TXFD4",
  price: "20800",
  lot: 3,
  marketType: FutOptMarketType.Future,
  priceType: FutOptPriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: FutOptOrderType.Auto,
  userDef: "fromJs"
};

sdk.futopt.placeOrder(acc,order);

```

```cs
var order = new FutOptOrder(
    BsAction.Buy,
    "TXFD4",
    null,      // 複式單第二隻腳 買賣別
    null,      // 複式單第二隻腳 商品
    "20800",
    3,
    FutOptMarketType.Future,
    FutOptPriceType.Limit,
    TimeInForce.Rod,
    FutOptOrderType.Auto,
    "FromCS"
);
sdk.FutOpt.PlaceOrder(acc, order);

```


---

### 速率限制

##### 連線次數限制[​](#連線次數限制 "Direct link to 連線次數限制")

| 行為   | 上限數 |
| ------ | ------ |
| 連線數 | 10     |

當連線超過上限，登入時將收到以下訊息 :

```json
Result {
  is_success: False,
  message: Login Error, 超過本應用程式連線限制==>[10]
  data: None
}

```

| 行為     | 每秒上限數 |
| -------- | ---------- |
| 下單     | 50         |
| 批次下單 | 10         |
| 帳務查詢 | 5          |

當次數超過上限，將收到以下訊息 :

```json
Result {
  is_success: False,
  message: Login Error, 業務系統流量控管
  data: None
}

```


---

