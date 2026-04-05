## 2.2.6 及以後版本使用更簡化 (使用 Exception 進行例外處理)
from fubon_neo.sdk import FugleAPIError

try:  
  response = reststock.corporate_actions.capital_changes(**{"start_date": "2025-12-06", "end_date": "2026-01-08"})
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"}   

print(response)


```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient

client.stock.corporateActions.capitalChanges({ start_date: '2025-12-06', end_date: '2026-01-08' })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;
using FugleMarketData.QueryModels.Stock.CorporateActions;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime();  // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

var dividend = await rest.CorporateActions.CapitalChanges(new()
{
    StartDate = fromDate,
    EndDate = toDate,
    Sort = SortType.Desc // Optional
});

var dividend_cont = dividend.Content.ReadAsStringAsync().Result;

Console.WriteLine(dividend_cont);

```

Response Body：

```json
{
    "start_date": "2025-01-01",
    "end_date": "2026-01-09",
    "sort": "desc",
    "data": [
        {
            "symbol": "4530",
            "name": "宏易",
            "actionType": "capital_reduction",
            "resumeDate": "2025-12-29",
            "haltDate": "2025-12-18",
            "exchange": "TPEx",
            "raw": {
                "exrightReferencePrice": 0,
                "limitDownPrice": 28.2,
                "limitUpPrice": 34.45,
                "openingReferencePrice": 31.3,
                "previousClose": 12.3,
                "reason": "彌補虧損",
                "referencePrice": 31.32,
                "refundPerShare": 0,
                "sharesPerThousand": 392.72795
            }
        },
        {
            "symbol": "3593",
            "name": "力銘",
            "actionType": "capital_reduction",
            "resumeDate": "2025-12-22",
            "haltDate": "2025-12-11",
            "exchange": "TWSE",
            "raw": {
                "limitDownPrice": 12.15,
                "limitUpPrice": 14.85,
                "openingReferencePrice": 13.5,
                "previousClose": 8.1,
                "reason": "彌補虧損",
                "referencePrice": 13.5,
                "refundPerShare": 0,
                "sharesPerThousand": 599.9999936
            }
        },
        {
            "symbol": "00715L",
            "name": "期街口布蘭特正2",
            "actionType": "etf_split_or_merge",
            "resumeDate": "2025-12-10",
            "haltDate": "2025-12-03",
            "exchange": "TWSE",
            "raw": {
                "limitDownPrice": 0.01,
                "limitUpPrice": 9999.95,
                "openingReferencePrice": 20.86,
                "previousClose": 10.43,
                "referencePrice": 20.86,
                "splitRatio": 0.5,
                "splitType": "反分割"
            }
        },
        {
            "symbol": "8103",
            "name": "瀚荃",
            "actionType": "capital_reduction",
            "resumeDate": "2025-12-08",
            "haltDate": "2025-11-27",
            "exchange": "TWSE",
            "raw": {
                "limitDownPrice": 77.5,
                "limitUpPrice": 94.7,
                "openingReferencePrice": 86.1,
                "previousClose": 74.7,
                "reason": "退還股款",
                "referencePrice": 86.11,
                "refundPerShare": 1.5,
                "sharesPerThousand": 850
            }
        },
        {
            "symbol": "0052",
            "name": "富邦科技",
            "actionType": "etf_split_or_merge",
            "resumeDate": "2025-11-26",
            "exchange": "TWSE",
            "raw": {
                "limitDownPrice": 31.54,
                "limitUpPrice": 38.54,
                "openingReferencePrice": 35.04,
                "previousClose": 245.3,
                "referencePrice": 35.04,
                "splitType": "分割"
            }
        },
        {
            "symbol": "1808",
            "name": "潤隆",
            "actionType": "capital_reduction",
            "resumeDate": "2025-11-24",
            "haltDate": "2025-11-13",
            "exchange": "TWSE",
            "raw": {
                "limitDownPrice": 33.45,
                "limitUpPrice": 40.85,
                "openingReferencePrice": 37.15,
                "previousClose": 34.45,
                "reason": "退還股款",
                "referencePrice": 37.16,
                "refundPerShare": 1,
                "sharesPerThousand": 900
            }
        },
        {
            "symbol": "6465",
            "name": "威潤",
            "actionType": "capital_reduction",
            "resumeDate": "2025-11-24",
            "haltDate": "2025-11-13",
            "exchange": "TPEx",
            "raw": {
                "exrightReferencePrice": 0,
                "limitDownPrice": 15.55,
                "limitUpPrice": 18.95,
                "openingReferencePrice": 17.25,
                "previousClose": 15.65,
                "reason": "彌補虧損",
                "referencePrice": 17.25,
                "refundPerShare": 0,
                "sharesPerThousand": 907.01186
            }
        },
        {
            "symbol": "9927",
            "name": "泰銘",
            "actionType": "capital_reduction",
            "resumeDate": "2025-11-24",
            "haltDate": "2025-11-13",
            "exchange": "TWSE",
            "raw": {
                "limitDownPrice": 62.2,
                "limitUpPrice": 76,
                "openingReferencePrice": 69.1,
                "previousClose": 52.4,
                "reason": "退還股款",
                "referencePrice": 69.11,
                "refundPerShare": 2.82805,
                "sharesPerThousand": 717.194904
            }
        },
        {
            "symbol": "8422",
            "name": "可寧衛",
            "actionType": "par_value_change",
            "resumeDate": "2025-11-17",
            "haltDate": "2025-11-06",
            "exchange": "TWSE",
            "raw": {
                "limitDownPrice": 22.5,
                "limitUpPrice": 27.5,
                "openingReferencePrice": 25,
                "previousClose": 250,
                "referencePrice": 25
            }
        },
        {
            "symbol": "5301",
            "name": "寶得利",
            "actionType": "capital_reduction",
            "resumeDate": "2025-11-12",
            "haltDate": "2025-11-05",
            "exchange": "TPEx",
            "raw": {
                "exrightReferencePrice": 0,
                "limitDownPrice": 12.55,
                "limitUpPrice": 15.25,
                "openingReferencePrice": 13.9,
                "previousClose": 6.41,
                "reason": "彌補虧損",
                "referencePrice": 13.89,
                "refundPerShare": 0,
                "sharesPerThousand": 461.57
            }
        },
        ...
    ]
}

```


---

### 除權息資料

取得 上市櫃除權息資料（依日期查詢）

```text
GET /corporate-actions/dividends/

```

版本資訊

v2.2.8 起新增功能

#### Parameters[​](#parameters "Direct link to Parameters")

| Name         | Type   | Description                                                  |
| ------------ | ------ | ------------------------------------------------------------ |
| `start_date` | string | 開始日期（格式：`yyyy-MM-dd`）                               |
| `end_date`   | string | 結束日期（格式：`yyyy-MM-dd`）可輸入未來日期取得未來預告資訊 |

#### Response[​](#response "Direct link to Response")

| Name                            | Type      | Description                |
| ------------------------------- | --------- | -------------------------- |
| `data`\*                        | object\[] | 除權息資料                 |
| `data.date`\*                   | string    | 除權息日期                 |
| `data.exchange`                 | string    | 交易所                     |
| `data.symbol`                   | string    | 股票代號                   |
| `data.name`                     | string    | 股票名稱                   |
| `data.previousClose`            | number    | 除權息前收盤價             |
| `data.referencePrice`           | number    | 除權息參考價               |
| `data.dividend`                 | number    | 除權息總金額 (權值 + 息值) |
| `data.dividendType`             | string    | 除權息類別                 |
| `data.limitUpPrice`             | number    | 除權後漲停價               |
| `data.limitDownPrice`           | number    | 除權後跌停價               |
| `data.openingReferencePrice`    | number    | 開盤競價參考價             |
| `data.exdividendReferencePrice` | number    | 減除股利參考價             |
| `data.cashDividend`             | number    | 現金除息                   |
| `data.stockDividendShares`      | number    | 每千股無償配股             |

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

## 2.2.6 及以後版本使用更簡化 (使用 Exception 進行例外處理)
from fubon_neo.sdk import FugleAPIError

try:  
  response = reststock.corporate_actions.dividends(**{"start_date": "2025-08-26", "end_date": "2026-01-08"})
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"}   

print(response)


```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線
const client = sdk.marketdata.restClient
client.stock.corporateActions.dividends({ start_date: '2025-08-26', end_date: '2026-01-08' })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;
using FugleMarketData.QueryModels.Stock.CorporateActions;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime();  // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

var dividend = await rest.CorporateActions.Dividends(new()
{
    StartDate = fromDate,
    EndDate = toDate,
    Sort = SortType.Desc // Optional
});

var dividend_cont = dividend.Content.ReadAsStringAsync().Result;

Console.WriteLine(dividend_cont);

```

Response Body：

```json
{
  "data": [
    { // 宣告未來的除權息資料不會帶入價格
      "date": "2026-01-08",
      "exchange": "TWSE",
      "symbol": "2247",
      "name": "汎德永業",
      "previousClose": null,
      "referencePrice": null,
      "dividend": 6.5,
      "dividendType": "息",
      "limitUpPrice": null,
      "limitDownPrice": null,
      "openingReferencePrice": null,
      "exdividendReferencePrice": null,
      "cashDividend": 6.5,
      "stockDividendShares": null
    },
    { // 已發生的除權息資料會帶入價格 (僅有息)
      "date": "2026-01-06",
      "exchange": "TWSE",
      "symbol": "00946",
      "name": "群益科技高息成長",
      "previousClose": 9.6,
      "referencePrice": 9.54,
      "dividend": 0.058,
      "dividendType": "息",
      "limitUpPrice": 10.49,
      "limitDownPrice": 8.59,
      "openingReferencePrice": 9.54,
      "exdividendReferencePrice": 9.54,
      "cashDividend": 0.058,
      "stockDividendShares": 0
    },
    { // 增資 ( 前日開盤會等於減除股利參考價或 stockDividendShares = 0 )
      "date": "2026-01-06",
      "exchange": "TWSE",
      "symbol": "2442",
      "name": "新美齊",
      "previousClose": 24.8,
      "referencePrice": 24.12,
      "dividend": 0.671598,
      "dividendType": "權",
      "limitUpPrice": 27.25,
      "limitDownPrice": 21.75,
      "openingReferencePrice": 24.8,
      "exdividendReferencePrice": 24.8,
      "cashDividend": 0,
      "stockDividendShares": 0
    },
    { // 已發生的除權息資料會帶入價格 (僅有權)
      "date": "2025-08-26",
      "exchange": "TPEx",
      "symbol": "6752",
      "name": "叡揚",
      "previousClose": 158.5,
      "referencePrice": 150.95,
      "dividend": 7.548635,
      "dividendType": "權",
      "limitUpPrice": 166,
      "limitDownPrice": 136,
      "openingReferencePrice": 151,
      "exdividendReferencePrice": 150.95,
      "cashDividend": 0,
      "stockDividendShares": 50.00706711
    },
    { // 已發生的除權息資料會帶入價格 (同時包含權息)
      "date": "2025-08-26",
      "exchange": "TPEx",
      "symbol": "4554",
      "name": "橙的",
      "previousClose": 36.95,
      "referencePrice": 32.39,
      "dividend": 4.560619,
      "dividendType": "權息",
      "limitUpPrice": 35.6,
      "limitDownPrice": 29.2,
      "openingReferencePrice": 32.4,
      "exdividendReferencePrice": 32.39,
      "cashDividend": 0.35,
      "stockDividendShares": 129.99998584
    }
  ]
}

```


