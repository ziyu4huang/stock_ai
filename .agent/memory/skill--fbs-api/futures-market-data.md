### 富邦新一代 API 期貨行情服務

本頁重點

* 富邦新一代 API（Fubon Neo API）期貨行情服務提供期貨即時與日內資料。
* 介面包含 Web API 與 WebSocket API。
* 資料來源為交易所與資訊供應商，使用前請詳閱規範與聲明。
* 下一步可從 [Web API](https://www.fbs.com.tw/TradeAPI/docs/market-data-future/http-api/getting-started.txt) 或 [WebSocket API](https://www.fbs.com.tw/TradeAPI/docs/market-data-future/websocket-api/getting-started.txt) 開始。

| 項目     | 說明                                             |
| -------- | ------------------------------------------------ |
| 服務     | 新一代 API 期貨行情服務                          |
| 市場     | 台灣期貨市場                                     |
| 介面     | Web API / WebSocket API                          |
| 資料來源 | 臺灣證券交易所、證券櫃檯買賣中心、臺灣期貨交易所 |
| 規範     | 使用規範與聲明                                   |

即時行情資料來源為臺灣證券交易所、證券櫃檯買賣中心、臺灣期貨交易所。請您詳閱相關使用[規範與聲明](#statement)。

#### Web API[​](#web-api "Direct link to Web API")

富邦行情 [Web API](https://www.fbs.com.tw/TradeAPI/docs/market-data-future/http-api/getting-started.txt) 提供開發者友善的 Web API 服務。您可以查詢期貨的日內行情數據。

#### WebSocket API[​](#websocket-api "Direct link to WebSocket API")

富邦行情 [WebSocket API](https://www.fbs.com.tw/TradeAPI/docs/market-data-future/websocket-api/getting-started.txt) 提供期貨即時行情服務。透過 WebSocket API 可以滿足您想要接收即時行情的需求。

#### 使用規範與聲明[​](#statement "Direct link to 使用規範與聲明")

* 透過本服務取得之行情資料僅供參考，成交值及成交量不含零股及鉅額交易，使用者依本資料交易發生交易損失需自行負責。
* 時報資訊與群馥科技對資料內容錯誤、更新延誤或傳輸中斷不負任何責任。您應對您所為之任何金融或投資決策自行負責。
* 使用者應遵守[臺灣證券交易所股份有限公司交易資訊使用管理辦法](https://www.selaw.com.tw/Chinese/RegulatoryInformationResult/Article?isIntegratedSearch=True\&sysNumber=LW10809760\&searchString1=%E8%87%BA%E7%81%A3%E8%AD%89%E5%88%B8%E4%BA%A4%E6%98%93%E6%89%80%E8%82%A1%E4%BB%BD%E6%9C%89%E9%99%90%E5%85%AC%E5%8F%B8%E4%BA%A4%E6%98%93%E8%B3%87%E8%A8%8A%E4%BD%BF%E7%94%A8%E7%AE%A1%E7%90%86%E8%BE%A6%E6%B3%95)、[臺灣期貨交易所股份有限公司交易資訊使用管理辦法](https://www.selaw.com.tw/Chinese/RegulatoryInformationResult/Article?isIntegratedSearch=True\&sysNumber=LW10815483\&searchString1=%E8%87%BA%E7%81%A3%E6%9C%9F%E8%B2%A8%E4%BA%A4%E6%98%93%E6%89%80%E8%82%A1%E4%BB%BD%E6%9C%89%E9%99%90%E5%85%AC%E5%8F%B8%E4%BA%A4%E6%98%93%E8%B3%87%E8%A8%8A%E4%BD%BF%E7%94%A8%E7%AE%A1%E7%90%86%E8%BE%A6%E6%B3%95)、[財團法人中華民國證券櫃檯買賣中心有價證券交易資訊使用管理辦法](https://www.selaw.com.tw/Chinese/RegulatoryInformationResult/Article?isIntegratedSearch=True\&sysNumber=LW10813063\&searchString1=%E8%B2%A1%E5%9C%98%E6%B3%95%E4%BA%BA%E4%B8%AD%E8%8F%AF%E6%B0%91%E5%9C%8B%E8%AD%89%E5%88%B8%E6%AB%83%E6%AA%AF%E8%B2%B7%E8%B3%A3%E4%B8%AD%E5%BF%83%E6%9C%89%E5%83%B9%E8%AD%89%E5%88%B8%E4%BA%A4%E6%98%93%E8%B3%87%E8%A8%8A%E4%BD%BF%E7%94%A8%E7%AE%A1%E7%90%86%E8%BE%A6%E6%B3%95)、各資訊來源提供者所定之資訊使用相關規範及智慧財產權相關法令，如有盜接、轉接交易資訊，或以其他方式出售、出租、轉讓、再授權交易資訊，或將交易資訊另行取樣並編製指數、其他衍生性商品或將之傳送予第三人，應負違約及侵權之相關民、刑事責任。
* 使用者須遵守臺灣證券交易所「[交易資訊使用管理辦法](https://www.twse.com.tw/downloads/zh/products/regulation_use.pdf)」等交易資訊管理相關規定，所有資訊以臺灣證券交易所公告資料為準。
* 富邦新一代 API 期貨行情服務由 [Fugle](https://www.fugle.tw/) 技術團隊開發提供。


---

### 建立連線

##### 建立行情連線[​](#建立行情連線 "Direct link to 建立行情連線")

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
  #ws = sdk.marketdata.websocket_client.stock  # WebSocket 介面 (證券)
  ws = sdk.marketdata.websocket_client.futopt  # WebSocket 介面 (期貨)

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
  //const ws = sdk.marketdata.webSocketClient.stock;  // (證券)
  const ws = sdk.marketdata.webSocketClient.futopt;  // (期貨)

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
//var websockets = new List<FugleMarketData.WebsocketClient.FugleWebsocketStockClient>();  // (證券)
var websockets = new List<FugleMarketData.WebsocketClient.FugleWebsocketFutOptClient>();  // (期貨)

// 建立 5 條行情連線
for (int i = 0; i < 5; i++)
{
  sdk.InitRealtime(); // 建立行情連線

  // 取得 WebSocket 介面 
  //var ws = sdk.MarketData.WebSocketClient.Stock;  // (證券)
  var ws = sdk.MarketData.WebSocketClient.FutureOption;  // (期貨)

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

* 本頁整理富邦新一代 API 期貨行情服務的 Web API / WebSocket 速率限制。
* 超過限制將回應 `429` 或訂閱錯誤訊息。
* 若短時間內大量建立連線可能被判定為異常。

| 項目      | 上限                 |
| --------- | -------------------- |
| 日內行情  | 300 / min            |
| 行情快照  | 300 / min            |
| 歷史行情  | 60 / min             |
| WebSocket | 200 訂閱數；5 連線數 |

富邦新一代 API 期貨行情服務提供 Web API 與 WebSocket，並設定了以下存取的限制

* 日內行情 : 300 / min
* WebSocket : 200 訂閱數；5連線數
* 行情快照 : 300 / min
* 歷史行情 : 60 / min

如果您 API 請求超過了限制，將收到帶有狀態碼 `429` 的回應。需再等候1分鐘。

```json
{"statusCode":429,"message":"Rate limit exceeded"}  

```

或是您的WebSocket 訂閱數超過上限，將收到以下訊息:

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

若短時間內，Socket持續大量建立連線，系統會判斷您為惡意攻擊，將會阻擋您的連線請求（404連線錯誤）， 若遇到此種狀況，請聯絡 <pm.trading.sec@fubon.com>


---

### 開始使用

本頁重點

* 富邦行情期貨 WebSocket API（富邦新一代 API 期貨行情服務）提供期貨即時行情訂閱。
* 支援 `Speed` / `Normal` 兩種行情模式。
* 需登入後建立行情連線並訂閱頻道。
* 下一步可前往各頻道說明頁。

| 項目     | 說明                  |
| -------- | --------------------- |
| 介面     | WebSocket API         |
| 市場     | 期貨 / 期權           |
| 模式     | Speed / Normal        |
| SDK      | Python / Node.js / C# |
| 訂閱方式 | 連線後訂閱頻道        |

富邦行情 WebSocket API 提供期貨即時行情服務。透過 WebSocket API 可以滿足您想要接收即時行情的需求。

#### 使用 SDK[​](#使用-sdk "Direct link to 使用 SDK")

富邦行情 WebSocket API 提供 Python 、 Node.js 與 C# SDK。您可以透過以下方式存取 WebSocket API、並透過行情Mode模式切換要訂閱的行情：

訂閱WebSocket Callback 方法獲得下方Callback訊息。

info

行情Mode提供Low Latency `Speed` Mode 以及完整資訊的 `Normal` Mode

* Python
* Node.js
* C#

```python
from fubon_neo.sdk import FubonSDK, Mode

def handle_message(message):
    print(message)

sdk = FubonSDK()
accounts = sdk.login("Your ID", "Your password", "Your cert path", "Your cert password")  # 需登入後，才能取得行情權限

sdk.init_realtime() # 建立行情連線
# 可指定行情連線的Mode ， Default 為Speed 
# sdk.init_realtime(Mode.Speed) or sdk.init_realtime(Mode.Normal)

futopt = sdk.marketdata.websocket_client.futopt
futopt.on('message', handle_message)
futopt.connect()

```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.initRealtime(); // 建立行情連線
// 可指定行情連線的Mode ， Default 為Speed 
// sdk.initRealtime(Mode.Speed) or sdk.initRealtime(Mode.Normal)

const futopt = sdk.marketdata.webSocketClient.futopt;

futopt.connect()

futopt.on("message", (message) => {
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
// 可指定行情連線的Mode ， Default 為Speed 
// sdk.InitRealtime(Mode.Speed) or sdk.InitRealtime(Mode.Normal)

var futopt = sdk.MarketData.WebSocketClient.FutureOption;

futopt.OnMessage += (msg) => Console.WriteLine($"receive: {msg}");

await futopt.Connect();

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
futopt.ping({
  'state' : '<ANY>'
})

```

```js
futopt.ping({state:'<ANY>'});

```

```cs
futopt.ping("<ANY>");

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

* `trades` - [接收訂閱期權商品最新成交資訊](https://www.fbs.com.tw/TradeAPI/docs/market-data-future/websocket-api/market-data-channels/trades.txt)
* `books` - [接收訂閱期權商品最新最佳五檔委買委賣資訊](https://www.fbs.com.tw/TradeAPI/docs/market-data-future/websocket-api/market-data-channels/books.txt)

##### 訂閱頻道[​](#訂閱頻道 "Direct link to 訂閱頻道")

要訂閱一個頻道可用下方範例進行訂閱：

* Python
* Node.js
* C#

```python
futopt.subscribe({
  "channel" : "<CHANNEL_NAME>",
  "symbol" : "<SYMBOL_ID>"
  #"afterHours" : True  若要訂閱夜盤行情，可再額外加入此參數
})

```

```js
futopt.subscribe({ 
  channel: '<CHANNEL_NAME>', 
  symbol: '<SYMBOL_ID>',
  //afterHours : true  若要訂閱夜盤行情，可再額外加入此參數
});

```

```cs
futopt.Subscribe(FutureOptionChannel.<CHANNEL_NAME>,"<SYMBOL_ID>");
//futopt.Subscribe(FutureOptionChannel.Trades,new FutureOptionParams{Symbol="<SYMBOL_ID>", AfterHours = true});   訂閱夜盤行情

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

支援訂閱同頻道的多檔商品：

* Python
* Node.js
* C#

```python
futopt.subscribe({
  "channel" : "<CHANNEL_NAME>",
  "symbols" : ["<SYMBOL_ID>","<SYMBOL_ID>"]
  #"afterHours" : True  若要訂閱夜盤行情，可再額外加入此參數
})

```

```js
futopt.subscribe({ 
  channel: '<CHANNEL_NAME>', 
  symbols: ['<SYMBOL_ID>','<SYMBOL_ID>']
  //afterHours : true  若要訂閱夜盤行情，可再額外加入此參數
});

```

```cs
futopt.Subscribe(FutureOptionChannel.<CHANNEL_NAME>,"<SYMBOL_ID>","<SYMBOL_ID>");
//futopt.Subscribe(FutureOptionChannel.Trades, new FutureOptionParams{Symbols = new List<string>{"<SYMBOL_ID>", "<SYMBOL_ID>"}, AfterHours = true});    訂閱多檔夜盤行情

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
futopt.unsubscribe({
    'id':'<CHANNEL_ID>'
})

```

```js
futopt.unsubscribe({ 
  id : '<CHANNEL_ID>'
});

```

```cs
futopt.Unsubscribe("<CHANNEL_ID>");

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
futopt.unsubscribe({
    'ids':['<CHANNEL_ID>','<CHANNEL_ID>']
})

```

```js
futopt.unsubscribe({ 
  ids : ['<CHANNEL_ID>','<CHANNEL_ID>']
});

```

```cs
futopt.Unsubscribe("<CHANNEL_ID>","<CHANNEL_ID>");

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

futopt.on("connect", handle_connect)
futopt.on("disconnect", handle_disconnect)
futopt.on("error", handle_error)

```

```js
futopt.on("connect", (message) => {
  const connect_msg = JSON.parse(message);
  console.log(connect_msg);
});

futopt.on("disconnect", (message) => {
  console.log(message);
});

futopt.on("error", (message) => {
  const err_msg = JSON.parse(message);
  console.log(err_msg);
});


```

```cs
futopt.OnError += (errmsg) => Console.WriteLine($"handle error: {errmsg}");
futopt.OnConnected += (connmsg) => Console.WriteLine($"Connect: {connmsg}");
futopt.OnDisconnected += (disconmsg) => Console.WriteLine($"Disconnect: {disconmsg}");

```

##### 斷線重連[​](#斷線重連 "Direct link to 斷線重連")

以下將用簡單範例，利用上述定義之錯誤處理方法，接收到斷線事件，程式自動進行連線

* Python
* Node.js
* C#

```python
def handle_disconnect(code, message):
    print(f'market data disconnect: {code}, {message}')
    futopt.connect()
    print("Reconnected Succuess")
    print("Resubscribe")
    futopt.subscribe({    # 重新訂閱您已訂閱過的Channel與Symbol
        'channel': '<CHANNEL_NAME>', 
        'symbol': '<SYMBOL_ID>'
    })

```

```js
futopt.on("disconnect", (message) => {
  console.log(message);
  futopt.connect()
  console.log("Reconnected Succuess");
  futopt.subscribe({ channel: '<CHANNEL_NAME>', symbol: '<SYMBOL_ID>' });  //重新訂閱您已訂閱過的Channel與Symbol
});


```

```cs
futopt.OnDisconnected += async (msg) =>
{
    Console.WriteLine($"disconnected at {DateTime.Now}");
    await Task.Delay(10);
    Console.WriteLine("Try Reconnected");
    await futopt.Connect();
    Console.WriteLine("Reconnected Success");
    Console.WriteLine("Resubscribe...");
    await futopt.Subscribe(FutureOptionChannel.<CHANNEL_NAME>, "<SYMBOL_ID>"); //重新訂閱您已訂閱過的Channel與Symbol
    Console.WriteLine("Resubscribe Success");
};

```


---

### Aggregates

接收訂閱期權聚合數據的行情資訊

#### Parameters[​](#parameters "Direct link to Parameters")

| Name           | Type   | Description                                                   |
| -------------- | ------ | ------------------------------------------------------------- |
| `channel`\*    | string | 訂閱頻道：`trades`, `books`, `aggregates`, `candles`          |
| `symbol`\*     | string | 期權代碼                                                      |
| `afterHours`\* | bool   | 訂閱夜盤行情 true : 夜盤行情 false : 日盤行情 default : false |

#### Response[​](#response "Direct link to Response")

| Name                  | Type   | Description                  |
| --------------------- | ------ | ---------------------------- |
| `date`\*              | string | 日期                         |
| `type`\*              | string | 期權類型                     |
| `exchange`            | string | 交易所                       |
| `symbol`              | string | 商品代號                     |
| `name`                | string | 商品名稱                     |
| `previousClose`       | number | 昨日收盤價                   |
| `openPrice`           | number | 開盤價                       |
| `openTime`            | number | 開盤價成交時間               |
| `highPrice`           | number | 最高價                       |
| `highTime`            | number | 最高價成交時間               |
| `lowPrice`            | number | 最低價                       |
| `lowTime`             | number | 最低價成交時間               |
| `closePrice`          | number | 收盤價（最後成交價）         |
| `closeTime`           | number | 收盤價（最後成交價）成交時間 |
| `avgPrice`            | number | 當日成交均價                 |
| `change`              | number | 最後成交價漲跌               |
| `changePercnet`       | number | 最後成交價漲跌幅             |
| `amplitude`           | number | 當日振幅                     |
| `lastPrice`           | number | 最後一筆成交價（含試撮）     |
| `lastSize`            | number | 最後一筆成交數量（含試撮）   |
| `total`               | object | 統計時間                     |
| >> `tradeVolume`      | number | 累計成交量                   |
| >> `tradeVolumeAtBid` | number | 委買成筆量                   |
| >> `tradeVolumeAtAsk` | number | 委賣成筆量                   |
| `lastTrade`           | object | 最後一筆成交資訊             |
| >> `price`            | number | 最後一筆成交價格             |
| >> `size`             | number | 最後一筆成交數量             |
| >> `time`             | number | 最後一筆成交時間             |
| >> `serial`           | number | 最後一筆成交流水號           |
| `serial`              | number | 流水號                       |
| `lastUpdated`         | number | 最後異動時間                 |

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

futopt = sdk.marketdata.websocket_client.futopt
futopt.on('message', handle_message)
futopt.connect()
futopt.subscribe({ 
    'channel': 'aggregates', 
    'symbol': 'TXFA4'
})


```

```js
const { FubonSDK, Mode } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.initRealtime(Mode.Normal); // 建立行情連線

const futopt = sdk.marketdata.webSocketClient.futopt;
futopt.connect().then(() => {
  futopt.subscribe({ channel: "aggregates", symbol: "TXFA4" });
});

futopt.on("message", (message) => {
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

var futopt = sdk.MarketData.WebSocketClient.FutureOption;

futopt.OnMessage += (msg) => Console.WriteLine($"receive: { msg }");

await futopt.Connect();

await futopt.Subscribe(FugleMarketData.WebsocketModels.FutureOptionChannel.Aggregates, "TXFA4");

```

訂閱多檔商品

同時訂閱多檔商品方式，請參考 [訂閱頻道](https://www.fbs.com.tw/TradeAPI/docs/market-data-future/websocket-api/getting-started.txt#%E8%A8%82%E9%96%B1%E9%A0%BB%E9%81%93)

##### Receive data[​](#receive-data "Direct link to Receive data")

```json
{
    "event": "data",
    "data": {
        "date": "2023-12-26",
        "type": "FUTURE",
        "exchange": "TAIFEX",
        "symbol": "TXFA4",
        "name": "臺股期貨014",
        "previousClose": 17622,
        "openPrice": 17651,
        "openTime": 1703551500020000,
        "highPrice": 17740,
        "highTime": 1703560921393000,
        "lowPrice": 17634,
        "lowTime": 1703552892448000,
        "closePrice": 17735,
        "closeTime": 1703569500075000,
        "avgPrice": 17700.51,
        "change": 113,
        "changePercent": 0.64,
        "amplitude": 0.6,
        "lastPrice": 17735,
        "lastSize": 7,
        "total": {
            "tradeVolume": 52553,
            "totalBidMatch": 26280,
            "totalAskMatch": 26273
        },
        "lastTrade": {
            "price": 17735,
            "size": 7,
            "time": 1703569500075000,
            "serial": "00136127"
        },
        "serial": 136127,
        "lastUpdated": 1703569500075000
    },
    "id": "<CHANNEL_ID>",
    "channel": "aggregates"
}

```


---

### Books

接收訂閱期權商品最新最佳五檔委買委賣資訊

#### Parameters[​](#parameters "Direct link to Parameters")

| Name           | Type   | Description                                                   |
| -------------- | ------ | ------------------------------------------------------------- |
| `channel`\*    | string | 訂閱頻道：`trades`, `books`, `aggregates`, `candles`          |
| `symbol`\*     | string | 契約代碼                                                      |
| `afterHours`\* | bool   | 訂閱夜盤行情 true : 夜盤行情 false : 日盤行情 default : false |

#### Response[​](#response "Direct link to Response")

| Name         | Type      | Description      |
| ------------ | --------- | ---------------- |
| `symbol`\*   | string    | 期權代碼         |
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

futopt = sdk.marketdata.websocket_client.futopt
futopt.on('message', handle_message)
futopt.connect()
futopt.subscribe({ 
    'channel': 'books', 
    'symbol': 'TXFA4'
    #'afterHours' : True   # 夜盤行情
})


```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.initRealtime(); // 建立行情連線

const futopt = sdk.marketdata.webSocketClient.futopt;

futopt.connect().then(() => {
  futopt.subscribe({ 
                     channel: "books", 
                     symbol: "TXFA4" 
                     // afterHours: true    //夜盤行情 
                   });
});

futopt.on("message", (message) => {
  const data = JSON.parse(message);
  console.log(data);
});

```

```cs
using FubonNeo.Sdk;

var sdk = new FubonSDK();

var result = sdk.Login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.InitRealtime();  // 建立行情連線

var futopt = sdk.MarketData.WebSocketClient.FutureOption;

futopt.OnMessage += (msg) => Console.WriteLine($"receive: { msg }");

await futopt.Connect();

await futopt.Subscribe(FugleMarketData.WebsocketModels.FutureOptionChannel.Books, "TXFA4");

// await futopt.Subscribe(FugleMarketData.WebsocketModels.FutureOptionChannel.Books,  new FutureOptionParams { Symbol = "TXFC4", AfterHours = true });  // 夜盤行情

```

訂閱多檔商品

同時訂閱多檔商品方式，請參考 [訂閱頻道](https://www.fbs.com.tw/TradeAPI/docs/market-data-future/websocket-api/getting-started.txt#%E8%A8%82%E9%96%B1%E9%A0%BB%E9%81%93)

##### Receive data[​](#receive-data "Direct link to Receive data")

```json
{
    "event": "data",
    "data": {
        "symbol": "TXFA4",
        "type": "FUTURE",
        "exchange": "TAIFEX",
        "bids": [
          {
            "price": 17539,
            "size": 2
          }, 
          {
            "price": 17538,
            "size": 4
          }, 
          {
            "price": 17537,
            "size": 3
          }, 
          {
            "price": 17536,
            "size": 10
          }, 
          {
            "price": 17535,
            "size": 10
          }
        ],
        "asks": [
          {
            "price": 17541,
            "size": 2
          }, 
          {
            "price": 17542,
            "size": 15
          }, 
          {
            "price": 17543,
            "size": 3
          }, 
          {
            "price": 17544,
            "size": 5
          }, 
          {
            "price": 17545,
            "size": 4
          }
        ],
        "time": 1702956500113000
    },
    "id": "<CHANNEL_ID>",
    "channel": "books"
}

```


---

### Candles

接收訂閱期權商品最新分鐘Ｋ

#### Parameters[​](#parameters "Direct link to Parameters")

| Name           | Type   | Description                                                   |
| -------------- | ------ | ------------------------------------------------------------- |
| `channel`\*    | string | 訂閱頻道：`trades`, `books`, `aggregates`, `candles`          |
| `symbol`\*     | string | 契約代碼                                                      |
| `afterHours`\* | bool   | 訂閱夜盤行情 true : 夜盤行情 false : 日盤行情 default : false |

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

futopt = sdk.marketdata.websocket_client.futopt
futopt.on('message', handle_message)
futopt.connect()
futopt.subscribe({ 
    'channel': 'candles', 
    'symbol': 'TXFA4'
})


```

```js
const { FubonSDK, Mode } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your password", "Your cert path","Your cert password");

sdk.initRealtime(Mode.Normal); // 建立行情連線

const futopt = sdk.marketdata.webSocketClient.futopt;

futopt.connect().then(() => {
  futopt.subscribe({ channel: "candles", symbol: "TXFA4" });
});

futopt.on("message", (message) => {
  const data = JSON.parse(message);
  console.log(data);
});

```

```cs
using FubonNeo.Sdk;
using FugleMarketData.WebsocketModels;

var sdk = new FubonSDK();

var result = sdk.Login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.InitRealtime(Mode.Normal);  // 建立行情連線

var futopt = sdk.MarketData.WebSocketClient.FutureOption;

futopt.OnMessage += (msg) => Console.WriteLine($"receive: { msg }");

await futopt.Connect();

await futopt.Subscribe(FugleMarketData.WebsocketModels.FutureOptionChannel.Candles, "TXFA4");

```

訂閱多檔商品

同時訂閱多檔商品方式，請參考 [訂閱頻道](https://www.fbs.com.tw/TradeAPI/docs/market-data-future/websocket-api/getting-started.txt#%E8%A8%82%E9%96%B1%E9%A0%BB%E9%81%93)

##### Receive data[​](#receive-data "Direct link to Receive data")

```json
{
    "event": "data",
    "data": {
        "symbol": "TXFA4",
        "type": "FUTURE",
        "exchange": "TAIFEX",
        "date": "2023-12-28T12:01:00.000+08:00",
        "open": 17861,
        "high": 17862,
        "low": 17859,
        "close": 17862,
        "volume": 22,
        "average": 17820.19
    },
    "id": "<CHANNEL_ID>",
    "channel": "candles"
}

```


---

### Trades

接收訂閱期權商品最新成交資訊

#### Parameters[​](#parameters "Direct link to Parameters")

| Name           | Type   | Description                                                   |
| -------------- | ------ | ------------------------------------------------------------- |
| `channel`\*    | string | 訂閱頻道：`trades`, `books`, `aggregates`, `candles`          |
| `symbol`\*     | string | 期貨商品代碼                                                  |
| `afterHours`\* | bool   | 訂閱夜盤行情 true : 夜盤行情 false : 日盤行情 default : false |

#### Response[​](#response "Direct link to Response")

| Name               | Type   | Description |
| ------------------ | ------ | ----------- |
| `symbol`\*         | string | 商品代號    |
| `type`\*           | string | Ticker 類型 |
| `exchange`\*       | string | 交易所      |
| `market`           | string | 市場別      |
| `trades`           | object | 成交報價    |
| >> `price`         | number | 成交價格    |
| >> `size`          | number | 成交單量    |
| >> `bid`           | number | 成交買價    |
| >> `ask`           | number | 成交賣價    |
| `total`            | object | 成交量      |
| >> `tradeVolume`   | number | 成交總量    |
| >> `totalBidMatch` | number | 委買成筆    |
| >> `totalAskMatch` | number | 委賣成筆    |
| `time`\*           | number | 時間        |
| `serial`\*         | number | 流水號      |

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

futopt = sdk.marketdata.websocket_client.futopt
futopt.on('message', handle_message)
futopt.connect()
futopt.subscribe({ 
    'channel': 'trades', 
    'symbol': 'TXFA4'
    #'afterHours' : True   # 夜盤行情
})


```

```js
const { FubonSDK } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.initRealtime(); // 建立行情連線

const futopt = sdk.marketdata.webSocketClient.futopt;

futopt.connect().then(() => {
  futopt.subscribe({ 
                     channel: "trades",
                     symbol: "TXFA4"
                     // afterHours: true    //夜盤行情 
                    });
});

futopt.on("message", (message) => {
  const data = JSON.parse(message);
  console.log(data);
});

```

```cs
using FubonNeo.Sdk;

var sdk = new FubonSDK();

var result = sdk.Login("Your ID", "Your password", "Your cert path", "Your cert password");

sdk.InitRealtime();  // 建立行情連線

var futopt = sdk.MarketData.WebSocketClient.FutureOption;

futopt.OnMessage += (msg) => Console.WriteLine($"receive: { msg }");

await futopt.Connect();

await futopt.Subscribe(FugleMarketData.WebsocketModels.FutureOptionChannel.Trades, "TXFA4");

// await futopt.Subscribe(FugleMarketData.WebsocketModels.FutureOptionChannel.Trades,  new FutureOptionParams { Symbol = "TXFC4", AfterHours = true });  // 夜盤行情

```

訂閱多檔商品

同時訂閱多檔商品方式，請參考 [訂閱頻道](https://www.fbs.com.tw/TradeAPI/docs/market-data-future/websocket-api/getting-started.txt#%E8%A8%82%E9%96%B1%E9%A0%BB%E9%81%93)

##### Receive data[​](#receive-data "Direct link to Receive data")

```json
{
    "event": "data",
    "data": {
        "symbol": "TXFA4",
        "type": "FUTURE",
        "exchange": "TAIFEX",
        "trades": [
          {
            "price": 17540,
            "size": 1,
            "bid": 17539,
            "ask": 17540
          }
        ],
        "total": {
            "tradeVolume": 12174,
            "totalBidMatch": 8760,
            "totalAskMatch": 7907
        },
        "time": 1702956487023000,
        "serial": 159250
    },
    "id": "<CHANNEL_ID>",
    "channel": "trades"
}

```


---

### 模式切換

**適用版本 v1.3.1 及以上**

#### 行情WebSocket模式切換[​](#行情websocket模式切換 "Direct link to 行情WebSocket模式切換")

新版本功能中，提供 Low Latency 行情與多資訊的 Socket 行情切換

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
futopt = sdk.marketdata.websocket_client.futopt
futopt.on('message', handle_message)
futopt.connect()

```

```js
sdk.initRealtime(Mode.Speed); // 提供Speed / Normal選擇 (default 為Speed)
// sdk.initRealtime(Mode.Speed) or sdk.initRealtime(Mode.Normal)
const futopt = sdk.marketdata.webSocketClient.futopt;

futopt.connect().then(() => {
  futopt.subscribe({ 
      channel: "trades",
      symbol: "TXFA4"
      // afterHours: true    //夜盤行情 
    });
});

futopt.on("message", (message) => {
  const data = JSON.parse(message);
  console.log(data);
});

```

```cs
using FubonNeo.Sdk;
using FugleMarketData.WebsocketModels; // 新增 "Mode"

sdk.InitRealtime(Mode.Speed); // 提供Speed / Normal選擇 (default 為Speed)
// sdk.initRealtime(Mode.Speed) or sdk.initRealtime(Mode.Normal)

var futopt = sdk.MarketData.WebSocketClient.FutureOption;

futopt.OnMessage += (msg) => Console.WriteLine($"receive: { msg }");

await futopt.Connect();

```


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
    end_date="20240517",
    intraday =False       # optional field
)


sdk.stock.single_condition(account, "20240426", "20240430", StopSign.Full, condition, order, tpsl)


```

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
    endDate: "20240517",
    intraday: false      // optional field
}




sdk.stock.singleCondition(account, "20240426", "20240430", StopSign.Full, condition, order, tpsl)

```

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


---

### 移動鎖利條件單

富邦新一代 API 支援移動鎖利單，

移動鎖利運作機制 : 當基準條件觸發後，若設定的回檔幅度未達到，則不會觸發，當後續股價持續往上，則會使用新的價格作為基準，若此時回檔幅度到達設定值，則送出委託

![trailProfit](/TradeAPI/assets/images/TrailProfit-57d72c840dd40176b183a22ef955bfdc.png)

##### 程式範例[​](#程式範例 "Direct link to 程式範例")

* Python
* Node.js
* C#

```py
# 設計條件內容
trail = TrailOrder(
    symbol = "2330",
    price = "1000",
    direction = Direction.Down ,
    percentage = 5,  # 漲跌 % 數
    buy_sell = BSAction.Sell,
    quantity = 2000,
    price_type = ConditionPriceType.MatchedPrice,
    diff = 1,     # 向上 or 向下追買 tick數 (向下為負值)
    time_in_force = TimeInForce.ROD,
    order_type = ConditionOrderType.Stock
)

sdk.stock.trail_profit(target_account, "20240427","20240516", StopSign.Full, trail)

```

```js
// 設計條件內容
const trail = {
    symbol: "2330",
    price: "1000",
    direction: Direction.Down,
    percentage: 5,  
    buySell: BSAction.Sell,
    quantity: 2000,
    priceType: ConditionPriceType.MatchedPrice,
    diff: 1,
    timeInForce: TimeInForce.ROD,
    orderType: ConditionOrderType.Stock
    
};

sdk.stock.trailProfit(target_account, "20240427","20240516", StopSign.Full, trail)


```

```cs
// 設計條件內容
var trail = new TrailOrder(
    "2330",
    "1000",
    Direction.Down ,
    5,  // 漲跌 % 數
    BsAction.Sell,
    2000,
    ConditionPriceType.MatchedPrice,
    1, // 向上 or 向下追買 tick數 (向下為負值)
    TimeInForce.Rod,
    ConditionOrderType.Stock
);


sdk.Stock.TrailProfit(target_account, "20240427","20240516", StopSign.Full, trail);

```


---

### 條件單使用說明

本公司為服務投資人，提供網路下單客戶得預先設定下單條件之條件下單功能服務（以下簡稱本功能或本服務）。本服務不收取任何費用，但本公司保留得隨時調整、修正、暫停或終止此項服務之權利；為確保使用者權益及網路交易安全秩序，請投資人於啟用本功能前，詳細閱讀以下條款，以保障投資人的權利，在投資人點選本使用同意書下方之同意鈕，即視為已經詳閱並接受本同意書之內容。<br /><!-- -->一、條件下單功能服務具有以下（但不限於此）之風險，投資人應自承以下風險所衍生之損害或損失：<br /><!-- -->1 網際網路之傳輸通訊，可能因不可抗力或不可歸責於本公司之事由，包括（但不在此限）斷電、斷線、網路壅塞等因素，導致資料傳輸延遲、停滯、無法發出或無法接收。<br /><!-- -->2 本功能或服務有可能因病毒、系統或機件發生故障、錯誤、執行障礙，駭客入侵等因素，導致資料或資訊傳送發生遺漏、錯誤、或遭竄改。<br /><!-- -->3 其他網路交易之風險。<br /><!-- -->二、條件下單功能服務，係依據投資人設定之個股或期貨商品種類及下單條件，提供即時有效之資訊，並結合既有之網路下單功能，允許投資人在同一網頁操作證券或期貨下單之委託指示。本公司目前所提供之條件下單功能服務包括觸價下單、觸量下單、定時下單、長效停損/停利下單及多條件下單共五種（簡稱為條件下單），不同種類的條件下單，其設定方式、效力有可能不同，或受有特別限制，投資人於設定每一筆條件下單前應詳細閱讀注意事項，審慎為之。<br /><!-- -->三、經觸發條件下單功能服務「直接下單」功能鍵後，具有與口頭、電話、傳真、網路下單相同之效力，如經成交即不得取消，投資人負有交割義務，以及證券、期貨相關法令所規定之各項責任。<br /><!-- -->四、投資人得於成交前以網路或電話方式變更或取消委託，投資人於下單前應充分瞭解有關變更或取消委託之操作。因不可抗力或不可歸責於本公司之事由，包括（但不在此限）斷電、斷線、網路壅塞、傳輸干擾等因素，致變更或取消意思表示無法即時送達之風險，由投資人自行負擔。<br /><!-- -->五、基於網路維運、符合資訊法規及網路規範、確保網路交易安全，或服從主管機關之命令等因素，對於有礙系統正常運作、或可能造成系統資訊不穩定、或其他異常狀況之條件下單，本公司保留刪除之權利。如經決定刪除，本公司將另以電子平台公告、行動簡訊或電子郵件通知投資人，並請投資人至富邦證券網站、HTS快易點、HTS2多易點及Online查詢條件下單訊息公告。<br /><!-- -->六、經公告於本公司網站上之其他有關電子下單、客戶資料使用、網站管理、保密措施等規範，於條件下單功能服務均有其適用。如有未盡事宜，悉照中華民國相關法令以及經公告之網際網路交易規範為準據。<br /><!-- -->七、請投資人依本公司條件下單之下單規則，於完成有效期間條件下單之設定後， 請於該條件下單之有效期間內之每一營業日當日查詢該條件下單狀態；條件下單之詳細狀態僅保留至當日，若投資人對條件下單狀態有疑慮，請於該筆交易當日洽詢所屬分公司營業同仁。如交易當日條件下單有故障或因不可抗力因素導致系統無法送單情形，則該交易視為未下單處理。<br /><!-- -->注意事項：

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

本頁面彙整了富邦新一代 API（條件單）中常見的狀態碼，方便開發者在遇到問題或進行狀態判斷時快速檢索。

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

