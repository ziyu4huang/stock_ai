### 富邦新一代 API 證券行情服務

本頁重點

* 富邦新一代 API（Fubon Neo API）證券行情服務提供台股即時與歷史資料。
* 介面包含 Web API 與 WebSocket API。
* 資料來源為交易所與資訊供應商，使用前請詳閱規範與聲明。
* 下一步可從 [Web API](https://www.fbs.com.tw/TradeAPI/docs/market-data/http-api/getting-started.txt) 或 [WebSocket API](https://www.fbs.com.tw/TradeAPI/docs/market-data/websocket-api/getting-started.txt) 開始。

| 項目     | 說明                                             |
| -------- | ------------------------------------------------ |
| 服務     | 新一代 API 證券行情服務                          |
| 市場     | 台灣證券市場                                     |
| 介面     | Web API / WebSocket API                          |
| 資料來源 | 臺灣證券交易所、證券櫃檯買賣中心、臺灣期貨交易所 |
| 規範     | 使用規範與聲明                                   |

即時行情資料來源為臺灣證券交易所、證券櫃檯買賣中心、臺灣期貨交易所。請您詳閱相關使用[規範與聲明](#statement)。

#### Web API[​](#web-api "Direct link to Web API")

富邦行情 [Web API](https://www.fbs.com.tw/TradeAPI/docs/market-data/http-api/getting-started.txt) 提供開發者友善的 API 服務。您可以查詢台股的日內行情、行情快照與歷史行情等數據；數據來源為時報資訊與群馥科技。

#### WebSocket API[​](#websocket-api "Direct link to WebSocket API")

富邦行情 [WebSocket API](https://www.fbs.com.tw/TradeAPI/docs/market-data/websocket-api/getting-started.txt) 提供台股即時行情服務。透過 WebSocket API 可以滿足您想要接收即時行情的需求。

#### 使用規範與聲明[​](#statement "Direct link to 使用規範與聲明")

* 透過本服務取得之行情資料僅供參考，成交值及成交量不含零股及鉅額交易，使用者依本資料交易發生交易損失需自行負責。
* 時報資訊與群馥科技對資料內容錯誤、更新延誤或傳輸中斷不負任何責任。您應對您所為之任何金融或投資決策自行負責。
* 使用者應遵守[臺灣證券交易所股份有限公司交易資訊使用管理辦法](https://www.selaw.com.tw/Chinese/RegulatoryInformationResult/Article?isIntegratedSearch=True\&sysNumber=LW10809760\&searchString1=%E8%87%BA%E7%81%A3%E8%AD%89%E5%88%B8%E4%BA%A4%E6%98%93%E6%89%80%E8%82%A1%E4%BB%BD%E6%9C%89%E9%99%90%E5%85%AC%E5%8F%B8%E4%BA%A4%E6%98%93%E8%B3%87%E8%A8%8A%E4%BD%BF%E7%94%A8%E7%AE%A1%E7%90%86%E8%BE%A6%E6%B3%95)、[臺灣期貨交易所股份有限公司交易資訊使用管理辦法](https://www.selaw.com.tw/Chinese/RegulatoryInformationResult/Article?isIntegratedSearch=True\&sysNumber=LW10815483\&searchString1=%E8%87%BA%E7%81%A3%E6%9C%9F%E8%B2%A8%E4%BA%A4%E6%98%93%E6%89%80%E8%82%A1%E4%BB%BD%E6%9C%89%E9%99%90%E5%85%AC%E5%8F%B8%E4%BA%A4%E6%98%93%E8%B3%87%E8%A8%8A%E4%BD%BF%E7%94%A8%E7%AE%A1%E7%90%86%E8%BE%A6%E6%B3%95)、[財團法人中華民國證券櫃檯買賣中心有價證券交易資訊使用管理辦法](https://www.selaw.com.tw/Chinese/RegulatoryInformationResult/Article?isIntegratedSearch=True\&sysNumber=LW10813063\&searchString1=%E8%B2%A1%E5%9C%98%E6%B3%95%E4%BA%BA%E4%B8%AD%E8%8F%AF%E6%B0%91%E5%9C%8B%E8%AD%89%E5%88%B8%E6%AB%83%E6%AA%AF%E8%B2%B7%E8%B3%A3%E4%B8%AD%E5%BF%83%E6%9C%89%E5%83%B9%E8%AD%89%E5%88%B8%E4%BA%A4%E6%98%93%E8%B3%87%E8%A8%8A%E4%BD%BF%E7%94%A8%E7%AE%A1%E7%90%86%E8%BE%A6%E6%B3%95)、各資訊來源提供者所定之資訊使用相關規範及智慧財產權相關法令，所有資訊以各資訊來源提供者公告資料為準。如有盜接 、轉接交易資訊，或以其他方式出售、出租、轉讓、再授權交易資訊，或將交易資訊另行取樣並編製指數、其他衍生性商品或將之傳送予第三人，應負違約及侵權之相關民、刑事責任。
* 富邦新一代 API 行情服務由 [Fugle](https://www.fugle.tw/) 技術團隊開發提供。


---

### 建立連線

##### 建立行情連線[​](#建立行情連線 "Direct link to 建立行情連線")

本頁重點

* 示範如何同時建立多條 WebSocket 行情連線（上限 5 條）。
* Web API 速率限制共用，WebSocket 每條連線可訂閱 200 檔。

以下範例程式碼示範如何同時建立 5 條行情 WebSocket 連線。

info

一般僅 WebSocket 需要同時啟用多條連線，若使用 Web API 則建議使用單一連線方便管理。 Web API 速率限制為所有連線共用；WebSocket 每條連線各可訂閱 200 檔標的。

* Python
* Node.js
* C#

```python
from fubon_neo.sdk import FubonSDK

sdk = FubonSDK()
accounts = sdk.login("Your ID", "Your password" ,"Your cert path" ,"Your cert password")  # 需登入後，才能取得行情權限

websockets = []  # 不同連線 WebSocket 介面列表

for _ in range(5):  # 建立 5 條連線
  sdk.init_realtime() # 啟動一條行情連線
  ws = sdk.marketdata.websocket_client.stock  # WebSocket 介面 (證券)
  #ws = sdk.marketdata.websocket_client.futopt  # WebSocket 介面 (期貨)

  # 將介面置入可用列表中
  websockets.append(ws)

# 設定 WebSocket Callback 並開啟連結
for ws in websockets:
  ## Callback functions 需另外定義 ##
  ws.on('message', handle_message)  # 註冊行情資料 callback
  ws.on("connect", handle_connect)  # 註冊連線事件 callback
  ws.on("disconnect", handle_disconnect)  # 註冊斷線事件 callback
  ws.on("error", handle_error)  # 註冊錯誤事件 callback
  
  ws.connect()  # 啟用連結

# 斷開連線
# for ws in websockets:
#   we.disconnect()

```

```js
const { FubonSDK } = require('fubon-neo');

// 需先登入才能取得行情權限
const sdk = new FubonSDK();
const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

const websockets = [];  // 不同連線 WebSocket 介面列表

// 建立 5 條行情連線
for (let i = 0; i < 5; i++) {
  // 啟動一條行情連線
  sdk.initRealtime();

  // 取得 WebSocket 介面 
  const ws = sdk.marketdata.webSocketClient.stock;  // (證券)
  //const ws = sdk.marketdata.webSocketClient.futopt;  // (期貨)

  // 將介面置入可用列表中
  websockets.push(ws);
}

// 設定 WebSocket Callback 並開啟連結
for (const ws of websockets) {
  /* -- Callback functions 需另外定義 -- */
  ws.on('message', handle_message);       // 註冊行情資料 callback
  ws.on('connect', handle_connect);       // 註冊連線事件 callback
  ws.on('disconnect', handle_disconnect); // 註冊斷線事件 callback
  ws.on('error', handle_error);           // 註冊錯誤事件 callback

  // 啟用連結
  (async () => {
    await ws.connect();
  })()
}

// 斷開連線
//for (const ws of websockets) {
//  (async () => {
//    await ws.disconnect();
//  })()
//}

```

```cs

using FubonNeo.Sdk;

// 需先登入才能取得行情權限
var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

// 不同連線 WebSocket 介面列表
var websockets = new List<FugleMarketData.WebsocketClient.FugleWebsocketStockClient>();  // (證券)
//var websockets = new List<FugleMarketData.WebsocketClient.FugleWebsocketFutOptClient>();  // (期貨)

// 建立 5 條行情連線
for (int i = 0; i < 5; i++)
{
  sdk.InitRealtime(); // 建立行情連線

  // 取得 WebSocket 介面 
  var ws = sdk.MarketData.WebSocketClient.Stock;  // (證券)
  //var ws = sdk.MarketData.WebSocketClient.FutureOption;  // (期貨)

  // 將介面置入可用列表中
  websockets.Add(ws);
}

// 設定 WebSocket Callback 並開啟連結
foreach (var ws in websockets)
{
  ws.OnMessage = (msg) => Console.WriteLine($"OnMessage receive: {msg}");  // 註冊行情資料 callback
  ws.OnError = (msg) => Console.WriteLine($"OnError receive: {msg}");  // 註冊錯誤事件 callback
  ws.OnConnected = (msg) => Console.WriteLine($"OnConnected receive: {msg}");  // 註冊連線事件 callback
  ws.OnDisconnected = (msg) => Console.WriteLine($"OnDisconnected receive: {msg}");  // 註冊斷線事件 callback

  // 啟用連結
  await ws.Connect();
}

// 斷開連線
// foreach (var ws in websockets)
// {
//   await ws.Disconnect();
// }

```


---

### 速率限制

##### 速率限制[​](#速率限制 "Direct link to 速率限制")

本頁重點

* 本頁整理富邦新一代 API 行情服務的 Web API / WebSocket 速率限制。
* 超過限制將回應 `429` 或訂閱錯誤訊息。
* 若短時間內大量建立連線可能被判定為異常。

| 項目      | 上限                                   |
| --------- | -------------------------------------- |
| 日內行情  | 300 / min                              |
| 行情快照  | 300 / min                              |
| 歷史行情  | 60 / min                               |
| WebSocket | 單一連線 200 訂閱數；可同時開啟 5 連線 |

富邦新一代 API 行情服務提供 Web API 與 WebSocket，並設定了以下存取的限制

* 日內行情 : 300 / min
* WebSocket : 單一連線 200 訂閱數；可同時開啟 5 連線
* 行情快照 : 300 / min
* 歷史行情 : 60 / min

如果您 API 請求超過了限制，將收到帶有狀態碼 `429` 的回應。需再等候1分鐘。

```json
{"statusCode":429,"message":"Rate limit exceeded"}  

```

或是您的 WebSocket 訂閱數超過上限，將收到以下訊息:

```json
{
  "event": "error",
  "data": {
    "code": 1001,
    "message": "Maximum number of connections reached"
  }
}

```

caution

若短時間內持續大量建立連線，系統會判斷您為惡意攻擊，將會阻擋您的連線請求（404連線錯誤）， 若遇到此種狀況，請聯絡 <pm.trading.sec@fubon.com>


---

### 開始使用

本頁重點

* 富邦行情 WebSocket API（富邦新一代 API 行情服務）提供台股即時行情訂閱。
* 支援 `Speed` / `Normal` 兩種行情模式。
* 需登入後建立行情連線並訂閱頻道。
* 下一步可前往各頻道說明頁。

| 項目     | 說明                  |
| -------- | --------------------- |
| 介面     | WebSocket API         |
| 市場     | 台股                  |
| 模式     | Speed / Normal        |
| SDK      | Python / Node.js / C# |
| 訂閱方式 | 連線後訂閱頻道        |

富邦行情 WebSocket API 提供台股即時行情服務。透過 WebSocket API 可以滿足您想要接收即時行情的需求。

#### 使用 SDK[​](#使用-sdk "Direct link to 使用 SDK")

富邦行情 WebSocket API 提供 Python 、 Node.js 與 C# SDK。您可以透過以下方式存取 WebSocket API、並透過行情Mode模式切換要訂閱的行情：

訂閱WebSocket Callback 方法獲得下方Callback訊息。

info

行情Mode提供Low Latency `Speed` Mode 以及完整資訊的 `Normal` Mode

* Python
* Node.js
* C#

```python
from fubon_neo.sdk import FubonSDK,Mode

def handle_message(message):
    print(message)

sdk = FubonSDK()
accounts = sdk.login("Your ID", "Your password", "Your cert path", "Your cert password")  # 需登入後，才能取得行情權限

sdk.init_realtime() # 建立行情連線
# 可指定行情連線的Mode ， Default 為Speed 
# sdk.init_realtime(Mode.Speed) or sdk.init_realtime(Mode.Normal)
stock = sdk.marketdata.websocket_client.stock
stock.on('message', handle_message)
stock.connect()

```

```js
const { FubonSDK, Mode } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.initRealtime(); // 建立行情連線
// 可指定行情連線的Mode ， Default 為Speed 
// sdk.initRealtime(Mode.Speed) or sdk.initRealtime(Mode.Normal)

const stock = sdk.marketdata.webSocketClient.stock;

stock.on("message", (message) => {
  const data = JSON.parse(message);
  console.log(data);
});

(async () => {
    await stock.connect();

    stock.subscribe({
        'channel': 'trades',
        'symbol': '2881'
    });
})()


```

```cs
using FubonNeo.Sdk;
using FugleMarketData.WebsocketModels;

var sdk = new FubonSDK();

var result = sdk.Login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.InitRealtime(); // 建立行情連線
// 可指定行情連線的Mode ， Default 為Speed 
// sdk.InitRealtime(Mode.Speed) or sdk.InitRealtime(Mode.Normal)

var stock = sdk.MarketData.WebSocketClient.Stock;

stock.OnMessage += (msg) => Console.WriteLine($"receive: {msg}");

await stock.Connect();

```

#### 身份驗證[​](#身份驗證 "Direct link to 身份驗證")

當驗證成功後，會收到以下訊息：

```json
{
  "event": "authenticated",
  "data": {
    "message": "Authenticated successfully"
  }
}

```

若驗證失敗，則收到以下訊息：

```json
{
  "event": "error",
  "data": {
    "message": "Invalid authentication credentials"
  }
}

```

#### Heartbeat[​](#heartbeat "Direct link to Heartbeat")

每隔 30 秒 WebSocket server 會送出一個 heartbeat 訊息：

```json
{
  "event": "heartbeat",
  "data": {
    "time": "<Timestamp>"
  }
}

```

#### Ping/Pong[​](#pingpong "Direct link to Ping/Pong")

SDK每五秒，將自動發送ping到server中， 也可自行發送ping ，另外額外自訂 `state` (state 為optioanal參數) ：

* Python
* Node.js
* C#

```python
stock.ping({
  'state' : '<ANY>'
})

```

```js
stock.ping({state:'<ANY>'});

```

```cs
stock.ping("<ANY>");

```

WebSocket Server 會回應以下訊息 (若 ping 未送 `state` 則不會有該欄位)：

```json
{
  "event": "pong",
  "data": {
    "time": "<TIMESTAMP>",
    "state": "<ANY>"
  }
}

```

#### Channels[​](#channels "Direct link to Channels")

富邦行情 WebSocket API 目前提供以下可訂閱頻道：

* `trades` - [接收訂閱股票最新成交資訊](https://www.fbs.com.tw/TradeAPI/docs/market-data/websocket-api/market-data-channels/trades.txt)
* `books` - [接收訂閱股票最新最佳五檔委買委賣資訊](https://www.fbs.com.tw/TradeAPI/docs/market-data/websocket-api/market-data-channels/books.txt)
* `indices` - [接收訂閱股票最新指數行情資料](https://www.fbs.com.tw/TradeAPI/docs/market-data/websocket-api/market-data-channels/indices.txt)

##### 訂閱頻道[​](#訂閱頻道 "Direct link to 訂閱頻道")

要訂閱一個頻道可用下方範例進行訂閱：

* Python
* Node.js
* C#

```python
stock.subscribe({
  "channel" : "<CHANNEL_NAME>",
  "symbol" : "<SYMBOL_ID>"
  #"intradayOddLot": True  若要訂閱盤中零股，可再額外加入此參數
})

```

```js
stock.subscribe({ 
  channel: '<CHANNEL_NAME>', 
  symbol: '<SYMBOL_ID>',
  //intradayOddLot: true  若要訂閱盤中零股，可再額外加入此參數
});

```

```cs
stock.Subscribe(StockChannel.<CHANNEL_NAME>,"<SYMBOL_ID>");
//stock.Subscribe(StockChannel.Trades,new StockSubscribeParams{Symbol="<SYMBOL_ID>", IntradayOddLot=true});   訂閱盤中零股

```

訂閱成功後，會收到以下事件回應：

```json
{
  "event": "subscribed",
  "data": {
    "id": "<CHANNEL_ID>",
    "channel": "<CHANNEL_NAME>",
    "symbol": "<SYMBOL_ID>"
  }
}

```

支援訂閱同頻道的多檔股票：

* Python
* Node.js
* C#

```python
stock.subscribe({
  "channel" : "<CHANNEL_NAME>",
  "symbols" : ["<SYMBOL_ID>","<SYMBOL_ID>"]
  #"intradayOddLot": True  若要訂閱盤中零股，可再額外加入此參數
})

```

```js
stock.subscribe({ 
  channel: '<CHANNEL_NAME>', 
  symbols: ['<SYMBOL_ID>','<SYMBOL_ID>']
  //intradayOddLot: true  若要訂閱盤中零股，可再額外加入此參數
});

```

```cs
stock.Subscribe(StockChannel.<CHANNEL_NAME>,"<SYMBOL_ID>","<SYMBOL_ID>");
//stock.Subscribe(StockChannel.Trades, new StockSubscribeParams{Symbols = new List<string>{"<SYMBOL_ID>", "<SYMBOL_ID>"}, IntradayOddLot=true});    訂閱多檔盤中零股

```

訂閱成功後，會收到以下事件回應：

```json
{
  "event": "subscribed",
  "data": [
    {
      "id": "<CHANNEL_ID>",
      "channel": "<CHANNEL_NAME>",
      "symbol": "<SYMBOL_ID_1>"
    },
    {
      "id": "<CHANNEL_ID>",
      "channel": "<CHANNEL_NAME>",
      "symbol": "<SYMBOL_ID_2>"
    }
  ]
}

```

##### 取消訂閱[​](#取消訂閱 "Direct link to 取消訂閱")

要取消頻道可用下方範例進行取消：

* Python
* Node.js
* C#

```python
stock.unsubscribe({
    'id':'<CHANNEL_ID>'
})

```

```js
stock.unsubscribe({ 
  id : '<CHANNEL_ID>'
});

```

```cs
stock.Unsubscribe("<CHANNEL_ID>");

```

取消訂閱成功後，會收到以下事件回應：

```json
{
  "event": "unsubscribed",
  "data": {
    "id": "<CHANNEL_ID>",
    "channel" : "<CHANNEL_NAME>",
    "symbol" : "<SYMBOL_ID>"
  }
}

```

支援取消訂閱多個頻道：

* Python
* Node.js
* C#

```python
stock.unsubscribe({
    'ids':['<CHANNEL_ID>','<CHANNEL_ID>']
})

```

```js
stock.unsubscribe({ 
  ids : ['<CHANNEL_ID>','<CHANNEL_ID>']
});

```

```cs
stock.Unsubscribe("<CHANNEL_ID>","<CHANNEL_ID>");

```

取消訂閱成功後，會收到以下事件回應：

```json
{
  "event": "unsubscribed",
  "data": [
    {
      "id": "<CHANNEL_ID_1>",
      "channel" : "<CHANNEL_NAME>",
      "symbol" : "<SYMBOL_ID>"
    },
    {
      "id": "<CHANNEL_ID_2>",
      "channel" : "<CHANNEL_NAME>",
      "symbol" : "<SYMBOL_ID>"
    }
  ]
}

```

#### 錯誤處理[​](#錯誤處理 "Direct link to 錯誤處理")

當您所訂閱或處理的WebSocket Callback有異常時，您可補充處理錯誤訊息如下 ：

* Python
* Node.js
* C#

```python
def handle_connect():
    print('market data connected')


def handle_disconnect(code, message):
    print(f'market data disconnect: {code}, {message}')


def handle_error(error):
    print(f'market data error: {error}')

stock.on("connect", handle_connect)
stock.on("disconnect", handle_disconnect)
stock.on("error", handle_error)

```

```js
stock.on("connect", (message) => {
  const connect_msg = JSON.parse(message);
  console.log(connect_msg);
});

stock.on("disconnect", (message) => {
  console.log(message);
});

stock.on("error", (message) => {
  const err_msg = JSON.parse(message);
  console.log(err_msg);
});


```

```cs
stock.OnError += (errmsg) => Console.WriteLine($"handle error: {errmsg}");
stock.OnConnected += (connmsg) => Console.WriteLine($"Connect: {connmsg}");
stock.OnDisconnected += (disconmsg) => Console.WriteLine($"Disconnect: {disconmsg}");

```

##### 斷線重連[​](#斷線重連 "Direct link to 斷線重連")

以下將用簡單範例，利用上述定義之錯誤處理方法，接收到斷線事件，程式自動進行連線

* Python
* Node.js
* C#

```python
def handle_disconnect(code, message):
    print(f'market data disconnect: {code}, {message}')
    stock.connect()
    print("Reconnected Succuess")
    print("Resubscribe")
    stock.subscribe({    # 重新訂閱您已訂閱過的Channel與Symbol
        'channel': '<CHANNEL_NAME>', 
        'symbol': '<SYMBOL_ID>'
    })

```

```js
stock.on("disconnect", (message) => {
  console.log(message);
  stock.connect()
  console.log("Reconnected Succuess");
  stock.subscribe({ channel: '<CHANNEL_NAME>', symbol: '<SYMBOL_ID>' });  //重新訂閱您已訂閱過的Channel與Symbol
});


```

```cs
stock.OnDisconnected += async (msg) =>
{
    Console.WriteLine($"disconnected at {DateTime.Now}");
    await Task.Delay(10);
    Console.WriteLine("Try Reconnected");
    await stock.Connect();
    Console.WriteLine("Reconnected Success");
    Console.WriteLine("Resubscribe...");
    await stock.Subscribe(StockChannel.<CHANNEL_NAME>, "<SYMBOL_ID>"); //重新訂閱您已訂閱過的Channel與Symbol
    Console.WriteLine("Resubscribe Success");
};

```


---

### Aggregates

接收訂閱股票聚合數據的行情資訊

#### Parameters[​](#parameters "Direct link to Parameters")

| Name             | Type    | Description                                                     |
| ---------------- | ------- | --------------------------------------------------------------- |
| `channel`\*      | string  | 訂閱頻道：`trades`, `candles`, `books`, `aggregates`, `indices` |
| `symbol`\*       | string  | 股票代碼                                                        |
| `intradayOddLot` | boolean | `intradayOddLot` true: 盤中零股, false: 股票, default: false    |

#### Response[​](#response "Direct link to Response")

| Name                  | Type      | Description                  |
| --------------------- | --------- | ---------------------------- |
| `date`\*              | string    | 日期                         |
| `type`\*              | string    | Ticker 類型                  |
| `exchange`\*          | string    | 交易所                       |
| `market`              | string    | 市場別                       |
| `symbol`\*            | string    | 股票代碼                     |
| `name`\*              | string    | 股票簡稱                     |
| `referencePrice`      | number    | 參考價                       |
| `previousClose`       | number    | 前一交易日收盤價             |
| `openPrice`           | number    | 開盤價                       |
| `openTime`            | number    | 開盤價成交時間               |
| `highPrice`           | number    | 最高價                       |
| `highTime`            | number    | 最高價成交時間               |
| `lowPrice`            | number    | 最低價                       |
| `lowTime`             | number    | 最低價成交時間               |
| `closePrice`          | number    | 收盤價（最後成交價）         |
| `closeTime`           | number    | 收盤價（最後成交價）成交時間 |
| `avgPrice`            | number    | 當日成交均價                 |
| `change`              | number    | 最後成交價漲跌               |
| `changePercent`       | number    | 最後成交價漲跌幅             |
| `amplitude`           | number    | 當日振幅                     |
| `lastPrice`           | number    | 最後一筆成交價（含試撮）     |
| `lastSize`            | number    | 最後一筆成交數量（含試撮）   |
| `bids`                | object\[] | 最佳五檔委買                 |
| >> `price`            | number    | 最佳五檔委買價格             |
| >> `size`             | number    | 最佳五檔委買數量             |
| `asks`                | object\[] | 最佳五檔委賣                 |
| >> `price`            | number    | 最佳五檔委賣價格             |
| >> `size`             | number    | 最佳五檔委賣數量             |
| `total`               | object    | 統計資訊                     |
| >> `tradeValue`       | number    | 累計成交金額                 |
| >> `tradeVolume`      | number    | 累計成交量                   |
| >> `tradeVolumeAtBid` | number    | 委買成筆                     |
| >> `tradeVolumeAtAsk` | number    | 委賣成筆                     |
| >> `transaction`      | number    | 累計成交筆數                 |
| >> `time`             | number    | 累計資訊時間                 |
| `lastTrade`           | object    | 最後一筆成交資訊             |
| >> `bid`              | number    | 最後一筆成交買價             |
| >> `ask`              | number    | 最後一筆成交賣價             |
| >> `price`            | number    | 最後一筆成交價格             |
| >> `size`             | number    | 最後一筆成交數量             |
| >> `time`             | number    | 最後一筆成交時間             |
| `lastTrial`           | object    | 最後一筆試撮資訊             |
| >> `bid`              | number    | 最後一筆試撮買價             |
| >> `ask`              | number    | 最後一筆試撮賣價             |
| >> `price`            | number    | 最後一筆試撮價格             |
| >> `size`             | number    | 最後一筆試撮數量             |
| >> `time`             | number    | 最後一筆試撮時間             |
| `isLimitDownPrice`    | boolean   | 最後成交價為跌停價：`true`   |
| `isLimitUpPrice`      | boolean   | 最後成交價為漲停價：`true`   |
| `isLimitDownBid`      | boolean   | 最佳一檔委買跌停價：`true`   |
| `isLimitUpBid`        | boolean   | 最佳一檔委買漲停價：`true`   |
| `isLimitDownAsk`      | boolean   | 最佳一檔委賣跌停價：`true`   |
| `isLimitUpAsk`        | boolean   | 最佳一檔委賣漲停價：`true`   |
| `isLimitDownHalt`     | boolean   | 暫緩撮合且瞬間趨跌：`true`   |
| `isLimitUpHalt`       | boolean   | 暫緩撮合且瞬間趨漲：`true`   |
| `isTrial`             | boolean   | 試撮階段：`true`             |
| `isDelayedOpen`       | boolean   | 延後開盤信號：`true`         |
| `isDelayedClose`      | boolean   | 延後收盤信號：`true`         |
| `isContinuous`        | boolean   | 最後成交為逐筆交易：`true`   |
| `isOpen`              | boolean   | 開盤信號：`true`             |
| `isClose`             | boolean   | 收盤信號：`true`             |
| `lastUpdated`         | number    | 最後更新時間                 |

#### Example[​](#example "Direct link to Example")

##### Subscribe channel[​](#subscribe-channel "Direct link to Subscribe channel")

* Python
* Node.js
* C#

```python
from fubon_neo.sdk import FubonSDK, Order, Mode

def handle_message(message):
    print(f'market data message: {message}')

sdk = FubonSDK()
accounts = sdk.login("Your ID", "Your password", "Your cert path", "Your cert password")  # 需登入後，才能取得行情權限

sdk.init_realtime(Mode.Normal) # 建立行情連線

stock = sdk.marketdata.websocket_client.stock
stock.on('message', handle_message)
stock.connect()
stock.subscribe({ 
    'channel': 'aggregates', 
    'symbol': '2330'
})

```

```js
const { FubonSDK, Mode } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.initRealtime(Mode.Normal); // 建立行情連線

const stock = sdk.marketdata.webSocketClient.stock;
stock.connect().then(() => {
  stock.subscribe({ channel: "aggregates", symbol: "0050" });
});

stock.on("message", (message) => {
  const data = JSON.parse(message);
  console.log(data);
});

```

```cs
using FubonNeo.Sdk;
using FugleMarketData.WebsocketModels;

var sdk = new FubonSDK();

var result = sdk.Login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.InitRealtime(Mode.Normal); // 建立行情連線

var stock = sdk.MarketData.WebSocketClient.Stock;

stock.OnMessage += (msg) => Console.WriteLine($"receive: { msg }");

await stock.Connect();

await stock.Subscribe(StockChannel.Aggregates, "2330");

```

訂閱多檔商品

同時訂閱多檔商品方式，請參考 [訂閱頻道](https://www.fbs.com.tw/TradeAPI/docs/market-data/websocket-api/getting-started.txt#%E8%A8%82%E9%96%B1%E9%A0%BB%E9%81%93)

##### Receive data[​](#receive-data "Direct link to Receive data")

```json
{
  "event": "data",
  "data": {
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
  },
  "id": "<CHANNEL_ID>",
  "channel": "aggregates"
}

```


---

### Books

接收訂閱股票最新最佳五檔委買委賣資訊

#### Parameters[​](#parameters "Direct link to Parameters")

| Name             | Type    | Description                                                     |
| ---------------- | ------- | --------------------------------------------------------------- |
| `channel`\*      | string  | 訂閱頻道：`trades`, `candles`, `books`, `aggregates`, `indices` |
| `symbol`\*       | string  | 股票代碼                                                        |
| `intradayOddLot` | boolean | `intradayOddLot` true: 盤中零股, false: 股票, default: false    |

#### Response[​](#response "Direct link to Response")

| Name         | Type      | Description      |
| ------------ | --------- | ---------------- |
| `symbol`\*   | string    | 股票代碼         |
| `type`\*     | string    | Ticker 類型      |
| `exchange`\* | string    | 交易所           |
| `market`     | string    | 市場別           |
| `time`\*     | number    | 時間             |
| `bids`       | object\[] | 最佳五檔委買     |
| >> `price`   | number    | 最佳五檔委買價格 |
| >> `size`    | number    | 最佳五檔委買數量 |
| `asks`       | object\[] | 最佳五檔委賣     |
| >> `price`   | number    | 最佳五檔委賣價格 |
| >> `size`    | number    | 最佳五檔委賣數量 |

info

'\*' 表示必揭示欄位。

#### Example[​](#example "Direct link to Example")

##### Subscribe channel[​](#subscribe-channel "Direct link to Subscribe channel")

* Python
* Node.js
* C#

```python
from fubon_neo.sdk import FubonSDK, Order

def handle_message(message):
    print(f'market data message: {message}')

sdk = FubonSDK()
accounts = sdk.login("Your ID", "Your password", "Your cert path", "Your cert password")  # 需登入後，才能取得行情權限

sdk.init_realtime() # 建立行情連線

stock = sdk.marketdata.websocket_client.stock
stock.on('message', handle_message)
stock.connect()
stock.subscribe({ 
    'channel': 'books', 
    'symbol': '2330'
})


```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.initRealtime(); // 建立行情連線

const stock = sdk.marketdata.webSocketClient.stock;

stock.connect().then(() => {
  stock.subscribe({ channel: "books", symbol: "2330" });
});

stock.on("message", (message) => {
  const data = JSON.parse(message);
  console.log(data);
});

```

```cs
using FubonNeo.Sdk;
using FugleMarketData.WebsocketModels;

var sdk = new FubonSDK();

var result = sdk.Login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.InitRealtime();  // 建立行情連線

var stock = sdk.MarketData.WebSocketClient.Stock;

stock.OnMessage += (msg) => Console.WriteLine($"receive: { msg }");

await stock.Connect();

await stock.Subscribe(StockChannel.Books, "2330");

```

訂閱多檔商品

同時訂閱多檔商品方式，請參考 [訂閱頻道](https://www.fbs.com.tw/TradeAPI/docs/market-data/websocket-api/getting-started.txt#%E8%A8%82%E9%96%B1%E9%A0%BB%E9%81%93)

##### Receive data[​](#receive-data "Direct link to Receive data")

```json
{
  "event": "data",
  "data": {
    "symbol": "2330",
    "type": "EQUITY",
    "exchange": "TWSE",
    "market": "TSE",
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
    "time": 1685338200000000
  },
  "id": "<CHANNEL_ID>",
  "channel": "books"
}

```


---

### Candles

接收訂閱股票最新分鐘Ｋ

#### Parameters[​](#parameters "Direct link to Parameters")

| Name             | Type    | Description                                                     |
| ---------------- | ------- | --------------------------------------------------------------- |
| `channel`\*      | string  | 訂閱頻道：`trades`, `candles`, `books`, `aggregates`, `indices` |
| `symbol`\*       | string  | 股票代碼                                                        |
| `intradayOddLot` | boolean | `intradayOddLot` true: 盤中零股, false: 股票, default: false    |

#### Response[​](#response "Direct link to Response")

| Name          | Type   | Description                                                                |
| ------------- | ------ | -------------------------------------------------------------------------- |
| `date`\*      | string | 日期                                                                       |
| `type`\*      | string | Ticker 類型                                                                |
| `exchange`\*  | string | 交易所                                                                     |
| `market`      | string | 市場別                                                                     |
| `symbol`\*    | string | 股票代號                                                                   |
| `timeframe`\* | number | Ｋ線週期                                                                   |
| `open`\*      | number | Ｋ線開盤價                                                                 |
| `high`\*      | number | Ｋ線最高價                                                                 |
| `low`\*       | number | Ｋ線最低價                                                                 |
| `close`\*     | number | Ｋ線收盤價                                                                 |
| `volume`\*    | number | Ｋ線成交量（整股：成交張數；興櫃股票及盤中零股：成交股數；指數：成交金額） |
| `average`\*   | number | 成交均價                                                                   |

#### Example[​](#example "Direct link to Example")

##### Subscribe channel[​](#subscribe-channel "Direct link to Subscribe channel")

* Python
* Node.js
* C#

```python
from fubon_neo.sdk import FubonSDK, Order, Mode

def handle_message(message):
    print(f'market data message: {message}')

sdk = FubonSDK()
accounts = sdk.login("Your ID", "Your password", "Your cert path", "Your cert password")  # 需登入後，才能取得行情權限

sdk.init_realtime(Mode.Normal) # 建立行情連線

stock = sdk.marketdata.websocket_client.stock
stock.on('message', handle_message)
stock.connect()
stock.subscribe({ 
    'channel': 'candles', 
    'symbol': '2330'
})


```

```js
const { FubonSDK, Mode } = require('fubon-neo');

const sdk = new FubonSDK(Mode.Normal);

const accounts = sdk.login("Your ID", "Your password", "Your cert path","Your cert password");

sdk.initRealtime(Mode.Normal); // 建立行情連線

const stock = sdk.marketdata.webSocketClient.stock;

stock.connect().then(() => {
  stock.subscribe({ channel: "candles", symbol: "0050" });
});

stock.on("message", (message) => {
  const data = JSON.parse(message);
  console.log(data);
});

```

```cs
using FubonNeo.Sdk;
using FugleMarketData.WebsocketModels;

var sdk = new FubonSDK();

var result = sdk.Login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.InitRealtime();  // 建立行情連線

var stock = sdk.MarketData.WebSocketClient.Stock;

stock.OnMessage += (msg) => Console.WriteLine($"receive: { msg }");

await stock.Connect();

await stock.Subscribe(StockChannel.Candles, "2330");

```

訂閱多檔商品

同時訂閱多檔商品方式，請參考 [訂閱頻道](https://www.fbs.com.tw/TradeAPI/docs/market-data/websocket-api/getting-started.txt#%E8%A8%82%E9%96%B1%E9%A0%BB%E9%81%93)

##### Receive data[​](#receive-data "Direct link to Receive data")

```json
{
  "event": "data",
  "data": {
    "symbol": "2330",
    "type": "EQUITY",
    "exchange": "TWSE",
    "market": "TSE",
    "date": "2023-05-29T13:30:00.000+08:00",
    "open": 568,
    "high": 568,
    "low": 568,
    "close": 568,
    "volume": 4778,
    "average": 568.77
  },
  "id": "<CHANNEL_ID>",
  "channel": "candles"
}

```


---

### Indices

接收訂閱股票最新指數行情資料

#### Example[​](#example "Direct link to Example")

##### Subscribe channel[​](#subscribe-channel "Direct link to Subscribe channel")

* Python
* Node.js
* C#

```python
from fubon_neo.sdk import FubonSDK, Order

def handle_message(message):
    print(f'market data message: {message}')


sdk = FubonSDK()
accounts = sdk.login("Your ID", "Your password", "Your cert path", "Your cert password")  # 需登入後，才能取得行情權限

sdk.init_realtime() # 建立行情連線

stock = sdk.marketdata.websocket_client.stock
stock.on('message', handle_message)
stock.connect()
stock.subscribe({ 
    'channel': 'indices', 
    'symbol': 'IR0001'
})


```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.initRealtime(); // 建立行情連線

const stock = sdk.marketdata.webSocketClient.stock;

stock.connect().then(() => {
  stock.subscribe({ channel: 'indices', symbol: 'IR0001' });
});

stock.on('message', (message) => {
  const data = JSON.parse(message);
  console.log(data);
});

```

```cs
using FubonNeo.Sdk;
using FugleMarketData.WebsocketModels;

var sdk = new FubonSDK();

var result = sdk.Login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.InitRealtime(); // 建立行情連線

var stock = sdk.MarketData.WebSocketClient.Stock;

stock.OnMessage += (msg) => Console.WriteLine($"receive: { msg }");

await stock.Connect();

await stock.Subscribe(StockChannel.Indices, "IR0001");

```

訂閱多檔商品

同時訂閱多檔商品方式，請參考 [訂閱頻道](https://www.fbs.com.tw/TradeAPI/docs/market-data/websocket-api/getting-started.txt#%E8%A8%82%E9%96%B1%E9%A0%BB%E9%81%93)

##### Receive data[​](#receive-data "Direct link to Receive data")

```json
{
    "event": "data",
    "data": {
        "symbol": "IR0001",
        "type": "INDEX",
        "exchange": "TWSE",
        "index": 35276.44,
        "time": 1686114510000000
    },
    "id": "<CHANNEL_ID>",
    "channel": "indices"
}

```


---

### Trades

接收訂閱股票最新成交資訊

#### Parameters[​](#parameters "Direct link to Parameters")

| Name             | Type    | Description                                                     |
| ---------------- | ------- | --------------------------------------------------------------- |
| `channel`\*      | string  | 訂閱頻道：`trades`, `candles`, `books`, `aggregates`, `indices` |
| `symbol`\*       | string  | 股票代碼                                                        |
| `intradayOddLot` | boolean | `intradayOddLot` true: 盤中零股, false: 股票, default: false    |

#### Response[​](#response "Direct link to Response")

| Name               | Type    | Description                |
| ------------------ | ------- | -------------------------- |
| `symbol`\*         | string  | 股票代號                   |
| `type`\*           | string  | Ticker 類型                |
| `exchange`\*       | string  | 交易所                     |
| `market`           | string  | 市場別                     |
| `time`\*           | number  | 時間                       |
| `serial`\*         | number  | 流水號                     |
| `bid`              | number  | 成交買價                   |
| `ask`              | number  | 成交賣價                   |
| `price`            | number  | 成交價格                   |
| `size`             | number  | 成交單量                   |
| `volume`           | number  | 成交總量                   |
| `isLimitDownPrice` | boolean | 最後成交價為跌停價：`true` |
| `isLimitUpPrice`   | boolean | 最後成交價為漲停價：`true` |
| `isLimitDownBid`   | boolean | 最佳一檔委買跌停價：`true` |
| `isLimitUpBid`     | boolean | 最佳一檔委買漲停價：`true` |
| `isLimitDownAsk`   | boolean | 最佳一檔委賣跌停價：`true` |
| `isLimitUpAsk`     | boolean | 最佳一檔委賣漲停價：`true` |
| `isLimitDownHalt`  | boolean | 暫緩撮合且瞬間趨跌：`true` |
| `isLimitUpHalt`    | boolean | 暫緩撮合且瞬間趨漲：`true` |
| `isTrial`          | boolean | 試撮階段：`true`           |
| `isDelayedOpen`    | boolean | 延後開盤信號：`true`       |
| `isDelayedClose`   | boolean | 延後收盤信號：`true`       |
| `isContinuous`     | boolean | 最後成交為逐筆交易：`true` |
| `isOpen`           | boolean | 開盤信號：`true`           |
| `isClose`          | boolean | 收盤信號：`true`           |
| `time`             | number  | 成交時間                   |

info

'\*' 表示必揭示欄位 、 上方Boolean型態皆為發生時才會顯示

#### Example[​](#example "Direct link to Example")

##### Subscribe channel[​](#subscribe-channel "Direct link to Subscribe channel")

* Python
* Node.js
* C#

```python
from fubon_neo.sdk import FubonSDK

def handle_message(message):
    print(message)

sdk = FubonSDK()
accounts = sdk.login("Your ID", "Your password", "Your cert path", "Your cert password")  # 需登入後，才能取得行情權限

sdk.init_realtime()  # 建立行情連線

stock = sdk.marketdata.websocket_client.stock
stock.on('message', handle_message)
stock.connect()
stock.subscribe({
    'channel': 'trades',
    'symbol': '2330'
})


```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.initRealtime(); // 建立行情連線

const stock = sdk.marketdata.webSocketClient.stock;

stock.connect().then(() => {
  stock.subscribe({ channel: "trades", symbol: "0050" });
});

stock.on("message", (message) => {
  const data = JSON.parse(message);
  console.log(data);
});

```

```cs
using FubonNeo.Sdk;
using FugleMarketData.WebsocketModels;

var sdk = new FubonSDK();

var result = sdk.Login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.InitRealtime(); // 建立行情連線

var stock = sdk.MarketData.WebSocketClient.Stock;

stock.OnMessage += (msg) => Console.WriteLine($"receive: { msg }");

await stock.Connect();

await stock.Subscribe(StockChannel.Trades, "2330");

```

訂閱多檔商品

同時訂閱多檔商品方式，請參考 [訂閱頻道](https://www.fbs.com.tw/TradeAPI/docs/market-data/websocket-api/getting-started.txt#%E8%A8%82%E9%96%B1%E9%A0%BB%E9%81%93)

##### Receive data[​](#receive-data "Direct link to Receive data")

```json
{
  "event": "data",
  "data": {
    "symbol": "2330",
    "type": "EQUITY",
    "exchange": "TWSE",
    "market": "TSE",
    "bid": 567,
    "ask": 568,
    "price": 568,
    "size": 4778,
    "volume": 54538,
    "isClose": true,
    "time": 1685338200000000,
    "serial": 6652422
  },
  "id": "<CHANNEL_ID>",
  "channel": "trades"
}

```


---

### 模式切換

**適用版本 v1.0.4 及以上**

#### 行情WebSocket模式切換[​](#行情websocket模式切換 "Direct link to 行情WebSocket模式切換")

新版功能中，提供 Low Latency 行情與多資訊的 Socket 行情切換

##### 引入套件 (僅 Python 、 NodeJs需要， C#不需額外操作)[​](#引入套件-僅-python--nodejs需要-c不需額外操作 "Direct link to 引入套件 (僅 Python 、 NodeJs需要， C#不需額外操作)")

* Python
* Node.js

```py
# 引入Mode 套件模組
from fubon_neo.sdk import FubonSDK, Mode 

```

```js
const { FubonSDK, Mode } = require('fubon-neo');

```

* Python
* Node.js
* C#

```python
sdk.init_realtime(Mode.Speed) # 提供Speed / Normal選擇 (default 為Speed)
#sdk.init_realtime(Mode.Speed) or sdk.init_realtime(Mode.Normal)
stock = sdk.marketdata.websocket_client.stock
stock.on('message', handle_message)
stock.connect()

```

```js
sdk.initRealtime(Mode.Speed); // 提供Speed / Normal選擇 (default 為Speed)
// sdk.initRealtime(Mode.Speed) or sdk.initRealtime(Mode.Normal)
const stock = sdk.marketdata.webSocketClient.stock;

stock.connect()

stock.on("message", (message) => {
  const data = JSON.parse(message);
  console.log(data);
});

```

```cs
using FubonNeo.Sdk;
using FugleMarketData.WebsocketModels; // 新增 "Mode"

sdk.InitRealtime(Mode.Speed); // 提供Speed / Normal選擇 (default 為Speed)
// sdk.initRealtime(Mode.Speed) or sdk.initRealtime(Mode.Normal)

var stock = sdk.MarketData.WebSocketClient.Stock;

stock.OnMessage += (msg) => Console.WriteLine($"receive: {msg}");

await stock.Connect();

```


---

### 開始使用

本頁重點

* 富邦行情期貨 Web API（富邦新一代 API 期貨行情服務）提供期貨與期權日內資料查詢。
* 超過速率限制會回應 `429`。
* 提供 Python / Node.js / C# SDK 範例。
* 下一步可從功能列表選擇對應 API 端點。

| 項目     | 說明                  |
| -------- | --------------------- |
| 介面     | Web API               |
| 市場     | 期貨 / 期權           |
| 資料類型 | 日內                  |
| 速率限制 | 超過回應 `429`        |
| SDK      | Python / Node.js / C# |

富邦行情 Web API 提供開發者友善的 API 服務。您可以查詢期貨與期權日內行情等數據。

#### 速率限制[​](#速率限制 "Direct link to 速率限制")

如果您 API 請求超過了限制，將收到帶有狀態碼 `429` 的回應。

#### 功能列表[​](#功能列表 "Direct link to 功能列表")

富邦行情期貨 Web API 提供的功能如下：

* `/intraday/products` - [期權契約列表（依條件查詢）](https://www.fbs.com.tw/TradeAPI/docs/market-data-future/http-api/intraday/products.txt)
* `/intraday/tickers` - [期權商品列表（依條件查詢）](https://www.fbs.com.tw/TradeAPI/docs/market-data-future/http-api/intraday/tickers.txt)
* `/intraday/quote/{symbol}` - [期貨即時報價（依代碼查詢）](https://www.fbs.com.tw/TradeAPI/docs/market-data-future/http-api/intraday/quote.txt)
* `/intraday/candles/{symbol}` - [期貨價格Ｋ線（依代碼查詢）](https://www.fbs.com.tw/TradeAPI/docs/market-data-future/http-api/intraday/candles.txt)
* `/intraday/trades/{symbol}` - [期貨成交明細（依代碼查詢）](https://www.fbs.com.tw/TradeAPI/docs/market-data-future/http-api/intraday/trades.txt)
* `/intraday/volumes/{symbol}` - [期貨分價量表（依代碼查詢）](https://www.fbs.com.tw/TradeAPI/docs/market-data-future/http-api/intraday/volumes.txt)

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

restfutopt = sdk.marketdata.rest_client.futopt  

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.initRealtime(); // 建立行情連線

const client = sdk.marketdata.restClient

const futopt = client.futopt;

```

```cs

using FubonNeo.Sdk;

var sdk = new FubonSDK();

var result = sdk.Login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.FutureOption;

```


---

### Intraday Candles

期權價格Ｋ線（依商品查詢）

```text
intraday/candles/{symbol}

```

#### Parameters[​](#parameters "Direct link to Parameters")

| Name        | Type   | Description                                                                             |
| ----------- | ------ | --------------------------------------------------------------------------------------- |
| `symbol`\*  | string | 期權代碼                                                                                |
| `session`   | string | session 類型，可選 `afterhours` 夜盤                                                    |
| `timeframe` | string | Ｋ線週期，可選 `1` 1分Ｋ；`5` 5分Ｋ；`10` 10分Ｋ；`15` 15分Ｋ；`30` 30分Ｋ；`60` 60分Ｋ |

#### Response[​](#response "Direct link to Response")

| Name          | Type      | Description |
| ------------- | --------- | ----------- |
| `date`\*      | string    | 日期        |
| `type`\*      | string    | Ticker 類型 |
| `exchange`\*  | string    | 交易所      |
| `market`      | string    | 市場別      |
| `symbol`\*    | string    | 期權代碼    |
| `timeframe`\* | string    | Ｋ線週期    |
| `data`\*      | object\[] | Ｋ線資料    |
| >> `time`     | number    | Ｋ線時間    |
| >> `open`\*   | number    | Ｋ線開盤價  |
| >> `high`\*   | number    | Ｋ線最高價  |
| >> `low`\*    | number    | Ｋ線最低價  |
| >> `close`\*  | number    | Ｋ線收盤價  |
| >> `volume`   | number    | Ｋ線成交量  |
| >> `average`  | number    | 成交均價    |

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
restfutopt = sdk.marketdata.rest_client.futopt
# restfutopt.intraday.candles(symbol='TXFA4')  # 2.2.3 及以前版本

## 2.2.4 及以後版本 (使用 Exception 進行例外處理)
from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

try:  
  restfutopt.intraday.candles(symbol='TXFA4')
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
client.futopt.intraday.candles({ symbol: 'TXFA4' })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;
using FugleMarketData.QueryModels.FuOpt;
using FugleMarketData.QueryModels.FuOpt.Intraday;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.FutureOption;

var candles = await rest.Intraday.Candles("TXFA4");
// var candles = await rest.Intraday.Candles("TXFA4", new(){TimeFrame=FugleMarketData.QueryModels.FuOpt.Intraday.CandlesTimeFrame.FifteenMin});  // 15分鐘K

var candle_cont = candles.Content.ReadAsStringAsync().Result;

Console.WriteLine(candle_cont);

```

Response Body：

```json
{
  "date": "2023-12-15",
  "type": "FUTURE",
  "exchange": "TAIFEX",
  "symbol": "TXFA4",
  "timeframe": "1",
  "data": [
    {
      "date": "2023-12-15T08:45:00.000+08:00",
      "open": 17744,
      "high": 17756,
      "low": 17744,
      "close": 17756,
      "volume": 98,
      "average": 17749.97
    },
    {
      "date": "2023-12-15T08:46:00.000+08:00",
      "open": 17755,
      "high": 17756,
      "low": 17742,
      "close": 17747,
      "volume": 48,
      "average": 17749.1
    },
    {
      "date": "2023-12-15T08:47:00.000+08:00",
      "open": 17746,
      "high": 17746,
      "low": 17731,
      "close": 17731,
      "volume": 26,
      "average": 17747.8
    },
    ...
  ]
}

```


---

### Products List

期權契約清單（依條件查詢）

```text
intraday/products

```

#### Parameters[​](#parameters "Direct link to Parameters")

| Name           | Type   | Description                                                                           |
| -------------- | ------ | ------------------------------------------------------------------------------------- |
| `type`\*       | string | 類型，可選 `FUTURE` 期貨 ； `OPTION` 選擇權                                           |
| `exchange`     | string | 交易所，可選 `TAIFEX` 臺灣期貨交易所                                                  |
| `session`      | string | 交易時段，可選 `REGULAR` 一般交易 或 `AFTERHOURS` 盤後交易                            |
| `contractType` | string | 契約類別，可選 `I` 指數類；`R` 利率類；`B` 債券類；`C` 商品類；`S` 股票類；`E` 匯率類 |
| `status`       | string | 契約狀態，可選 `N` 正常；`P` 暫停交易；`U` 即將上市                                   |

#### Response[​](#response "Direct link to Response")

| Name                  | Type      | Description                                  |
| --------------------- | --------- | -------------------------------------------- |
| `type`\*              | string    | 期權類型                                     |
| `exchange`            | string    | 交易所                                       |
| `session`             | string    | 交易時段                                     |
| `contractType`        | string    | 契約類別                                     |
| `status`              | string    | 契約狀態                                     |
| `data`                | object\[] | 契約列表                                     |
| >> `symbol`           | string    | 契約代號                                     |
| >> `type`             | string    | 期權類型                                     |
| >> `name`             | string    | 契約名稱                                     |
| >> `underlyingSymbol` | string    | 股票現貨代號                                 |
| >> `contractType`     | string    | 契約類別                                     |
| >> `contractSize`     | string    | 契約乘數                                     |
| >> `statusCode`       | string    | 狀態碼                                       |
| >> `tradingCurrency`  | string    | 交易幣別                                     |
| >> `quoteAcceptable`  | boolean   | 是否可報價                                   |
| >> `startDate`        | string    | 上市日期                                     |
| >> `canBlockTrade`    | boolean   | 是否可鉅額交易                               |
| >> `expiryType`       | string    | 到期別，`S` 標準；`W` 週                     |
| >> `underlyingType`   | string    | 股票現貨類別，`E` ETF；`S` 個股              |
| >> `marketCloseGroup` | string    | 商品收盤時間群組                             |
| >> `endSession`       | string    | 交易時段，`0` 一般交易時段；`1` 盤後交易時段 |

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

restfutopt = sdk.marketdata.rest_client.futopt 
# restfutopt.intraday.products(type='FUTURE', exchange='TAIFEX',session='REGULAR', contractType='E')  # 2.2.3 及以前版本

## 2.2.4 及以後版本 (使用 Exception 進行例外處理)
from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

try:  
  restfutopt.intraday.products(type='FUTURE', exchange='TAIFEX',session='REGULAR', contractType='E')
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
client.futopt.intraday.products({ type: 'FUTURE', exchange:'TAIFEX', session:'REGULAR', contractType:'E'})
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;
using FugleMarketData.QueryModels.FuOpt;
using FugleMarketData.QueryModels.FuOpt.Intraday;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.FutureOption;

var contracts = await rest.Intraday.Products(FutOptType.Future, new(){
        Exchange=FutOptExchangeType.TaiFex,
        Session=SessionType.AfterHours,
        ContractType=ContractType.E
        });
var contracts_cont = contracts.Content.ReadAsStringAsync().Result;

Console.WriteLine(contracts_cont);

```

Response Body：

```json
{
    "type": "FUTURE",
    "exchange": "TAIFEX",
    "session": "AFTERHOURS",
    "contractType": "E",
    "data": [
      {
        "symbol": "RHF",
        "type": "FUTURE_AH",
        "canBlockTrade": true,
        "contractSize": 100000,
        "contractType": "E",
        "endSession": "0",
        "expiryType": "S",
        "marketCloseGroup": 10,
        "name": "美元兌人民幣期貨",
        "quoteAcceptable": true,
        "startDate": "",
        "statusCode": "N",
        "tradingCurrency": "CNY",
        "underlyingSymbol": "",
        "underlyingType": ""
      }, 
      {
        "symbol": "RTF",
        "type": "FUTURE_AH",
        "canBlockTrade": true,
        "contractSize": 20000,
        "contractType": "E",
        "endSession": "0",
        "expiryType": "S",
        "marketCloseGroup": 10,
        "name": "小型美元兌人民幣期貨",
        "quoteAcceptable": true,
        "startDate": "",
        "statusCode": "N",
        "tradingCurrency": "CNY",
        "underlyingSymbol": "",
        "underlyingType": ""
      },
      {
        "symbol": "XAF",
        "type": "FUTURE_AH",
        "canBlockTrade": true,
        "contractSize": 25000,
        "contractType": "E",
        "endSession": "0",
        "expiryType": "S",
        "marketCloseGroup": 10,
        "name": "澳幣兌美元期貨",
        "quoteAcceptable": true,
        "startDate": "",
        "statusCode": "N",
        "tradingCurrency": "USD",
        "underlyingSymbol": "",
        "underlyingType": ""
      }
    ]
}

```


---

### Intraday Quote

期權即時報價（依商品代碼查詢）

```text
intraday/quote/{symbol}

```

#### Parameters[​](#parameters "Direct link to Parameters")

| Name       | Type   | Description                          |
| ---------- | ------ | ------------------------------------ |
| `symbol`\* | string | 期權代碼                             |
| `session`  | string | 交易時段，可選 `afterhours` 盤後交易 |

#### Response[​](#response "Direct link to Response")

| Name               | Type   | Description                  |
| ------------------ | ------ | ---------------------------- |
| `date`\*           | string | 日期                         |
| `type`\*           | string | 期權類型                     |
| `exchange`         | string | 交易所                       |
| `symbol`           | string | 商品代號                     |
| `name`             | string | 商品名稱                     |
| `previousClose`    | number | 昨日收盤價                   |
| `openPrice`        | number | 開盤價                       |
| `openTime`         | number | 開盤價成交時間               |
| `highPrice`        | number | 最高價                       |
| `highTime`         | number | 最高價成交時間               |
| `lowPrice`         | number | 最低價                       |
| `lowTime`          | number | 最低價成交時間               |
| `closePrice`       | number | 收盤價（最後成交價）         |
| `closeTime`        | number | 收盤價（最後成交價）成交時間 |
| `avgPrice`         | number | 當日成交均價                 |
| `change`           | number | 最後成交價漲跌               |
| `changePercnet`    | number | 最後成交價漲跌幅             |
| `amplitude`        | number | 當日振幅                     |
| `lastPrice`        | number | 最後一筆成交價（含試撮）     |
| `lastSize`         | number | 最後一筆成交數量（含試撮）   |
| `total`            | object | 統計時間                     |
| >> `tradeVolume`   | number | 累計成交量                   |
| >> `totalBidMatch` | number | 委買成筆                     |
| >> `totalAskMatch` | number | 委賣成筆                     |
| `lastTrade`        | object | 最後一筆成交資訊             |
| >> `bid`           | number | 最後一筆成交買價             |
| >> `ask`           | number | 最後一筆成交賣價             |
| >> `price`         | number | 最後一筆成交價格             |
| >> `size`          | number | 最後一筆成交數量             |
| >> `time`          | number | 最後一筆成交時間             |
| >> `serial`        | number | 最後一筆成交流水號           |
| `serial`           | number | 流水號                       |
| `lastUpdated`      | number | 最後異動時間                 |

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

restfut = sdk.marketdata.rest_client.futopt  
# restfut.intraday.quote(symbol='TXFA4')  # 2.2.3 及以前版本

## 2.2.4 及以後版本 (使用 Exception 進行例外處理)
from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

try:  
  restfut.intraday.quote(symbol='TXFA4')
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
client.futopt.intraday.quote({ symbol: 'TXFA4' })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;
using FugleMarketData.QueryModels.FuOpt;
using FugleMarketData.QueryModels.FuOpt.Intraday;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.FutureOption;

var quote = await rest.Intraday.Quote("TXFA4");
// var quote = await rest.Intraday.Quote("TXFA4", new(){Session=FugleMarketData.QueryModels.FuOpt.TradeSession.AfterHours});  // 盤後交易

var quote_cont = quote.Content.ReadAsStringAsync().Result;

Console.WriteLine(quote_cont);

```

Response Body：

```json
{
    "date": "2023-12-12",
    "type": "FUTURE",
    "exchange": "TAIFEX",
    "symbol": "TXFA4",
    "name": "臺股期貨014",
    "previousClose": 17416,
    "openPrice": 17514,
    "openTime": 1702341900070000,
    "highPrice": 17540,
    "highTime": 1702342491330000,
    "lowPrice": 17427,
    "lowTime": 1702355400574000,
    "closePrice": 17460,
    "closeTime": 1702359886936000,
    "avgPrice": 17478.89,
    "change": 44,
    "changePercent": 0.25,
    "amplitude": 0.65,
    "lastPrice": 17460,
    "lastSize": 1,
    "total": {
        "tradeVolume": 1626,
        "totalBidMatch": 0,
        "totalAskMatch": 0
    },
    "lastTrade": {
        "bid": 17459,
        "ask": 17460,
        "price": 17460,
        "size": 1,
        "time": 1702359886936000,
        "serial": "00165753"
    },
    "serial": 165753,
    "lastUpdated": 1702359886936000
}

```


---

### Intraday Ticker

期權基本資料（依條件查詢）

```text
intraday/ticker/

```

#### Parameters[​](#parameters "Direct link to Parameters")

| Name       | Type   | Description                                                |
| ---------- | ------ | ---------------------------------------------------------- |
| `symbol`\* | string | 商品代碼                                                   |
| `session`  | string | 交易時段，可選 `REGULAR` 一般交易 或 `AFTERHOURS` 盤後交易 |

#### Response[​](#response "Direct link to Response")

| Name             | Type   | Description |
| ---------------- | ------ | ----------- |
| `date`\*         | string | 日期        |
| `type`\*         | string | 期權類型    |
| `exchange`\*     | string | 交易所      |
| `symbol`\*       | string | 商品代號    |
| `name`\*         | string | 商品名稱    |
| `referencePrice` | string | 參考價      |
| `settlementDate` | string | 最後結算日  |
| >> `startDate`   | string | 上市日期    |
| >> `endDate`     | string | 下市日期    |

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

restfut = sdk.marketdata.rest_client.futopt  
# restfut.intraday.ticker(symbol='TXFI4')  # 2.2.3 及以前版本

## 2.2.4 及以後版本 (使用 Exception 進行例外處理)
from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

try:  
  restfut.intraday.ticker(symbol='TXFI4')
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
client.futopt.intraday.ticker({ symbol: 'TXFI4'})
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;
using FugleMarketData.QueryModels.FuOpt;
using FugleMarketData.QueryModels.FuOpt.Intraday;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.FutureOption;

var contracts = await rest.Intraday.Ticker("TXFI4", new(){
        Session=TradeSession.AfterHours
        });
var contracts_cont = contracts.Content.ReadAsStringAsync().Result;

Console.WriteLine(contracts_cont);

```

Response Body：

```json
{
    "date": "2024-09-18",
    "type": "FUTURE",
    "exchange": "TAIFEX",
    "symbol": "TXFI4",
    "name": "臺股期貨094",
    "referencePrice": 21703,
    "settlementDate": "2024-09-18",
    "startDate": "2023-09-21",
    "endDate": "2024-09-18"
}
    

```


---

### Intraday Tickers

期權商品列表（依條件查詢）

```text
intraday/tickers/

```

#### Parameters[​](#parameters "Direct link to Parameters")

| Name           | Type   | Description                                                                           |
| -------------- | ------ | ------------------------------------------------------------------------------------- |
| `type`\*       | string | 類型，可選 `FUTURE` 期貨 ； `OPTION` 選擇權                                           |
| `exchange`     | string | 交易所，可選 `TAIFEX` 臺灣期貨交易所                                                  |
| `session`      | string | 交易時段，可選 `REGULAR` 一般交易 或 `AFTERHOURS` 盤後交易                            |
| `product`      | string | 期權商品代碼                                                                          |
| `contractType` | string | 契約類別，可選 `I` 指數類；`R` 利率類；`B` 債券類；`C` 商品類；`S` 股票類；`E` 匯率類 |

#### Response[​](#response "Direct link to Response")

| Name                  | Type      | Description          |
| --------------------- | --------- | -------------------- |
| `date`\*              | string    | 日期                 |
| `type`\*              | string    | 期權類型             |
| `exchange`            | string    | 交易所               |
| `session`             | string    | 交易時段             |
| `contractType`        | string    | 契約類別             |
| `data`                | object\[] | 契約列表             |
| >> `type`             | string    | 期權類型             |
| >> `symbol`           | string    | 商品代號             |
| >> `name`             | string    | 商品名稱             |
| >> `referencePrice`   | string    | 參考價               |
| >> `contractType`     | string    | 契約類別             |
| >> `startDate`        | string    | 上市日期             |
| >> `endDate`          | string    | 下市日期             |
| >> `flowGroup`        | string    | 流程群組             |
| >> `settlementDate`   | string    | 最後結算日           |
| >> `isDynamicBanding` | boolean   | 是否適用動態價格穩定 |

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

restfut = sdk.marketdata.rest_client.futopt  
# restfut.intraday.tickers(type='FUTURE', exchange='TAIFEX',session='REGULAR', contractType='E')  # 2.2.3 及以前版本

## 2.2.4 及以後版本 (使用 Exception 進行例外處理)
from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

try:  
  restfut.intraday.tickers(type='FUTURE', exchange='TAIFEX',session='REGULAR', contractType='E')
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
client.futopt.intraday.tickers({ type: 'FUTURE', exchange:'TAIFEX', session:'REGULAR', contractType:'E'})
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;
using FugleMarketData.QueryModels.FuOpt;
using FugleMarketData.QueryModels.FuOpt.Intraday;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.FutureOption;

var contracts = await rest.Intraday.Tickers(FutOptType.Future, new(){
        Exchange=FutOptExchangeType.TaiFex,
        Session=SessionType.AfterHours,
        ContractType=ContractType.E
        });
var contracts_cont = contracts.Content.ReadAsStringAsync().Result;

Console.WriteLine(contracts_cont);

```

Response Body：

```json
{
    "type": "FUTURE",
    "exchange": "TAIFEX",
    "session": "REGULAR",
    "contractType": "E",
    "data": [
    {
        "symbol": "RHFA4",
        "type": "FUTURE",
        "contractType": "E",
        "endDate": "2024-01-17",
        "flowGroup": 5,
        "isDynamicBanding": true,
        "name": "美元兌人民幣期貨014",
        "referencePrice": 7.1387,
        "settlementDate": "2024-01-17",
        "startDate": "2023-11-16"
    }, 
    {
        "symbol": "RHFC4",
        "type": "FUTURE",
        "contractType": "E",
        "endDate": "2024-03-20",
        "flowGroup": 5,
        "isDynamicBanding": true,
        "name": "美元兌人民幣期貨034",
        "referencePrice": 7.108,
        "settlementDate": "2024-03-20",
        "startDate": "2023-01-31"
    }, 
    {
        "symbol": "RHFF4",
        "type": "FUTURE",
        "contractType": "E",
        "endDate": "2024-06-19",
        "flowGroup": 5,
        "isDynamicBanding": true,
        "name": "美元兌人民幣期貨064",
        "referencePrice": 7.0619,
        "settlementDate": "2024-06-19",
        "startDate": "2023-04-20"
    }, 
    {
        "symbol": "RHFI4",
        "type": "FUTURE",
        "contractType": "E",
        "endDate": "2024-09-19",
        "flowGroup": 5,
        "isDynamicBanding": true,
        "name": "美元兌人民幣期貨094",
        "referencePrice": 7.0189,
        "settlementDate": "2024-09-19",
        "startDate": "2023-07-20"
    }, 
    {
        "symbol": "RHFL3",
        "type": "FUTURE",
        "contractType": "E",
        "endDate": "2023-12-20",
        "flowGroup": 5,
        "isDynamicBanding": true,
        "name": "美元兌人民幣期貨123",
        "referencePrice": 7.1531,
        "settlementDate": "2023-12-20",
        "startDate": "2022-10-20"
    }, 
    ......
    ]
}

```


---

### Intraday Trades

期權成交明細（依代碼查詢）

```text
intraday/trades/{symbol}

```

#### Parameters[​](#parameters "Direct link to Parameters")

| Name       | Type   | Description                      |
| ---------- | ------ | -------------------------------- |
| `symbol`\* | string | 期權代碼                         |
| `session`  | string | 交易時段，可選 `afterhours` 夜盤 |
| `offset`   | number | 偏移量                           |
| `limit`    | number | 限制量                           |

#### Response[​](#response "Direct link to Response")

| Name         | Type      | Description |
| ------------ | --------- | ----------- |
| `date`\*     | string    | 日期        |
| `type`\*     | string    | 期權類型    |
| `exchange`\* | string    | 交易所      |
| `market`     | string    | 市場別      |
| `symbol`\*   | string    | 商品代號    |
| `data`\*     | object\[] | 成交明細    |
| >> `price`   | number    | 成交價格    |
| >> `size`    | number    | 成交單量    |
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

restfutopt = sdk.marketdata.rest_client.futopt 
# restfutopt.intraday.trades(symbol='TXFA4')  # 2.2.3 及以前版本

## 2.2.4 及以後版本 (使用 Exception 進行例外處理)
from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

try:  
  restfutopt.intraday.trades(symbol='TXFA4')
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
client.futopt.intraday.trades({ symbol: 'TXFA4' })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.FutureOption;

var trade = await rest.Intraday.Trades("TXFA4");
// var trade = await rest.Intraday.Trades("TXFA4", new(){Session=FugleMarketData.QueryModels.FuOpt.TradeSession.AfterHours});  // 盤後交易

var trade_cont = trade.Content.ReadAsStringAsync().Result;

Console.WriteLine(trade_cont);

```

Response Body：

```json
{
    "date": "2023-12-20",
    "type": "FUTURE",
    "exchange": "TAIFEX",
    "symbol": "TXFA4",
    "data": [
        {
            "price": 17660,
            "size": 3,
            "time": 1703051099834000,
            "serial": 218307
        },
        {
            "price": 17661,
            "size": 2,
            "time": 1703051099779000,
            "serial": 218304
        },
        {
            "price": 17661,
            "size": 1,
            "time": 1703051099778000,
            "serial": 218303
        },
        {
            "price": 17661,
            "size": 1,
            "time": 1703051099778000,
            "serial": 218301
        },
        ....
    ]
}

```


---

### Intraday Volumes

期權分價量表（依代碼查詢）

```text
intraday/volumes/{symbol}

```

#### Parameters[​](#parameters "Direct link to Parameters")

| Name       | Type   | Description                      |
| ---------- | ------ | -------------------------------- |
| `symbol`\* | string | 期權代碼                         |
| `session`  | string | 交易時段，可選 `afterhours` 夜盤 |

#### Response[​](#response "Direct link to Response")

| Name         | Type      | Description          |
| ------------ | --------- | -------------------- |
| `date`\*     | string    | 日期                 |
| `type`\*     | string    | 期權類型             |
| `exchange`\* | string    | 交易所               |
| `market`     | string    | 市場別               |
| `symbol`\*   | string    | 商品代號             |
| `data`       | object\[] | 分價量表             |
| >> `price`   | number    | 成交價               |
| >> `volume`  | number    | 該成交價之累計成交量 |

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

restfutopt = sdk.marketdata.rest_client.futopt  
# restfutopt.intraday.volumes(symbol='TXFA4')  # 2.2.3 及以前版本

## 2.2.4 及以後版本 (使用 Exception 進行例外處理)
from fubon_neo.fugle_marketdata.rest.base_rest import FugleAPIError

try:  
  restfutopt.intraday.volumes(symbol='TXFA4')
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
client.futopt.intraday.volumes({ symbol: 'TXFA4' })
  .then(data => console.log(data));

```

```cs

using FubonNeo.Sdk;


var sdk = new FubonSDK();
var result = sdk.Login("Your ID", "Your Password", "Your Cert Path", "Your Cert Password");

sdk.InitRealtime(); // 建立行情連線

var rest = sdk.MarketData.RestClient.FutureOption;

var volume = await rest.Intraday.Volumes("TXFA4");
// var volume = await rest.Intraday.Volumes("TXFA4", new(){Session=FugleMarketData.QueryModels.FuOpt.TradeSession.AfterHours});  // 盤後交易
var volume_cont = volume.Content.ReadAsStringAsync().Result;

Console.WriteLine(volume_cont);

```

Response Body：

```json
{
    "date": "2023-12-20",
    "type": "FUTURE",
    "exchange": "TAIFEX",
    "symbol": "TXFA4",
    "data": [
        {
            "price": 17676,
            "volume": 68
        },
        {
            "price": 17675,
            "volume": 392
        },
        {
            "price": 17674,
            "volume": 265
        },
        {
            "price": 17673,
            "volume": 396
        },
        {
            "price": 17672,
            "volume": 430
        },
        {
            "price": 17671,
            "volume": 518
        },
        {
            "price": 17670,
            "volume": 681
        },
        {
            "price": 17669,
            "volume": 338
        },
        {
            "price": 17668,
            "volume": 395
        },
        {
            "price": 17667,
            "volume": 568
        },
        {
            "price": 17666,
            "volume": 670
        },
        {
            "price": 17665,
            "volume": 641
        },
        {
            "price": 17664,
            "volume": 542
        },
        {
            "price": 17663,
            "volume": 898
        },
        {
            "price": 17662,
            "volume": 870
        },
        {
            "price": 17661,
            "volume": 656
        },
        {
            "price": 17660,
            "volume": 876
        },
        {
            "price": 17659,
            "volume": 402
        },
        {
            "price": 17658,
            "volume": 352
        },
        {
            "price": 17657,
            "volume": 190
        },
        {
            "price": 17656,
            "volume": 470
        },
        ....
    ]
}

```


---