---

### 申請上市櫃公司

取得 申請上市櫃公司資料

```text
GET /corporate-actions/listing-applicants/

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

| Name                   | Type   | Description                              |
| ---------------------- | ------ | ---------------------------------------- |
| `symbol`               | string | 公司代號                                 |
| `name`                 | string | 公司簡稱                                 |
| `exchange`             | string | 申請類別：`TWSE`（上市）、`TPEx`（上櫃） |
| `applicationDate`      | string | 申請日期                                 |
| `chairman`             | string | 董事長                                   |
| `capitalAtApplication` | number | 申請時股本（仟元）                       |
| `reviewCommitteeDate`  | string | 上市審議委員審議日                       |
| `boardApprovalDate`    | string | 交易所董事會通過日                       |
| `contractFilingDate`   | string | 主管機關核准日期                         |
| `listedDate`           | string | 股票上市、櫃買賣日期                     |
| `underwriter`          | string | 承銷商                                   |
| `underwritingPrice`    | number | 承銷價                                   |
| `remarks`              | string | 備註                                     |

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

## 2.2.6 及以後版本使用更簡化 (使用 Exception 進行例外處理)
from fubon_neo.sdk import FugleAPIError

try:  
  response = reststock.corporate_actions.listing_applicants(**{"start_date": "2025-01-07", "end_date": "2026-01-07"})
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"}   

print(response)


```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient
client.stock.corporateActions.listingApplicants({ start_date: '2025-01-07', end_date: '2026-01-07' })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;
using FugleMarketData.QueryModels.Stock.CorporateActions;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime();  // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

var listing = await rest.CorporateActions.ListingApplicants(new()
{
    StartDate = fromDate,
    EndDate = toDate,
    Sort = SortType.Desc // Optional
});

var listing_cont = listing.Content.ReadAsStringAsync().Result;

Console.WriteLine(listing_cont);

```

Response Body：

```json
{
    "start_date": "2025-01-07",
    "end_date": "2026-01-07",
    "sort": "desc",
    "data": [
        ...
        {
            "symbol": "7762",
            "name": "吉晟生",
            "exchange": "TWSE",
            "applicationDate": "2025-09-24",
            "chairman": "楊朝堂",
            "capitalAtApplication": 572328,
            "reviewCommitteeDate": null,
            "boardApprovalDate": null,
            "contractFilingDate": null,
            "listedDate": null,
            "underwriter": "宏遠",
            "underwritingPrice": null,
            "remarks": "創新板，114-10-31撤件"
        },
        {
            "symbol": "6961",
            "name": "旅天下",
            "exchange": "TPEx",
            "applicationDate": "2025-09-17",
            "chairman": "李嘉寅",
            "capitalAtApplication": 236340000,
            "reviewCommitteeDate": "2025-11-06",
            "boardApprovalDate": "2025-11-21",
            "contractApprovalDate": "2025-11-25",
            "listedDate": null,
            "underwriter": "福邦",
            "underwritingPrice": null,
            "remarks": ""
        },
        {
            "symbol": "4590",
            "name": "富田",
            "exchange": "TWSE",
            "applicationDate": "2025-09-15",
            "chairman": "張金鋒",
            "capitalAtApplication": 511941,
            "reviewCommitteeDate": "2025-10-23",
            "boardApprovalDate": "2025-11-18",
            "contractFilingDate": "2025-11-26",
            "listedDate": null,
            "underwriter": "中信",
            "underwritingPrice": null,
            "remarks": "創新板"
        },
        {
            "symbol": "6725",
            "name": "矽科宏晟",
            "exchange": "TPEx",
            "applicationDate": "2025-09-11",
            "chairman": "郭錦松",
            "capitalAtApplication": 330000000,
            "reviewCommitteeDate": "2025-10-20",
            "boardApprovalDate": "2025-10-30",
            "contractApprovalDate": "2025-11-04",
            "listedDate": "2025-12-30",
            "underwriter": "台新",
            "underwritingPrice": 188,
            "remarks": ""
        }
    ]
}

```


---

### 開始使用

本頁重點

* 富邦行情 Web API（富邦新一代 API 行情服務）提供台股盤中、快照與歷史資料查詢。
* 超過速率限制會回應 `429`，詳見速率限制說明。
* 提供 Python / Node.js / C# SDK 範例。
* 下一步可從功能列表選擇對應 API 端點。

| 項目     | 說明                  |
| -------- | --------------------- |
| 介面     | Web API               |
| 市場     | 台股                  |
| 資料類型 | 盤中 / 快照 / 歷史    |
| 速率限制 | 超過回應 `429`        |
| SDK      | Python / Node.js / C# |

富邦行情 Web API 提供開發者友善的 API 服務。您可以查詢台股的日內行情、行情快照與歷史行情等數據。

#### 速率限制[​](#速率限制 "Direct link to 速率限制")

如果您 API 請求超過了限制，將收到帶有狀態碼 `429` 的回應。 ( 限制細節請參考[速率限制](https://www.fbs.com.tw/TradeAPI/docs/market-data/rate-limit.txt) )

#### 功能列表[​](#功能列表 "Direct link to 功能列表")

功能列表依資料類型可分為 **盤中行情（intraday）**、**行情快照（snapshot）**、**歷史行情（historical）**。富邦行情 Web API 提供的功能如下：

* `/intraday/tickers` - [股票或指數列表（依條件查詢）](https://www.fbs.com.tw/TradeAPI/docs/market-data/http-api/intraday/tickers.txt)
* `/intraday/ticker/{symbol}` - [股票基本資料（依代碼查詢）](https://www.fbs.com.tw/TradeAPI/docs/market-data/http-api/intraday/ticker.txt)
* `/intraday/quote/{symbol}` - [股票即時報價（依代碼查詢）](https://www.fbs.com.tw/TradeAPI/docs/market-data/http-api/intraday/quote.txt)
* `/intraday/candles/{symbol}` - [股票價格Ｋ線（依代碼查詢）](https://www.fbs.com.tw/TradeAPI/docs/market-data/http-api/intraday/candles.txt)
* `/intraday/trades/{symbol}` - [股票成交明細（依代碼查詢）](https://www.fbs.com.tw/TradeAPI/docs/market-data/http-api/intraday/trades.txt)
* `/intraday/volumes/{symbol}` - [股票分價量表（依代碼查詢）](https://www.fbs.com.tw/TradeAPI/docs/market-data/http-api/intraday/volumes.txt)
* `/snapshot/quotes/{market}` - [股票行情快照（依市場別）](https://www.fbs.com.tw/TradeAPI/docs/market-data/http-api/snapshot/quotes.txt)
* `/snapshot/movers/{market}` - [股票漲跌幅排行（依市場別）](https://www.fbs.com.tw/TradeAPI/docs/market-data/http-api/snapshot/movers.txt)
* `/snapshot/actives/{market}` - [股票成交量值排行（依市場別）](https://www.fbs.com.tw/TradeAPI/docs/market-data/http-api/snapshot/actives.txt)
* `/historical/candles/{symbol}` - [取得 1 年內歷史股價（依代碼查詢）](https://www.fbs.com.tw/TradeAPI/docs/market-data/http-api/historical/candles.txt)
* `/historical/stats/{symbol}` - [取得近 52 週股價數據（依代碼查詢）](https://www.fbs.com.tw/TradeAPI/docs/market-data/http-api/historical/stats.txt)

#### 使用 SDK[​](#使用-sdk "Direct link to 使用 SDK")

富邦行情 Web API 提供 Python 、 Node.js SDK 與 C# SDK。您可以透過以下方式存取 API：

* Python
* Node.js
* C#

```python
from fubon_neo.sdk import FubonSDK, Order
# 2.2.4 以後版本, 增加下列匯入: 
# from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

sdk = FubonSDK()

accounts = sdk.login("Your ID", "Your password", "Your cert path", "Your cert password")  # 需登入後，才能取得行情權限

sdk.init_realtime()  # 建立行情連線

reststock = sdk.marketdata.rest_client.stock

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient

const stock = client.stock;

```

```cs

using FubonNeo.Sdk;

var sdk = new FubonSDK();

var result = sdk.Login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

```


---

### Historical Candles

取得 1 年內的上市櫃歷史股價（依代碼查詢），個股資料區間最遠可回溯至 2010 年，指數部分最遠可回溯至 2015 年！

```text
historical/candles/{symbol}

```

#### Parameters[​](#parameters "Direct link to Parameters")

| Name        | Type   | Description                                                                                                           |
| ----------- | ------ | --------------------------------------------------------------------------------------------------------------------- |
| `symbol`\*  | string | 股票代碼                                                                                                              |
| `from`      | string | 開始日期（格式：`yyyy-MM-dd`）                                                                                        |
| `to`        | string | 結束日期（格式：`yyyy-MM-dd`）                                                                                        |
| `timeframe` | string | Ｋ線週期，可選 `1` 1分Ｋ；`5` 5分Ｋ；`10` 10分Ｋ；`15` 15分Ｋ；`30` 30分Ｋ；`60` 60分Ｋ；`D` 日Ｋ；`W` 週Ｋ；`M` 月Ｋ |
| `adjusted`  | string | 還原股價啟用，可選 `true`、`false` ***(v2.2.8 新增)***                                                                |
| `fields`    | string | 欄位，可選：`open,high,low,close,volume,turnover,change`                                                              |
| `sort`      | string | 時間排序，預設為 `desc` 降冪；可選 `asc` 升冪                                                                         |

caution

目前分Ｋ無法指定開始日期（from） 與 結束日期（to），一律回傳近五日資料，並且無法選擇 turnover 與 change 的欄位。

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
# reststock.historical.candles(**{"symbol": "0050", "from": "2023-02-06", "to": "2023-02-08"})  # 2.2.3 及以前版本

## 2.2.4 及以後版本 (使用 Exception 進行例外處理)
from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

try:  
  reststock.historical.candles(**{"symbol": "0050", "from": "2023-02-06", "to": "2023-02-08"})
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"}     

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient
client.stock.historical.candles({ symbol: '0050', from: '2023-02-06', to: '2023-02-08', fields: 'open,high,low,close,volume,change' })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;
using FugleMarketData.QueryModels.Stock.History;   //引入 HistoryTimeFrame
using FugleMarketData.QueryModels;                 //引入 FieldsType



var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

var candle = await rest.History.Candles("2330", new(DateTime.Today.AddDays(-100),DateTime.Today,HistoryTimeFrame.Week,FieldsType.High|FieldsType.Low));

var candle_con = candle.Content.ReadAsStringAsync().Result;

Console.WriteLine(candle_con);


```

Response Body：

```json
{
  "symbol": "0050",
  "type": "EQUITY",
  "exchange": "TWSE",
  "market": "TSE",
  "data": [
    {
      "date": "2023-02-08",
      "open": 120.1,
      "high": 120.95,
      "low": 120,
      "close": 120.85,
      "volume": 9239321,
      "change": 1.85
    },
    {
      "date": "2023-02-07",
      "open": 119.1,
      "high": 119.25,
      "low": 118.55,
      "close": 119,
      "volume": 8787291,
      "change": -0.25
    },
    {
      "date": "2023-02-06",
      "open": 120.1,
      "high": 120.1,
      "low": 119.25,
      "close": 119.25,
      "volume": 14297030,
      "change": -1.75
    }
  ]
}

```

#### Response[​](#response "Direct link to Response")

| Name         | Type   | Description |
| ------------ | ------ | ----------- |
| `date`\*     | string | 日期        |
| `type`\*     | string | 證券類型    |
| `exchange`\* | string | 交易所      |
| `market`     | string | 市場別      |
| `symbol`\*   | string | 股票代號    |
| `timeframe*` | string | Ｋ線週期    |
| `data`       | Candle | Ｋ線資料    |

info

'\*' 表示必揭示欄位。


---

### Historical Stats

取得近 52 週股價數據（依代碼查詢）

```text
historical/stats/{symbol}

```

#### Parameters[​](#parameters "Direct link to Parameters")

| Name       | Type   | Description |
| ---------- | ------ | ----------- |
| `symbol`\* | string | 股票代碼    |

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
# reststock.historical.stats(symbol = "0050")  # 2.2.3 及以前版本

## 2.2.4 及以後版本 (使用 Exception 進行例外處理)
from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

try:  
  reststock.historical.stats(symbol = "0050")
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"}    

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime();  // 建立行情連線

const client = sdk.marketdata.restClient
client.stock.historical.stats({ symbol: '0050' })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

var stats = await rest.History.Stats("0050");
var stats_cont = stats.Content.ReadAsStringAsync().Result;

Console.WriteLine(stats_cont);

```

Response Body：

```json
{
  "date": "2023-02-09",
  "type": "EQUITY",
  "exchange": "TWSE",
  "market": "TSE",
  "symbol": "0050",
  "name": "元大台灣50",
  "openPrice": 120.5,
  "highPrice": 121,
  "lowPrice": 120.3,
  "closePrice": 120.9,
  "change": 0.05,
  "changePercent": 0.04,
  "tradeVolume": 5032245,
  "tradeValue": 607543546,
  "previousClose": 120.85000000000001,
  "week52High": 145.05,
  "week52Low": 96.5
}

```

#### Response[​](#response "Direct link to Response")

| Name              | Type   | Description        |
| ----------------- | ------ | ------------------ |
| `date`\*          | string | 日期               |
| `type`\*          | string | Ticker 類型        |
| `exchange`\*      | string | 交易所             |
| `market`\*        | string | 市場別             |
| `symbol`\*        | string | 股票代碼           |
| `name`\*          | string | 股票簡稱           |
| `openPrice`\*     | number | 最後交易日開盤價   |
| `highPrice`\*     | number | 最後交易日最高價   |
| `lowPrice`\*      | number | 最後交易日最低價   |
| `closePrice`\*    | number | 最後交易日收盤價   |
| `change`\*        | number | 最後交易日漲跌     |
| `changePercent`\* | number | 最後交易日漲跌幅   |
| `tradeVolume`\*   | number | 最後交易日成交量   |
| `tradeValue`\*    | number | 最後交易日成交金額 |
| `previousClose`\* | number | 前一交易日收盤價   |
| `week52High`\*    | number | 近 52 週高點       |
| `week52Low`\*     | number | 近 52 週低點       |

info

'\*' 表示必揭示欄位。


---

### Intraday Candles

股票價格Ｋ線（依代碼查詢）

```text
intraday/candles/{symbol}

```

#### Parameters[​](#parameters "Direct link to Parameters")

| Name        | Type   | Description                                                                             |
| ----------- | ------ | --------------------------------------------------------------------------------------- |
| `symbol`\*  | string | 股票代碼                                                                                |
| `type`      | string | Ticker 類型，可選 `oddlot` 盤中零股                                                     |
| `timeframe` | string | Ｋ線週期，可選 `1` 1分Ｋ；`5` 5分Ｋ；`10` 10分Ｋ；`15` 15分Ｋ；`30` 30分Ｋ；`60` 60分Ｋ |
| `sort`      | string | 時間排序，預設為 `asc` 升冪 ；可選 `desc` 降冪                                          |

#### Response[​](#response "Direct link to Response")

| Name          | Type      | Description                                                                |
| ------------- | --------- | -------------------------------------------------------------------------- |
| `date`\*      | string    | 日期                                                                       |
| `type`\*      | string    | Ticker 類型                                                                |
| `exchange`\*  | string    | 交易所                                                                     |
| `market`      | string    | 市場別                                                                     |
| `symbol`\*    | string    | 股票代號                                                                   |
| `timeframe`\* | string    | Ｋ線週期                                                                   |
| `data`\*      | object\[] | Ｋ線資料                                                                   |
| >>`open`      | number    | Ｋ線開盤價                                                                 |
| >>`high`      | number    | Ｋ線最高價                                                                 |
| >>`low`       | number    | Ｋ線最低價                                                                 |
| >>`close`     | number    | Ｋ線收盤價                                                                 |
| >>`volume`    | number    | Ｋ線成交量（整股：成交張數；興櫃股票及盤中零股：成交股數；指數：成交金額） |
| >>`average`   | number    | 成交均價                                                                   |

info

'\*' 表示必揭示欄位。

#### Example[​](#example "Direct link to Example")

* Python
* Node.js
* C#

```python
from fubon_neo.sdk import FubonSDK, Order

sdk = FubonSDK()

accounts = sdk.login("Your ID", "Your password" ,"Your cert path" ,"Your cert password")  # 需登入後，才能取得行情權限

sdk.init_realtime() # 建立行情連線
reststock = sdk.marketdata.rest_client.stock
# reststock.intraday.candles(symbol='2330')  # 2.2.3 及以前版本

## 2.2.4 及以後版本 (使用 Exception 進行例外處理)
from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

try:  
  reststock.intraday.candles(symbol='2330')
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"} 

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient
client.stock.intraday.candles({ symbol: '2330' })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

var candles = await rest.Intraday.Candles("2330");
// var candles = await rest.Intraday.Candles("2330", new(){TimeFrame=FugleMarketData.QueryModels.Stock.Intraday.IntradayTimeFrame.TenMin});  // 10分K

var candle_cont = candles.Content.ReadAsStringAsync().Result;

Console.WriteLine(candle_cont);

```

Response Body：

```json
{
  "date": "2023-05-29",
  "type": "EQUITY",
  "exchange": "TWSE",
  "market": "TSE",
  "symbol": "2330",
  "data": [
    {
      "date": "2023-05-29T09:00:00.000+08:00",
      "open": 574,
      "high": 574,
      "low": 572,
      "close": 572,
      "volume": 8450,
      "average": 573.82
    },
    {
      "date": "2023-05-29T09:01:00.000+08:00",
      "open": 572,
      "high": 573,
      "low": 571,
      "close": 571,
      "volume": 594,
      "average": 573.68
    },
    {
      "date": "2023-05-29T09:02:00.000+08:00",
      "open": 572,
      "high": 572,
      "low": 569,
      "close": 570,
      "volume": 1372,
      "average": 573.26
    },
    ......
  ]
}

```


---

### Intraday Quote

股票即時報價（依代碼查詢）

```text
intraday/quote/{symbol}

```

#### Parameters[​](#parameters "Direct link to Parameters")

| Name       | Type   | Description                  |
| ---------- | ------ | ---------------------------- |
| `symbol`\* | string | 股票代碼                     |
| `type`     | string | 類型，可選 `oddlot` 盤中零股 |

#### Response[​](#response "Direct link to Response")

| Name                  | Type      | Description                         |
| --------------------- | --------- | ----------------------------------- |
| `date`\*              | string    | 日期                                |
| `type`\*              | string    | Ticker 類型                         |
| `exchange`\*          | string    | 交易所                              |
| `market`\*            | string    | 市場別                              |
| `symbol`\*            | string    | 股票代碼                            |
| `name`\*              | string    | 股票簡稱                            |
| `referencePrice`      | number    | 今日參考價                          |
| `previousClose`       | number    | 昨日收盤價                          |
| `openPrice`           | number    | 開盤價                              |
| `openTime`            | number    | 開盤價成交時間                      |
| `highPrice`           | number    | 最高價                              |
| `highTime`            | number    | 最高價成交時間                      |
| `lowPrice`            | number    | 最低價                              |
| `lowTime`             | number    | 最低價成交時間                      |
| `closePrice`          | number    | 收盤價（最後成交價）                |
| `closeTime`           | number    | 收盤價（最後成交價）成交時間        |
| `lastPrice`           | number    | 最後一筆成交價（含試撮）            |
| `lastSize`            | number    | 最後一筆成交數量（含試撮）          |
| `avgPrice`            | number    | 當日成交均價                        |
| `change`              | number    | 最後成交價漲跌                      |
| `changePercent`       | number    | 最後成交價漲跌幅                    |
| `amplitude`           | number    | 當日振幅                            |
| `bids`                | object\[] | 最佳五檔委買                        |
| >> `price`            | number    | 最佳五檔委買價格                    |
| >> `size`             | number    | 最佳五檔委買數量                    |
| `asks`                | object\[] | 最佳五檔委賣                        |
| >> `price`            | number    | 最佳五檔委賣價格                    |
| >> `size`             | number    | 最佳五檔委賣數量                    |
| `total`               | object    | 統計資訊                            |
| >> `tradeValue`       | number    | 累計成交金額                        |
| >> `tradeVolume`      | number    | 累計成交量                          |
| >> `tradeVolumeAtBid` | number    | 委買成筆                            |
| >> `tradeVolumeAtAsk` | number    | 委賣成筆                            |
| >> `transaction`      | number    | 累計成交筆數                        |
| >> `time`             | number    | 統計時間                            |
| `lastTrade`           | object    | 最後一筆成交資訊                    |
| >> `bid`              | number    | 最後一筆成交買價                    |
| >> `ask`              | number    | 最後一筆成交賣價                    |
| >> `price`            | number    | 最後一筆成交價格                    |
| >> `size`             | number    | 最後一筆成交數量                    |
| >> `time`             | number    | 最後一筆成交時間                    |
| `lastTrial`           | object    | 最後一筆試撮資訊                    |
| >> `bid`              | number    | 最後一筆試撮買價                    |
| >> `ask`              | number    | 最後一筆試撮賣價                    |
| >> `price`            | number    | 最後一筆試撮價格                    |
| >> `size`             | number    | 最後一筆試撮數量                    |
| >> `time`             | number    | 最後一筆試撮時間                    |
| `opHaltStatus`        | object    | 暫停交易                            |
| >> `isHalted`         | boolean   | 暫停交易：`true`；恢復交易：`false` |
| >> `time`             | number    | 暫停交易時間／恢復交易時間          |
| `isLimitDownPrice`    | boolean   | 最後成交價為跌停價：`true`          |
| `isLimitUpPrice`      | boolean   | 最後成交價為漲停價：`true`          |
| `isLimitDownBid`      | boolean   | 最佳一檔委買跌停價：`true`          |
| `isLimitUpBid`        | boolean   | 最佳一檔委買漲停價：`true`          |
| `isLimitDownAsk`      | boolean   | 最佳一檔委賣跌停價：`true`          |
| `isLimitUpAsk`        | boolean   | 最佳一檔委賣漲停價：`true`          |
| `isLimitDownHalt`     | boolean   | 暫緩撮合且瞬間趨跌：`true`          |
| `isLimitUpHalt`       | boolean   | 暫緩撮合且瞬間趨漲：`true`          |
| `isTrial`             | boolean   | 試撮階段：`true`                    |
| `isDelayedOpen`       | boolean   | 延後開盤信號：`true`                |
| `isDelayedClose`      | boolean   | 延後收盤信號：`true`                |
| `isContinuous`        | boolean   | 最後成交為逐筆交易：`true`          |
| `isOpen`              | boolean   | 開盤信號：`true`                    |
| `isClose`             | boolean   | 收盤信號：`true`                    |
| `lastUpdated`         | number    | 最後更新時間                        |

info

'\*' 表示必揭示欄位。

#### Example[​](#example "Direct link to Example")

* Python
* Node.js
* C#

```python
from fubon_neo.sdk import FubonSDK, Order

sdk = FubonSDK()

accounts = sdk.login("Your ID", "Your password" , "Your cert path", "Your cert password")  # 需登入後，才能取得行情權限

sdk.init_realtime() # 建立行情連線

reststock = sdk.marketdata.rest_client.stock
# reststock.intraday.quote(symbol='2330')  # 2.2.3 及以前版本

## 2.2.4 及以後版本 (使用 Exception 進行例外處理)
from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

try:  
  reststock.intraday.quote(symbol='2330')
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"} 

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient
client.stock.intraday.quote({ symbol: '2330' })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

var quote = await rest.Intraday.Quote("2330");
// var quote = await rest.Intraday.Quote("2330", new(){Type=FugleMarketData.QueryModels.TickerType.OddLot});  // 零股

var quote_cont = quote.Content.ReadAsStringAsync().Result;

Console.WriteLine(quote_cont);

```

Response Body：

```json
{
  "date": "2023-05-29",
  "type": "EQUITY",
  "exchange": "TWSE",
  "market": "TSE",
  "symbol": "2330",
  "name": "台積電",
  "referencePrice": 566,
  "previousClose": 566,
  "openPrice": 574,
  "openTime": 1685322000049353,
  "highPrice": 574,
  "highTime": 1685322000049353,
  "lowPrice": 564,
  "lowTime": 1685327142152580,
  "closePrice": 568,
  "closeTime": 1685338200000000,
  "avgPrice": 568.77,
  "change": 2,
  "changePercent": 0.35,
  "amplitude": 1.77,
  "lastPrice": 568,
  "lastSize": 4778,
  "bids": [
    {
      "price": 567,
      "size": 87
    },
    {
      "price": 566,
      "size": 2454
    },
    {
      "price": 565,
      "size": 611
    },
    {
      "price": 564,
      "size": 609
    },
    {
      "price": 563,
      "size": 636
    }
  ],
  "asks": [
    {
      "price": 568,
      "size": 800
    },
    {
      "price": 569,
      "size": 806
    },
    {
      "price": 570,
      "size": 3643
    },
    {
      "price": 571,
      "size": 1041
    },
    {
      "price": 572,
      "size": 2052
    }
  ],
  "total": {
    "tradeValue": 31019803000,
    "tradeVolume": 54538,
    "tradeVolumeAtBid": 19853,
    "tradeVolumeAtAsk": 27900,
    "transaction": 9530,
    "time": 1685338200000000
  },
  "lastTrade": {
    "bid": 567,
    "ask": 568,
    "price": 568,
    "size": 4778,
    "time": 1685338200000000,
    "serial": 6652422
  },
  "lastTrial": {
    "bid": 567,
    "ask": 568,
    "price": 568,
    "size": 4772,
    "time": 1685338196400347,
    "serial": 6651941
  },
  "isClose": true,
  "serial": 6652422,
  "lastUpdated": 1685338200000000
}

```


---

### Intraday Ticker

取得股票資訊 (依股票代碼查詢)

```text
intraday/ticker/{symbol}

```

#### Parameters[​](#parameters "Direct link to Parameters")

| Name       | Type   | Description                  |
| ---------- | ------ | ---------------------------- |
| `symbol`\* | string | 股票代碼                     |
| `type`     | string | 類型，可選 `oddlot` 盤中零股 |

#### Response[​](#response "Direct link to Response")

| Name                          | Type    | Description                                                               |
| ----------------------------- | ------- | ------------------------------------------------------------------------- |
| `date`\*                      | string  | 日期                                                                      |
| `type`\*                      | string  | Ticker 類型                                                               |
| `exchange`\*                  | string  | 交易所                                                                    |
| `market`                      | string  | 市場別                                                                    |
| `symbol`\*                    | string  | 股票代碼                                                                  |
| `name`\*                      | string  | 股票簡稱                                                                  |
| `nameEn`                      | string  | 股票英文簡稱                                                              |
| `industry`                    | string  | 產業別                                                                    |
| `securityType`                | string  | 證券別，參閱 [證券別代碼](#%E8%AD%89%E5%88%B8%E5%88%A5%E4%BB%A3%E7%A2%BC) |
| `referencePrice`              | number  | 參考價                                                                    |
| `limitUpPrice`                | number  | 漲停價                                                                    |
| `limitDownPrice`              | number  | 跌停價                                                                    |
| `canDayTrade`                 | boolean | 可買賣現沖                                                                |
| `canBuyDayTrade`              | boolean | 可先買現沖                                                                |
| `canBelowFlatMarginShortSell` | boolean | 平盤下得融券賣出                                                          |
| `canBelowFlatSBLShortSell`    | boolean | 平盤下得借券賣出                                                          |
| `isAttention`                 | boolean | 注意股                                                                    |
| `isDisposition`               | boolean | 處置股                                                                    |
| `isUnusuallyRecommended`      | boolean | 投資理財節目異常推介個股                                                  |
| `isSpecificAbnormally`        | boolean | 特殊異常個股                                                              |
| `matchingInterval`            | number  | 撮合循環秒數                                                              |
| `securityStatus`              | string  | 證券狀態，分別有 `NORMAL`, `TERMINATED`, `SUSPENDED` 狀態                 |
| `boardLot`                    | number  | 交易單位                                                                  |
| `tradingCurrency`             | string  | 交易幣別                                                                  |
| `exercisePrice`               | number  | 履約價格（限權證）                                                        |
| `exercisedVolume`             | number  | 前一交易日履約數量（限權證）                                              |
| `cancelledVolume`             | number  | 前一交易日註銷數量（限權證）                                              |
| `remainingVolume`             | number  | 發行餘額量（限權證）                                                      |
| `exerciseRatio`               | number  | 行使比率（限權證）                                                        |
| `knockInPrice`                | number  | 上限價格（限權證）                                                        |
| `knockOutPrice`               | number  | 下限價格（限權證）                                                        |
| `maturityDate`                | string  | 到期日（限權證）                                                          |
| `previousClose`               | number  | 前一交易日收盤價                                                          |
| `openTime`                    | string  | 開盤時間（限指數）                                                        |
| `closeTime`                   | string  | 收盤時間（限指數）                                                        |

info

'\*' 表示必揭示欄位。

#### Example[​](#example "Direct link to Example")

* Python
* Node.js
* C#

```python
from fubon_neo.sdk import FubonSDK, Order

sdk = FubonSDK()

accounts = sdk.login("Your ID", "Your password", "Your cert path","Your cert password")  

sdk.init_realtime() # 建立行情連線
reststock = sdk.marketdata.rest_client.stock  
# reststock.intraday.ticker(symbol='2330')  # 2.2.3 及以前版本

## 2.2.4 及以後版本 (使用 Exception 進行例外處理)
from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

try:  
  reststock.intraday.ticker(symbol='2330')
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"} 

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient

client.stock.intraday.ticker({ symbol: '2330' })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

var ticker = await rest.Intraday.Ticker("2330");
var ticker_cont = ticker.Content.ReadAsStringAsync().Result;

Console.WriteLine(ticker_cont);

```

Response Body：

```json
{
  "date": "2023-05-29",
  "type": "EQUITY",
  "exchange": "TWSE",
  "market": "TSE",
  "symbol": "2330",
  "name": "台積電",
  "industry": "24",
  "securityType": "01",
  "previousClose": 566,
  "referencePrice": 566,
  "limitUpPrice": 622,
  "limitDownPrice": 510,
  "canDayTrade": true,
  "canBuyDayTrade": true,
  "canBelowFlatMarginShortSell": true,
  "canBelowFlatSBLShortSell": true,
  "isAttention": false,
  "isDisposition": false,
  "isUnusuallyRecommended": false,
  "isSpecificAbnormally": false,
  "matchingInterval": 0,
  "securityStatus": "NORMAL",
  "boardLot": 1000,
  "tradingCurrency": "TWD"
}

```

#### 證券別代碼[​](#證券別代碼 "Direct link to 證券別代碼")

| 代碼 | 證券別                                     | 代碼 | 證券別                 |
| ---- | ------------------------------------------ | ---- | ---------------------- |
| `01` | 一般股票                                   | `24` | ETF                    |
| `02` | 轉換公司債                                 | `25` | ETF（外幣計價）        |
| `03` | 交換公司債或交換金融債                     | `26` | 槓桿型ETF              |
| `04` | 一般特別股                                 | `27` | 槓桿型 ETF（外幣計價） |
| `05` | 可交換特別股                               | `28` | 反向型 ETF             |
| `06` | 認股權憑證                                 | `29` | 反向型 ETF（外幣計價） |
| `07` | 附認股權特別股                             | `30` | 期信託 ETF             |
| `08` | 附認股權公司債                             | `31` | 期信託 ETF（外幣計價） |
| `09` | 附認股權公司債履約或分拆後之公司債         | `32` | 債券 ETF               |
| `10` | 國內標的認購權證                           | `33` | 債券 ETF（外幣計價）   |
| `11` | 國內標的認售權證                           | `34` | 金融資產證券化受益證券 |
| `12` | 外國標的認購權證                           | `35` | 不動產資產信託受益證券 |
| `13` | 外國標的認售權證                           | `36` | 不動產投資信託受益證券 |
| `14` | 國內標的下限型認購權證                     | `37` | ETN                    |
| `15` | 國內標的上限型認售權證                     | `38` | 槓桿型 ETN             |
| `16` | 國內標的可展延下限型認購權證               | `39` | 反向型 ETN             |
| `17` | 國內標的可展延上限型認售權證               | `40` | 債券型 ETN             |
| `18` | 受益憑證(封閉式基金)                       | `41` | 期權策略型 ETN         |
| `19` | 存託憑證                                   | `42` | 中央登錄公債           |
| `20` | 存託憑證可轉換公司債                       | `43` | 外國債券               |
| `21` | 存託憑證附認股權公司債                     | `44` | 黃金現貨               |
| `22` | 存託憑證附認股權公司債履約或分拆後之公司債 | `00` | 未知或保留代碼         |
| `23` | 存託憑證認股權憑證                         |      |                        |


---

### Intraday Tickers

股票或指數列表（依條件查詢）

```text
intraday/tickers

```

#### Parameters[​](#parameters "Direct link to Parameters")

| Name            | Type    | Description                                                                                 |
| --------------- | ------- | ------------------------------------------------------------------------------------------- |
| `type`\*        | string  | Ticker 類型，可選 `EQUITY` 股票；`INDEX` 指數；`WARRANT` 權證；`ODDLOT` 盤中零股            |
| `exchange`      | string  | 交易所，可選 `TWSE` 臺灣證券交易所；`TPEx` 證券櫃檯買賣中心                                 |
| `market`        | string  | 市場別，可選 `TSE` 上市；`OTC` 上櫃；`ESB` 興櫃一般板；`TIB` 臺灣創新板；`PSB` 興櫃戰略新板 |
| `industry`      | string  | 產業別，參閱 [產業別代碼](#%E7%94%A2%E6%A5%AD%E5%88%A5%E4%BB%A3%E7%A2%BC)                   |
| `isNormal`      | boolean | 查詢正常股票（非注意、處置股票）：`true`                                                    |
| `isAttention`   | boolean | 查詢注意股票：`true`                                                                        |
| `isDisposition` | boolean | 查詢處置股票：`true`                                                                        |
| `isHalted`      | boolean | 查詢暫停交易股票：`true`                                                                    |

股票專用條件

`isNormal`、`isAttention`、`isDisposition`、`isHalted` 為股票專用條件，若設定此類條件，將回傳對應股票列表

#### Response[​](#response "Direct link to Response")

| Name            | Type      | Description                      |
| --------------- | --------- | -------------------------------- |
| `date`\*        | string    | 日期                             |
| `type`\*        | string    | Ticker 類型                      |
| `exchange`\*    | string    | 交易所                           |
| `market`        | string    | 市場別                           |
| `industry`      | string    | 產業別                           |
| `isNormal`      | boolean   | 查詢正常股票（非注意、處置股票） |
| `isAttention`   | boolean   | 查詢注意股票                     |
| `isDisposition` | boolean   | 查詢處置股票                     |
| `isHalted`      | boolean   | 查詢暫停交易股票                 |
| `data`          | object\[] | 股票列表                         |
| >> `symbol`     | string    | 股票代碼                         |
| >> `name`       | string    | 股票簡稱                         |

info

'\*' 表示必揭示欄位。

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
# reststock.intraday.tickers(type='EQUITY', exchange="TWSE", isNormal=True)  # 2.2.3 及以前版本

## 2.2.4 及以後版本 (使用 Exception 進行例外處理)
from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

try:  
  reststock.intraday.tickers(type='EQUITY', exchange="TWSE", isNormal=True)
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"} 

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient
client.stock.intraday
  .tickers({ type: "EQUITY", exchange: "TWSE", isNormal: true })
  .then((data) => console.log(data));

```

```cs

using FubonNeo.Sdk;
using FugleMarketData.QueryModels;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

var ticker = await rest.Intraday.Tickers(FugleMarketData.QueryModels.Stock.Intraday.TickersType.Equity);
var ticker_cont = ticker.Content.ReadAsStringAsync().Result;

Console.WriteLine(ticker_cont);

```

Response Body：

```json
{
  "date": "2023-02-09",
  "type": "EQUITY",
  "exchange": "TWSE",
  "isNormal": true,
  "data": [
    {
      "symbol": "0050",
      "name": "元大台灣50"
    },
    {
      "symbol": "0051",
      "name": "元大中型100"
    },
    {
      "symbol": "0052",
      "name": "富邦科技"
    },
    ......
  ]
}

```

#### 產業別代碼[​](#產業別代碼 "Direct link to 產業別代碼")

| 代碼 | 產業別   | 代碼 | 產業別           | 代碼 | 產業別       |
| ---- | -------- | ---- | ---------------- | ---- | ------------ |
| `01` | 水泥工業 | `16` | 觀光餐旅         | `29` | 電子通路業   |
| `02` | 食品工業 | `17` | 金融保險         | `30` | 資訊服務業   |
| `03` | 塑膠工業 | `18` | ~~貿易百貨~~     | `31` | 其他電子業   |
| `04` | 紡織纖維 | `19` | 綜合             | `32` | 文化創意業   |
| `05` | 電機機械 | `20` | 其他             | `33` | 農業科技業   |
| `06` | 電器電纜 | `21` | 化學工業         | `34` | ~~電子商務~~ |
| `08` | 玻璃陶瓷 | `22` | 生技醫療業       | `35` | 綠能環保     |
| `09` | 造紙工業 | `23` | 油電燃氣業       | `36` | 數位雲端     |
| `10` | 鋼鐵工業 | `24` | 半導體業         | `37` | 運動休閒     |
| `11` | 橡膠工業 | `25` | 電腦及週邊設備業 | `38` | 居家生活     |
| `12` | 汽車工業 | `26` | 光電業           | `80` | 管理股票     |
| `14` | 建材營造 | `27` | 通信網路業       |      |              |
| `15` | 航運業   | `28` | 電子零組件業     |      |              |


---

### Intraday Trades

股票成交明細（依代碼查詢）

```text
intraday/trades/{symbol}

```

#### Parameters[​](#parameters "Direct link to Parameters")

| Name       | Type   | Description                         |
| ---------- | ------ | ----------------------------------- |
| `symbol`\* | string | 股票代碼                            |
| `type`     | string | Ticker 類型，可選 `oddlot` 盤中零股 |
| `offset`   | number | 偏移量                              |
| `limit`    | number | 限制量                              |

#### Response[​](#response "Direct link to Response")

| Name         | Type      | Description |
| ------------ | --------- | ----------- |
| `date`\*     | string    | 日期        |
| `type`\*     | string    | Ticker 類型 |
| `exchange`\* | string    | 交易所      |
| `market`     | string    | 市場別      |
| `symbol`\*   | string    | 股票代號    |
| `data`\*     | object\[] | 成交明細    |
| >> `bid`     | number    | 成交買價    |
| >> `ask`     | number    | 成交賣價    |
| >> `price`   | number    | 成交價格    |
| >> `size`    | number    | 成交單量    |
| >> `volume`  | number    | 成交總量    |
| >> `time`    | number    | 成交時間    |

info

'\*' 表示必揭示欄位。

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
# reststock.intraday.trades(symbol='2330')  # 2.2.3 及以前版本

## 2.2.4 及以後版本 (使用 Exception 進行例外處理)
from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

try:  
  reststock.intraday.trades(symbol='2330') 
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"} 

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient
client.stock.intraday.trades({ symbol: '2330' })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

var trade = await rest.Intraday.Trades("2330");
// var trade = await rest.Intraday.Trades("2330", new(){Type=FugleMarketData.QueryModels.TickerType.OddLot});  // 零股

var trade_cont = trade.Content.ReadAsStringAsync().Result;

Console.WriteLine(trade_cont);

```

Response Body：

```json
{
  "date": "2023-05-29",
  "type": "EQUITY",
  "exchange": "TWSE",
  "market": "TSE",
  "symbol": "2330",
  "data": [
    {
      "price": 568,
      "size": 32,
      "time": 1685341800000000,
      "serial": 99999999
    },
    {
      "bid": 567,
      "ask": 568,
      "price": 568,
      "size": 4778,
      "volume": 54538,
      "time": 1685338200000000,
      "serial": 6652422
    },
    {
      "bid": 565,
      "ask": 566,
      "price": 566,
      "size": 1,
      "volume": 49760,
      "time": 1685337899721587,
      "serial": 6622549
    },
    ......
  ]
}

```


---

### Intraday Volumes

股票分價量表（依代碼查詢）

```text
intraday/volumes/{symbol}

```

#### Parameters[​](#parameters "Direct link to Parameters")

| Name       | Type   | Description                  |
| ---------- | ------ | ---------------------------- |
| `symbol`\* | string | 股票代碼                     |
| `type`     | string | 類型，可選 `oddlot` 盤中零股 |

#### Response[​](#response "Direct link to Response")

| Name             | Type      | Description          |
| ---------------- | --------- | -------------------- |
| `date`\*         | string    | 日期                 |
| `type`\*         | string    | Ticker 類型          |
| `exchange`\*     | string    | 交易所               |
| `market`         | string    | 市場別               |
| `symbol`\*       | string    | 股票代號             |
| `data`           | object\[] | 分價量表             |
| >> `price`       | number    | 成交價               |
| >> `volume`      | number    | 該成交價之累計成交量 |
| >> `volumeAtBid` | number    | 委買成筆             |
| >> `volumeAtAsk` | number    | 委賣成筆             |

info

'\*' 表示必揭示欄位。

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
# reststock.intraday.volumes(symbol='2330')  # 2.2.3 及以前版本

## 2.2.4 及以後版本 (使用 Exception 進行例外處理)
from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

try:  
  reststock.intraday.volumes(symbol='2330') 
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"}

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient
client.stock.intraday.volumes({ symbol: '2330' })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

var volume = await rest.Intraday.Volume("2330");
// var volume = await rest.Intraday.Volume("2330", , new(){Type=FugleMarketData.QueryModels.TickerType.OddLot});  // 零股
var volume_cont = volume.Content.ReadAsStringAsync().Result;

Console.WriteLine(volume_cont);

```

Response Body：

```json
{
  "date": "2023-05-29",
  "type": "EQUITY",
  "exchange": "TWSE",
  "market": "TSE",
  "symbol": "2330",
  "data": [
    {
      "price": 574,
      "volume": 7309,
      "volumeAtBid": 0,
      "volumeAtAsk": 524
    },
    {
      "price": 573,
      "volume": 771,
      "volumeAtBid": 657,
      "volumeAtAsk": 114
    },
    {
      "price": 572,
      "volume": 3364,
      "volumeAtBid": 843,
      "volumeAtAsk": 2521
    },
    {
      "price": 571,
      "volume": 3723,
      "volumeAtBid": 1026,
      "volumeAtAsk": 2697
    },
    {
      "price": 570,
      "volume": 5541,
      "volumeAtBid": 3019,
      "volumeAtAsk": 2522
    },
    {
      "price": 569,
      "volume": 1952,
      "volumeAtBid": 1318,
      "volumeAtAsk": 634
    },
    {
      "price": 568,
      "volume": 7906,
      "volumeAtBid": 1422,
      "volumeAtAsk": 6484
    },
    {
      "price": 567,
      "volume": 10498,
      "volumeAtBid": 2816,
      "volumeAtAsk": 7682
    },
    {
      "price": 566,
      "volume": 8206,
      "volumeAtBid": 5988,
      "volumeAtAsk": 2218
    },
    {
      "price": 565,
      "volume": 4833,
      "volumeAtBid": 2329,
      "volumeAtAsk": 2504
    },
    {
      "price": 564,
      "volume": 435,
      "volumeAtBid": 435,
      "volumeAtAsk": 0
    }
  ]
}

```


---

### Snapshot Actives

股票成交量值排行（依市場別）

```text
snapshot/actives/{market}

```

#### Parameters[​](#parameters "Direct link to Parameters")

| Name       | Type   | Description                                                                                 |
| ---------- | ------ | ------------------------------------------------------------------------------------------- |
| `market`\* | string | 市場別，可選 `TSE` 上市；`OTC` 上櫃；`ESB` 興櫃一般板；`TIB` 臺灣創新板；`PSB` 興櫃戰略新板 |
| `trade`\*  | string | 成交量／成交值，可選 `volume` 成交量；`value` 成交值                                        |
| `type`     | string | 標的類型，可選 `ALLBUT099` 包含一般股票、特別股及ETF ； `COMMONSTOCK` 為一般股票            |

#### Response[​](#response "Direct link to Response")

| Name               | Type      | Description    |
| ------------------ | --------- | -------------- |
| `date`\*           | string    | 日期           |
| `time`\*           | string    | 時間           |
| `market`\*         | string    | 市場別         |
| `trade`\*          | string    | 成交量／成交值 |
| `data`\*           | object\[] | 快照資料       |
| >> `type`          | string    | Ticker 類型    |
| >> `symbol`        | string    | 股票代碼       |
| >> `name`          | string    | 股票簡稱       |
| >> `openPrice`     | number    | 開盤價         |
| >> `highPrice`     | number    | 最高價         |
| >> `lowPrice`      | number    | 最低價         |
| >> `closePrice`    | number    | 收盤價         |
| >> `change`        | number    | 漲跌           |
| >> `changePercent` | number    | 漲跌幅         |
| >> `tradeVolume`   | number    | 成交量         |
| >> `tradeValue`    | number    | 成交金額       |
| >> `lastUpdated`   | number.   | 快照時間       |

info

'\*' 表示必揭示欄位。

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
# reststock.snapshot.actives(market='TSE', trade='value')  # 2.2.3 及以前版本

## 2.2.4 及以後版本 (使用 Exception 進行例外處理)
from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

try:  
  reststock.snapshot.actives(market='TSE', trade='value') 
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"} 

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient
client.stock.snapshot.actives({ market: 'TSE', trade: 'value' })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;
using FugleMarketData.QueryModels;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime();  // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

var quote = await rest.Snapshot.Quotes(FugleMarketData.QueryModels.MarketType.TSE)
var quote_cont = quote.Content.ReadAsStringAsync().Result;

Console.WriteLine(quote_cont);

```

Response Body：

```json
{
  "date": "2023-05-29",
  "time": "140000",
  "market": "TSE",
  "trade": "value",
  "data": [
    {
      "type": "EQUITY",
      "symbol": "2330",
      "name": "台積電",
      "openPrice": 574,
      "highPrice": 574,
      "lowPrice": 564,
      "closePrice": 568,
      "change": 2,
      "changePercent": 0.35,
      "tradeVolume": 54538,
      "tradeValue": 31019803000,
      "lastUpdated": 1685338200000000
    },
    {
      "type": "EQUITY",
      "symbol": "3231",
      "name": "緯創",
      "openPrice": 66,
      "highPrice": 69.2,
      "lowPrice": 65.9,
      "closePrice": 68,
      "change": 3,
      "changePercent": 4.62,
      "tradeVolume": 202089,
      "tradeValue": 13763365000,
      "lastUpdated": 1685338200000000
    },
    {
      "type": "EQUITY",
      "symbol": "3661",
      "name": "世芯-KY",
      "openPrice": 1555,
      "highPrice": 1660,
      "lowPrice": 1550,
      "closePrice": 1660,
      "change": 150,
      "changePercent": 9.93,
      "tradeVolume": 6937,
      "tradeValue": 11264055000,
      "lastUpdated": 1685338200000000
    },
    ......
  ]
}

```


---

### Snapshot Movers

股票漲跌幅排行（依市場別）

```text
snapshot/movers/{market}

```

#### Parameters[​](#parameters "Direct link to Parameters")

| Name          | Type   | Description                                                                      |
| ------------- | ------ | -------------------------------------------------------------------------------- |
| `market`\*    | string | 市場別                                                                           |
| `direction`\* | string | 上漲／下跌，可選 `up` 上漲；`down` 下跌                                          |
| `change`\*    | string | 漲跌／漲跌幅，可選 `percent` 漲跌幅；`value` 漲跌                                |
| `gt`          | number | 篩選大於漲跌／漲跌幅的股票                                                       |
| `type`        | string | 標的類型，可選 `ALLBUT099` 包含一般股票、特別股及ETF ； `COMMONSTOCK` 為一般股票 |
| `gte`         | number | 篩選大於或等於漲跌／漲跌幅的股票                                                 |
| `lt`          | number | 篩選小於漲跌／漲跌幅的股票                                                       |
| `lte`         | number | 篩選小於或等於漲跌／漲跌幅的股票                                                 |
| `eq`          | number | 篩選等於漲跌／漲跌幅的股票                                                       |

#### Response[​](#response "Direct link to Response")

| Name               | Type      | Description  |
| ------------------ | --------- | ------------ |
| `date`\*           | string    | 日期         |
| `time`\*           | string    | 時間         |
| `market`\*         | string    | 市場別       |
| `change`\*         | string    | 漲跌／漲跌幅 |
| `data`\*           | object\[] | 快照資料     |
| >> `type`          | string    | Ticker 類型  |
| >> `symbol`        | string    | 股票代碼     |
| >> `name`          | string    | 股票簡稱     |
| >> `openPrice`     | number    | 開盤價       |
| >> `highPrice`     | number    | 最高價       |
| >> `lowPrice`      | number    | 最低價       |
| >> `closePrice`    | number    | 收盤價       |
| >> `change`        | number    | 漲跌         |
| >> `changePercent` | number    | 漲跌幅       |
| >> `tradeVolume`   | number    | 成交量       |
| >> `tradeValue`    | number    | 成交金額     |
| >> `lastUpdated`   | number    | 快照時間     |

info

'\*' 表示必揭示欄位。

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
# reststock.snapshot.movers(market='TSE', direction='up', change='percent')  # 2.2.3 及以前版本

## 2.2.4 及以後版本 (使用 Exception 進行例外處理)
from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

try:  
  reststock.snapshot.movers(market='TSE', direction='up', change='percent')
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"}  

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient
client.stock.snapshot.movers({ market: 'TSE', direction: 'up', change: 'percent' })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;
using FugleMarketData.QueryModels;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

var movers = await rest.Snapshot.Movers(FugleMarketData.QueryModels.MarketType.TSE, FugleMarketData.QueryModels.DirectionType.Up, 
                                          FugleMarketData.QueryModels.ChangeType.Percent);

var movers_cont = movers.Content.ReadAsStringAsync().Result;

Console.WriteLine(movers_cont);

```

Response Body：

```json
{
  "date": "2023-05-29",
  "time": "133500",
  "market": "TSE",
  "change": "percent",
  "data": [
    {
      "type": "EQUITY",
      "symbol": "2901",
      "name": "欣欣",
      "openPrice": 27.1,
      "highPrice": 29.7,
      "lowPrice": 27.1,
      "closePrice": 29.7,
      "change": 2.7,
      "changePercent": 10,
      "tradeVolume": 640,
      "tradeValue": 18824300,
      "lastUpdated": 1685338200000000
    },
    {
      "type": "EQUITY",
      "symbol": "2345",
      "name": "智邦",
      "openPrice": 357.5,
      "highPrice": 357.5,
      "lowPrice": 346.5,
      "closePrice": 357.5,
      "change": 32.5,
      "changePercent": 10,
      "tradeVolume": 9350,
      "tradeValue": 3331334500,
      "lastUpdated": 1685338200000000
    },
    {
      "type": "EQUITY",
      "symbol": "3025",
      "name": "星通",
      "openPrice": 42.3,
      "highPrice": 44.65,
      "lowPrice": 41.8,
      "closePrice": 44.65,
      "change": 4.05,
      "changePercent": 9.98,
      "tradeVolume": 25625,
      "tradeValue": 1114127050,
      "lastUpdated": 1685338200000000
    },
    ......
  ]
}

```


---

### Snapshot Quotes

股票行情快照（依市場別）

```text
snapshot/quotes/{market}

```

#### Parameters[​](#parameters "Direct link to Parameters")

| Name       | Type   | Description                                                                                 |
| ---------- | ------ | ------------------------------------------------------------------------------------------- |
| `market`\* | string | 市場別，可選 `TSE` 上市；`OTC` 上櫃；`ESB` 興櫃一般板；`TIB` 臺灣創新板；`PSB` 興櫃戰略新板 |
| `type`     | string | 標的類型，可選 `ALLBUT099` 包含一般股票、特別股及ETF ； `COMMONSTOCK` 為一般股票            |

#### Response[​](#response "Direct link to Response")

| Name               | Type      | Description              |
| ------------------ | --------- | ------------------------ |
| `date`\*           | string    | 日期                     |
| `time`\*           | string    | 時間                     |
| `market`\*         | string    | 市場別                   |
| `data`\*           | object\[] | 快照資料                 |
| >> `type`          | string    | Ticker 類型              |
| >> `symbol`        | string    | 股票代碼                 |
| >> `name`          | string    | 股票簡稱                 |
| >> `openPrice`     | number    | 開盤價                   |
| >> `highPrice`     | number    | 最高價                   |
| >> `lowPrice`      | number    | 最低價                   |
| >> `closePrice`    | number    | 收盤價                   |
| >> `change`        | number    | 漲跌                     |
| >> `changePercent` | number    | 漲跌幅                   |
| >> `tradeVolume`   | number    | 成交量                   |
| >> `tradeValue`    | number    | 成交金額                 |
| >> `lastUpdated`   | number    | 快照時間                 |
| >> `lastPrice`     | number    | 最後一筆成交價（含試撮） |
| >> `isTrial`       | bool      | 試撮註記                 |

info

'\*' 表示必揭示欄位。

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
# reststock.snapshot.quotes(market='TSE')  # 2.2.3 及以前版本

## 2.2.4 及以後版本 (使用 Exception 進行例外處理)
from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

try:  
  reststock.snapshot.quotes(market='TSE')
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"}   

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient
client.stock.snapshot.quotes({ market: 'TSE' })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;
using FugleMarketData.QueryModels;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線
 
var rest = sdk.MarketData.RestClient.Stock;

var quotes = await rest.Snapshot.Quotes(FugleMarketData.QueryModels.MarketType.TSE);

var quotes_cont = quotes.Content.ReadAsStringAsync().Result;

Console.WriteLine(quotes_cont);

```

Response Body：

```json
{
  "date": "2023-05-29",
  "time": "133500",
  "market": "TSE",
  "data": [
    {
      "type": "EQUITY",
      "symbol": "0050",
      "name": "元大台灣50",
      "openPrice": 127.45,
      "highPrice": 127.55,
      "lowPrice": 126.5,
      "closePrice": 126.75,
      "change": 0.8,
      "changePercent": 0.64,
      "tradeVolume": 14444,
      "tradeValue": 1832941850,
      "lastUpdated": 1685338200000000
    },
    {
      "type": "EQUITY",
      "symbol": "0051",
      "name": "元大中型100",
      "openPrice": 61,
      "highPrice": 62.2,
      "lowPrice": 61,
      "closePrice": 62,
      "change": 1.3,
      "changePercent": 2.14,
      "tradeVolume": 191,
      "tradeValue": 11816050,
      "lastUpdated": 1685338200000000
    },
    {
      "type": "EQUITY",
      "symbol": "0052",
      "name": "富邦科技",
      "openPrice": 111.8,
      "highPrice": 112.1,
      "lowPrice": 111.1,
      "closePrice": 111.7,
      "change": 1.2,
      "changePercent": 1.09,
      "tradeVolume": 565,
      "tradeValue": 63088400,
      "lastUpdated": 1685338200000000
    },
    ......
  ]
}

```


---

### Technical Bbands

取得特定股票在指定時間範圍內的布林通通 (Bollinger Bands)

```text
technical/bb/{symbol}

```

版本資訊

v2.2.6 起新增功能

#### Parameters[​](#parameters "Direct link to Parameters")

| Name          | Type   | Description                                                                                                           |
| ------------- | ------ | --------------------------------------------------------------------------------------------------------------------- |
| `symbol`\*    | string | 股票代碼                                                                                                              |
| `from`\*      | string | 開始日期（格式：`yyyy-MM-dd`）                                                                                        |
| `to`\*        | string | 結束日期（格式：`yyyy-MM-dd`）                                                                                        |
| `timeframe`\* | string | Ｋ線週期，可選 `1` 1分Ｋ；`5` 5分Ｋ；`10` 10分Ｋ；`15` 15分Ｋ；`30` 30分Ｋ；`60` 60分Ｋ；`D` 日Ｋ；`W` 週Ｋ；`M` 月Ｋ |
| `period`\*    | number | Bolling 週期                                                                                                          |

caution

目前分Ｋ無法指定開始日期（from） 與 結束日期（to），一律回傳近 30 日資料。

#### Response[​](#response "Direct link to Response")

| Name                | Type   | Description  |
| ------------------- | ------ | ------------ |
| `symbol`\*          | string | 股票代號     |
| `from`\*            | string | 開始日期     |
| `to`\*              | string | 結束日期     |
| `timeframe`\*       | string | Ｋ線週期     |
| `period`\*          | string | RSI 週期     |
| `data`              | Object | RSI 資料     |
| >> `data[0].date`   | string | 資料日期     |
| >> `data[0].upper`  | number | Bolling 上軌 |
| >> `data[0].middle` | number | Bolling 中軌 |
| >> `data[0].lower`  | number | Bolling 下軌 |

info

'\*' 表示必揭示欄位。

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

## 2.2.6 及以後版本使用更簡化 (使用 Exception 進行例外處理)
from fubon_neo.sdk import FugleAPIError

try:  
  reststock.technical.bb(**{"symbol": "2330", "from": "2024-08-01", "to": "2024-08-10","timeframe":"D", "period": 10})
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"}     

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient
client.stock.technical.bb({ symbol: '2330', from: '2024-08-01', to: '2024-08-10', timeframe: 'D', period: 10 })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;
using FugleMarketData.QueryModels.Stock.History;   //引入 HistoryTimeFrame
using FugleMarketData.QueryModels;                 //引入 FieldsType



var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

DateTime fromDate = new DateTime(2024, 8, 1);
DateTime toDate = new DateTime(2024, 8, 10);

var bband = await rest.Technical.Bb("2330" ,new(10, fromDate, toDate, HistoryTimeFrame.Day)); 
//BbRequest 參數  (period, DateTime, DateTime, HistoryTimeFrame)

var bband_con = bband.Content.ReadAsStringAsync().Result;

Console.WriteLine(bband_con);


```

Response Body：

```json
{
  "symbol": "2330",
  "from": "2024-08-01",
  "to": "2024-08-10",
  "timeframe": "D",
  "period": 20,
  "data": [
    {
      "date": "2024-08-01",
      "upper": 1089.4184184924368,
      "middle": 997.45,
      "lower": 905.4815815075632
    },
    {
      "date": "2024-08-02",
      "upper": 1094.230862990929,
      "middle": 993.65,
      "lower": 893.0691370090711
    },
    {
      "date": "2024-08-05",
      "upper": 1111.086637737101,
      "middle": 984.15,
      "lower": 857.2133622628991
    },
    {
      "date": "2024-08-06",
      "upper": 1112.2099400640175,
      "middle": 977.9,
      "lower": 843.5900599359824
    },
    {
      "date": "2024-08-07",
      "upper": 1106.0354361011682,
      "middle": 972.15,
      "lower": 838.2645638988317
    },
    {
      "date": "2024-08-08",
      "upper": 1098.9536939789348,
      "middle": 964.95,
      "lower": 830.9463060210653
    },
    {
      "date": "2024-08-09",
      "upper": 1088.7976815866498,
      "middle": 959.4,
      "lower": 830.0023184133502
    }
  ]
}

```


---

### Technical KDJ

取得特定股票在指定時間範圍內的隨機指標 (KDJ)

```text
technical/kdj/{symbol}

```

版本資訊

v2.2.6 起新增功能

#### Parameters[​](#parameters "Direct link to Parameters")

| Name          | Type   | Description                                                                                                           |
| ------------- | ------ | --------------------------------------------------------------------------------------------------------------------- |
| `symbol`\*    | string | 股票代碼                                                                                                              |
| `from`\*      | string | 開始日期（格式：`yyyy-MM-dd`）                                                                                        |
| `to`\*        | string | 結束日期（格式：`yyyy-MM-dd`）                                                                                        |
| `timeframe`\* | string | Ｋ線週期，可選 `1` 1分Ｋ；`5` 5分Ｋ；`10` 10分Ｋ；`15` 15分Ｋ；`30` 30分Ｋ；`60` 60分Ｋ；`D` 日Ｋ；`W` 週Ｋ；`M` 月Ｋ |
| `rPeriod`\*   | number | KDJ 週期                                                                                                              |
| `kPeriod`\*   | number | %K 週期                                                                                                               |
| `dPeriod`\*   | number | %D 週期                                                                                                               |

caution

目前分Ｋ無法指定開始日期（from） 與 結束日期（to），一律回傳近 30 日資料。

#### Response[​](#response "Direct link to Response")

| Name              | Type   | Description |
| ----------------- | ------ | ----------- |
| `symbol`\*        | string | 股票代號    |
| `from`\*          | string | 開始日期    |
| `to`\*            | string | 結束日期    |
| `timeframe`\*     | string | Ｋ線週期    |
| `rPeriod`\*       | number | KDJ 週期    |
| `kPeriod`\*       | number | %K 週期     |
| `dPeriod`\*       | number | %D 週期     |
| `data`\*          | Object | KDJ 資料    |
| >> `data[0].date` | string | 資料日期    |
| >> `data[0].k`    | number | K           |
| >> `data[0].d`    | number | D           |
| >> `data[0].j`    | number | J           |

info

'\*' 表示必揭示欄位。

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

## 2.2.6 及以後版本使用更簡化 (使用 Exception 進行例外處理)
from fubon_neo.sdk import FugleAPIError

try:  
  reststock.technical.kdj(**{"symbol": "2330", "from": "2024-08-01", "to": "2024-08-10","timeframe":"D", "rPeriod": 9, "kPeriod":3, "dPeriod":3})
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"}     

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient
client.stock.technical.kdj({ symbol: '2330', from: '2024-08-01', to: '2024-08-10', timeframe: 'D', rPeriod: 9, kPeriod: 3, dPeriod: 3 })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;
using FugleMarketData.QueryModels.Stock.History;   //引入 HistoryTimeFrame
using FugleMarketData.QueryModels;                 //引入 FieldsType



var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

DateTime fromDate = new DateTime(2024, 8, 1);
DateTime toDate = new DateTime(2024, 8, 10);

var kdj = await rest.Technical.Kdj("2330" ,new(9,3,3, fromDate, toDate, HistoryTimeFrame.Day)); 
//Kdj 參數  (rPeriod, kPeriod, dPeriod, DateTime, DateTime, HistoryTimeFrame)

var kdj_con = kdj.Content.ReadAsStringAsync().Result;

Console.WriteLine(kdj_con);


```

Response Body：

```json
{
  "symbol": "2330",
  "from": "2024-08-01",
  "to": "2024-08-10",
  "timeframe": "D",
  "rPeriod": 9,
  "kPeriod": 3,
  "dPeriod": 3,
  "data": [
    {
      "date": "2024-08-01",
      "k": 32.96296296296297,
      "d": 27.77777777777779,
      "j": 43.333333333333336
    },
    {
      "date": "2024-08-02",
      "k": 23.703703703703713,
      "d": 27.901234567901245,
      "j": 15.308641975308653
    },
    {
      "date": "2024-08-05",
      "k": 17.068273092369484,
      "d": 24.578313253012055,
      "j": 2.048192771084345
    },
    {
      "date": "2024-08-06",
      "k": 13.855421686746999,
      "d": 18.20913282760673,
      "j": 5.147999405027534
    },
    {
      "date": "2024-08-07",
      "k": 36.718669549994864,
      "d": 22.547454776370447,
      "j": 65.0610990972437
    },
    {
      "date": "2024-08-08",
      "k": 54.05210585933478,
      "d": 34.87539903202555,
      "j": 92.40551951395325
    },
    {
      "date": "2024-08-09",
      "k": 66.45299145299147,
      "d": 52.40792228744038,
      "j": 94.54312978409367
    }
  ]
}


```


---

### Technical MACD

取得特定股票在指定時間範圍內的指數平滑異同移動平均線 (MACD)

```text
technical/macd/{symbol}

```

版本資訊

v2.2.6 起新增功能

#### Parameters[​](#parameters "Direct link to Parameters")

| Name          | Type   | Description                                                                                                           |
| ------------- | ------ | --------------------------------------------------------------------------------------------------------------------- |
| `symbol`\*    | string | 股票代碼                                                                                                              |
| `from`\*      | string | 開始日期（格式：`yyyy-MM-dd`）                                                                                        |
| `to`\*        | string | 結束日期（格式：`yyyy-MM-dd`）                                                                                        |
| `timeframe`\* | string | Ｋ線週期，可選 `1` 1分Ｋ；`5` 5分Ｋ；`10` 10分Ｋ；`15` 15分Ｋ；`30` 30分Ｋ；`60` 60分Ｋ；`D` 日Ｋ；`W` 週Ｋ；`M` 月Ｋ |
| `fast`\*      | number | 快線週期                                                                                                              |
| `slow`\*      | number | 慢線週期                                                                                                              |
| `signal`\*    | number | 信號線週期                                                                                                            |

caution

目前分Ｋ無法指定開始日期（from） 與 結束日期（to），一律回傳近 30 日資料。

#### Response[​](#response "Direct link to Response")

| Name                    | Type   | Description |
| ----------------------- | ------ | ----------- |
| `symbol`\*              | string | 股票代號    |
| `from`\*                | string | 開始日期    |
| `to`\*                  | string | 結束日期    |
| `timeframe`\*           | string | Ｋ線週期    |
| `fast`\*                | number | 快線週期    |
| `slow`\*                | number | 慢線週期    |
| `signal`\*              | number | 信號線週期  |
| `data`                  | Object | RSI 資料    |
| >> `data[0].date`       | string | 資料日期    |
| >> `data[0].macdLine`   | number | MACD 線     |
| >> `data[0].signalLine` | number | 信號線      |

info

'\*' 表示必揭示欄位。

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

## 2.2.6 及以後版本使用更簡化 (使用 Exception 進行例外處理)
from fubon_neo.sdk import FugleAPIError

try:  
  reststock.technical.macd(**{"symbol": "2330", "from": "2024-08-01", "to": "2024-08-10","timeframe":"D", "fast": 12, "slow":26, "signal":9})
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"}     

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient
client.stock.technical.macd({ symbol: '2330', from: '2024-08-01', to: '2024-08-10', timeframe: 'D', fast: 12, slow: 26, signal: 9, dPeriod: 3 })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;
using FugleMarketData.QueryModels.Stock.History;   //引入 HistoryTimeFrame
using FugleMarketData.QueryModels;                 //引入 FieldsType



var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

DateTime fromDate = new DateTime(2024, 8, 1);
DateTime toDate = new DateTime(2024, 8, 10);

var macd = await rest.Technical.Macd("2330" ,new( 12, 26, 9, fromDate, oDate, HistoryTimeFrame.Day));
//MacdRequest 參數  (fast, slow, signal  DateTime, DateTime, HistoryTimeFrame)

var macd_con = macd.Content.ReadAsStringAsync().Result;

Console.WriteLine(macd_con);


```

Response Body：

```json
{
  "symbol": "2330",
  "from": "2024-08-01",
  "to": "2024-08-10",
  "timeframe": "D",
  "fast": 12,
  "slow": 26,
  "signal": 9,
  "data": [
    {
      "date": "2024-08-01",
      "macdLine": -8.888098865883194,
      "signalLine": 1.1835714956164298
    },
    {
      "date": "2024-08-02",
      "macdLine": -13.342205320023709,
      "signalLine": -1.721583867511598
    },
    {
      "date": "2024-08-05",
      "macdLine": -23.69978495993405,
      "signalLine": -6.117224085996089
    },
    {
      "date": "2024-08-06",
      "macdLine": -26.359429578554114,
      "signalLine": -10.165665184507695
    },
    {
      "date": "2024-08-07",
      "macdLine": -24.951921179141777,
      "signalLine": -13.12291638343451
    },
    {
      "date": "2024-08-08",
      "macdLine": -25.47934996958338,
      "signalLine": -15.594203100664284
    },
    {
      "date": "2024-08-09",
      "macdLine": -22.570875660446973,
      "signalLine": -16.989537612620822
    }
  ]
}

```


---

### Technical RSI

取得特定股票在指定時間範圍內的相對強弱指標 (RSI)

```text
technical/rsi/{symbol}

```

版本資訊

v2.2.6 起新增功能

#### Parameters[​](#parameters "Direct link to Parameters")

| Name          | Type   | Description                                                                                                           |
| ------------- | ------ | --------------------------------------------------------------------------------------------------------------------- |
| `symbol`\*    | string | 股票代碼                                                                                                              |
| `from`\*      | string | 開始日期（格式：`yyyy-MM-dd`）                                                                                        |
| `to`\*        | string | 結束日期（格式：`yyyy-MM-dd`）                                                                                        |
| `timeframe`\* | string | Ｋ線週期，可選 `1` 1分Ｋ；`5` 5分Ｋ；`10` 10分Ｋ；`15` 15分Ｋ；`30` 30分Ｋ；`60` 60分Ｋ；`D` 日Ｋ；`W` 週Ｋ；`M` 月Ｋ |
| `period`\*    | number | RSI 週期                                                                                                              |

caution

目前分Ｋ無法指定開始日期（from） 與 結束日期（to），一律回傳近 30 日資料。

#### Response[​](#response "Direct link to Response")

| Name              | Type   | Description |
| ----------------- | ------ | ----------- |
| `symbol`\*        | string | 股票代號    |
| `from`\*          | string | 開始日期    |
| `to`\*            | string | 結束日期    |
| `timeframe`\*     | string | Ｋ線週期    |
| `period`\*        | string | RSI 週期    |
| `data`            | Object | RSI 資料    |
| >> `data[0].date` | string | 資料日期    |
| >> `data[0].rsi`  | number | RSI         |

info

'\*' 表示必揭示欄位。

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

## 2.2.6 及以後版本使用更簡化 (使用 Exception 進行例外處理)
from fubon_neo.sdk import FugleAPIError

try:  
  reststock.technical.rsi(**{"symbol": "2330", "from": "2024-08-01", "to": "2024-08-10","timeframe":"D", "period": 6})
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"}     

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient
client.stock.technical.rsi({ symbol: '2330', from: '2024-08-01', to: '2024-08-10', timeframe: 'D', period: 6 })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;
using FugleMarketData.QueryModels.Stock.History;   //引入 HistoryTimeFrame
using FugleMarketData.QueryModels;                 //引入 FieldsType



var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

DateTime fromDate = new DateTime(2024, 8, 1);
DateTime toDate = new DateTime(2024, 8, 10);

var rsi = await rest.Technical.Rsi("2330" ,new(6, fromDate, toDate, HistoryTimeFrame.Day)); 
//RsiRequest 參數  (period, DateTime, DateTime, HistoryTimeFrame)

var rsi_con = rsi.Content.ReadAsStringAsync().Result;

Console.WriteLine(rsi_con);


```

Response Body：

```json
{
  "symbol": "2330",
  "from": "2024-08-01",
  "to": "2024-08-10",
  "timeframe": "D",
  "period": 6,
  "data": [
    {
      "date": "2024-08-01",
      "rsi": 41.44144144144145
    },
    {
      "date": "2024-08-02",
      "rsi": 25.641025641025635
    },
    {
      "date": "2024-08-05",
      "rsi": 15.026786880961723
    },
    {
      "date": "2024-08-06",
      "rsi": 37.83577095879935
    },
    {
      "date": "2024-08-07",
      "rsi": 48.119604933543954
    },
    {
      "date": "2024-08-08",
      "rsi": 42.99811400274545
    },
    {
      "date": "2024-08-09",
      "rsi": 52.58621466649552
    }
  ]
}

```


---

### Technical SMA

取得特定股票在指定時間範圍內的簡單移動平均 (SMA)

```text
technical/sma/{symbol}

```

版本資訊

v2.2.6 起新增功能

#### Parameters[​](#parameters "Direct link to Parameters")

| Name          | Type   | Description                                                                                                           |
| ------------- | ------ | --------------------------------------------------------------------------------------------------------------------- |
| `symbol`\*    | string | 股票代碼                                                                                                              |
| `from`\*      | string | 開始日期（格式：`yyyy-MM-dd`）                                                                                        |
| `to`\*        | string | 結束日期（格式：`yyyy-MM-dd`）                                                                                        |
| `timeframe`\* | string | Ｋ線週期，可選 `1` 1分Ｋ；`5` 5分Ｋ；`10` 10分Ｋ；`15` 15分Ｋ；`30` 30分Ｋ；`60` 60分Ｋ；`D` 日Ｋ；`W` 週Ｋ；`M` 月Ｋ |
| `period`\*    | int    | SMA 週期                                                                                                              |

caution

目前分Ｋ無法指定開始日期（from） 與 結束日期（to），一律回傳近 30 日資料。

#### Response[​](#response "Direct link to Response")

| Name              | Type   | Description |
| ----------------- | ------ | ----------- |
| `symbol`\*        | string | 股票代號    |
| `from`\*          | string | 開始日期    |
| `to`\*            | string | 結束日期    |
| `timeframe`\*     | string | Ｋ線週期    |
| `period`\*        | string | SMA 週期    |
| `data`            | Object | SMA 資料    |
| >> `data[0].date` | string | 資料日期    |
| >> `data[0].sma`  | number | SMA         |

info

'\*' 表示必揭示欄位。

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

## 2.2.6 及以後版本使用更簡化 (使用 Exception 進行例外處理)
from fubon_neo.sdk import FugleAPIError

try:  
  reststock.technical.sma(**{"symbol": "2330", "from": "2024-08-01", "to": "2024-08-10","timeframe":"D", "period": 5})
except FugleAPIError as e:
  print(f"Error: {e}")
  print("------------")
  print(f"Status Code: {e.status_code}")  # 例: 429
  print(f"Response Text: {e.response_text}")  # 例: {"statusCode":429,"message":"Rate limit exceeded"}     

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient
client.stock.technical.sma({ symbol: '2330', from: '2024-08-01', to: '2024-08-10', timeframe: 'D', period: 5 })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;
using FugleMarketData.QueryModels.Stock.History;   //引入 HistoryTimeFrame
using FugleMarketData.QueryModels;                 //引入 FieldsType



var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.Stock;

DateTime fromDate = new DateTime(2024, 8, 1);
DateTime toDate = new DateTime(2024, 8, 10);

var sma = await rest.Technical.Sma("2330" ,new(5, fromDate, toDate, HistoryTimeFrame.Day)); 
//SmaRequest 參數  (period, DateTime, DateTime, HistoryTimeFrame)

var sma_con = sma.Content.ReadAsStringAsync().Result;

Console.WriteLine(sma_con);


```

Response Body：

```json
{
  "symbol": "2330",
  "from": "2024-08-01",
  "to": "2024-08-10",
  "timeframe": "D",
  "period": 5,
  "data": [
    {
      "date": "2024-08-01",
      "sma": 940.4
    },
    {
      "date": "2024-08-02",
      "sma": 936.2
    },
    {
      "date": "2024-08-05",
      "sma": 910.4
    },
    {
      "date": "2024-08-06",
      "sma": 898.4
    },
    {
      "date": "2024-08-07",
      "sma": 895.6
    },
    {
      "date": "2024-08-08",
      "sma": 882.8
    },
    {
      "date": "2024-08-09",
      "sma": 889
    }
  ]
}

```


---

