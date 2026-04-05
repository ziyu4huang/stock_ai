## accounts = sdk.login("您的身分證號", "您的登入密碼", "您的憑證路徑位置")  # 若憑證選用＂預設密碼＂, SDK v1.3.2與較新版本適用

acc  = accounts.data[0]

# 建立委託物件
order = Order(
  buy_sell = BSAction.Buy,
  symbol = "2881",
  quantity   = 2000,
  price = "66",
  market_type = MarketType.Common,
  price_type  = PriceType.Limit,
  time_in_force = TimeInForce.ROD,
  order_type = OrderType.Stock,
  user_def = "FromPy" # optional field
)

# 下單
order_res = sdk.stock.place_order(acc, order)
print(order_res)

```

恭喜您<!-- -->🎊<!-- -->，完成下單，即可看到系統回覆的結果

```python
Result {  is_success: True,   message: None,  data : OrderResult{ date: "2023/10/13", seq_no: "00000000015", branch_no: "6460", account: "26", order_no: "bA626", asset_type: 0, market: "TAIEX", market_type: Common, stock_no: "2881", buy_sell: Buy, price_type: Limit, price: 66.0, quantity: 2000, time_in_force: ROD, order_type: Stock, is_pre_order: false, status: 10, after_price_type: Limit, after_price: 66.0, unit: 1000, after_qty: 2000, filled_qty: 0, filled_money: 0, before_qty: 0, before_price: 66.0, user_def: "FromPy", last_time: "16:48:09.247", error_message: None} }

```

```js
const { FubonSDK, BSAction, TimeInForce, OrderType, PriceType, MarketType } = require('fubon-neo');

// 載入設定檔與登入
const sdk = new FubonSDK();

const accounts = sdk.login("您的身分證號", "您的登入密碼", "您的憑證路徑", "您的憑證密碼");
// const accounts = sdk.login("您的身分證號", "您的登入密碼", "您的憑證路徑位置");  // 若憑證選用＂預設密碼＂, SDK v1.3.2與較新版本適用

const acc = accounts.data[0];

// 建立委託物件
const order = {
buySell: BSAction.Buy,
symbol: 2881,
price: "66",
quantity: 2000,
marketType: MarketType.Common,
priceType: PriceType.Limit,
timeInForce: TimeInForce.ROD,
orderType: OrderType.Stock,
userDef: "fromJs"
};

// 下單
sdk.stock.placeOrder(acc,order);

```

恭喜您<!-- -->🎊<!-- --> ，完成下單，即可看到系統回覆的結果

```js
{ isSuccess: true,  data:{ date : '2023/10/13', seqNo : '00000000016', branchNo : '6460', account : '26', orderNo : 'bA627', assetType : 0, market : 'TAIEX', marketType : 'Common', stockNo : '2881', buySell : 'Buy', priceType : 'Limit', price : 66, quantity : 2000, timeInForce : 'ROD', orderType : 'Stock', isPreOrder : false, status : 10, afterPriceType : 'Limit', afterPrice : 66, unit : 1000, afterQty : 2000, filledQty : 0, filledMoney : 0, beforeQty : 0, beforePrice : 66, userDef : 'FromJS', lastTime : '17:19:06.048' } }

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
var order = new Order(
  BsAction.Buy,
  "2881",
  "66",
  2000,
  MarketType.Common,
  PriceType.Limit,
  TimeInForce.ROD,
  OrderType.DayTrade,
  "FromCS"
);

// 下單
var order_res = sdk.Stock.PlaceOrder(acc, order);
Console.WriteLine(order_res);

```

恭喜您<!-- -->🎊<!-- -->，完成下單，即可看到系統回覆的結果

```cs
{ isSuccess = True, message = , data = { date = 2023/10/13, seqNo = 00000000016, branchNo = 6460, account = 26, orderNo = bA627, assetType = 0, market = TAIEX, marketType = Common, stockNo = 2881, buySell = Buy, priceType = Limit , price = 66, quantity = 2000, timeInForce = ROD, orderType = DayTrade, isPreOrder = False, status = 10, afterPriceType = Limit, afterPrice = 66, unit = 1000, afterQty = 2000, filledQty = 0, filledMoney = 0, beforeQty = 0, beforePrice = 66, userDef = FromC#, lastTime = 17:19:06.048, errorMessage =  } }

```


---

### 快速開始

本頁重點

* 本頁提供富邦新一代 API（Fubon Neo API）登入與下單範例。
* 開始前請先完成[事前準備](https://www.fbs.com.tw/TradeAPI/docs/trading/prepare.txt)（開戶、憑證、聲明與連線測試）。
* 安裝方式與版本相容性請見[安裝與版本相容性](https://www.fbs.com.tw/TradeAPI/docs/install-compatibility.txt)。

| 項目     | 說明                                                                       |
| -------- | -------------------------------------------------------------------------- |
| 前置條件 | 完成事前準備                                                               |
| 支援語言 | Python / C# / JavaScript (Node.js) / C++、Go（僅支援證券交易帳務及條件單） |
| 下載來源 | SDK 下載頁面                                                               |
| 安裝方式 | 請見安裝與版本相容性                                                       |
| 下一步   | 依語言完成登入與下單範例                                                   |

請先參考[事前準備](https://www.fbs.com.tw/TradeAPI/docs/trading/prepare.txt)完成風險預告書申請與憑證下載。

#### 安裝與版本相容性[​](#安裝與版本相容性 "Direct link to 安裝與版本相容性")

安裝步驟與最低版本需求請見[安裝與版本相容性](https://www.fbs.com.tw/TradeAPI/docs/install-compatibility.txt)，下載 SDK 請至[SDK 下載頁面](https://www.fbs.com.tw/TradeAPI/docs/download/download-sdk.txt)。

#### 套件安裝[​](#套件安裝 "Direct link to 套件安裝")

<!-- -->

* Python
* Node.js
* C#

```py
pip install fubon_neo-<version>-cp37-abi3-win_amd64.whl

```

首先先建立一個project資料夾，並利用下列指令初始化

```bash
npm init

```

官網下載回來的檔案會像是 fubon-neo-\<version>.tgz，並且把它放入剛剛建立的資料夾中

把 node.js project 裡面的 package.json 增加一行

```json
"dependencies": {
  ...
  "fubon-neo": "file://<path-to-js-binary>/fubon-neo-<version>.tgz",
  ...
}

```

然後到命令行輸入以下指令

```bash
npm install

```

官網下載回來的檔案會像是 FubonNeo.\<version>.nupkg

* 使用方法一 : Visual Studio Nuget 套件管理員安裝( **以.NET 7為例** ) :

  將套件放置在新增的專案下方後，點擊 `工具` -> `Nuget套件管理員` -> `套件管理員設定`

  ![nugetset](/TradeAPI/assets/images/nugetset-dcd96ee20e4663a6c4b996264d1b962e.png)

  跳出視窗後，

  1. 點擊右上角的 `+`
  2. 點擊下方的 `...` 選取套件存放位置
  3. 將名稱更名
  4. 點擊 `更新`
  5. 按下確定 ![nugetsource](/TradeAPI/assets/images/nugetsource-8523418911907166548dd2f4afd61d1c.png)

  點擊`工具` -> `Nuget套件管理員` -> `管理方案的Nuget套件`

  ![nugetpackage](/TradeAPI/assets/images/nugetpackage-d71a41371e06f457ac161917ba1a0af6.png)

  接者選取

  1. 套件來源，選取前一個步驟命名的套件名稱
  2. 點選瀏覽
  3. 點擊套件
  4. 選取專案
  5. 安裝版本

  ![nugetinstall](/TradeAPI/assets/images/nugetinstall-1cce8168b9c84304d8a20eaf103da69d.png)

  安裝完成! 即可使用新一代SDK。

***

* 使用方法二 : Visual Studio Nuget 套件管理員安裝( **以.NETFramework 4.7.2為例**) :<br /><!-- -->將套件放置在新增的專案下方後，點擊 `工具` -> `Nuget套件管理員` -> `套件管理員設定`

  ![nugetset](/TradeAPI/assets/images/nugetset-dcd96ee20e4663a6c4b996264d1b962e.png)

  跳出視窗後，

  1. 點擊右上角的 `+`
  2. 點擊下方的 `...` 選取套件存放位置
  3. 將名稱更名
  4. 點擊 `更新`
  5. 按下確定 ![nugetsource](/TradeAPI/assets/images/nugetsource-8523418911907166548dd2f4afd61d1c.png)

  點擊右方方案總管中`在解決方案與可用檢視之間切換` (若找不到方案總管，可至上方檢視中開啟) ， 開啟資料夾後，找到.csproj檔案

  ![csprojadd](/TradeAPI/assets/images/csprojadd-884ccdbfb9631160aff1fabfc80d6f80.png)

  csproj 檔案裡面增加 sdk 的 reference (Version 內容再依版號填入)

  ```xml

  <ItemGroup>
    ...
    <PackageReference Include="FubonNeo" Version="1.0.0" />  
    ...
  </ItemGroup>
  ...

  ```

  點擊檔案總管中的 `.sln`， 於專案名稱中點選右鍵，重新載入專案

  ![csprojimport](/TradeAPI/assets/images/csprojimport-a4dfc02aa766b2640c3b75692978f535.png)

***

* 若不使用 Visual Studio Nuget套件管理員，可使用 Command Line如下

  在 project 資料夾下面新增 nuget.config 檔案：

  ```bash
  cd /您的資料夾路徑/
  cd .>nuget.config   #新增nuget.config

  ```

  並將下列內容貼至nuget.config

  ```xml
  <?xml version="1.0" encoding="utf-8"?>
  <configuration>
      <packageSources>
          <add key="nuget.org" value="https://api.nuget.org/v3/index.json" />
          <add key="local" value="./" />
      </packageSources>
      <config>
          <add key="repositoryPath" value="./" />
      </config>
      <packageSourceMapping>
          <!-- key value for <packageSource> should match key values from <packageSources> element -->
          <packageSource key="nuget.org">
              <package pattern="*" />
          </packageSource>
          <packageSource key="local">
              <package pattern="FubonNeo*" />
          </packageSource>
      </packageSourceMapping>
  </configuration>

  ```

  接著在 project 的 csproj 檔案裡面增加 sdk 的 reference (Version 內容再依版號填入)

  ```xml
    ...
    <ItemGroup>
      <PackageReference Include="FubonNeo" Version="1.0.0" />   
    </ItemGroup>
    ...

  ```

  接著

  ```cs
  dotnet restore

  ```

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
from fubon_neo.sdk import FubonSDK, Order
from fubon_neo.constant import TimeInForce, OrderType, PriceType, MarketType, BSAction

sdk = FubonSDK()
   
accounts = sdk.login("您的身分證字號", "您的登入密碼", "您的憑證位置", "您的憑證密碼") #若有歸戶，則會回傳多筆帳號資訊
## accounts = sdk.login("您的身分證號", "您的登入密碼", "您的憑證路徑位置")  # 若憑證選用＂預設密碼＂, SDK v1.3.2與較新版本適用

acc = accounts.data[0]

```

若您未曾使用 SDK 進行過登入，或更換了 SDK 執行環境，請在資料夾新增一個 `index.js` 檔案，貼上以下內容並執行：

```js
const { FubonSDK, BSAction, TimeInForce, OrderType, PriceType, MarketType } = require('fubon-neo');

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
order = Order(
    buy_sell = BSAction.Buy,
    symbol = "2881",
    price = None,
    quantity = 2000,
    market_type = MarketType.Common,
    price_type = PriceType.LimitDown,
    time_in_force = TimeInForce.ROD,
    order_type = OrderType.Stock,
    user_def = "FromPy" # 使用者可自定義填入 , Optional field
)
sdk.stock.place_order(acc, order)

```

```js
const order = {
  buySell: BSAction.Buy,
  symbol: 2881,
  price: "66",
  quantity: 2000,
  marketType: MarketType.Common,
  priceType: PriceType.LimitDown,
  timeInForce: TimeInForce.ROD,
  orderType: OrderType.Stock,
  userDef: "fromJs"
};
//
sdk.stock.placeOrder(acc, order);

```

```cs
var order = new Order(
    BsAction.Buy,
    "2881",
    "66",
    2000,
    MarketType.Common,
    PriceType.LimitDown,
    TimeInForce.Rod,
    OrderType.Stock,
    null
);

sdk.Stock.PlaceOrder(acc ,order); // 使用阻塞委託下單

```


---

### 速率限制

本頁重點

* 本頁整理富邦新一代 API（Fubon Neo API）連線與下單速率限制。
* 連線數上限為 10；下單與查詢有每秒限制。
* 超過上限時，會回傳錯誤訊息（如下方範例）。

| 項目     | 上限    |
| -------- | ------- |
| 連線數   | 10      |
| 下單     | 50 / 秒 |
| 批次下單 | 10 / 秒 |
| 帳務查詢 | 5 / 秒  |

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

### 帳務

***

我們可以透過混和部位查詢及權益數來確部位等資訊。

info

查詢發送的次數為每秒5次，若超出上限，請稍後再試試！

#### 部位查詢[​](#部位查詢 "Direct link to 部位查詢")

* Python
* Node.js
* C#

```python
h_position = sdk.futopt_accounting.query_hybrid_position(accounts.data[0])
print(h_position)

```

```py
Result {
    is_success: True,
    message: None,
    data : [HybridPosition {
                date : "2024/04/08",                     # 部位建立日期 (str)
                branch_no : "15901",                      # 分公司代號 (str)
                account : "1234567",                      # 帳號 (str)
                is_spread : False,                        # 是否為複式部位 (bool)
                position_kind : 1,                        # 部位種類  (int)
                symbol : "FITX",                          # 商品代號 (str)
                expiry_date : 202404,                     # 履約日 (string)
                strike_price : None,                      # 履約價 (int or None)
                call_put : None,                          # 權利別 (int or None)
                buy_sell : Buy,                             # 買賣別  (BSAction)
                price : 20325.3333,                       # 成交價 (float)
                orig_lots : 3,                            # 原始口數 (int)
                tradable_lot : 3,                         # 可交易口數 (int)
                order_type : New,                           # 委託別 (FutOptOrderType)
                currency : "TWD",                         # 幣別 (str)
                market_price : "20351",                   # 即時價 (str)
                initial_margin : 0.0,                     # 原始保證金 (float)
                maintenance_margin : 0.0,                 # 維持保證金 (float)
                clearing_margin : 0.0,                    # 結算保證金 (float)
                initial_margin_all_single : 0.0,          # 原始保證金 (float)
                opt_value : 0.0,                          # 選擇權市值 (float)
                opt_long_value : 0.0,                     # 選擇權買進市值 (float)
                opt_short_value : 0.0,                    # 選擇權賣出市值 (float)
                profit_or_loss : 0.0,                     # 部位損益 (float)
                premium : 0.0,                            # 權利金 (float)
                spreads : None,                           # 複式部位 (List[SpreadPosition])
            }
            ...
    ]
}

```

```js
const h_position = sdk.futoptAccounting.queryHybridPosition(accounts.data[0])
console.log(h_position)

```

```js
{
  isSuccess: true,
  data:[
         {
          date : "2024/04/08",                      // 部位建立日期 (string)
          branchNo : "15901",                       // 分公司代號 (string)
          account : "1234567",                      // 帳號 (string)
          isSpread : false,                         // 是否為複式部位 (boolean)
          positionKind : 1,                         // 部位種類 (number)
          symbol : "FITX",                          // 商品代號 (string)
          expiryDate : '202404',                    // 履約日 (string)
          buySell : Buy,                            // 買賣別 (number)
          price : 20325.3333,                       // 成交價 (number)
          origLots : 3,                             // 原始口數 (number)
          tradableLot : 3,                          // 可交易口數 (number)
          orderType : New,                          // 委託別 (FutOptOrderType)
          currency : "TWD",                         // 幣別 (string)
          marketPrice : "20351",                    // 即時價 (string)
          initialMargin : 0.0,                      // 原始保證金 (number)
          maintenanceMargin : 0.0,                  // 維持保證金 (number)
          clearingMargin : 0.0,                     // 結算保證金 (number)
          initialMarginAllSingle : 0.0,             // 原始保證金 (number)
          optValue : 0.0,                           // 選擇權市值 (number)
          optLongValue : 0.0,                       // 選擇權買進市值 (number)
          optShortValue : 0.0,                      // 選擇權賣出市值 (number)
          profitOrLoss : 0.0,                       // 部位損益 (number)
          premium : 0.0                             // 權利金 (number)
      },
  ]
}


```

```cs
var h_position = sdk.FutOptAccounting.QueryHybridPosition(accounts.data[0]);
foreach (var detail in h_position.data)
{
  Console.WriteLine(detail);
}

```

```cs
{
    {
              date = 2024/04/08,                       // 部位建立日期 (string)
              branchNo = 15901,                         // 分公司代號 (string)
              account = 1234567,                        // 帳號 (string)
              isSpread = false,                         // 是否為複式部位 (boolean)
              positionKind = 1,                         // 部位種類 : `1` 期貨、`2` 選擇權 (int)
              symbol = FITX,                            // 商品代號 (string)
              expiryDate = 202404,                      // 履約日 (string)
              strikePrice = None,                       // 履約價 (double)
              callPut = None,                           // 權利別 (CallPut)
              buySell = Buy,                            // 買賣別 (BsAction)
              price = 20325.3333,                       // 成交價 (double)
              origLots = 3,                             // 原始口數 (int)
              tradableLot = 3,                          // 可交易口數 (int)
              orderType = New,                          // 委託別 (FutOptOrderType)
              currency = TWD,                           // 幣別 (string)
              marketPrice = 20351,                      // 即時價 (string)
              initialMargin = 0.0,                      // 原始保證金 (double)
              maintenanceMargin = 0.0,                  // 維持保證金 (double)
              clearingMargin = 0.0,                     // 結算保證金 (double)
              initialMarginAllSingle = 0.0,             // 原始保證金 (double)
              optValue = 0.0,                           // 選擇權市值 (double)
              optLongValue = 0.0,                       // 選擇權買進市值 (double)
              optShortValue = 0.0,                      // 選擇權賣出市值 (double)
              profitOrLoss = 0.0,                       // 部位損益 (double)
              premium = 0.0,                            // 權利金 (double)
              spreads = None,                           // 複式部位 (SpreadPosition[])
      },
      ...
}

```

* 以下範例回傳僅擷取data內容

#### 權益數查詢[​](#權益數查詢 "Direct link to 權益數查詢")

可以查詢帳務權益數，確認我們帳戶權益的狀況。

* Python
* Node.js
* C#

```python
equity = sdk.futopt_accounting.query_margin_equity(accounts.data[0])
print(equity.data)

```

```js
const equity = sdk.futoptAccounting.queryMarginEquity(accounts.data[0])
console.log(equity.data)

```

```cs
var equity = sdk.FutOptAccounting.QueryMarginEquity(accounts.data[0]);
Console.WriteLine(equity.data);

```

* Python
* Node.js
* C#

```py
Equity({
          date: "2024/04/08",                   # 查詢日期 (string)
          branch_no: "15901",                   # 分公司代號 (string)
          account: "1234567",                   # 帳號 (string)
          currency: "NTD",                      # 幣別 (string)
          yesterday_balance: 22435152.4,        # 昨日餘額 (float)
          today_balance: 22434910.4,            # 今日餘額 (float)
          initial_margin: 1114946.0,            # 原始保證金 (float)
          maintenance_margin: 939214.0,         # 維持保證金 (float)
          clearing_margin: 915760.0,            # 結算保證金 (float)
          today_equity: 22694910.4,             # 本日權益 (float)
          ...
          withhold: 126402.0,                   # 委託預扣款 (float)
          available_margin: 21453562.4,         # 可動用保證金 (float)
          risk_index: 0.0,                      # 風險指標 (float)
          disgorgement: 0.0,                    # 追繳金額 (float)
          opt_pnl: -248600.0,                   # 未沖銷選擇權浮動損益 (float)
          opt_value: -193100.0,                 # 選擇權市值 (float)
          opt_long_value: 311900.0,             # 未沖銷選擇權買方市值 (float)
          opt_short_value: 505000.0,            # 未沖銷選擇權賣方市值 (float)
          fut_realized_pnl: 0.0,                # 期貨平倉損益 (float)
          fut_unrealized_pnl: 60700.0,          # 期貨期貨未平倉損益 (float)
          yesterday_equity: 22634452.4,         # 昨日權益 (float)
          buy_lot: 22,                          # 買進口數 (int)
          sell_lot: 7                           # 賣出口數 (int)
      })

```

```js
 {
        date: '2024/04/08',                    // 查詢日期 (string)
        branchNo: '15901',                      // 分公司代號 (string)
        account: '1234567',                     // 帳號 (string)
        currency: 'NTD',                        // 幣別 (string)
        yesterdayBalance: 22435152.4,          // 昨日餘額 (number)
        todayBalance: 22434910.4,              // 今日餘額 (number)
        initialMargin: 1114946.0,              // 原始保證金 (number)
        maintenanceMargin: 939214.0,           // 維持保證金 (number)
        clearingMargin: 915760.0,              // 結算保證金 (number)
        initialMarginAllSingle: 0.0,           // 原始保證金 (number)
        todayEquity: 22694910.4,               // 本日權益 (number)
        ...
        withhold: 126402.0,                    // 委託預扣款 (number)
        availableMargin: 21453562.4,           // 可動用保證金 (number)
        riskIndex: 0.0,                        // 風險指標 (number)
        disgorgement: 0.0,                     // 追繳金額 (number)
        optPnl: -248600.0,                     // 未沖銷選擇權浮動損益 (number)
        optValue: -193100.0,                   // 選擇權市值 (number)
        optLongValue: 311900.0,                // 未沖銷選擇權買方市值 (number)
        optShortValue: 505000.0,               // 未沖銷選擇權賣方市值 (number)
        futRealizedPnl: 0.0,                   // 期貨平倉損益 (number)
        futUnrealizedPnl: 60700.0,             // 期貨未平倉損益 (number)
        yesterdayEquity: 22634452.4,           // 昨日權益 (number)
        buyLot: 22,                            // 買進口數 (number)
        sellLot: 7,                            // 賣出口數 (number)
      }

```

```cs
{
        date = 2024/04/08,                     // 查詢日期 (string)
        branchNo = 15901,                       // 分公司代號 (string)
        account = 1234567,                      // 帳號 (string)
        currency = NTD,                         // 幣別 (string)
        yesterdayBalance = 22435152.4,           // 昨日餘額 (double)
        todayBalance = 22434910.4,               // 今日餘額 (double)
        initialMargin = 1114946.0,               // 原始保證金 (double)
        maintenanceMargin = 939214.0,            // 維持保證金 (double)
        clearingMargin = 915760.0,               // 結算保證金 (double)
        initialMarginAllSingle = 0.0,            // 原始保證金 (double)
        todayEquity = 22694910.4,                // 本日權益 (double)
        ...
        withhold = 126402.0,                     // 委託預扣款 (double)
        availableMargin = 21453562.4,            // 可動用保證金 (double)
        riskIndex = 0.0,                         // 風險指標 (double)
        disgorgement = 0.0,                      // 追繳金額 (double)
        optPnl = -248600.0,                      // 未沖銷選擇權浮動損益 (double)
        optValue = -193100.0,                    // 選擇權市值 (double)
        optLongValue = 311900.0,                 // 未沖銷選擇權買方市值 (double)
        optShortValue = 505000.0,                // 未沖銷選擇權賣方市值 (double)
        futRealizedPnl = 0.0,                    // 期貨平倉損益 (double)
        futUnrealizedPnl = 60700.0,              // 期貨未平倉損益 (double)
        yesterdayEquity = 22634452.4,            // 昨日權益 (double)
        buyLot = 22,                             // 買進口數 (int)
        sellLot = 7,                             // 賣出口數 (int)
}

```


---

### 非阻塞下單

首先，我們先理解阻塞與非阻塞的概念， 阻塞（Block）和非阻塞（Unblock）是用來描述事件、操作或通信方式的兩種不同方式，以下是它們的基本概念和區別：

##### 阻塞（Block）：[​](#阻塞block "Direct link to 阻塞（Block）：")

阻塞操作是指事件或操作按照預定的順序進行，並且一個操作完成後，才會回覆結果。

![sync](/TradeAPI/assets/images/normal-24db51cb3aa3d4c74d40c79d4ab38ac8.png)

##### 非阻塞（Unblock）：[​](#非阻塞unblock "Direct link to 非阻塞（Unblock）：")

非阻塞操作是指事件或操作不必按照固定的順序進行，可以並行執行，且一個操作不需要等待另一個操作的完成。 當您發出Request後就直接回覆，即為非阻塞。

![async](/TradeAPI/assets/images/async-f1632167c6d8f31259864d28445e3ffc.png)

##### 使用阻塞機制下單[​](#使用阻塞機制下單 "Direct link to 使用阻塞機制下單")

委託後，回覆的Order Response即會帶回完整的資料內容。

* Python
* Node.js
* C#

```py
#建立委託單內容
order = FutOptOrder(
    buy_sell = BSAction.Buy,
    symbol = "TXFD4",
    price = "20000",
    lot = 1,
    market_type = FutOptMarketType.Future,
    price_type = FutOptPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = FutOptOrderType.Auto,
    user_def = "FromPy" # optional field
) 

sdk.futopt.place_order(accounts.data[0], order)  #下單委託

#或採用
#sdk.futopt.place_order(accounts.data[0], order, False)

```

```js
const order = {
  buySell: BSAction.Buy,
  symbol: "TXFD4",
  price: "20100",
  lot: 1,
  marketType: FutOptMarketType.Future,
  priceType: FutOptPriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: FutureOrderType.Auto,
  userDef: "fromJs"
};

sdk.futopt.placeOrder(accounts.data[0], order); 

// 或採用
//sdk.futopt.placeOrder(accounts.data[0], order, false);

```

```cs
var order = new FutOptOrder(
    BsAction.Buy,
    "TXFD4",
    null,
    null,
    "20000",
    1,
    FutOptMarketType.Future,
    FutOptPriceType.Limit,
    TimeInForce.Rod,
    FutureOrderType.Auto,
    null
);

sdk.FutOpt.PlaceOrder(accounts.data[0] ,order); // 使用阻塞委託下單
// 或採用
//sdk.futopt.placeOrder(accounts.data[0], order, false);

```

##### 使用非阻塞機制下單[​](#使用非阻塞機制下單 "Direct link to 使用非阻塞機制下單")

委託後，回覆的Order Response可能會少帶出委託書號···等資訊。

* Python
* Node.js
* C#

```py
#建立委託單內容
order = FutOptOrder(
    buy_sell = BSAction.Buy,
    symbol = "TXFD4",
    price = "20000",
    lot = 1,
    market_type = FutOptMarketType.Future,
    price_type = FutOptPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = FutOptOrderType.Auto,
    user_def = "FromPy" # optional field
) 

sdk.futopt.place_order(accounts.data[0], order, True)  #下單委託

```

```js
const order = {
  buySell: BSAction.Buy,
  symbol: "TXFD4",
  price: "20100",
  lot: 1,
  marketType: FutOptMarketType.Future,
  priceType: FutOptPriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: FutureOrderType.Auto,
  userDef: "fromJs"
};

sdk.futopt.placeOrder(accounts.data[0], order, true);

```

```cs
var order = new FutOptOrder(
    BsAction.Buy,
    "TXFD4",
    null,     // 複式單第二隻腳買賣別
    null,     // 複式單第二隻腳商品
    "20000", 
    1,
    FutOptMarketType.Future,
    FutOptPriceType.Limit,
    TimeInForce.Rod,
    FutureOrderType.Auto,
    null
);

sdk.FutOpt.PlaceOrder(accounts.data[0] ,order, true); // 使用非阻塞委託下單



```

以下為支援非阻塞委託的函數

* PlaceOrder - 下單委託
* ModifyPrice - 修改委託價格
* ModifyLot - 修改委託口數
* CancelOrder - 取消委託


---

### Python主動回報例外處理

使用 Python 主動回報時，因為主動回報機制於獨自的 thread 中運行，因此若主動回報 callback function 內自己寫的程式碼執行產生錯誤，不會自動丟回錯誤訊息，可能造成程式 DEBUG 上的障礙。

這個時候可以在 callback function 上面加入以下 decorator，當其中程式碼發生錯誤時，錯誤將如往常一樣顯示在 console 內：

```python
import os
import sys
import traceback
import functools

def handle_exceptions(func):
    @functools.wraps(func)
    def wrapper(*args, **kwargs):
        try:
            return func(*args, **kwargs)
        except Exception as exp:
            # Extract the full traceback
            tb_lines = traceback.format_exc().splitlines()

            # Find the index of the line related to the original function
            func_line_index = next((i for i, line in enumerate(tb_lines) if func.__name__ in line), -1)

            # Highlight the specific part in the traceback where the exception occurred
            relevant_tb = "\n".join(tb_lines[func_line_index:])  # Include traceback from the function name

            error_text = f"{func.__name__} exception: {exp}\nTraceback (most recent call last):\n{relevant_tb}"
            print(error_text, file=sys.stderr)

            # 若要程式完全跳出，可加入下行 (P.S. jupyter 環境不適用)
            # os._exit(-1)

    return wrapper

def on_event(code, content):
    print("===event=====")
    print(code)
    print(content)
    print("========")

@handle_exceptions
def on_order(code, content):
    print("==Order==")
    print(code)
    print(content)
    print("========")

    # Add some code that would cause exceptions
    test = {}
    print(test["key"])  # 這行會產生錯誤


```

以上範例程式碼中，我們故意在 on\_order 中放入一行 `print(test["key"])`，因為 `"key"` 沒有在字典 `test` 裡面，所以會產生錯誤。

這時因為我們有加入 `@handle_exceptions` decorator，所以收到新單主動回報後， console 會印出：

```python
on_order exception: 'key'
Traceback (most recent call last):
  File "xxx.py", line 47, in on_order
    print(test["key"])
          ~~~~^^^^^^^
KeyError: 'key'

```


---

### 斷線重連

以下將用簡單示範，利用callback偵測交易Socket，接收到斷線事件後，程式自動進行Socket重新連線:

* Python
* Node.js
* C#

```py
# A callback to receive event 
def on_event(code, content):
    print("===event=====")
    print(code)
    print(content)
    if code == "300":
        print("Reconnect")
        try:
            accounts = sdk.login("您的身分證字號", "您的登入密碼", "您的憑證路徑", "您的憑證密碼")
            print("Reconnect successs")
        except Exception as e:
            print("Reconnect failed")
            print(e)
    print("========")
    
sdk.set_on_event(on_event) 

```

```js
sdk.setOnEvent(function(order, content) { 
    console.log("===Event===\n",order, content)
    if(content[0] == "300"){
        console.log("Reconnect")
        try{
             accounts = sdk.login("您的身分證字號", "您的登入密碼", "您的憑證路徑", "您的憑證密碼");
             console.log("Reconnect success")
        }catch(e){
            console.log("Reconnect Failed")
        }
    }});

```

```cs
public class MyCallback : Callback
{
    public string response = "";
    private FubonSDK _sdk;

    public MyCallback(ref FubonSDK sdk)
    {
        _sdk = sdk;
    }
    public void OnEvent(string code, string data)
    {

        Console.WriteLine(code);
        Console.WriteLine(data);
        if (code == "300")
        {
            Console.WriteLine("Reconnect");

            try
            {
                _sdk.Login("您的身分證字號", "您的登入密碼", "您的憑證路徑", "您的憑證密碼");
                Console.WriteLine("Reconnect success");
            }
            catch (Exception e)
            {
                Console.WriteLine(e);
                Console.WriteLine("Reconnect Failed");
            }
        }
    }
}


var callback = new MyCallback(ref sdk);
sdk.RegisterCallback(callback);


```


---

### 錯誤碼與狀態碼對照表

本頁面彙整了富邦新一代 API（期貨交易）中常見的狀態碼、功能代碼與事件代碼，方便開發者在遇到問題或進行狀態判斷時快速檢索。

#### 委託單狀態 (Status)[​](#委託單狀態-status "Direct link to 委託單狀態 (Status)")

委託單的當前狀態，可透過 `GetOrderResult` 查詢或由主動回報取得。

| 狀態名稱     | 狀態碼 (Value) | 說明                                                     |
| ------------ | -------------- | -------------------------------------------------------- |
| 預約單       | `0`            | 預約單                                                   |
| 中台收到委託 | `4`            | 請用 `GetOrderResult` 查詢狀態                           |
| 後台連線逾時 | `9`            | 請稍後再使用 `GetOrderResult` 查詢狀態 or 聯絡您的營業員 |
| 委託成功     | `10`           | 委託成功                                                 |
| 刪單成功     | `30`           | 刪單成功                                                 |
| 完全成交     | `50`           | 完全成交                                                 |
| 改價失敗     | `19`           | 改價失敗                                                 |
| 改量失敗     | `29`           | 改量失敗                                                 |
| 刪單失敗     | `39`           | 刪單失敗                                                 |
| 失敗         | `90`           | 失敗                                                     |

#### 功能類別 (Function Type)[​](#功能類別-function-type "Direct link to 功能類別 (Function Type)")

表示該筆回報或查詢結果所屬的操作類型。

| 功能名稱 | 代碼 (Value) | 說明     |
| -------- | ------------ | -------- |
| 新單     | `0`          | 新單     |
| 新單執行 | `10`         | 新單執行 |
| 改價     | `15`         | 改價     |
| 改量     | `20`         | 改量     |
| 刪單     | `30`         | 刪單     |
| 失敗     | `90`         | 失敗     |

#### 事件代碼 (Event Code)[​](#事件代碼-event-code "Direct link to 事件代碼 (Event Code)")

透過 `on_event` 訂閱事件通知時，系統會回傳以下代碼。

| 回傳代碼 | 意義                                               |
| -------- | -------------------------------------------------- |
| `100`    | 連線建立成功                                       |
| `200`    | 登入成功                                           |
| `201`    | 登入警示 (例如：90天未更換密碼)                    |
| `300`    | 斷線                                               |
| `301`    | 未收到連線 pong 回傳                               |
| `302`    | 用戶執行登出，並斷線                               |
| `304`    | API Key 異動 (Revoked)，已強制登出 (2.2.7版本新增) |
| `500`    | 錯誤                                               |


---

### 主動回報

***

<!-- -->

#### 訂閱委託回報[​](#訂閱委託回報 "Direct link to 訂閱委託回報")

* Python
* Node.js
* C#

```py
# A callback to receive order data
def on_order(code, content):
    print("==Order==")
    print(code)
    print(content)
    # print(content.seq_no)  # 印出委託單流水號
    print("========")

sdk.set_on_futopt_order(on_order) 

```

info

詳細回傳內容，可參照[FutOptOrderResult Object](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptorderresult)

```js
//Callback to receive order data
sdk.setOnFutoptOrder(function(code, content) 
    { console.log("====order===\n",code, content)});

```

info

詳細回傳內容，可參照[FutOptOrderResult Object](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptorderresult)

```cs
public void OnFutoptOrder(string code, FutOptOrderResult data)
{
    if(data != null)
    {
        response = data.ToString();
        Console.WriteLine("On Order" + response);
    }

}

或使用下方方法

sdk.OnFutoptOrder += (code, FutOptOrderResult) =>
{
    Console.WriteLine(code + FutOptOrderResult.ToString());
}

```

info

詳細回傳內容，可參照[FutOptOrderResult Object](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptorderresult)

#### 訂閱改價/改量/刪單回報[​](#訂閱改價改量刪單回報 "Direct link to 訂閱改價/改量/刪單回報")

* Python
* Node.js
* C#

```py
def on_order_changed(code, content):
    print("=Modified==")
    print(code)
    print(content)
    # print(content.seq_no)  # 印出委託單流水號
    print("========")
    
    
sdk.set_on_order_futopt_changed(on_order_changed) 

```

info

詳細回傳內容，可參照[FutOptOrderResult Object](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptorderresult)

```js
//Callback to receive Modified data
sdk.setOnFutoptOrderChanged(function(order, content) 
    { console.log("===Modified===\n", order, content)});

```

info

詳細回傳內容，可參照[FutOptOrderResult Object](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptorderresult)

```cs
public void OnFutoptOrderChanged(string code, FutOptOrderResult data)
{
    if(data != null)
    {
        response = data.ToString();

        Console.WriteLine(code);
        Console.WriteLine("Modified" + response);
    }

}

或使用下方方法

sdk.OnFutoptOrderChanged += (code, FutOptOrderResult) =>
{
    Console.WriteLine(code + FutOptOrderResult.ToString());
}


```

info

詳細回傳內容，可參照[FutOptOrderResult Object](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptorderresult)

#### 訂閱成交回報[​](#訂閱成交回報 "Direct link to 訂閱成交回報")

* Python
* Node.js
* C#

```py
def on_filled(code, content):
    print("==Filled==")
    print(code)
    print(content)
    # print(content.filled_no)  # 印出成交流水號
    print("========")
    
sdk.set_on_futopt_filled(on_filled)

```

info

詳細回傳內容，可參照[FutOptFilledData Object](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptfilleddata)

```js
sdk.setOnFutoptFilled(function(code, content) 
    { console.log("===Filled===\n",code, content)})

```

info

詳細回傳內容，可參照[FutOptFilledData Object](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptfilleddata)

```cs
public void OnFutoptFilled(string code, FutOptFilledData data)
{
    if(data != null)
    {
        response = data.ToString();

        Console.WriteLine(code);
        Console.WriteLine("Filled" + response);
    }
}

或使用下方方法

sdk.OnFutoptFilled += (code, FutOptFilledData) =>
{
    Console.WriteLine(code + FutOptFilledData.ToString());
}


```

info

詳細回傳內容，可參照[FutOptFilledData Object](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptfilleddata)

#### 訂閱事件通知[​](#訂閱事件通知 "Direct link to 訂閱事件通知")

* Python
* Node.js
* C#

```py
def on_event(code, content):
    print("===event=====")
    print(code)
    print(content)
    print("========")
    
sdk.set_on_event(on_event) 

```

```js
sdk.setOnEvent(function(code, content) 
    { console.log("===Event===\n",code, content)})

```

```cs
public void OnEvent(String code, String data)
{
    if(data != null)
    {
        response = data.ToString();
        Console.WriteLine(code);
        Console.WriteLine("Event" + response);
    }

}

或使用下方方法

sdk.OnEvent += (code, msg) =>
{
    Console.WriteLine(code + msg );
};


```

事件包含以下情況回傳

| 回傳代碼 | 意義                                               |
| -------- | -------------------------------------------------- |
| 100      | 連線建立成功                                       |
| 200      | 登入成功                                           |
| 201      | 登入警示 , Ex : 90天未更換密碼                     |
| 300      | 斷線                                               |
| 301      | 未收到連線pong回傳                                 |
| 302      | 登出 , 並斷線                                      |
| 304      | API Key 異動 (Revoked), 已強制登出 (2.2.7版本新增) |
| 500      | 錯誤                                               |

#### 訂閱範例[​](#訂閱範例 "Direct link to 訂閱範例")

使用者可訂閱不同的callback，來接收系統主動發送的委託及成交通知。

* Python
* Node.js
* C#

```py
# A callback to receive order data
def on_order(code, content):
    print("==Order==")
    print(code)
    print(content)
    print("========")
    
    
sdk.set_on_futopt_order(on_order) 


# A callback to receive Modified data
def on_order_changed(code, content):
    print("=Modified==")
    print(code)
    print(content)
    print("========")
    
    
sdk.set_on_order_futopt_changed(on_order_changed) 

def on_filled(code, content):
    print("==Filled==")
    print(code)
    print(content)
    print("========")
    
sdk.set_on_futopt_filled(on_filled)

# A callback to receive Event data
def on_event(code, content):
    print("===event=====")
    print(code)
    print(content)
    print("========")
    
sdk.set_on_event(on_event) 

```

info

詳細回傳內容，可參照[SDK Reference 參數對照表](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptorderresult)

```js
//Callback to receive order data
sdk.setOnFutoptOrder(function(code, content) 
    { console.log("====order===\n",code, content)});

//Callback to receive Modified data
sdk.setOnFutoptOrderChanged(function(code, content) 
    { console.log("===Modified===\n", code, content)});

//Callback to receive Filled data
sdk.setOnFutoptFilled(function(code, content) 
    { console.log("===Filled===\n",code, content)})

//Callback to receive Event data
sdk.setOnEvent(function(code, content) 
    { console.log("===Event===\n",code, content)})

```

info

詳細回傳內容，可參照[SDK Reference 參數對照表](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptorderresult)

```cs
public class MyCallback : Callback
{
    public string code ="";
    public string response = "";

    //Callback to receive order data
    public void OnFutoptOrder(string code, FutOptOrderResult data)
    {
        if(data != null)
        {
            response = data.ToString();
            Console.WriteLine("On Order" + response);
        }

    }

    //Callback to receive Modified data
    public void OnFutoptOrderChanged(string code, FutOptOrderResult data)
    {
        if(data != null)
        {
            response = data.ToString();

            Console.WriteLine(code);
            Console.WriteLine("Modified" + response);
        }

    }

    //Callback to receive Filled data
    public void OnFutoptFilled(string code, FutOptFilledData data)
    {
        if(data != null)
        {
            response = data.ToString();

            Console.WriteLine(code);
            Console.WriteLine("Filled" + response);
        }

    }

        //Callback to receive order data
    public void OnOrder(string code, OrderResult data)
    {
        if(data != null)
        {
            response = data.ToString();
            Console.WriteLine("On Order" + response);
        }

    }

    //Callback to receive Modified data
    public void OnOrderChanged(string code, OrderResult data)
    {
        if(data != null)
        {
            response = data.ToString();

            Console.WriteLine(code);
            Console.WriteLine("Modified" + response);
        }

    }

    //Callback to receive Filled data
    public void OnFilled(string code, FilledData data)
    {
        if(data != null)
        {
            response = data.ToString();

            Console.WriteLine(code);
            Console.WriteLine("Filled" + response);
        }

    }

    //Callback to receive Event data
    public void OnEvent(String code, String data)
    {
        response = data.ToString();
        Console.WriteLine(code);
        Console.WriteLine("Event" + response);
    }
}

var callback = new MyCallback();
sdk.RegisterCallback(callback);

或使用下方方法分別訂閱

sdk.OnEvent += (code, msg) =>
{
    Console.WriteLine(code + msg );
}

sdk.OnFutoptOrder += (code, FutOptOrderResult) =>
{
    Console.WriteLine(code + FutOptOrderResult.ToString());
}

sdk.OnFutoptOrderChanged += (code, FutOptOrderResult) =>
{
    Console.WriteLine(code + FutOptOrderResult.ToString());
}
sdk.OnFutoptFilled += (code, FutOptFilledData) =>
{
    Console.WriteLine(code + FutOptFilledData.ToString());
}

```

info

詳細回傳內容，可參照[SDK Reference 參數對照表](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptorderresult)


---

### 交易

***

<!-- -->

此篇教學將示範如何進行一個完整的買賣流程

#### 買入部位[​](#買入部位 "Direct link to 買入部位")

假設今天開盤後，我們想買進2口台指期20000點，我們可以這樣撰寫程式並執行：

* Python
* Node.js
* C#

```python
from fubon_neo.sdk import FubonSDK, FutOptOrder
from fubon_neo.constant import TimeInForce, FutOptOrderType, FutOptPriceType, FutOptMarketType, BSAction

sdk = FubonSDK()
   
accounts = sdk.login("您的身分證字號", "您的登入密碼", "您的憑證位置", "您的憑證密碼") #若有歸戶，則會回傳多筆帳號資訊
  
#建立委託單內容
order = FutOptOrder(
    buy_sell = BSAction.Buy,
    symbol = "TXFD4",
    price = "20000",
    lot =  2,
    market_type = FutOptMarketType.Future,
    price_type = FutOptPriceType.Limit,
    time_in_force= TimeInForce.ROD,
    order_type = FutOptOrderType.Auto,
    user_def = "FromPy" # optional field
) 


sdk.futopt.place_order(accounts.data[0], order)  #下單委託

```

```js
const { FubonSDK, BSAction, TimeInForce, FutOptMarketType, FutOptPriceType, FutOptOrderType } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("您的身分證字號", "您的登入密碼", "您的憑證路徑" ,"您的憑證密碼");



const order = {
  buySell: BSAction.Buy,
  symbol: "TXFD4",
  price: "20000",
  lot: 2,
  marketType: FutOptMarketType.Future,
  priceType: FutOptPriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: FutOptOrderType.Auto,
  userDef: "fromJs"
};

sdk.futopt.placeOrder(accounts.data[0],order);

```

```cs

    
using FubonNeo.Sdk;

var sdk = new FubonSDK();

var accounts = sdk.Login("您的身分證字號", "您的登入密碼", "您的憑證路徑", "您的憑證密碼"); // 若有歸戶，則會回傳多筆帳號資訊

var order = new FutOptOrder(
    BsAction.Buy,
    "TXFD4",
    null,     // 複式單第二隻腳買賣別
    null,     // 複式單第二隻腳商品
    "20000",
    2,
    FutOptMarketType.Future,
    FutOptPriceType.Limit,
    TimeInForce.Rod,
    FutOptOrderType.Auto,
    null
);

sdk.FutOpt.PlaceOrder(accounts.data[0],order); 

```

#### 確認委託與成交回報[​](#確認委託與成交回報 "Direct link to 確認委託與成交回報")

若要確定該筆的狀態，可以依照下方範例查詢指定的委託單 ( 需指定查詢的市場別 : 期貨, 選擇權, 期貨夜盤, 選擇權夜盤 ):

* Python
* Node.js
* C#

```python
orderResults = sdk.futopt.get_order_results(accounts.data[0])
print(orderResults)

```

```js
const orderResults = sdk.futopt.getOrderResults(accounts.data[0])
console.log(orderResults)

```

```cs
var orderResults = sdk.FutOpt.GetOrderResults(accounts.data[0]);
Console.WriteLine(orderResults);
foreach (var results in orderResults)
{
  Console.WriteLine(results);
}

```

根據回報的結果，我們可以判斷此筆委託是否成交，成交了多少數量：

* Python
* Node.js
* C#

```py
Result {  
  is_success: true,   
  message: None,  
  data : [
      FutOptOrderResult{
        ...
        buy_sell: Buy,      #買賣別 (BSAction)
        price: 20000,       #原始委託價格 (float)
        lot: 2,             #原始委託數量 (int)
        after_price: 20000, #有效委託價格 (float)
        after_lot: 2,       #有效委託數量 (int)
        filled_lot: 0,      #已成交數量 (int)
        filled_money: 0,    #成交價金 (float)
        symbol: "FITX",     #商品代號 (string)
        order_no: "bA888",  #委託書號 (string)
        last_time: "10:10:10.123", #最後異動時間 (string)
        ...
      }
   ]
}


```

```js
{ 
  isSuccess: true,  
  data:[
    {
      ...
      buySell: 'Buy',      //買賣別 (string)
      price: 20000,        //原始委託價格 (number)
      lot: 2,              //原始委託數量 (number)
      afterPrice: 20000,   //有效委託價格 (number)
      afterLot: 2,         //有效委託數量 (number)
      filledLot: 0,        //已成交數量 (number)
      filledMoney: 0,      //成交價金 (number)
      symbol: 'FITX',      //商品代號 (string)
      orderNo: 'bA888',    //委託書號 (string)
      lastTime: '10:10:10.123', //最後異動時間 (string)
      ...
    }
  ]
}


```

```cs
{ 
  isSuccess = True, 
  message = , 
  data = [
      FutOptOrderResult{
        ...
        buySell = Buy,       //買賣別 (BsAction)
        price = 20000,       //原始委託價格 (double)
        lot = 2,             //原始委託數量 (int)
        afterPrice = 20000,  //有效委託價格 (double)
        afterLot = 2,        //有效委託數量 (int)
        filledLot = 0,       //已成交數量 (int)
        filledMoney = 0,     //成交價金 (double)
        symbol = FITX,       //商品代號 (string)
        orderNo = bA888,     //委託書號 (string)
        lastTime = 10:10:10.123, //最後異動時間 (string)
        ...
      }
   ]
}


```

#### 修改委託價格[​](#修改委託價格 "Direct link to 修改委託價格")

由於原先的價格一直無法成交，我們調整原先的委託價格，改用20100的點數價格買入：

* Python
* Node.js
* C#

```python
orderResults = sdk.futopt.get_order_results(accounts.data[0])

modified_pirce = sdk.futopt.make_modify_price_obj(orderResults.data[0],"20100")
sdk.futopt.modify_price(accounts.data[0], modified_pirce)

```

```js
orderResults = sdk.futopt.getOrderResults(accounts.data[0])

const modified_pirce = sdk.futopt.makeModifyPriceObj(orderResults.data[0],"20100")
sdk.futopt.modifyPrice(accounts.data[0],modified_pirce)

```

```cs
orderResults = sdk.FutOpt.GetOrderResults(accounts.data[0]);

var modified_pirce =  sdk.FutOpt.MakeModifyPriceObj(orderResults.data[0],"20100",null);  //將價格調整成20100點
sdk.FutOpt.ModifyPrice(accounts.data[0],modified_pirce);  

```

幾分鐘後，我們再查詢一次委託狀態，發現成交了：

* Python
* Node.js
* C#

```python
orderResults = sdk.futopt.get_order_results(accounts.data[0])
print(orderResults.data[0])

```

```py
[
  {
    ...
    buy_sell: Buy,        #買賣別 (BSAction)
    price: 20000,         #原始委託價格 (float)
    lot: 2,               #原始委託數量 (int)
    after_price: 20100,   #有效委託價格 (float)
    after_lot: 2,         #有效委託數量 (int)
    filled_lot: 1,        #已成交數量 (int)
    filled_money: 0,      #成交價金 (float)
    symbol: "FITX",       #商品代號 (string)
    order_no: "bA888",    #委託書號 (string)
    last_time: "10:13:12.123", #最後異動時間 (string)
    ...
  }
]

```

```js
orderResults = sdk.futopt.getOrderResults(accounts.data[0])
console.log(orderResults.data[0])

```

```json
{
    ...
    "buySell" : "Buy",       //買賣別 (string)
    "price" : 20000,         //原始委託價格 (number)
    "lot" :  2,              //原始委託數量 (number)
    "afterPrice" : 20100,    //有效委託價格 (number)
    "afterLot" :  2,         //有效委託數量 (number)
    "filledQty" : 1,         //已成交數量 (number)
    "filledMoney" : 20100,   //成交價金 (number)
    "futoptNo" : "FITX ",    //商品代號 (string)
    "orderNo" : "bA888",     //委託書號 (string)
    "lastTime" : "10:13:12.123", //最後異動時間 (string)
    ...
}

```

```cs
orderResults = sdk.FutOpt.GetOrderResults(accounts.data[0]);
Console.WriteLine(orderResults.data[0]);

```

```cs
{
    ...
    buySell = Buy,       //買賣別 (BsAction)
    price = 66,          //原始委託價格 (double)
    lot =  2,            //原始委託數量 (int)
    afterPrice = 20100,  //有效委託價格 (double)
    afterLot =  1,       //有效委託數量 (int)
    filledLot = 1,    //已成交數量 (int)
    filledMoney = 20100, //成交價金 (double)
    symbol = "FITX",     //商品代號 (string)
    orderNo = "bA888",   //委託書號 (string)
    lastTime = "10:13:12.123", //最後異動時間 (string)
    ...
}

```

#### 賣出部位[​](#賣出部位 "Direct link to 賣出部位")

最後決定在收盤前，賣出一口：

* Python
* Node.js
* C#

```python
#建立委託單內容
order = Order(
    buy_sell = BSAction.Sell,
    symbol = "TXFD4",
    price = "20100",
    lot =  1,
    market_type = FutOptMarketType.Future,
    price_type = FutOptPriceType.Limit,
    time_in_force= TimeInForce.ROD,
    order_type = FutOptOrderType.Auto,
    user_def = "FromPy" # optional field
) 


sdk.futopt.place_order(accounts.data[0], order)  #下單委託

```

```js
order = {
  buySell: BSAction.Sell,
  symbol: "TXFD4",
  price: "20100",
  lot: 1,
  marketType: FutOptMarketType.Future,
  priceType: FuturePriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: FutureOrderType.Auto,
  userDef: "fromJs"
};

sdk.futopt.placeOrder(accounts.data[0],order)

```

```cs
order = new Order(
        BsAction.Sell,
        "TXFD4",
        null,     // 複式單第二隻腳買賣別
        null,     // 複式單第二隻腳商品
        "20100",
         1,
        FutOptMarketType.Future,
        FutOptPriceType.Limit,
        TimeInForce.Rod,
        FutOptOrderType.Auto,
        null
);

sdk.FutOpt.PlaceOrder(accounts.data[0],order); 

```


---

### 富邦新一代 API

***

本頁重點

* 本頁介紹富邦新一代 API 期貨交易與行情服務範圍。
* 支援多語言與跨平台環境，適用期貨程式交易開發。
* 下一步建議先完成[事前準備](https://www.fbs.com.tw/TradeAPI/docs/trading-future/prepare.txt)。

| 項目 | 說明                                                                       |
| ---- | -------------------------------------------------------------------------- |
| 產品 | 富邦新一代 API（Fubon Neo API）                                            |
| 服務 | 期貨交易 / 行情                                                            |
| 平台 | Windows / macOS / Linux                                                    |
| 語言 | Python / C# / JavaScript (Node.js) / C++、Go（僅支援證券交易帳務及條件單） |
| 登入 | 憑證登入（可搭配 API Key）                                                 |

#### 概述[​](#概述 "Direct link to 概述")

富邦新一代 API 為您的程式交易，提供了完善的交易與行情 API，滿足每一位開發者的量化與自動化交易需求。<br /><!-- -->提供用戶自由選擇主流開發語言( Python、C#、JavaScript)，在創建程式交易的過程中，輕易地取用 API，使用交易、行情服務。

#### 特色[​](#特色 "Direct link to 特色")

支援跨平台：<br /><!-- -->新一代 API 支援 Windows、MacOS、Linux。<br /><!-- -->新一代 API 支援 Python、C#、JavaScript（Node.js）、C++、Go 主流語言（C++、Go 僅支援證券交易帳務及條件單）。<br /><!-- -->穩定的技術架構，直連交易所一觸即達。<br /><!-- -->客製化的行情解决方案。

#### 主要功能[​](#主要功能 "Direct link to 主要功能")

直接管理交易：建立單式委託單、複式委託單，修改或取消委託單，以及查詢委託單狀態，取得歷史委託紀錄和成交明细···等。<br /><!-- -->查看帳戶資訊：取得帳戶內的部位損益、權益數···等。<br /><!-- -->接收即時行情：股票、權證、期權價格，等多種的行情資訊。

#### 版本支援[​](#版本支援 "Direct link to 版本支援")

Python 版本支援：3.7（~v1.3.2）、3.8–3.13（v2.0.1~，不含 3.14）。<br /><!-- -->Node.js 版本支援 : 目前支援Node.js 16以上的版本。<br /><!-- -->C# 使用.NET Standard 2.0開發，建議您使用 .netcoreapp 3.1 以上；如使用.NETFramework 建議您使用.NETFramework 4.7.2以上版本。


---

### 平倉查詢

ClosePositionRecord

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數      | 類別                                                                                                | 說明       |
| --------- | --------------------------------------------------------------------------------------------------- | ---------- |
| account   | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account) | 帳號       |
| startDate | string                                                                                              | 查詢開始日 |
| endDate   | string (空值預設與開始日相同)                                                                       | 查詢終止日 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別   | 說明                             |
| --------- | ------ | -------------------------------- |
| isSuccess | bool   | 是否成功                         |
| data      | List   | 回傳已實現彙總資訊               |
| message   | string | 當isSuccess = False 回傳錯誤訊息 |

##### 平倉 CloseRecord 欄位[​](#平倉-closerecord-欄位 "Direct link to 平倉 CloseRecord 欄位")

Return type = Object

| 參數           | 類別                                                                                                                | 說明                                                                       |
| -------------- | ------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| date           | string                                                                                                              | 資料日期                                                                   |
| branchNo       | string                                                                                                              | 分公司代號                                                                 |
| positionKind   | int                                                                                                                 | 部位種類 : `1` 期貨 、`2` 選擇權                                           |
| account        | string                                                                                                              | 帳號                                                                       |
| orderNo        | string                                                                                                              | 委託書號                                                                   |
| market         | string                                                                                                              | 市場別 : `TAIMEX` 期貨、選擇權                                             |
| symbol         | string                                                                                                              | 商品代號                                                                   |
| expiryDate     | string                                                                                                              | 履約日                                                                     |
| strikePrice    | double                                                                                                              | 履約價                                                                     |
| callPut        | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                 | 權利別 : `Call` Call 、`Put` Put                                           |
| buySell        | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)               | 買賣別 : `Buy` 買、`Sell` 賣                                               |
| orderType      | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptordertype) | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖 |
| price          | double                                                                                                              | 成交價                                                                     |
| origLots       | int                                                                                                                 | 原始口數                                                                   |
| transactionFee | double                                                                                                              | 交易手續費                                                                 |
| tax            | double                                                                                                              | 交易稅                                                                     |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
Console.WriteLine(sdk.FutOptAccounting.ClosePositionRecord(accounts,20240310,20240410));

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess = True,
    message = ,
    data = [
           CloseRecord {
                    date = 2024/04/10,              // 資料日期(string)
                    branchNo = 15000,               // 分公司代號(string)
                    account = 9974825,              // 帳號 (string)
                    positionKind = 1,               // 部位種類  `1` 期貨, `2` 選擇權 (int)
                    orderNo = 15001-0000,           // 委託書號 (string)
                    market = TAIMEX,                // 市場別   `TAIMEX` (string)
                    symbol = FITX,                  // 商品代號 (string)
                    expiryDate = 202404,            // 履約日 (string)
                    strikePrice = ,                 // 履約價 (double)
                    callPut = ,                     // Call/Put   Call、 Put (CallPut)
                    buySell = Buy,                  // Buy/Sell   Buy 、 Sell(BsAction)
                    price = 20847.0,                // 成交價 (double)
                    origLots = 1,                   // 原始口數 (int)
                    transactionFee = 40.0,          // 交易手續費 (double)
                    tax = 83.0,                     // 交易稅 (double)
                },
                CloseRecord {
                    date = 2024/04/10,              // 資料日期 (string)
                    branchNo = 15000,               // 分公司代號 (string)
                    account = 9974825,              // 帳號 (string)
                    positionKind = 1,               // 部位種類   `1` 期貨, `2` 選擇權 (int)
                    orderNo = C0005-0000,           // 委託書號 (string)
                    market = TAIMEX,                // 市場別  期貨, (string)
                    symbol = FITX,                  // 商品代號 (string)
                    expiryDate = 202405,            // 履約日 (string)
                    strikePrice = ,                 // 履約價 (double)
                    callPut = ,                     // Call/Put  Call, Put (CallPut)
                    buySell = Buy,                  // Buy/Sell  Buy, Sell (BsAction)
                    price = 20890.0,                // 成交價 (double)
                    origLots = 1,                   // 原始口數 (int)
                    transactionFee = 40.0,          // 交易手續費 (double)
                    tax = 84.0,                     // 交易稅 (double)
                }
         ]
}

```


---

### 混合部位查詢

QueryHybridPPosition

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                                | 說明 |
| ------- | --------------------------------------------------------------------------------------------------- | ---- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account) | 帳號 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別   | 說明                             |
| --------- | ------ | -------------------------------- |
| isSuccess | bool   | 是否成功                         |
| data      | List   | 回傳部位資訊                     |
| message   | string | 當isSuccess = False 回傳錯誤訊息 |

##### 部位 HybridPosition 欄位[​](#部位-hybridposition-欄位 "Direct link to 部位 HybridPosition 欄位")

Return type : Object

| 參數                 | 類別                                                                                                                | 說明                                              |
| -------------------- | ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| > date               | string                                                                                                              | 部位建立日期                                      |
| > branchNo           | string                                                                                                              | 分公司代號                                        |
| > account            | string                                                                                                              | 帳號                                              |
| > isSpread           | bool                                                                                                                | 是否為複式部位                                    |
| > positionKind       | int                                                                                                                 | 部位種類 : `1` 期貨 、`2` 選擇權                  |
| > symbol             | string                                                                                                              | 商品代號                                          |
| > expiryDate         | string                                                                                                              | 履約日                                            |
| > strikePrice        | double                                                                                                              | 履約價                                            |
| > callPut            | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                 | 權利別 : `Call` Call 、`Put` Put                  |
| > buySell            | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)               | 買賣別 : `Buy` 買、`Sell` 賣                      |
| > price              | double                                                                                                              | 成交價                                            |
| > origLots           | int                                                                                                                 | 原始口數                                          |
| > tradableLots       | int                                                                                                                 | 可交易口數                                        |
| > orderType          | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptordertype) | 委託別 : `New` 新倉、`Close`平倉、`FdayTrade`當沖 |
| > currency           | string                                                                                                              | 幣別                                              |
| > marketPrice        | string                                                                                                              | 即時價                                            |
| > initialMargin      | double                                                                                                              | 原始保證金                                        |
| > maintenanceMargin  | double                                                                                                              | 維持保證金                                        |
| > clearingMargin     | double                                                                                                              | 結算保證金                                        |
| > optValue           | double                                                                                                              | 選擇權市值                                        |
| > optLongValue       | double                                                                                                              | 選擇權買進市值                                    |
| > optShortValue      | double                                                                                                              | 選擇權賣出市值                                    |
| > profitOrLoss       | double                                                                                                              | 部位損益                                          |
| > premium            | double                                                                                                              | 權利金                                            |
| >> spread            | object                                                                                                              | 複式部位解析                                      |
| >> date              | string                                                                                                              | 部位建立日期                                      |
| >> branchNo          | string                                                                                                              | 分公司代號                                        |
| >> account           | string                                                                                                              | 帳號                                              |
| >> isSpread          | bool                                                                                                                | 是否為複式部位                                    |
| >> positionKind      | int                                                                                                                 | 部位種類 : `1` 期貨 、`2` 選擇權                  |
| >> symbol            | string                                                                                                              | 商品代號                                          |
| >> expiryDate        | string                                                                                                              | 履約日                                            |
| >> strikePrice       | double                                                                                                              | 履約價                                            |
| >> callPut           | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                 | 權利別 : `Call` Call 、`Put` Put                  |
| >> buySell           | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)               | 買賣別 : `Buy` 買、`Sell` 賣                      |
| >> price             | double                                                                                                              | 成交價                                            |
| >> origLots          | int                                                                                                                 | 原始口數                                          |
| >> tradableLots      | int                                                                                                                 | 可交易口數                                        |
| >> orderType         | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptordertype) | 委託別 : `New` 新倉、`Close`平倉、`FdayTrade`當沖 |
| >> currency          | string                                                                                                              | 幣別                                              |
| >> marketPrice       | string                                                                                                              | 即時價                                            |
| >> initialMargin     | double                                                                                                              | 原始保證金                                        |
| >> maintenanceMargin | double                                                                                                              | 維持保證金                                        |
| >> clearingMargin    | double                                                                                                              | 結算保證金                                        |
| >> optValue          | double                                                                                                              | 選擇權市值                                        |
| >> optLongValue      | double                                                                                                              | 選擇權買進市值                                    |
| >> optShortValue     | double                                                                                                              | 選擇權賣出市值                                    |
| >> profitOrLoss      | double                                                                                                              | 部位損益                                          |
| >> premium           | double                                                                                                              | 權利金                                            |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
Console.WriteLine(sdk.FutOptAccounting.QueryHybridPosition(account));

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess  = True,
    message  = ,
    data  = [
          HybridPosition {
              date = 2024/04/08,                       // 部位建立日期 (string)
              branchNo = 15901,                         // 分公司代號 (string)
              account = 1234567,                        // 帳號 (string)
              isSpread = false,                         // 是否為複式部位 (boolean)
              positionKind = 1,                         // 部位種類 : `1` 期貨、`2` 選擇權 (int)
              symbol = FITX,                            // 商品代號 (string)
              expiryDate = 202404,                      // 履約日 (string)
              strikePrice = ,                           // 履約價 (double)
              callPut = ,                               // 權利別 (CallPut)
              buySell = Buy,                            // 買賣別 (BsAction)
              price = 20325.3333,                       // 成交價 (double)
              origLots = 3,                             // 原始口數 (int)
              tradableLot = 3,                          // 可交易口數 (int)
              orderType = New,                          // 委託別 (FutOptOrderType)
              currency = TWD,                           // 幣別 (string)
              marketPrice = 20351,                      // 即時價 (string)
              initialMargin = 0.0,                      // 原始保證金 (double)
              maintenanceMargin = 0.0,                  // 維持保證金 (double)
              clearingMargin = 0.0,                     // 結算保證金 (double)
              initialMarginAllSingle = 0.0,             // 原始保證金 (double)
              optValue = 0.0,                           // 選擇權市值 (double)
              optLongValue = 0.0,                       // 選擇權買進市值 (double)
              optShortValue = 0.0,                      // 選擇權賣出市值 (double)
              profitOrLoss = 0.0,                       // 部位損益 (double)
              premium = 0.0,                            // 權利金 (double)
              spreads = ,                               // 複式部位 (SpreadPosition[])
          },
          HybridPosition {
              date = 2024/04/08,                       // 部位建立日期 (string)
              branchNo = 15901,                         // 分公司代號 (string)
              account = 1234567,                        // 帳號 (string)
              isSpread = false,                         // 是否為複式部位 (boolean)
              positionKind = 2,                         // 部位種類 : `1` 期貨、`2` 選擇權 (int)
              symbol = TX1,                             // 商品代號 (string)
              expiryDate = 202404,                      // 履約日 (string)
              strikePrice = 20600,                      // 履約價 (double)
              callPut = Call,                           // 權利別 (CallPut)
              buySell = Buy,                            // 買賣別 (BsAction)
              price = 10,                               // 成交價 (double)
              origLots = 4,                             // 原始口數 (int)
              tradableLot = 4,                          // 可交易口數 (int)
              orderType = New,                          // 委託別 (FutOptOrderType)
              currency = TWD,                           // 幣別 (string)
              marketPrice = 4.6,                        // 即時價 (string)
              initialMargin = 0.0,                      // 原始保證金 (double)
              maintenanceMargin = 0.0,                  // 維持保證金 (double)
              clearingMargin = 0.0,                     // 結算保證金 (double)
              initialMarginAllSingle = 0.0,             // 原始保證金 (double)
              optValue = 920.0,                         // 選擇權市值 (double)
              optLongValue = 920.0,                     // 選擇權買進市值 (double)
              optShortValue = 0.0,                      // 選擇權賣出市值 (double)
              profitOrLoss = -1080.0,                   // 部位損益 (double)
              premium = 0.0,                            // 權利金 (double)
              spreads = ,                               // 複式部位 (SpreadPosition[])
          },
          HybridPosition {
              date = 2024/04/08,                        // 部位建立日期 (string)
              branchNo = 15901,                         // 分公司代號 (string)
              account = 1234567,                        // 帳號 (string)
              isSpread = false,                         // 是否為複式部位 (boolean)
              positionKind = 2,                         // 部位種類 : `1` 期貨、`2` 選擇權 (int)
              symbol = TXO,                             // 商品代號 (string)
              expiryDate = 202404,                      // 履約日 (string)
              strikePrice = 20000,                     // 履約價 (double)
              callPut = Call,                           // 權利別 (CallPut)
              buySell = Buy,                            // 買賣別 (BsAction)
              price = 500,                              // 成交價 (double)
              origLots = 2,                             // 原始口數 (int)
              tradableLot = 2,                          // 可交易口數 (int)
              orderType = New,                          // 委託別 (FutOptOrderType)
              currency = TWD,                           // 幣別 (string)
              marketPrice = 430,                        // 即時價 (string)
              initialMargin = 0.0,                      // 原始保證金 (double)
              maintenanceMargin = 0.0,                  // 維持保證金 (double)
              clearingMargin = 0.0,                     // 結算保證金 (double)
              initialMarginAllSingle = 0.0,             // 原始保證金 (double)
              optValue = 43000.0,                       // 選擇權市值 (double)
              optLongValue = 43000.0,                   // 選擇權買進市值 (double)
              optShortValue = 0.0,                      // 選擇權賣出市值 (double)
              profitOrLoss = -7000.0,                   // 部位損益 (double)
              premium = 0.0,                            // 權利金 (double)
              spreads = ,                           // 複式部位 (SpreadPosition[])
          },
          HybridPosition {
              date = 2024/04/08,                       // 部位建立日期 (string)
              branchNo = 15901,                         // 分公司代號 (string)
              account = 1234567,                        // 帳號 (string)
              isSpread = false,                         // 是否為複式部位 (boolean)
              positionKind = 2,                         // 部位種類 (int)
              symbol = TXO,                             // 商品代號 (string)
              expiryDate = 202404,                      // 履約日 (string)
              strikePrice = 20000,                     // 履約價 (double)
              callPut = Put,                            // 權利別 (CallPut)
              buySell = Buy,                            // 買賣別 (BsAction)
              price = 344,                              // 成交價 (double)
              origLots = 2,                             // 原始口數 (int)
              tradableLot = 2,                          // 可交易口數 (int)
              orderType = New,                          // 委託別 (FutOptOrderType)
              currency = TWD,                           // 幣別 (string)
              marketPrice = 82,                         // 即時價 (string)
              initialMargin = 0.0,                      // 原始保證金 (double)
              maintenanceMargin = 0.0,                  // 維持保證金 (double)
              clearingMargin = 0.0,                     // 結算保證金 (double)
              initialMarginAllSingle = 0.0,             // 原始保證金 (double)
              optValue = 8200.0,                        // 選擇權市值 (double)
              optLongValue = 8200.0,                    // 選擇權買進市值 (double)
              optShortValue = 0.0,                      // 選擇權賣出市值 (double)
              profitOrLoss = -26200.0,                  // 部位損益 (double)
              premium = 0.0,                            // 權利金 (double)
              spreads = ,                                // 複式部位 (SpreadPosition[])
          },
          HybridPosition {
              date = 2024/04/08,                       // 部位建立日期 (string)
              branchNo = 15901,                         // 分公司代號 (string)
              account = 1234567,                        // 帳號 (string)
              isSpread = true,                          // 是否為複式部位 (boolean)
              positionKind = 2,                         // 部位種類 (int)
              symbol = TXO20100D4:20000P4,              // 商品代號 (string)
              expiryDate = 1,                           // 履約日 (string)
              strikePrice = 1,                          // 履約價 (double)
              callPut = ,                           // 權利別 (CallPut)
              buySell = Buy,                            // 買賣別 (BsAction)
              price = ,                             // 成交價 (double)
              origLots = 2,                             // 原始口數 (int)
              tradableLot = 2,                          // 可交易口數 (int)
              orderType = New,                          // 委託別 (FutOptOrderType)
              currency = TWD,                           // 幣別 (string)
              marketPrice = 0.0,                        // 即時價 (string)
              initialMargin = 0.0,                      // 原始保證金 (double)
              maintenanceMargin = 0.0,                  // 維持保證金 (double)
              clearingMargin = 0.0,                     // 結算保證金 (double)
              initialMarginAllSingle = 0.0,             // 原始保證金 (double)
              optValue = 0.0,                           // 選擇權市值 (double)
              optLongValue = 0.0,                       // 選擇權買進市值 (double)
              optShortValue = 0.0,                      // 選擇權賣出市值 (double)
              profitOrLoss = 0.0,                       // 部位損益 (double)
              premium = 0.0,                            // 權利金 (double)
              spreads = [                               // 複式部位 (SpreadPosition[])
                  SpreadPosition {
                      date = 2024/04/08,             // 部位建立日期 (string)
                      branchNo = 15901,               // 分公司代號 (string)
                      account = 1234567,              // 帳號 (string)
                      positionKind = 2,                // 部位種類 (int)
                      symbol = TXO,                    // 商品代號 (string)
                      expiryDate = 202404,             // 履約日 (string)
                      strikePrice = 20100,            // 履約價 (double)
                      callPut = Call,                  // 權利別 (CallPut)
                      buySell = Buy,                   // 買賣別 (BsAction)
                      price = 185,                     // 成交價 (double)
                      origLots = 2,                    // 原始口數 (int)
                      tradableLot = 2,                 // 可交易口數 (int)
                      orderType = ,                // 委託別 (FutOptOrderType)
                      currency = TWD,                  // 幣別 (string)
                      marketPrice = 365,               // 即時價 (string)
                      initialMargin = 0.0,             // 原始保證金 (double)
                      maintenanceMargin = 0.0,         // 維持保證金 (double)
                      clearingMargin = 0.0,            // 結算保證金 (double)
                      initialMarginAllSingle = 0.0,    // 原始保證金 (double)
                      optValue = 36500.0,              // 選擇權市值 (double)
                      optLongValue = 36500.0,          // 選擇權買進市值 (double)
                      optShortValue = 0.0,             // 選擇權賣出市值 (double)
                      profitOrLoss = 18000.0,          // 部位損益 (double)
                      premium = 0.0,                   // 權利金 (double)
                  },
                  SpreadPosition {
                      date = 2024/04/08,               // 部位建立日期 (string)
                      branchNo = 15901,                // 分公司代號 (string)
                      account = 1234567,               // 帳號 (string)
                      positionKind = 2,                // 部位種類 (int)
                      symbol = TXO,                    // 商品代號 (string)
                      expiryDate = 202404,             // 履約日 (string)
                      strikePrice = 20000,            // 履約價 (double)
                      callPut = Put,                   // 權利別 (CallPut)
                      buySell = Buy,                   // 買賣別 (BsAction)
                      price = 354,                     // 成交價 (double)
                      origLots = 2,                    // 原始口數 (int)
                      tradableLot = 2,                 // 可交易口數 (int)
                      orderType = ,                // 委託別 (FutOptOrderType)
                      currency = TWD,                  // 幣別 (string)
                      marketPrice = 82,                // 即時價 (string)
                      initialMargin = 0.0,             // 原始保證金 (double)
                      maintenanceMargin = 0.0,         // 維持保證金 (double)
                      clearingMargin = 0.0,            // 結算保證金 (double)
                      initialMarginAllSingle = 0.0,    // 原始保證金 (double)
                      optValue = 8200.0,               // 選擇權市值 (double)
                      optLongValue = 8200.0,           // 選擇權買進市值 (double)
                      optShortValue = 0.0,             // 選擇權賣出市值 (double)
                      profitOrLoss = -27200.0,         // 部位損益 (double)
                      premium = 0.0,                   // 權利金 (double)
                  },
              ],
          }
      ]
}

```


---

### 權益數查詢

QueryMarginEquity

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                                | 說明 |
| ------- | --------------------------------------------------------------------------------------------------- | ---- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account) | 帳號 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別   | 說明                             |
| --------- | ------ | -------------------------------- |
| isSuccess | bool   | 是否成功                         |
| data      | List   | 回傳未實現資訊                   |
| message   | string | 當isSuccess = False 回傳錯誤訊息 |

###### 權益數 Equity 欄位[​](#權益數-equity-欄位 "Direct link to 權益數 Equity 欄位")

Return type : Object

| 參數              | 類別   | 說明                                                                        |
| ----------------- | ------ | --------------------------------------------------------------------------- |
| date              | string | 查詢日期                                                                    |
| branchNo          | string | 分公司代號                                                                  |
| account           | string | 帳號                                                                        |
| currency          | string | 幣別 : `NTD` 約當台幣 、`TWD` 新台幣、`USD` 美元、`CNY` 人民幣 、`JPY` 日圓 |
| yesterdayBalance  | double | 昨日餘額                                                                    |
| todayBalance      | double | 今日餘額                                                                    |
| initialMargin     | double | 原始保證金                                                                  |
| maintenanceMargin | double | 維持保證金                                                                  |
| clearingMargin    | double | 結算保證金                                                                  |
| todayEquity       | double | 本日權益                                                                    |
| todayDeposit      | double | 今日入金                                                                    |
| todayWithdrawal   | double | 今日出金                                                                    |
| todayTradingFee   | double | 今日交易手續費                                                              |
| todayTradingTax   | double | 今日交易稅                                                                  |
| receivablePremium | double | 收取權利金                                                                  |
| payablePremium    | double | 付出權利金                                                                  |
| excessMargin      | double | 超額保證金                                                                  |
| availableMargin   | double | 可動用保證金                                                                |
| disgorgement      | double | 追繳金額                                                                    |
| optPnl            | double | 未沖銷選擇權浮動損益                                                        |
| optValue          | double | 選擇權市值                                                                  |
| optLongValue      | double | 未沖銷選擇權買方市值                                                        |
| optShortValue     | double | 未沖銷選擇權賣方市值                                                        |
| futRealizedPnl    | double | 期貨平倉損益                                                                |
| futUnrealizedPnl  | double | 期貨未平倉損益                                                              |
| buyLot            | int    | 買進口數                                                                    |
| sellLot           | int    | 賣出口數                                                                    |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
Console.WriteLine(sdk.FutOptAccounting.QueryMarginEquity(account));

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess  = True,
    message  = ,
    data  = [
            Equity{
                  date = 2024/04/08,                     // 查詢日期 (string)
                  branchNo = 15901,                       // 分公司代號 (string)
                  account = 1234567,                      // 帳號 (string)
                  currency = NTD,                         // 幣別 (string)
                  yesterdayBalance = 22435152.4,           // 昨日餘額 (double)
                  todayBalance = 22434910.4,               // 今日餘額 (double)
                  initialMargin = 1114946.0,               // 原始保證金 (double)
                  maintenanceMargin = 939214.0,            // 維持保證金 (double)
                  clearingMargin = 915760.0,               // 結算保證金 (double)
                  todayEquity = 22694910.4,                // 本日權益 (double)
                  todayDeposit= 0.0,                     // 本日入金 (double)
                  todayWithdrawal= 2102.0,               // 本日出金 (double)
                  todayTradingFee= 16.0,                 // 本日交易手續費 (double)
                  todayTradingTax= 0.0,                  // 本日交易稅 (double)
                  receivablePremium= 0.0,                // 收取權利金 (double)
                  payablePremium= 9250.0,                // 付出權利金 (double)
                  excessMargin= 28744525.0,              // 超額保證金 (double)
                  availableMargin = 21453562.4,            // 可動用保證金 (double)
                  disgorgement = 0.0,                      // 追繳金額 (double)
                  optPnl = -248600.0,                      // 未沖銷選擇權浮動損益 (double)
                  optValue = -193100.0,                    // 選擇權市值 (double)
                  optLongValue = 311900.0,                 // 未沖銷選擇權買方市值 (double)
                  optShortValue = 505000.0,                // 未沖銷選擇權賣方市值 (double)
                  futRealizedPnl = 0.0,                    // 期貨平倉損益 (double)
                  futUnrealizedPnl = 60700.0,              // 期貨未平倉損益 (double)
                  buyLot = 22,                             // 買進口數 (int)
                  sellLot = 7,                             // 賣出口數 (int)
              },
            Equity {
                  date = 2024/04/08,                     // 查詢日期 (string)
                  branchNo = 15901,                       // 分公司代號 (string)
                  account = 1234567,                      // 帳號 (string)
                  currency = TWD,                         // 幣別 (string)
                  yesterdayBalance = 19880310.0,           // 昨日餘額 (double)
                  todayBalance = 19880068.0,               // 今日餘額 (double)
                  initialMargin = 1114946.0,               // 原始保證金 (double)
                  maintenanceMargin = 939214.0,            // 維持保證金 (double)
                  clearingMargin = 915760.0,               // 結算保證金 (double)
                  todayEquity = 20140068.0,                // 本日權益 (double)
                  todayDeposit= 0.0,                     // 本日入金 (double)
                  todayWithdrawal= 2102.0,               // 本日出金 (double)
                  todayTradingFee= 16.0,                 // 本日交易手續費 (double)
                  todayTradingTax= 0.0,                  // 本日交易稅 (double)
                  receivablePremium= 0.0,                // 收取權利金 (double)
                  payablePremium= 9250.0,                // 付出權利金 (double)
                  excessMargin= 28744525.0,              // 超額保證金 (double)
                  availableMargin = 18898720.0,            // 可動用保證金 (double)
                  disgorgement = 0.0,                      // 追繳金額 (double)
                  optPnl = -248600.0,                      // 未沖銷選擇權浮動損益 (double)
                  optValue = -193100.0,                    // 選擇權市值 (double)
                  optLongValue = 311900.0,                 // 未沖銷選擇權買方市值 (double)
                  optShortValue = 505000.0,                // 未沖銷選擇權賣方市值 (double)
                  futRealizedPnl = 0.0,                    // 期貨平倉損益 (double)
                  futUnrealizedPnl = 60700.0,              // 期貨未平倉損益 (double)
                  buyLot = 22,                             // 買進口數 (int)
                  sellLot = 7                              // 賣出口數 (int)
              }
          ]
}

```


---

### 單式部位查詢

QuerySinglePosition

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                                | 說明 |
| ------- | --------------------------------------------------------------------------------------------------- | ---- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account) | 帳號 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別   | 說明                             |
| --------- | ------ | -------------------------------- |
| isSuccess | bool   | 是否成功                         |
| data      | List   | 回傳部位資訊                     |
| message   | string | 當isSuccess = False 回傳錯誤訊息 |

##### Position 欄位[​](#position-欄位 "Direct link to Position 欄位")

Return type : Object

| 參數              | 類別                                                                                                                | 說明                                              |
| ----------------- | ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| date              | string                                                                                                              | 部位建立日期                                      |
| branchNo          | string                                                                                                              | 分公司代號                                        |
| account           | string                                                                                                              | 帳號                                              |
| isSpread          | bool                                                                                                                | 是否為複式部位                                    |
| positionKind      | int                                                                                                                 | 部位種類 : `1` 期貨 、`2` 選擇權                  |
| symbol            | string                                                                                                              | 商品代號                                          |
| symbolName        | string                                                                                                              | 商品名稱                                          |
| expiryDate        | string                                                                                                              | 履約日                                            |
| strikePrice       | double                                                                                                              | 履約價                                            |
| callPut           | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                 | 權利別 : `1` Call 、`2` Put                       |
| buySell           | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)               | 買賣別 : `Buy` 買、`Sell` 賣                      |
| price             | double                                                                                                              | 成交價                                            |
| origLots          | int                                                                                                                 | 原始口數                                          |
| tradableLots      | int                                                                                                                 | 可交易口數                                        |
| orderType         | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptordertype) | 委託別 : `New` 新倉、`Close`平倉、`FdayTrade`當沖 |
| currency          | string                                                                                                              | 幣別                                              |
| marketPrice       | string                                                                                                              | 即時價                                            |
| initialMargin     | double                                                                                                              | 原始保證金                                        |
| maintenanceMargin | double                                                                                                              | 維持保證金                                        |
| clearingMargin    | double                                                                                                              | 結算保證金                                        |
| optValue          | double                                                                                                              | 選擇權市值                                        |
| optLongValue      | double                                                                                                              | 選擇權買進市值                                    |
| optShortValue     | double                                                                                                              | 選擇權賣出市值                                    |
| profitOrLoss      | double                                                                                                              | 部位損益                                          |
| premium           | double                                                                                                              | 權利金                                            |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
sdk.FutOptAccounting.QuerySinglePosition(account)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess = True,
    message = ,
    data =  [
                Position {
                    date = 2024/04/08,                  // 部位建立日期 (string)
                    branchNo = 15901,                   // 分公司代號 (string)
                    account = 1234567,                  // 帳號 (string)
                    orderNo = l0001-0000,               // 訂單編號 (string)
                    positionKind = 1,                   // 部位種類 (int)
                    symbol = FITX,                      // 商品代號 (string)
                    expiryDate = 202404,                // 履約日 (string)
                    strikePrice = ,                     // 履約價 (double)
                    callPut = ,                         // 權利別 (CallPut)
                    buySell = Buy,                      // 買賣別 (BsAction)
                    price = 20362,                      // 成交價 (double)
                    origLots = 2,                       // 原始口數 (int)
                    tradableLot = 2,                    // 可交易口數 (int)
                    orderType = New,                    // 委託別 (FutOptOrderType)
                    currency = TWD,                     // 幣別 (string)
                    marketPrice = 20521.0000,           // 即時價 (string)
                    initialMargin = 358000.0,           // 原始保證金 (double)
                    maintenanceMargin = 274000.0,       // 維持保證金 (double)
                    clearingMargin = 264000.0,          // 結算保證金 (double)
                    profitOrLoss = 63600.0,             // 部位損益 (double)
                    premium = 0.0,                      // 權利金 (double)
                },
                Position {
                    date = 2024/03/29,                  // 部位建立日期 (string)
                    branchNo = 15901,                   // 分公司代號 (string)
                    account = 1234567,                  // 帳號 (string)
                    orderNo = l0007-0000,               // 訂單編號 (string)
                    positionKind = 2,                   // 部位種類 (int)
                    symbol = TX1,                       // 商品代號 (string)
                    expiryDate = 202404,                // 履約日 (string)
                    strikePrice = 20600,               // 履約價 (double)
                    callPut = Call,                     // 權利別 (CallPut)
                    buySell = Buy,                      // 買賣別 (BsAction)
                    price = 10,                         // 成交價 (double)
                    origLots = 2,                       // 原始口數 (int)
                    tradableLot = 2,                    // 可交易口數 (int)
                    orderType = New,                    // 委託別 (FutOptOrderType)
                    currency = TWD,                     // 幣別 (string)
                    marketPrice = 4.6000,               // 即時價 (string)
                    initialMargin = 52660.0,            // 原始保證金 (double)
                    maintenanceMargin = 36460.0,        // 維持保證金 (double)
                    clearingMargin = 34460.0,           // 結算保證金 (double)
                    profitOrLoss = -540.0,              // 部位損益 (double)
                    premium = -1000.0,                  // 權利金 (double)
                },
                Position {
                    date = 2024/03/29,                  // 部位建立日期 (string)
                    branchNo = 15901,                   // 分公司代號 (string)
                    account = 1234567,                  // 帳號 (string)
                    orderNo = l0007-0001,               // 訂單編號 (string)
                    positionKind = 2,                   // 部位種類 (int)
                    symbol = TX1,                       // 商品代號 (string)
                    expiryDate = 202404,                // 履約日 (string)
                    strikePrice = 20600,               // 履約價 (double)
                    callPut = Call,                        // 權利別 (CallPut)
                    buySell = Buy,                        // 買賣別 (BsAction)
                    price = 10,                         // 成交價 (double)
                    origLots = 2,                       // 原始口數 (int)
                    tradableLot = 2,                    // 可交易口數 (int)
                    orderType = New,                    // 委託別 (FutOptOrderType)
                    currency = TWD,                     // 幣別 (string)
                    marketPrice = 4.6000,               // 即時價 (string)
                    initialMargin = 52660.0,            // 原始保證金 (double)
                    maintenanceMargin = 36460.0,        // 維持保證金 (double)
                    clearingMargin = 34460.0,           // 結算保證金 (double)
                    profitOrLoss = -540.0,              // 部位損益 (double)
                    premium = -1000.0,                  // 權利金 (double)
                },
                Position {
                    date = 2024/03/01,                  // 部位建立日期 (string)
                    branchNo = 15901,                   // 分公司代號 (string)
                    account = 1234567,                  // 帳號 (string)
                    orderNo = l0002-0000,               // 訂單編號 (string)
                    positionKind = 2,                   // 部位種類  (int)
                    symbol = TXO,                       // 商品代號 (string)
                    expiryDate = 202404,                // 履約日 (string)
                    strikePrice = 18500,               // 履約價 (double)
                    callPut = Call,                     // 權利別 (CallPut)
                    buySell = Sell,                     // 買賣別 (BsAction)
                    price = 625,                        // 成交價 (double)
                    origLots = 5,                       // 原始口數 (int)
                    tradableLot = 4,                    // 可交易口數 (int)
                    orderType = New,                    // 委託別 (FutOptOrderType)
                    currency = TWD,                     // 幣別 (string)
                    marketPrice = 2020.0000,            // 即時價 (string)
                    initialMargin = 584000.0,           // 原始保證金 (double)
                    maintenanceMargin = 544000.0,       // 維持保證金 (double)
                    clearingMargin = 536000.0,          // 結算保證金 (double)
                    profitOrLoss = -279000.0,           // 部位損益 (double)
                    premium = 125000.0,                 // 權利金 (double)
                }
    ]
}


```


---

### 參數對照表

#### 類別[​](#類別 "Direct link to 類別")

Class

##### OrderObject[​](#orderobject "Direct link to OrderObject")

| Parameter          | Type             | Meaning                                                                                                                         |
| ------------------ | ---------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| buySell            | BsAction         | [買賣別](#bsaction) 可選用參數`Buy` 買 、 `Sell` 賣                                                                             |
| stockNo            | string           | 商品代號                                                                                                                        |
| price              | string           | 委託價格                                                                                                                        |
| quantity           | int              | 委託數量                                                                                                                        |
| marketType         | FutOptMarketType | [盤別](#futoptmarkettype) 可選用參數`Future` 期貨日盤 、 `Option` 選擇權日盤、 `FutureNight` 期貨夜盤、`OptionNight` 選擇權夜盤 |
| priceType          | FutOptPriceType  | [價格旗標](#futoptpricetype) 可選用參數為 `Limit`限價 、 `Market`市價 、 `RangeMarket` 範圍市價 、 `Reference`參考價            |
| timeInForce        | TimeInForce      | [委託條件](#timeinforce) 可選用參數為 `Rod`、`Fok`、`Ioc`                                                                       |
| orderType          | FutOptOrderType  | [委託類別](#futoptordertype) 可選用參數為 `New` 新倉、`Close`平倉、`Auto`自動、`FdayTrade`當沖                                  |
| userDef (optional) | string           | 用戶自定義 (最長10個字元，不支援特殊字元)                                                                                       |

##### FutOptOrderResult[​](#futoptorderresult "Direct link to FutOptOrderResult")

委託列表，透過 [GetOrderResult(accounts)](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/trade/GetOrderResults.txt) 取得。

| 參數            | 類別             | 說明                                                                                                                           |
| --------------- | ---------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| functionType    | int              | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單、`90`失敗                                                  |
| date            | string           | 交易日期                                                                                                                       |
| seqNo           | string           | 委託單流水序號                                                                                                                 |
| branchNo        | string           | 分公司代號                                                                                                                     |
| account         | string           | 帳號                                                                                                                           |
| orderNo         | string           | 委託書號                                                                                                                       |
| assetType       | int              | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                               |
| market          | string           | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                               |
| marketType      | FutOptMarketType | [盤別種類](#futoptmarkettype) : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤 |
| unit            | int              | 單位數                                                                                                                         |
| currency        | string           | 幣別                                                                                                                           |
| symbol          | string           | 商品代號                                                                                                                       |
| expiryDate      | string           | 到期日                                                                                                                         |
| strikePrice     | double           | 履約價                                                                                                                         |
| callPut         | CallPut          | [買賣權](#callput) : `Call` 買權、 `Put` 賣權                                                                                  |
| buySell         | BsAction         | [買賣別](#bsaction) : `Buy` 買 、 `Sell` 賣                                                                                    |
| symbolLeg2      | string           | 商品代號 - 複式第二隻腳                                                                                                        |
| expiryDateLeg2  | string           | 到期日 - 複式第二隻腳                                                                                                          |
| strikePriceLeg2 | double           | 履約價 - 複式第二隻腳                                                                                                          |
| callPutLeg2     | string           | [買賣權](#callput) - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                   |
| buySellLeg2     | BsAction         | [買賣別](#bsaction) - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                      |
| priceType       | FutOptPriceType  | [原始委託價格別](#futoptpricetype) : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價               |
| price           | double           | 價格                                                                                                                           |
| lot             | int              | 原始委託股口數                                                                                                                 |
| timeInForce     | TimeInforce      | [委託條件別](#timeinforce) : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                               |
| orderType       | FutOptOrderType  | [委託單類型](#futoptordertype) : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                 |
| isPreOrder      | bool             | 是否為預約單                                                                                                                   |
| status          | int              | 委託單狀態，詳細列表請參考 [STATUS](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#status)          |
| afterPriceType  | FutOptPriceType  | [有效委託價格別](#futoptpricetype) : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價               |
| afterPrice      | double           | 有效委託價格                                                                                                                   |
| afterLot        | int              | 有效委託口數                                                                                                                   |
| filledLot       | int              | 成交口數                                                                                                                       |
| filledMoney     | double           | 成交價金                                                                                                                       |
| beforeLot       | int              | 改單前有效口數                                                                                                                 |
| beforePrice     | double           | 改單前有效價                                                                                                                   |
| userDef         | string           | 自訂欄位                                                                                                                       |
| lastTime        | string           | 最後異動時間                                                                                                                   |
| errorMessage    | string           | 錯誤訊息                                                                                                                       |
| detail          | list             | 委託歷程 (查詢orderResultDetail or OrderHistory才有值)                                                                         |
| >> functionType | int              | 功能別 : `10` 新單、`15` 改價、 `20` 改量、`30`刪單、`50` 成交 、`90`失敗                                                      |
| >> modifiedTime | string           | 修改時間                                                                                                                       |
| >> beforeLot    | int              | 原始委託口數                                                                                                                   |
| >> afterLot     | int              | 有效委託口數                                                                                                                   |
| >> beforePrice  | double           | 原始委託價                                                                                                                     |
| >> afterPrice   | double           | 有效委託價                                                                                                                     |
| >> filledMoney  | double           | 成交價金                                                                                                                       |
| >> errorMessage | string           | 錯誤訊息                                                                                                                       |

##### BatchResult[​](#batchresult "Direct link to BatchResult")

批次委託列表，透過 [BatchOrderLists(accounts)](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/trade/batchOrder/BatchOrderList.txt) 取得。

| Parameter    | Type   | Meaning                                                                       |
| ------------ | ------ | ----------------------------------------------------------------------------- |
| functionType | int    | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單、`90`失敗 |
| date         | string | 交易日期                                                                      |
| branchNo     | string | 分公司代號                                                                    |
| account      | string | 帳號                                                                          |
| batchSeqNo   | string | 批次單流水序號                                                                |

##### FutoptModifyPrice[​](#futoptmodifyprice "Direct link to FutoptModifyPrice")

改價物件

| Parameter         | Type            | Meaning                                                                                                            |
| ----------------- | --------------- | ------------------------------------------------------------------------------------------------------------------ |
| FutOptOrderResult | Object          | [委託列表](#futoptorderresult)                                                                                     |
| price             | string          | 改單後的價格                                                                                                       |
| FutOptPriceType   | FutOptPriceType | [改單後的價格類型](#futoptpricetype) : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價 |

##### FutOptModifyLot[​](#futoptmodifylot "Direct link to FutOptModifyLot")

改量物件

| Parameter         | Type   | Meaning                        |
| ----------------- | ------ | ------------------------------ |
| FutOptOrderResult | Object | [委託列表](#futoptorderresult) |
| lot               | int    | 改單後的委託量                 |

##### FutOptFilledData[​](#futoptfilleddata "Direct link to FutOptFilledData")

成交回報物件

| Parameter       | Type            | Meaning                                                                                        |
| --------------- | --------------- | ---------------------------------------------------------------------------------------------- |
| date            | string          | 日期                                                                                           |
| branchNo        | string          | 分公司代號                                                                                     |
| account         | string          | 帳號                                                                                           |
| seqNo           | string          | 委託單流水序號 (只有主動回報才回傳此欄位)                                                      |
| orderNo         | string          | 委託書號                                                                                       |
| symbol          | string          | 商品代號                                                                                       |
| expiryDate      | string          | 到期日                                                                                         |
| strikePrice     | double          | 履約價                                                                                         |
| callPut         | CallPut         | [買賣權](#callput) : `Call` 買權、 `Put` 賣權                                                  |
| buySell         | BsAction        | \[買賣別] (#bsaction) : `Buy` 買 、 `Sell` 賣                                                  |
| symbolLeg2      | string          | 商品代號 - 複式第二隻腳                                                                        |
| expiryDateLeg2  | string          | 到期日 - 複式第二隻腳                                                                          |
| strikePriceLeg2 | double          | 履約價 - 複式第二隻腳                                                                          |
| callPutLeg2     | CallPut         | [買賣權](#callput) - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                   |
| buySellLeg2     | BsAction        | [買賣別](#bsaction) - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                      |
| orderType       | FutOptOrderType | [委託單類型](#futoptordertype) : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖 |
| filledNo        | string          | 成交流水號                                                                                     |
| filledAvgPrice  | double          | 成交均價                                                                                       |
| filledLot       | int             | 成交股數                                                                                       |
| filledPrice     | double          | 成交單價                                                                                       |
| filledTime      | string          | 成交時間                                                                                       |
| userDef         | string          | 用戶自定義 (只有主動回報才回傳此欄位)                                                          |

##### Account[​](#account "Direct link to Account")

帳號資訊

| Parameter   | Type   | Meaning                               |
| ----------- | ------ | ------------------------------------- |
| name        | string | 客戶姓名                              |
| account     | string | 帳號                                  |
| branchNo    | string | 分公司代號                            |
| accountType | string | 帳戶類型 : `stock` 證券 `futopt` 期貨 |

#### Constants ( 欄位對應數值 )[​](#constants--欄位對應數值- "Direct link to Constants ( 欄位對應數值 )")

##### BSAction[​](#bsaction "Direct link to BSAction")

買賣別 (buySell)

| Name | Meaning |
| ---- | ------- |
| Buy  | 買      |
| Sell | 賣      |

##### CallPut[​](#callput "Direct link to CallPut")

買賣別 (buySell)

| Name | Meaning |
| ---- | ------- |
| Call | Call    |
| Put  | Put     |

##### FutOptMarketType[​](#futoptmarkettype "Direct link to FutOptMarketType")

盤別

| Name        | Meaning    |
| ----------- | ---------- |
| Future      | 期貨日盤   |
| FutureNight | 期貨夜盤   |
| Option      | 選擇權日盤 |
| OptionNight | 選擇權夜盤 |

##### FutOptPriceType[​](#futoptpricetype "Direct link to FutOptPriceType")

價格類型 (priceType)

| Name        | Meaning  |
| ----------- | -------- |
| Limit       | 限價     |
| Market      | 市價     |
| RangeMarket | 範圍市價 |
| Reference   | 參考價   |

##### TimeInForce[​](#timeinforce "Direct link to TimeInForce")

委託條件 (TimeInForce)

| Name | Meaning                               |
| ---- | ------------------------------------- |
| ROD  | 當日有效(Rest of Day)                 |
| FOK  | 全部成交否則取消(Fill-or-Kill)        |
| IOC  | 立即成交否則取消(Immediate-or-Cancel) |

##### FutOptOrderType[​](#futoptordertype "Direct link to FutOptOrderType")

委託類別 (orderType)

| Name      | Meaning |
| --------- | ------- |
| New       | 新倉    |
| Close     | 平倉    |
| Auto      | 自動    |
| FdayTrade | 當沖    |

##### function\_type[​](#function_type "Direct link to function_type")

功能類別

| Name     | Value |
| -------- | ----- |
| 新單     | 0     |
| 新單執行 | 10    |
| 改價     | 15    |
| 改量     | 20    |
| 刪單     | 30    |
| 失敗     | 90    |

##### market[​](#market "Direct link to market")

市場

| Name       | Value  |
| ---------- | ------ |
| 期貨交易所 | TAIMEX |

##### status[​](#status "Direct link to status")

委託單狀態

| Name         | Value                                                      |
| ------------ | ---------------------------------------------------------- |
| 預約單       | 0                                                          |
| 中台收到委託 | 4 ( 請用GetOrderResult查詢狀態 )                           |
| 後台連線逾時 | 9 ( 請稍後再使用GetOrderResult查詢狀態 or 聯絡您的營業員 ) |
| 委託成功     | 10                                                         |
| 刪單成功     | 30                                                         |
| 完全成交     | 50                                                         |
| 改價失敗     | 19                                                         |
| 改量失敗     | 29                                                         |
| 刪單失敗     | 39                                                         |
| 失敗         | 90                                                         |

#### Month[​](#month "Direct link to Month")

月份代號

##### 期貨[​](#期貨 "Direct link to 期貨")

| 一月 | 二月 | 三月 | 四月 | 五月   | 六月   |
| ---- | ---- | ---- | ---- | ------ | ------ |
| A    | B    | C    | D    | E      | F      |
| 七月 | 八月 | 九月 | 十月 | 十一月 | 十二月 |
| G    | H    | I    | J    | K      | L      |

##### 選擇權[​](#選擇權 "Direct link to 選擇權")

Call

| 一月 | 二月 | 三月 | 四月 | 五月   | 六月   |
| ---- | ---- | ---- | ---- | ------ | ------ |
| A    | B    | C    | D    | E      | F      |
| 七月 | 八月 | 九月 | 十月 | 十一月 | 十二月 |
| G    | H    | I    | J    | K      | L      |

Put

| 一月 | 二月 | 三月 | 四月 | 五月   | 六月   |
| ---- | ---- | ---- | ---- | ------ | ------ |
| M    | N    | O    | P    | Q      | R      |
| 七月 | 八月 | 九月 | 十月 | 十一月 | 十二月 |
| S    | T    | U    | V    | W      | X      |


---

### 登入

ApikeyLogin

版本資訊

v2.2.7 起新增功能

相關說明請參閱 [API Key 說明](https://www.fbs.com.tw/TradeAPI/docs/trading/api-key-apply.txt) 頁面

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別   | 說明           |
| ---------- | ------ | -------------- |
| personalID | String | 登入的ID       |
| key        | String | 申請的 API Key |
| certPath   | String | 憑證路徑       |
| certPass   | String | 憑證密碼       |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別   | 說明                             |
| --------- | ------ | -------------------------------- |
| isSuccess | bool   | 是否成功                         |
| data      | List   | 回傳帳號資訊                     |
| message   | string | 當isSuccess = False 回傳錯誤訊息 |

##### 帳號資訊 Account 欄位[​](#帳號資訊-account-欄位 "Direct link to 帳號資訊 Account 欄位")

Return type : Object

| 參數        | 類別   | 說明                                     |
| ----------- | ------ | ---------------------------------------- |
| name        | String | 客戶姓名                                 |
| account     | String | 客戶帳號                                 |
| branchNo    | String | 分公司代號                               |
| accountType | string | 帳號類型 回傳 `stock` 證券 `futopt` 期貨 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
using FubonNeo.Sdk;

var sdk = new FubonSDK();

var result = sdk.ApikeyLogin("Your ID", "Your Key","Your Cert Path","Your Cert Password");

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess = True,
    message = ,
    data = [
        Account{
                name = 富邦Bill,   // 客戶姓名 (string)
                account = 28,      // 客戶帳號 (string)
                branchNo = 6460,   // 分公司代號 (string)
                accountType = stock  // 帳號類型 (string)
        }
    ]
}

```


---

### 登入

Login

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別   | 說明       |
| ---------- | ------ | ---------- |
| personalID | String | 登入的ID   |
| password   | String | 登入的密碼 |
| certPath   | String | 憑證路徑   |
| certPass   | String | 憑證密碼   |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別   | 說明                             |
| --------- | ------ | -------------------------------- |
| isSuccess | bool   | 是否成功                         |
| data      | List   | 回傳帳號資訊                     |
| message   | string | 當isSuccess = False 回傳錯誤訊息 |

##### 帳號資訊 Account 欄位[​](#帳號資訊-account-欄位 "Direct link to 帳號資訊 Account 欄位")

Return type : Object

| 參數        | 類別   | 說明                                     |
| ----------- | ------ | ---------------------------------------- |
| name        | String | 客戶姓名                                 |
| account     | String | 客戶帳號                                 |
| branchNo    | String | 分公司代號                               |
| accountType | string | 帳號類型 回傳 `stock` 證券 `futopt` 期貨 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
using FubonNeo.Sdk;

var sdk = new FubonSDK();

var result = sdk.Login("Your ID", "Your Password","Your Cert Path","Your Cert Password");

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess = True,
    message = ,
    data = [
        Account{
                name = 富邦Bill,   // 客戶姓名 (string)
                account = 28,      // 客戶帳號 (string)
                branchNo = 6460,   // 分公司代號 (string)
                accountType = stock  // 帳號類型 (string)
        }
    ]
}

```


---

### 登出

Logout

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別 | 說明     |
| --------- | ---- | -------- |
| isSuccess | bool | 是否成功 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
using FubonNeo.Sdk;

var sdk = new FubonSDK();

var result = sdk.Logout();

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
True

```


---

### 刪除批次委託單

BatchCancelOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                            | 說明               |
| ----------- | --------------------------------------------------------------------------------------------------------------- | ------------------ |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)             | 帳號               |
| orderResult | [BatchList](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptorderresult) | 批次取消委託單內容 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別   | 說明                             |
| --------- | ------ | -------------------------------- |
| isSuccess | bool   | 是否成功                         |
| data      | List   | 回傳修改資訊                     |
| message   | string | 當isSuccess = False 回傳錯誤訊息 |

##### 修改資訊 FutOptOrderResult 欄位[​](#修改資訊-futoptorderresult-欄位 "Direct link to 修改資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                  | 說明                                                                                                                                            |
| --------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | int                                                                                                                   | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                | 委託書號                                                                                                                                        |
| assetType       | int                                                                                                                   | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | int                                                                                                                   | 單位數                                                                                                                                          |
| currency        | string                                                                                                                | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                | 到期日                                                                                                                                          |
| strikePrice     | double                                                                                                                | 履約價                                                                                                                                          |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | double                                                                                                                | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                                                |
| buySellLeg2     | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | double                                                                                                                | 價格                                                                                                                                            |
| lot             | int                                                                                                                   | 原始委託股口數                                                                                                                                  |
| timeInForce     | [TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#timeinforce)           | 委託條件別 : `Rod` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                  | 是否為預約單                                                                                                                                    |
| status          | int                                                                                                                   | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | double                                                                                                                | 有效委託價格                                                                                                                                    |
| afterLot        | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| filledLot       | int                                                                                                                   | 成交口數                                                                                                                                        |
| filledMoney     | double                                                                                                                | 成交價金                                                                                                                                        |
| beforeLot       | int                                                                                                                   | 改單前有效口數                                                                                                                                  |
| beforePrice     | double                                                                                                                | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                  | 委託歷程 (查詢OrderResultDetail or OrderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs

// 批次刪單(利用batch detail回傳的內容刪單)
var cancel_object = new List<FutOptOrderResult>{
    batch_detail.data[0],
    batch_detail.data[1],
};
sdk.Stock.BatchCancelOrder(account, cancel_object);

// 批次刪單(利用不同的單筆委託)
var cancel_object = new List<FutOptOrderResult>{
    order_res.data[0],
    order_res.data[1],
};

sdk.Stock.BatchCancelOrder(account, cancel_object);



```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess = True,
    message = ,
    data =[
            FutOptOrderResult {  // 批次刪單(利用batch detail回傳的內容刪單)
                functionType = 30,                      // 功能別 (int)
                date = 2024/03/25,                      // 交易日期 (string)
                seqNo = 00110212609,                    // 委託單流水序號 (string)
                branchNo = 15901,                       // 分公司代號 (string)
                account = 1234567,                      // 帳號 (string)
                orderNo = C0002,                        // 委託書號 (string)
                assetType = 1,                          // 資產類別 (int)
                market = TAIMEX,                        // 市場類型 (string)
                marketType = Future,                    // 盤別種類 (FutOptMarketType)
                unit = 1,                               // 單位數 (int)
                currency = TWD,                         // 幣別 (string)
                symbol = FITF,                          // 商品代號 (string)
                expiryDate = 202404,                    // 到期日 (string)
                strikePrice = ,                         // 履約價 (double)
                callPut = ,                             // 買賣權 (string)
                buySell = Buy,                           // 買賣別 (BsAction)
                symbolLeg2 = ,                          // 商品代號 - 複式第二隻腳 (string)
                expiryDateLeg2 = ,                      // 到期日 - 複式第二隻腳 (string)
                strikePriceLeg2 = ,                     // 履約價 - 複式第二隻腳 (double)
                callPutLeg2 = ,                         // 買賣權 - 複式第二隻腳 (string)
                buySellLeg2 = ,                         // 買賣別 - 複式第二隻腳 (BsAction)
                priceType = Limit,                       // 原始委託價格別 (FutOptPriceType)
                price = 1822.6,                          // 價格 (double)
                lot = 2,                                 // 原始委託股口數 (int)
                timeInForce = Rod,                       // 委託條件別 (TimeInforce)
                orderType = Auto,                        // 委託單類型 (FutOptOrderType)
                isPreOrder = false,                      // 是否為預約單 (bool)
                status = 30,                             // 委託單狀態 (int)
                afterPriceType = ,                       // 有效委託價格別 (FutOptPriceType)
                afterPrice = 1822.6,                     // 有效委託價格 (double)
                afterLot = 0,                            // 有效委託股口數 (int)
                filledLot = 0,                           // 成交股口數 (int)
                filledMoney = 0,                         // 成交價金 (int)
                beforeLot = ,                        // 改單前有效股口數 (int)
                beforePrice = ,                      // 改單前有效價格 (double)
                userDef = ,                          // 自訂欄位 (string)
                lastTime = 13:21:34,                     // 最後異動時間 (string)
                detail = ,                           // 委託歷程 (list)
                errorMessage = ,                     // 錯誤訊息 (string)
        },
          OrderResult {
                functionType = 30,                      // 功能別 (int)
                date = 2024/03/25,                      // 交易日期 (string)
                seqNo = 00110212610,                    // 委託單流水序號 (string)
                branchNo = 15901,                       // 分公司代號 (string)
                account = 1234567,                      // 帳號 (string)
                orderNo = C0003,                        // 委託書號 (string)
                assetType = 1,                          // 資產類別 (int)
                market = TAIMEX,                        // 市場類型 (string)
                marketType = Future,                    // 盤別種類 (FutOptMarketType)
                unit = 1,                               // 單位數 (int)
                currency = TWD,                         // 幣別 (string)
                symbol = FITF,                          // 商品代號 (string)
                expiryDate = 202404,                    // 到期日 (string)
                strikePrice = ,                         // 履約價 (double)
                callPut = ,                             // 買賣權 (string)
                buySell = Buy,                           // 買賣別 (BsAction)
                symbolLeg2 = ,                          // 商品代號 - 複式第二隻腳 (string)
                expiryDateLeg2 = ,                      // 到期日 - 複式第二隻腳 (string)
                strikePriceLeg2 = ,                     // 履約價 - 複式第二隻腳 (double)
                callPutLeg2 = ,                         // 買賣權 - 複式第二隻腳 (string)
                buySellLeg2 = ,                         // 買賣別 - 複式第二隻腳 (BsAction)
                priceType = Limit,                       // 原始委託價格別 (FutOptPriceType)
                price = 1822.6,                          // 價格 (double)
                lot = 2,                                 // 原始委託股口數 (int)
                timeInForce = Rod,                       // 委託條件別 (TimeInforce)
                orderType = Auto,                        // 委託單類型 (FutOptOrderType)
                isPreOrder = false,                      // 是否為預約單 (bool)
                status = 30,                             // 委託單狀態 (int)
                ...
          }
        ] 
}

```


---

### 批次修改委託價格

BatchModifyPrice

##### 先使用 MakeModifyPriceObj 建立 FutOptModifyPrice物件[​](#先使用-makemodifypriceobj-建立-futoptmodifyprice物件 "Direct link to 先使用 MakeModifyPriceObj 建立 FutOptModifyPrice物件")

| 參數        | 類別                                                                                                                    | 說明             |
| ----------- | ----------------------------------------------------------------------------------------------------------------------- | ---------------- |
| orderResult | [FutOptOrderResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptorderresult) | 預計修改的委託單 |
| price       | string                                                                                                                  | 修改後的價格     |
| priceType   | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)     | 修改後的價格旗標 |

caution

當 price 欄位有填入值時，priceType 需為null ； 當 priceType 欄位有填入值時，price 需為null

將回傳的物件放入 BatchModifyPrice 的方法中

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數              | 類別                                                                                                                                    | 說明               |
| ----------------- | --------------------------------------------------------------------------------------------------------------------------------------- | ------------------ |
| account           | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                                     | 帳號               |
| FutOptModifyPrice | [FutOptModifyPrice](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmodifyprice)(list of object) | 批次修改委託單內容 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別   | 說明                             |
| --------- | ------ | -------------------------------- |
| isSuccess | bool   | 是否成功                         |
| data      | List   | 回傳修改資訊                     |
| message   | string | 當isSuccess = False 回傳錯誤訊息 |

##### 修改資訊 FutOptOrderResult 欄位[​](#修改資訊-futoptorderresult-欄位 "Direct link to 修改資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                  | 說明                                                                                                                                            |
| --------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | int                                                                                                                   | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                | 委託書號                                                                                                                                        |
| assetType       | int                                                                                                                   | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | int                                                                                                                   | 單位數                                                                                                                                          |
| currency        | string                                                                                                                | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                | 到期日                                                                                                                                          |
| strikePrice     | double                                                                                                                | 履約價                                                                                                                                          |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | double                                                                                                                | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                                                |
| buySellLeg2     | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | double                                                                                                                | 價格                                                                                                                                            |
| lot             | int                                                                                                                   | 原始委託股口數                                                                                                                                  |
| timeInForce     | [TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#timeinforce)           | 委託條件別 : `Rod` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                  | 是否為預約單                                                                                                                                    |
| status          | int                                                                                                                   | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | double                                                                                                                | 有效委託價格                                                                                                                                    |
| afterLot        | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| filledLot       | int                                                                                                                   | 成交口數                                                                                                                                        |
| filledMoney     | double                                                                                                                | 成交價金                                                                                                                                        |
| beforeLot       | int                                                                                                                   | 改單前有效口數                                                                                                                                  |
| beforePrice     | double                                                                                                                | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                  | 委託歷程 (查詢OrderResultDetail or OrderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
// 批次改價(利用batch detail回傳的內容改單)
var modified_orders = new List<FutOptModifyPrice> {
    sdk.FutOpt.MakeModifyPriceObj(batch_result.data[0], "19900", null),
    sdk.FutOpt.MakeModifyPriceObj(batch_result.data[1], "19900", null)
};

var batch_modify_price = sdk.FutOpt.BatchModifyPrice(account, modified_orders);
Console.WriteLine(batch_modify_price);


// 批次改價(利用不同的單筆委託)
var modified_orders = new List<FutOptModifyPrice> {
    sdk.FutOpt.MakeModifyPriceObj(orderResults.data[37], "19900", null),
    sdk.FutOpt.MakeModifyPriceObj(orderResults.data[35], "19900", null)
};

var batch_modify_price = sdk.FutOpt.BatchModifyPrice(account, modified_orders);
Console.WriteLine(batch_modify_price);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess = True,
    message = ,
    data =[ 
            FutOptOrderResult{ // 批次改價(利用batch detail回傳的內容改單)
                function_type = 15,                     // 功能別 (int)
                date = 2024/03/25,                      // 交易日期 (string)
                seq_no = 00110212664,                   // 委託單流水序號 (string)
                branch_no = 15901,                      // 分公司代號 (string)
                account = 1234567,                      // 帳號 (string)
                order_no = C0006,                       // 委託書號 (string)
                asset_type = 1,                         // 資產類別 (int)
                market = TAIMEX,                        // 市場類型 (string)
                market_type = Future,                   // 盤別種類 (FutOptMarketType)
                unit = ,                                // 單位數 (int)
                currency = TWD,                         // 幣別 (string)
                symbol = FITX,                          // 商品代號 (string)
                expiry_date = 0,                        // 到期日 (string)
                strike_price = 0,                       // 履約價 (double)
                call_put = ,                            // 買賣權 (string)
                buy_sell = Buy,                         // 買賣別 (BsAction)
                symbol_leg2 = ,                         // 商品代號 - 複式第二隻腳 (string)
                expiry_date_leg2 = ,                    // 到期日 - 複式第二隻腳 (string)
                strike_price_leg2 = ,                   // 履約價 - 複式第二隻腳 (double)
                call_put_leg2 = ,                       // 買賣權 - 複式第二隻腳 (string)
                buy_sell_leg2 = ,                       // 買賣別 - 複式第二隻腳 (BsAction)
                price_type = Limit,                     // 原始委託價格別 (FutOptPriceType)
                price = 20000,                          // 價格 (double)
                lot = 1,                                // 原始委託股口數 (int)
                time_in_force = Rod,                    // 委託條件別 (TimeInforce)
                order_type = New,                       // 委託單類型 (FutOptOrderType)
                is_pre_order = false,                   // 是否為預約單 (bool)
                status = 10,                            // 委託單狀態 (int)
                after_price_type = Limit,               // 有效委託價格別 (FutOptPriceType)
                after_price = 19900,                    // 有效委託價格 (double)
                after_lot = 1,                          // 有效委託股口數 (int)
                filled_lot = 0,                         // 成交股口數 (int)
                filled_money = 0,                       // 成交價金 (int)
                before_lot = 0,                         // 改單前有效股口數 (int)
                before_price = 20000,                   // 改單前有效價格 (double)
                user_def = ,                            // 自訂欄位 (string)
                last_time = 13:39:05,                   // 最後異動時間 (string)
                detail = ,                              // 委託歷程 (list)
                error_message =                         // 錯誤訊息 (string)
                },
          FutOptOrderResult{
                function_type = 15,                     // 功能別 (int)
                date = 2024/03/25,                      // 交易日期 (string)
                seq_no = 00110212665,                   // 委託單流水序號 (string)
                branch_no = 15901,                      // 分公司代號 (string)
                account = 1234567,                      // 帳號 (string)
                order_no = C0007,                       // 委託書號 (string)
                asset_type = 1,                         // 資產類別 (int)
                market = TAIMEX,                        // 市場類型 (string)
                market_type = Future,                   // 盤別種類 (FutOptMarketType)
                unit = ,                                // 單位數 (int)
                currency = TWD,                         // 幣別 (string)
                symbol = FITX,                          // 商品代號 (string)
                expiry_date = 0,                        // 到期日 (string)
                strike_price = 0,                       // 履約價 (double)
                call_put = ,                            // 買賣權 (string)
                buy_sell = Buy,                         // 買賣別 (BsAction)
                symbol_leg2 = ,                         // 商品代號 - 複式第二隻腳 (string)
                expiry_date_leg2 = ,                    // 到期日 - 複式第二隻腳 (string)
                strike_price_leg2 = ,                   // 履約價 - 複式第二隻腳 (double)
                call_put_leg2 = ,                       // 買賣權 - 複式第二隻腳 (string)
                buy_sell_leg2 = ,                       // 買賣別 - 複式第二隻腳 (BsAction)
                price_type = Limit,                     // 原始委託價格別 (FutOptPriceType)
                price = 20000,                          // 價格 (double)
                lot = 1,                                // 原始委託股口數 (int)
                time_in_force = Rod,                    // 委託條件別 (TimeInforce)
                order_type = New,                       // 委託單類型 (FutOptOrderType)
                is_pre_order = false,                   // 是否為預約單 (bool)
                status = 10,                            // 委託單狀態 (int)
                ...
            }
        ]
} 

以下擷取data內容
[  FutOptOrderResult{ // 批次改價(利用不同的單筆委託)
                function_type = 15,                     // 功能別 (int)
                date = 2024/03/25,                      // 交易日期 (string)
                seq_no = 00110212668,                   // 委託單流水序號 (string)
                branch_no = 15901,                      // 分公司代號 (string)
                account = 1234567,                      // 帳號 (string)
                order_no = C0008,                       // 委託書號 (string)
                asset_type = 1,                         // 資產類別 (int)
                market = TAIMEX,                        // 市場類型 (string)
                market_type = Future,                   // 盤別種類 (FutOptMarketType)
                unit = ,                                // 單位數 (int)
                currency = TWD,                         // 幣別 (string)
                symbol = FITX,                          // 商品代號 (string)
                expiry_date = 0,                        // 到期日 (string)
                strike_price = 0,                       // 履約價 (double)
                call_put = ,                            // 買賣權 (string)
                buy_sell = Buy,                         // 買賣別 (BsAction)
                symbol_leg2 = ,                         // 商品代號 - 複式第二隻腳 (string)
                expiry_date_leg2 = ,                    // 到期日 - 複式第二隻腳 (string)
                strike_price_leg2 = ,                   // 履約價 - 複式第二隻腳 (double)
                call_put_leg2 = ,                       // 買賣權 - 複式第二隻腳 (string)
                buy_sell_leg2 = ,                       // 買賣別 - 複式第二隻腳 (BsAction)
                price_type = Limit,                     // 原始委託價格別 (FutOptPriceType)
                price = 20000,                          // 價格 (double)
                lot = 1,                                // 原始委託股口數 (int)
                time_in_force = Rod,                    // 委託條件別 (TimeInforce)
                order_type = New,                       // 委託單類型 (FutOptOrderType)
                is_pre_order = false,                   // 是否為預約單 (bool)
                status = 10,                            // 委託單狀態 (int)
                after_price_type = Limit,               // 有效委託價格別 (FutOptPriceType)
                after_price = 19900,                    // 有效委託價格 (double)
                after_lot = 1,                          // 有效委託股口數 (int)
                filled_lot = 0,                         // 成交股口數 (int)
                filled_money = 0,                       // 成交價金 (int)
                before_lot = 0,                         // 改單前有效股口數 (int)
                before_price = 20000,                   // 改單前有效價格 (double)
                user_def = ,                            // 自訂欄位 (string)
                last_time = 13:39:05,                   // 最後異動時間 (string)
                error_message =                         // 錯誤訊息 (string)
    },
    FutOptOrderResult{
                function_type = 15,                     // 功能別 (int)
                date = 2024/03/25,                      // 交易日期 (string)
                seq_no = 00110212664,                   // 委託單流水序號 (string)
                branch_no = 15901,                      // 分公司代號 (string)
                account = 1234567,                      // 帳號 (string)
                order_no = C0009,                       // 委託書號 (string)
                asset_type = 1,                         // 資產類別 (int)
                market = TAIMEX,                        // 市場類型 (string)
                market_type = Future,                   // 盤別種類 (FutOptMarketType)
                unit = ,                                // 單位數 (int)
                currency = TWD,                         // 幣別 (string)
                symbol = FITX,                          // 商品代號 (string)
                expiry_date = 0,                        // 到期日 (string)
                strike_price = 0,                       // 履約價 (double)
                call_put = ,                            // 買賣權 (string)
                buy_sell = Buy,                         // 買賣別 (BsAction)
                symbol_leg2 = ,                         // 商品代號 - 複式第二隻腳 (string)
                expiry_date_leg2 = ,                    // 到期日 - 複式第二隻腳 (string)
                strike_price_leg2 = ,                   // 履約價 - 複式第二隻腳 (double)
                call_put_leg2 = ,                       // 買賣權 - 複式第二隻腳 (string)
                buy_sell_leg2 = ,                       // 買賣別 - 複式第二隻腳 (BsAction)
                price_type = Limit,                     // 原始委託價格別 (FutOptPriceType)
                price = 20000,                          // 價格 (double)
                lot = 1,                                // 原始委託股口數 (int)
                time_in_force = Rod,                    // 委託條件別 (TimeInforce)
                order_type = New,                       // 委託單類型 (FutOptOrderType)
                is_pre_order = false,                   // 是否為預約單 (bool)
                status = 10,                            // 委託單狀態 (int)
                ...
    }
]

```


---

### 批次修改委託數量

BatchModifyLot

##### 先使用 MakeModifyLotObj 建立 FutOptModifyLot 物件[​](#先使用-makemodifylotobj-建立-futoptmodifylot-物件 "Direct link to 先使用 MakeModifyLotObj 建立 FutOptModifyLot 物件")

| 參數        | 類別                                                                                                                    | 說明                                                |
| ----------- | ----------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| orderResult | [FutOptOrderResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptorderresult) | 預計修改的委託單                                    |
| lots        | int                                                                                                                     | 修改後的委託量 ( 修改後數量包含此委託單已成交部份 ) |

將回傳的物件放入 ModifyLot 的方法中

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數         | 類別                                                                                                                                 | 說明               |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------ | ------------------ |
| account      | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                                  | 帳號               |
| ModifyLotObj | [FutOptModifyLot](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmodifylot) (list of object) | 批次改量委託單內容 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別   | 說明                             |
| --------- | ------ | -------------------------------- |
| isSuccess | bool   | 是否成功                         |
| data      | List   | 回傳修改資訊                     |
| message   | string | 當isSuccess = False 回傳錯誤訊息 |

##### 修改資訊 FutOptOrderResult 欄位[​](#修改資訊-futoptorderresult-欄位 "Direct link to 修改資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                  | 說明                                                                                                                                            |
| --------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | int                                                                                                                   | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                | 委託書號                                                                                                                                        |
| assetType       | int                                                                                                                   | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | int                                                                                                                   | 單位數                                                                                                                                          |
| currency        | string                                                                                                                | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                | 到期日                                                                                                                                          |
| strikePrice     | double                                                                                                                | 履約價                                                                                                                                          |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | double                                                                                                                | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                                                |
| buySellLeg2     | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | double                                                                                                                | 價格                                                                                                                                            |
| lot             | int                                                                                                                   | 原始委託股口數                                                                                                                                  |
| timeInForce     | [TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#timeinforce)           | 委託條件別 : `Rod` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                  | 是否為預約單                                                                                                                                    |
| status          | int                                                                                                                   | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | double                                                                                                                | 有效委託價格                                                                                                                                    |
| afterLot        | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| filledLot       | int                                                                                                                   | 成交口數                                                                                                                                        |
| filledMoney     | double                                                                                                                | 成交價金                                                                                                                                        |
| beforeLot       | int                                                                                                                   | 改單前有效口數                                                                                                                                  |
| beforePrice     | double                                                                                                                | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                  | 委託歷程 (查詢OrderResultDetail or OrderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
// 批次改量(利用batch detail回傳的內容改單)
var modified_orders = new List<FutOptModifyLot> {
    sdk.FutOpt.MakeModifyLotObj(batch_result.data[0], 1000),
    sdk.FutOpt.MakeModifyLotObj(batch_result.data[1], 1000)
};

var batch_modify_qty = sdk.FutOpt.BatchModifyLot(account, modified_orders);
Console.WriteLine(batch_modify_qty);


// 批次改量(利用不同的單筆委託)
var modified_orders = new List<FutOptModifyLot> {
    sdk.FutOpt.MakeModifyLotObj(orderResults.data[0], 1000),
    sdk.FutOpt.MakeModifyLotObj(orderResults.data[1], 1000)
};

var batch_modify_qty = sdk.FutOpt.BatchModifyLot(account, modified_orders);
Console.WriteLine(batch_modify_qty);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess = True,
    message = ,
    data =[  FutOptOrderResult{ // 批次改量(利用batch detail回傳的內容改單)
               functionType = 20,                        // 功能別 (int)
                date = 2024/04/12,                        // 交易日期 (string)
                seqNo = 03100161319,                      // 委託單流水序號 (string)
                branchNo = 15901,                         // 分公司代號 (string)
                account = 1234567,                        // 帳號 (string)
                orderNo = l001D,                          // 委託書號 (string)
                assetType = 1,                            // 資產類別 (int)
                market = TAIMEX,                          // 市場類型 (string)
                marketType = FutureNight,                 // 盤別種類 (FutOptMarketType)
                unit =  ,                                // 單位數 (int)
                currency = TWD,                           // 幣別 (string)
                symbol = FIMTX,                           // 商品代號 (string)
                expiryDate = ,                          // 到期日 (string)
                strikePrice = ,                         // 履約價 (double)
                callPut =  ,                             // 買賣權 (Callput)
                buySell = Buy,                            // 買賣別 (BsAction)
                symbolLeg2 =  ,                          // 商品代號 - 複式第二隻腳 (string)
                expiryDateLeg2 =  ,                      // 到期日 - 複式第二隻腳 (string)
                strikePriceLeg2 =  ,                     // 履約價 - 複式第二隻腳 (double)
                callPutLeg2 =  ,                         // 買賣權 - 複式第二隻腳 (CallPut)
                buySellLeg2 =  ,                         // 買賣別 - 複式第二隻腳 (BsAction)
                priceType = Limit,                        // 原始委託價格別 (FutOptPriceType)
                price = 20000,                            // 價格 (double)
                lot = 3,                                  // 原始委託股口數 (int)
                timeInForce = ROD,                        // 委託條件別 (TimeInforce)
                orderType = New,                          // 委託單類型 (OrderType)
                isPreOrder = false,                       // 是否為預約單 (bool)
                status = 10,                              // 委託單狀態 (int)
                afterPriceType = Limit,                   // 有效委託價格別 (FutOptPriceType)
                afterPrice = 20000,                       // 有效委託價格 (double)
                afterLot = 2,                             // 有效委託股口數 (int)
                filledLot = 0,                            // 成交股口數 (int)
                filledMoney = 0,                          // 成交價金 (int)
                beforeLot = 0,                            // 改單前有效股口數 (int)
                beforePrice = 20000,                      // 改單前有效價格 (double)
                userDef =  =,                             // 自訂欄位 (string)
                lastTime = 18:24:40,                      // 最後異動時間 (string)
                detail = ,                               // 委託歷程 (list)
                errorMessage =                           // 錯誤訊息 (string)
        },
        FutOptOrderResult{ // 批次改量(利用batch detail回傳的內容改單)
                 functionType = 20,                        // 功能別 (int)
                date = 2024/03/25,                        // 交易日期 (string)
                seqNo = 03100161319,                      // 委託單流水序號 (string)
                branchNo = 15901,                         // 分公司代號 (string)
                account = 1234567,                        // 帳號 (string)
                orderNo = l001D,                          // 委託書號 (string)
                assetType = 1,                            // 資產類別 (int)
                market = TAIMEX,                          // 市場類型 (string)
                marketType = FutureNight,                 // 盤別種類 (FutOptMarketType)
                unit =  ,                                // 單位數 (int)
                currency = TWD,                           // 幣別 (string)
                symbol = FIMTX,                           // 商品代號 (string)
                expiryDate =  ,                          // 到期日 (string)
                strikePrice =  ,                         // 履約價 (double)
                callPut =  ,                             // 買賣權 (Callput)
                buySell = Buy,                            // 買賣別 (BsAction)
                symbolLeg2 =  ,                          // 商品代號 - 複式第二隻腳 (string)
                expiryDateLeg2 =  ,                      // 到期日 - 複式第二隻腳 (string)
                strikePriceLeg2 =  ,                     // 履約價 - 複式第二隻腳 (double)
                callPutLeg2 =  ,                         // 買賣權 - 複式第二隻腳 (CallPut)
                buySellLeg2 =  ,                         // 買賣別 - 複式第二隻腳 (BsAction)
                priceType = Limit,                        // 原始委託價格別 (FutOptPriceType)
                price = 20000,                            // 價格 (double)
                lot = 3,                                  // 原始委託股口數 (int)
                timeInForce = ROD,                        // 委託條件別 (TimeInforce)
                orderType = New,                          // 委託單類型 (OrderType)
                isPreOrder = false,                       // 是否為預約單 (bool)
                status = 10,                              // 委託單狀態 (int)
                afterPriceType = Limit,                   // 有效委託價格別 (FutOptPriceType)
                afterPrice = 20000,                       // 有效委託價格 (double)
                afterLot = 2,                             // 有效委託股口數 (int)
                filledLot = 0,                            // 成交股口數 (int)
                filledMoney = 0,                          // 成交價金 (int)
                beforeLot = 0,                            // 改單前有效股口數 (int)
                beforePrice = 20000,                      // 改單前有效價格 (double)
                userDef =  =,                             // 自訂欄位 (string)
                lastTime = 18:24:40,                      // 最後異動時間 (string)
                errorMessage =                           // 錯誤訊息 (string)
            }
        ] 
}

以下擷取data內容

[  
   FutOptOrderResult{ // 批次改量(利用不同的單筆委託)
        functionType = 20,         // 功能別 (int)
        date = 2024/03/08,         // 交易日期 (string)
        seqNo = 00000000043,       // 委託單流水序號 (string)
        branchNo = 20112,          // 分公司代號 (string)
        account = "26",            // 帳號 (string)
        orderNo = x0027,           // 委託書號 (string)
        assetType = 0,             // 資產類別 (int)
        market = TAIEX,            // 市場類型 (string)
        marketType = Common,       // 盤別種類 (MaeketType)
        stockNo = 2881,            // 商品代號 (string)
        buySell = Buy,             // 買賣別 (BsAction)
        priceType = Limit,         // 原始委託價格別 (FutOptPriceType)
        price = 66,                // 價格 (double)
        quantity = 2000,           // 原始委託股數 (int)
        timeInForce = Rod,         // 委託條件別 (TimeInForce)
        orderType = FutOpt,         // 委託單類型 (OrderType)
        isPreOrder = False,        // 是否為預約單 (bool)
        status = 10,               // 委託單狀態 (int)
        afterPriceType = ,         // 有效委託價格別 (FutOptPriceType)
        afterPrice = 66,           // 有效委託價格 (double)
        unit = 1000,               // 單位數 (int)
        afterQty = 1000,           // 有效委託股數 (int)
        filledQty = 0,             // 成交股數 (int)
        filledMoney = 0,           // 成交價金 (int)
        beforeQty = 2000,          // 改單前有效量 (int)
        beforePrice = ,            // 改單前有效價 (int)
        userDef = Test,            // 自訂欄位 (string)
        lastTime = 12:21:53.023,   // 最後異動時間 (string)
        errorMessage =             // 錯誤訊息 (string)
        },
    FutOptOrderResult{
        functionType = 20,         // 功能別 (int)
        date = 2024/03/08,         // 交易日期 (string)
        seqNo = 00000000044,       // 委託單流水序號 (string)
        branchNo = 20112,          // 分公司代號 (string)
        account = 26,              // 帳號 (string)
        orderNo = x0028,           // 委託書號 (string)
        assetType = 0,             // 資產類別 (int)
        market = TAIEX,            // 市場類型 (string)
        marketType = Common,       // 盤別種類 (FutOptMarketType)
        stockNo = 2881,            // 商品代號 (string)
        buySell = Buy,             // 買賣別 (BsAction)
        priceType = Limit,         // 原始委託價格別 (FutOptPriceType)
        price = 66,                // 價格 (double)
        quantity = 2000,           // 原始委託股數 (int)
        timeInForce = Rod,         // 委託條件別 (TimeInForce)
        orderType = FutOpt,         // 委託單類型 (OrderType)
        isPreOrder = False,        // 是否為預約單 (bool)
        status = 10,               // 委託單狀態 (int)
        afterPriceType = ,         // 有效委託價格別 (FutOptPriceType)
        afterPrice = 66,           // 有效委託價格 (double)
        unit = 1000,               // 單位數 (int)
        afterQty = 1000,           // 有效委託股數 (int)
        filledQty = 0,             // 成交股數 (int)
        filledMoney = 0,           // 成交價金 (int)
        beforeQty = 2000,          // 改單前有效量 (int)
        beforePrice = ,            // 改單前有效價 (string)
        userDef = Test,            // 自訂欄位 (string)
        lastTime = 12:21:53.023,   // 最後異動時間 (string)
        errorMessage =             // 錯誤訊息 (string)
    }
] 

```


---

### 取得批次委託明細

BatchOrderDetail

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數           | 類別                                                                                                        | 說明         |
| -------------- | ----------------------------------------------------------------------------------------------------------- | ------------ |
| account        | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)         | 帳號         |
| BatchOrderList | [BatchResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#batchresult) | 批次委託列表 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別   | 說明                             |
| --------- | ------ | -------------------------------- |
| isSuccess | bool   | 是否成功                         |
| data      | List   | 回傳委託資訊                     |
| message   | string | 當isSuccess = False 回傳錯誤訊息 |

##### 委託資訊 FutOptOrderResult 欄位[​](#委託資訊-futoptorderresult-欄位 "Direct link to 委託資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                  | 說明                                                                                                                                            |
| --------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | int (optional)                                                                                                        | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單、`90`失敗                                                                   |
| date            | string                                                                                                                | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                | 委託書號                                                                                                                                        |
| assetType       | int                                                                                                                   | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | int                                                                                                                   | 單位數                                                                                                                                          |
| currency        | string                                                                                                                | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                | 到期日                                                                                                                                          |
| strikePrice     | double                                                                                                                | 履約價                                                                                                                                          |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | double                                                                                                                | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                                                |
| buySellLeg2     | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | double                                                                                                                | 價格                                                                                                                                            |
| lot             | int                                                                                                                   | 原始委託股口數                                                                                                                                  |
| timeInForce     | [TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#timeinforce)           | 委託條件別 : `Rod` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                  | 是否為預約單                                                                                                                                    |
| status          | int                                                                                                                   | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | double                                                                                                                | 有效委託價格                                                                                                                                    |
| afterLot        | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| filledLot       | int                                                                                                                   | 成交口數                                                                                                                                        |
| filledMoney     | double                                                                                                                | 成交價金                                                                                                                                        |
| beforeLot       | int                                                                                                                   | 改單前有效口數                                                                                                                                  |
| beforePrice     | double                                                                                                                | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                  | 委託歷程 (查詢OrderResultDetail or OrderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                | 錯誤訊息                                                                                                                                        |

caution

此功能僅供查詢批次送單執行結果，欲取得委託單最新狀態請使用單筆委託單查詢功能

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
var batch_order_list =  sdk.FutOpt.BatchOrderLists(account);
Console.WriteLine(sdk.FutOpt.BatchOrderDetail(account, batch_order_list.data[0]));

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess = True,
    message = ,
    data =  [ FutOptOrderResult{
                functionType = ,                       // 功能類型 (int)
                date = 2024/03/25,                     // 交易日期 (string)
                seqNo = 00110222608,                   // 委託單流水序號 (string)
                branchNo = 15901,                      // 分公司代號 (string)
                account = 1234567,                     // 帳號 (string)
                orderNo = C0101,                       // 委託書號 (string)
                assetType = 1,                         // 資產類別 (int)
                market = TAIMEX,                       // 市場類型 (string)
                marketType = Future,                   // 盤別種類 (FutOptMarketType)
                unit = 1,                              // 單位數 (int)
                currency = TWD,                        // 幣別 (string)
                symbol = FITF,                         // 商品代號 (string)
                expiryDate = 202404,                   // 到期日 (string)
                strikePrice = ,                        // 履約價 (double)
                callPut = ,                            // 買賣權 (string)
                buySell = Buy,                         // 買賣別 (BsAction)
                symbolLeg2 = ,                         // 商品代號 - 複式第二隻腳 (string)
                expiryDateLeg2 = ,                     // 到期日 - 複式第二隻腳 (string)
                strikePriceLeg2 = ,                    // 履約價 - 複式第二隻腳 (double)
                callPutLeg2 = ,                        // 買賣權 - 複式第二隻腳 (string)
                buySellLeg2 = ,                        // 買賣別 - 複式第二隻腳 (BsAction)
                priceType = Limit,                     // 原始委託價格別 (FutOptPriceType)
                price = 1822.6,                        // 價格 (double)
                lot = 2,                               // 原始委託股口數 (int)
                timeInForce = Rod,                     // 委託條件別 (TimeInforce)
                orderType = Auto,                      // 委託單類型 (FutOptOrderType)
                isPreOrder = false,                    // 是否為預約單 (bool)
                status = 10,                           // 委託單狀態 (int)
                afterPriceType = ,                     // 有效委託價格別 (FutOptPriceType)
                afterPrice = 1822.6,                   // 有效委託價格 (double)
                afterLot = 2,                          // 有效委託股口數 (int)
                filledLot = 0,                         // 成交股口數 (int)
                filledMoney = 0,                       // 成交價金 (int)
                beforeLot = ,                         // 改單前有效股口數 (int)
                beforePrice = ,                       // 改單前有效價格 (double)
                userDef = ,                           // 自訂欄位 (string)
                lastTime = 12:20:27,                    // 最後異動時間 (string)
                detail = ,                            // 委託歷程 (list)
                errorMessage =                        // 錯誤訊息 (string)
            },
            ...
        ] 
}

```


---

### 取得批次委託列表

BatchOrderLists

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                                | 說明 |
| ------- | --------------------------------------------------------------------------------------------------- | ---- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account) | 帳號 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別   | 說明                             |
| --------- | ------ | -------------------------------- |
| isSuccess | bool   | 是否成功                         |
| data      | List   | 回傳批次單資訊                   |
| message   | string | 當isSuccess = False 回傳錯誤訊息 |

##### 批次單 BatchResult 欄位[​](#批次單-batchresult-欄位 "Direct link to 批次單 BatchResult 欄位")

Return type : Object

| 參數         | 類別   | 說明                                                                               |
| ------------ | ------ | ---------------------------------------------------------------------------------- |
| functionType | int    | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價 、 `20` 改量 、 `30` 刪單 、`90`失敗 |
| date         | string | 交易日期                                                                           |
| branchNo     | string | 分公司代號                                                                         |
| account      | string | 帳號                                                                               |
| batchSeqNo   | string | 批次單流水序號                                                                     |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
Console.WriteLine(sdk.FutOpt.BatchOrderLists(account));

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess = True,
    message = ,
    data =  [
                BatchResult{
                    functionType = 0,   // 功能種類 (int)
                    date = 2023/10/04,  // 交易日期 (string)
                    branchNo = 6460,    // 分公司代號 (string)
                    account = 26,       // 帳號 (string)
                    batchSeqNo = 11EE626533D072228000000C29304663 // 批次單流水序號 (string)
                },
                ...
            ]
} 

```


---

### 建立批次委託單

BatchPlaceOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                                         | 說明     |
| ----------- | ---------------------------------------------------------------------------------------------------------------------------- | -------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                          | 帳號     |
| OrderObject | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#orderobject) (list of object) | 委託內容 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別   | 說明                             |
| --------- | ------ | -------------------------------- |
| isSuccess | bool   | 是否成功                         |
| data      | List   | 回傳委託資訊                     |
| message   | string | 當isSuccess = False 回傳錯誤訊息 |

##### 委託資訊 FutOptOrderResult 欄位[​](#委託資訊-futoptorderresult-欄位 "Direct link to 委託資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                  | 說明                                                                                                                                            |
| --------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | int                                                                                                                   | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單、`90`失敗                                                                   |
| date            | string                                                                                                                | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                | 委託書號                                                                                                                                        |
| assetType       | int                                                                                                                   | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | int                                                                                                                   | 單位數                                                                                                                                          |
| currency        | string                                                                                                                | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                | 到期日                                                                                                                                          |
| strikePrice     | double                                                                                                                | 履約價                                                                                                                                          |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | double                                                                                                                | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                                                |
| buySellLeg2     | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | double                                                                                                                | 價格                                                                                                                                            |
| lot             | int                                                                                                                   | 原始委託股口數                                                                                                                                  |
| timeInForce     | [TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#timeinforce)           | 委託條件別 : `Rod` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                  | 是否為預約單                                                                                                                                    |
| status          | int                                                                                                                   | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | double                                                                                                                | 有效委託價格                                                                                                                                    |
| afterLot        | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| filledLot       | int                                                                                                                   | 成交口數                                                                                                                                        |
| filledMoney     | double                                                                                                                | 成交價金                                                                                                                                        |
| beforeLot       | int                                                                                                                   | 改單前有效口數                                                                                                                                  |
| beforePrice     | double                                                                                                                | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                  | 委託歷程 (查詢OrderResultDetail or OrderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
var orders = new FutOptOrder[]{
   new FutOptOrder(
    BsAction.Buy,
    "TXO20000E4",
    "530",
    1,
    FutOptMarketType.Option,
    FutOptPriceType.Limit,
    TimeInForce.Rod,
    FutOptOrderType.Auto,
    "FromCS" // optional field
), new FutOptOrder(
    BsAction.Buy,
    "TXO20000E4",
    "530",
    1,
    FutOptMarketType.Option,
    FutOptPriceType.Limit,
    TimeInForce.Rod,
    FutOptOrderType.Auto,
    "FromCS" // optional field
)};

sdk.Futopt.BatchPlaceOrder(account,orders.ToList());

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess = True,
    message = ,
    data =  [ FutOptOrderResult{
                functionType = 0,                            // 功能別 (int)
                date = 2024/03/25,                           // 交易日期 (string)
                seqNo = 00230177110,                         // 委託單流水序號 (string)
                branchNo = 15901,                            // 分公司代號 (string)
                account = 1234567,                           // 帳號 (string)
                orderNo = C0022,                             // 委託書號 (string)
                assetType = 2,                               // 資產類別 (int)
                market = TAIMEX,                             // 市場類型 (string)
                marketType = Option,                         // 盤別種類 (FutOptMarketType)
                unit = 1,                                    // 單位數 (int)
                currency = TWD,                              // 幣別 (string)
                symbol = TXO,                                // 商品代號 (string)
                expiryDate = 202404,                         // 到期日 (string)
                strikePrice = 20000,                        // 履約價 (double)
                callPut = Call,                              // 買賣權 (string)
                buySell = Buy,                               // 買賣別 (BsAction)
                symbolLeg2 =,                                // 商品代號 - 複式第二隻腳 (string)
                expiryDateLeg2 =,                             // 到期日 - 複式第二隻腳 (string)
                strikePriceLeg2 =,                            // 履約價 - 複式第二隻腳 (double)
                callPutLeg2 =,                                // 買賣權 - 複式第二隻腳 (string)
                buySellLeg2 =,                                // 買賣別 - 複式第二隻腳 (BsAction)
                priceType = Limit,                           // 原始委託價格別 (FutOptPriceType)
                price = 500,                                 // 價格 (double)
                lot = 1,                                     // 原始委託股口數 (int)
                timeInForce = Rod,                           // 委託條件別 (TimeInforce)
                orderType = Auto,                            // 委託單類型 (FutOptOrderType)
                isPreOrder = false,                          // 是否為預約單 (bool)
                status = 10,                                 // 委託單狀態 (int)
                afterPriceType = Limit,                      // 有效委託價格別 (FutOptPriceType)
                afterPrice = 500,                            // 有效委託價格 (double)
                afterLot = 1,                                // 有效委託股口數 (int)
                filledLot = 0,                               // 成交股口數 (int)
                filledMoney = 0,                             // 成交價金 (int)
                beforeLot = 0,                               // 改單前有效股口數 (int)
                beforePrice = 500,                           // 改單前有效價格 (double)
                userDef = From csharp,                       // 自訂欄位 (string)
                lastTime = 11:50:58,                         // 最後異動時間 (string)
                detail = ,                                   // 委託歷程 (list)
                errorMessage =                                 // 錯誤訊息 (string)
            },
            FutOptOrderResult{
                functionType = 0,                            // 功能別 (int)
                date = 2024/03/25,                           // 交易日期 (string)
                seqNo = 00230177111,                         // 委託單流水序號 (string)
                branchNo = 15901,                            // 分公司代號 (string)
                account = 1234567,                           // 帳號 (string)
                orderNo = C0023,                             // 委託書號 (string)
                assetType = 2,                               // 資產類別 (int)
                market = TAIMEX,                             // 市場類型 (string)
                marketType = Option,                         // 盤別種類 (FutOptMarketType)
                unit = 1,                                    // 單位數 (int)
                currency = TWD,                              // 幣別 (string)
                symbol = TXO,                                // 商品代號 (string)
                expiryDate = 202404,                         // 到期日 (string)
                strikePrice = 20000,                        // 履約價 (double)
                callPut = Call,                              // 買賣權 (string)
                buySell = Buy,                               // 買賣別 (BsAction)
                symbolLeg2 =,                                // 商品代號 - 複式第二隻腳 (string)
                expiryDateLeg2 =,                             // 到期日 - 複式第二隻腳 (string)
                strikePriceLeg2 =,                            // 履約價 - 複式第二隻腳 (double)
                callPutLeg2 =,                                // 買賣權 - 複式第二隻腳 (string)
                buySellLeg2 =,                                // 買賣別 - 複式第二隻腳 (BsAction)
                priceType = Limit,                           // 原始委託價格別 (FutOptPriceType)
                price = 500,                                 // 價格 (double)
                lot = 1,                                     // 原始委託股口數 (int)
                timeInForce = Rod,                           // 委託條件別 (TimeInforce)
                orderType = Auto,                            // 委託單類型 (FutOptOrderType)
                isPreOrder = false,                          // 是否為預約單 (bool)
                status = 10,                                 // 委託單狀態 (int)
                ...
            }
    ] 
}

```


---

### 刪除委託單

CancelOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                                    | 說明               |
| ----------- | ----------------------------------------------------------------------------------------------------------------------- | ------------------ |
| account     | Account                                                                                                                 | 帳號               |
| orderResult | [FutOptOrderResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptorderresult) | 欲取消的委託單物件 |
| unblock     | bool (optional) (default = false)                                                                                       | 是否採用非阻塞     |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別              | 說明                             |
| --------- | ----------------- | -------------------------------- |
| isSuccess | bool              | 是否成功                         |
| data      | FutOptOrderResult | 回傳修改資訊                     |
| message   | string            | 當isSuccess = False 回傳錯誤訊息 |

##### 修改資訊 FutOptOrderResult 欄位[​](#修改資訊-futoptorderresult-欄位 "Direct link to 修改資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                  | 說明                                                                                                                                            |
| --------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | int                                                                                                                   | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                | 委託書號                                                                                                                                        |
| assetType       | int                                                                                                                   | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | int                                                                                                                   | 單位數                                                                                                                                          |
| currency        | string                                                                                                                | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                | 到期日                                                                                                                                          |
| strikePrice     | double                                                                                                                | 履約價                                                                                                                                          |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | double                                                                                                                | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                                                |
| buySellLeg2     | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | double                                                                                                                | 價格                                                                                                                                            |
| lot             | int                                                                                                                   | 原始委託股口數                                                                                                                                  |
| timeInForce     | [TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#timeinforce)           | 委託條件別 : `Rod` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                  | 是否為預約單                                                                                                                                    |
| status          | int                                                                                                                   | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | double                                                                                                                | 有效委託價格                                                                                                                                    |
| afterLot        | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| filledLot       | int                                                                                                                   | 成交口數                                                                                                                                        |
| filledMoney     | double                                                                                                                | 成交價金                                                                                                                                        |
| beforeLot       | int                                                                                                                   | 改單前有效口數                                                                                                                                  |
| beforePrice     | double                                                                                                                | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                | 最後異動時間                                                                                                                                    |
| 新單執行        |                                                                                                                       |                                                                                                                                                 |
| errorMessage    | string                                                                                                                | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
Console.WriteLine(sdk.FutOpt.CancelOrder(account, cancel_order));

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess = True,
    message = ,
    data =   FutOptOrderResult {
                functionType = 30,                      // 功能別 (int)
                date = 2024/03/25,                      // 交易日期 (string)
                seqNo = 00110212608,                    // 委託單流水序號 (string)
                branchNo = 15901,                       // 分公司代號 (string)
                account = 1234567,                      // 帳號 (string)
                orderNo = C0001,                        // 委託書號 (string)
                assetType = 1,                          // 資產類別 (int)
                market = TAIMEX,                        // 市場類型 (string)
                marketType = Future,                    // 盤別種類 (FutOptMarketType)
                unit = 1,                               // 單位數 (int)
                currency = TWD,                         // 幣別 (string)
                symbol = FITF,                          // 商品代號 (string)
                expiryDate = 202404,                    // 到期日 (string)
                strikePrice = ,                         // 履約價 (double)
                callPut = ,                             // 買賣權 (string)
                buySell = Buy,                           // 買賣別 (BsAction)
                symbolLeg2 = ,                          // 商品代號 - 複式第二隻腳 (string)
                expiryDateLeg2 = ,                      // 到期日 - 複式第二隻腳 (string)
                strikePriceLeg2 = ,                     // 履約價 - 複式第二隻腳 (double)
                callPutLeg2 = ,                         // 買賣權 - 複式第二隻腳 (string)
                buySellLeg2 = ,                         // 買賣別 - 複式第二隻腳 (BsAction)
                priceType = Limit,                       // 原始委託價格別 (FutOptPriceType)
                price = 1822.6,                          // 價格 (double)
                lot = 2,                                 // 原始委託股口數 (int)
                timeInForce = Rod,                       // 委託條件別 (TimeInforce)
                orderType = Auto,                        // 委託單類型 (FutOptOrderType)
                isPreOrder = false,                      // 是否為預約單 (bool)
                status = 30,                             // 委託單狀態 (int)
                afterPriceType = ,                       // 有效委託價格別 (FutOptPriceType)
                afterPrice = 1822.6,                     // 有效委託價格 (double)
                afterLot = 0,                            // 有效委託股口數 (int)
                filledLot = 0,                           // 成交股口數 (int)
                filledMoney = 0,                         // 成交價金 (int)
                beforeLot = ,                        // 改單前有效股口數 (int)
                beforePrice = ,                      // 改單前有效價格 (double)
                userDef = ,                          // 自訂欄位 (string)
                lastTime = 13:21:34,                     // 最後異動時間 (string)
                detail = ,                                   // 委託歷程 (list)
                errorMessage = ,                     // 錯誤訊息 (string)
            }
}

```


---

### 商品代號轉換

ConvertSymbol

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                             | 說明                              |
| ----------- | ---------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| symbol      | string                                                                                                           | 帳務商品代號                      |
| expiryDate  | string                                                                                                           | 履約日                            |
| strikePrice | double ( Optional )                                                                                              | 履約價                            |
| callPut     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput) ( Optional ) | 買賣權 : `Call` Call 、 `Put` Put |

info

月份代號可參閱[參數對照表](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#month)

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數   | 類別   | 說明               |
| ------ | ------ | ------------------ |
| symbol | string | 行情與下單商品代號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
//期貨
sdk.FutOpt.ConvertSymbol("FITX","202404",null, null)

//選擇權
sdk.FutOpt.ConvertSymbol("TXO","202404",20000,CallPut.Call)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs

//期貨
"TXFD4"

//選擇權
"TXO20000D4"

```


---

### 商品保證金查詢

QueryEstimateMargin

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                        | 說明     |
| ----------- | ----------------------------------------------------------------------------------------------------------- | -------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)         | 帳號     |
| orderObject | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#orderobject) | 委託內容 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別           | 說明                             |
| --------- | -------------- | -------------------------------- |
| isSuccess | bool           | 是否成功                         |
| data      | EstimateMargin | 回傳保證金資訊                   |
| message   | string         | 當isSuccess = False 回傳錯誤訊息 |

##### EstimateMargin 欄位[​](#estimatemargin-欄位 "Direct link to EstimateMargin 欄位")

Return type : Object

| 參數           | 類別   | 說明       |
| -------------- | ------ | ---------- |
| date           | string | 查詢日期   |
| currency       | string | 幣別       |
| estimateMargin | double | 預估保證金 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
order = FutOptOrder(
    BsAction.Buy,
    "TXFE4",
    "20890",
    1,
    FutOptMarketType.Future,
    FutOptPriceType.Limit,
    TimeInForce.Rod,
    FutOptOrderType.Auto,
    "FromCS" // optional field
)



var estimate = sdk.Futopt.QueryEstimateMargin(account, order);
Console.WriteLine(estimate.data);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess = True,
    message = ,
    data =  EstimateMargin{ 
                date = 2024/04/10,        // 日期 (string)
                currency = TWD,           // 幣別 (string)
                estimateMargin = 179000   // 預估保證金 (double)
            }
} 

```


---

### 查詢歷史成交

FilledHistory

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                 | 說明                                                                                                      |
| ---------- | -------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                  | 帳號                                                                                                      |
| marketType | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `FutureNight` 期貨夜盤 、 `Option` 選擇權日盤 、 `OptionNight` 選擇權夜盤 |
| startDate  | string                                                                                                               | 查詢開始日                                                                                                |
| endDate    | string (空值預設與開始日相同)                                                                                        | 查詢終止日                                                                                                |

info

可查詢最近兩日之歷史資料

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別   | 說明                             |
| --------- | ------ | -------------------------------- |
| isSuccess | bool   | 是否成功                         |
| data      | List   | 回傳成交資訊                     |
| message   | string | 當isSuccess = False 回傳錯誤訊息 |

##### 成交資訊 FilledData 欄位[​](#成交資訊-filleddata-欄位 "Direct link to 成交資訊 FilledData 欄位")

Return type : Object

| 參數            | 類別                                                                                                                | 說明                                                        |
| --------------- | ------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
| date            | string                                                                                                              | 成交日期                                                    |
| branchNo        | string                                                                                                              | 分公司代號                                                  |
| account         | string                                                                                                              | 帳號                                                        |
| seqNo           | string ?                                                                                                            | 委託單流水序號 (只有主動回報才回傳此欄位)                   |
| orderNo         | string                                                                                                              | 委託書號                                                    |
| symbol          | string                                                                                                              | 商品代號                                                    |
| expiryDate      | string                                                                                                              | 到期日                                                      |
| strikePrice     | number                                                                                                              | 履約價                                                      |
| callPut         | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                 | 買賣權 : `Call` 買權、 `Put` 賣權                           |
| buySell         | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)               | 買賣別 : `Buy` 買 、 `Sell` 賣                              |
| symbolLeg2      | string                                                                                                              | 商品代號 - 複式第二隻腳                                     |
| expiryDateLeg2  | string                                                                                                              | 到期日 - 複式第二隻腳                                       |
| strikePriceLeg2 | number                                                                                                              | 履約價 - 複式第二隻腳                                       |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                 | 買賣權- 複式第二隻腳: `Call` 買權、 `Put` 賣權              |
| buySellLeg2     | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)               | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                |
| orderType       | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptordertype) | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `FdayTrade` 當沖 |
| filledNo        | string                                                                                                              | 成交流水號                                                  |
| filledAvgPrice  | number                                                                                                              | 成交均價                                                    |
| filledLot       | number                                                                                                              | 成交股數                                                    |
| filledPrice     | number                                                                                                              | 成交單價                                                    |
| filledTime      | string                                                                                                              | 成交時間                                                    |
| userDef         | string ?                                                                                                            | 用戶自定義 (只有主動回報才回傳此欄位)                       |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
Console.WriteLine(sdk.Futopt.FilledHistory(account,FutOptMarketType.Future,"20230921","20230922"));

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess = True,
    message = ,
    data =  [
                FutOptFilledData{ 
                        date = 2023/09/15,               // 成交日期 (string)
                        branchNo = 6460,                 // 分公司代號 (string)
                        account = 26,                    // 帳號 (string)
                        orderNo = bA422,                 // 委託書號 (string)
                        seqNo = ,                        // 委託單流水序號 (string)
                        symbol = FITX,                   // 商品代號 (string)
                        expiryDate = 202404,             // 履約日 (string)
                        strikePrice = ,                  // 履約價 (double)
                        callPut = ,                      // 買賣權 (CallPut)
                        buySell = Buy,                   // 買賣別 (BsAction)
                        symbolLeg2 = ,                   // 商品代號 - 複式第二隻腳 (string)
                        expiryDateLeg2 = ,               // 履約日 - 複式第二隻腳 (string)
                        strikePriceLeg2 = ,              // 履約價 - 複式第二隻腳 (double)
                        callPutLeg2 = ,                  // 買賣權 - 複式第二隻腳 (CallPut)
                        buySellLeg2 = ,                  // 買賣別 - 複式第二隻腳 (BsAction)
                        filledNo = 00000000001,          // 成交流水號 (string)
                        filledAvgPrice = 20890.0,        // 成交均價 (double)
                        filledLots = 1,                  // 成交股數 (int)
                        filledPrice = 20890.0,           // 成交單價 (double)
                        orderType = New,                 // 委託單類型 (FutOptOrderType)
                        filledTime = 10:31:00.931,       // 成交時間 (string)
                        userDef =                        // 用戶自定義 (string)
                },
                ...
            ]
}

```


---

### 取得委託單結果

GetOrderResult

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                                         | 說明                                                                                                      |
| ---------- | -------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                                          | 帳號                                                                                                      |
| marketType | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmarkettype) (Optional : 不帶全盤別) | 盤別種類 : `Future` 期貨日盤 、 `FutureNight` 期貨夜盤 、 `Option` 選擇權日盤 、 `OptionNight` 選擇權夜盤 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別   | 說明                             |
| --------- | ------ | -------------------------------- |
| isSuccess | bool   | 是否成功                         |
| data      | List   | 回傳委託資訊                     |
| message   | string | 當isSuccess = False 回傳錯誤訊息 |

##### 委託資訊 FutOptOrderResult 欄位[​](#委託資訊-futoptorderresult-欄位 "Direct link to 委託資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                  | 說明                                                                                                                                            |
| --------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | int (optional)                                                                                                        | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                | 委託書號                                                                                                                                        |
| assetType       | int                                                                                                                   | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | int                                                                                                                   | 單位數                                                                                                                                          |
| currency        | string                                                                                                                | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                | 到期日                                                                                                                                          |
| strikePrice     | double                                                                                                                | 履約價                                                                                                                                          |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | double                                                                                                                | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                                                |
| buySellLeg2     | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | double                                                                                                                | 價格                                                                                                                                            |
| lot             | int                                                                                                                   | 原始委託股口數                                                                                                                                  |
| timeInForce     | [TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#timeinforce)           | 委託條件別 : `Rod` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                  | 是否為預約單                                                                                                                                    |
| status          | int                                                                                                                   | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | double                                                                                                                | 有效委託價格                                                                                                                                    |
| afterLot        | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| filledLot       | int                                                                                                                   | 成交口數                                                                                                                                        |
| filledMoney     | double                                                                                                                | 成交價金                                                                                                                                        |
| beforeLot       | int                                                                                                                   | 改單前有效口數                                                                                                                                  |
| beforePrice     | double                                                                                                                | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                  | 委託歷程 (查詢OrderResultDetail or OrderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
var orderResults = sdk.FutOpt.GetOrderResults(account);

Console.WriteLine(orderResults);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess = True,
    message = ,
    data =  [ FutOptOrderResult{
                functionType = ,                       // 功能類型 (int)
                date = 2024/03/25,                     // 交易日期 (string)
                seqNo = 00110212608,                   // 委託單流水序號 (string)
                branchNo = 15901,                      // 分公司代號 (string)
                account = 1234567,                     // 帳號 (string)
                orderNo = C0001,                       // 委託書號 (string)
                assetType = 1,                         // 資產類別 (int)
                market = TAIMEX,                       // 市場類型 (string)
                marketType = Future,                   // 盤別種類 (FutOptOrderType)
                unit = 1,                              // 單位數 (int)
                currency = TWD,                        // 幣別 (string)
                symbol = FITF,                         // 商品代號 (string)
                expiryDate = 202404,                   // 到期日 (string)
                strikePrice = ,                        // 履約價 (double)
                callPut = ,                            // 買賣權 (string)
                buySell = Buy,                         // 買賣別 (BsAction)
                symbolLeg2 = ,                         // 商品代號 - 複式第二隻腳 (string)
                expiryDateLeg2 = ,                     // 到期日 - 複式第二隻腳 (string)
                strikePriceLeg2 = ,                    // 履約價 - 複式第二隻腳 (double)
                callPutLeg2 = ,                        // 買賣權 - 複式第二隻腳 (string)
                buySellLeg2 = ,                        // 買賣別 - 複式第二隻腳 (BsAction)
                priceType = Limit,                     // 原始委託價格別 (PriceType)
                price = 1822.6,                        // 價格 (double)
                lot = 2,                               // 原始委託股口數 (int)
                timeInForce = Rod,                     // 委託條件別 (TimeInforce)
                orderType = Auto,                      // 委託單類型 (FutOptOrderType)
                isPreOrder = false,                    // 是否為預約單 (bool)
                status = 10,                           // 委託單狀態 (int)
                afterPriceType = ,                     // 有效委託價格別 (PriceType)
                afterPrice = 1822.6,                   // 有效委託價格 (double)
                afterLot = 2,                          // 有效委託股口數 (int)
                filledLot = 0,                         // 成交股口數 (int)
                filledMoney = 0,                       // 成交價金 (int)
                beforeLot = ,                         // 改單前有效股口數 (int)
                beforePrice = ,                       // 改單前有效價格 (double)
                userDef = ,                           // 自訂欄位 (string)
                lastTime = 10:20:27,                    // 最後異動時間 (string)
                detail = ,                             // 委託歷程 (list)
                errorMessage =                        // 錯誤訊息 (string)
            },
            ...
        ] 
}

```


---

### 取得委託單結果 (含歷程)

GetOrderResultDetail

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                                        | 說明                                                                                                      |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                                         | 帳號                                                                                                      |
| marketType | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmarkettype)(Optioanl : 不帶全盤別) | 盤別種類 : `Future` 期貨日盤 、 `FutureNight` 期貨夜盤 、 `Option` 選擇權日盤 、 `OptionNight` 選擇權夜盤 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別   | 說明                             |
| --------- | ------ | -------------------------------- |
| isSuccess | bool   | 是否成功                         |
| data      | List   | 回傳委託資訊                     |
| message   | string | 當isSuccess = False 回傳錯誤訊息 |

##### 委託資訊 FutOptOrderResult 欄位[​](#委託資訊-futoptorderresult-欄位 "Direct link to 委託資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                  | 說明                                                                                                                                            |
| --------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | int (Optional)                                                                                                        | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                | 委託書號                                                                                                                                        |
| assetType       | int                                                                                                                   | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | int                                                                                                                   | 單位數                                                                                                                                          |
| currency        | string                                                                                                                | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                | 到期日                                                                                                                                          |
| strikePrice     | double                                                                                                                | 履約價                                                                                                                                          |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | double                                                                                                                | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                                                |
| buySellLeg2     | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | double                                                                                                                | 價格                                                                                                                                            |
| lot             | int                                                                                                                   | 原始委託股口數                                                                                                                                  |
| timeInForce     | [TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#timeinforce)           | 委託條件別 : `Rod` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                  | 是否為預約單                                                                                                                                    |
| status          | int                                                                                                                   | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | double                                                                                                                | 有效委託價格                                                                                                                                    |
| afterLot        | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| filledLot       | int                                                                                                                   | 成交口數                                                                                                                                        |
| filledMoney     | double                                                                                                                | 成交價金                                                                                                                                        |
| beforeLot       | int                                                                                                                   | 改單前有效口數                                                                                                                                  |
| beforePrice     | double                                                                                                                | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                | 最後異動時間                                                                                                                                    |
| errorMessage    | string                                                                                                                | 錯誤訊息                                                                                                                                        |
| detail          | list                                                                                                                  | 委託歷程                                                                                                                                        |
| >> functionType | int                                                                                                                   | 功能別 : `10` 新單、`15` 改價、 `20` 改量、`30`刪單、`50` 成交 、`90`失敗                                                                       |
| >> modifiedTime | string                                                                                                                | 修改時間                                                                                                                                        |
| >> beforeLot    | int                                                                                                                   | 原始委託口數                                                                                                                                    |
| >> afterLot     | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| >> beforePrice  | double                                                                                                                | 原始委託價                                                                                                                                      |
| >> afterPrice   | double                                                                                                                | 有效委託價                                                                                                                                      |
| >> filledMoney  | double                                                                                                                | 成交價金                                                                                                                                        |
| >> errorMessage | string                                                                                                                | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
var orderResults = sdk.FutOpt.GetOrderResultsDetail(account);

Console.WriteLine(orderResults);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess = True,
    message = ,
    data =  [ FutOptOrderResult{
                functionType = ,                       // 功能類型 (int)
                date = 2024/03/25,                     // 交易日期 (string)
                seqNo = 00110212608,                   // 委託單流水序號 (string)
                branchNo = 15901,                      // 分公司代號 (string)
                account = 1234567,                     // 帳號 (string)
                orderNo = C0001,                       // 委託書號 (string)
                assetType = 1,                         // 資產類別 (int)
                market = TAIMEX,                       // 市場類型 (string)
                marketType = Future,                   // 盤別種類 (FutOptOrderType)
                unit = 1,                              // 單位數 (int)
                currency = TWD,                        // 幣別 (string)
                symbol = FITF,                         // 商品代號 (string)
                expiryDate = 202404,                   // 到期日 (string)
                strikePrice = ,                        // 履約價 (double)
                callPut = ,                            // 買賣權 (string)
                buySell = Buy,                         // 買賣別 (BsAction)
                symbolLeg2 = ,                         // 商品代號 - 複式第二隻腳 (string)
                expiryDateLeg2 = ,                     // 到期日 - 複式第二隻腳 (string)
                strikePriceLeg2 = ,                    // 履約價 - 複式第二隻腳 (double)
                callPutLeg2 = ,                        // 買賣權 - 複式第二隻腳 (string)
                buySellLeg2 = ,                        // 買賣別 - 複式第二隻腳 (BsAction)
                priceType = Limit,                     // 原始委託價格別 (PriceType)
                price = 1822.6,                        // 價格 (double)
                lot = 2,                               // 原始委託股口數 (int)
                timeInForce = Rod,                     // 委託條件別 (TimeInforce)
                orderType = Auto,                      // 委託單類型 (FutOptOrderType)
                isPreOrder = false,                    // 是否為預約單 (bool)
                status = 10,                           // 委託單狀態 (int)
                afterPriceType = ,                     // 有效委託價格別 (PriceType)
                afterPrice = 1822.6,                   // 有效委託價格 (double)
                afterLot = 2,                          // 有效委託股口數 (int)
                filledLot = 0,                         // 成交股口數 (int)
                filledMoney = 0,                       // 成交價金 (int)
                beforeLot = ,                         // 改單前有效股口數 (int)
                beforePrice = ,                       // 改單前有效價格 (double)
                userDef = ,                           // 自訂欄位 (string)
                lastTime = 10:20:27,                    // 最後異動時間 (string)
                errorMessage =                        // 錯誤訊息 (string)
                detail = [                            // 委託歷程 (List)
                    OrderDetail{
                        functionType = 10,            // 功能別 (int)
                        modifiedTime = 10:20:27,      // 修改時間 (string)
                        beforeLot = 0,                // 原始委託口數 (int)
                        afterLot = 2,                 // 有效委託口數 (int)
                        beforePrice = 1822.6,         // 原始委託價格 (double)
                        afterPrice = 1822.6,           // 有效委託價格 (double)
                        errorMessage =                 // 錯誤訊息 (string)
                    }
                ]
            },
            ...
        ] 
}

```


---

### 修改委託價格

ModifyPrice

##### 先使用 MakeModifyPriceObj 建立 FutOptModifyPrice 物件[​](#先使用-makemodifypriceobj-建立-futoptmodifyprice-物件 "Direct link to 先使用 MakeModifyPriceObj 建立 FutOptModifyPrice 物件")

| 參數        | 類別                                                                                                                    | 說明             |
| ----------- | ----------------------------------------------------------------------------------------------------------------------- | ---------------- |
| orderResult | [FutOptOrderResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptorderresult) | 預計修改的委託單 |
| price       | string                                                                                                                  | 修改後的價格     |
| priceType   | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)     | 修改後的價格旗標 |

caution

當 price 欄位有填入值時，priceType 需為 ； 當 priceType 欄位有填入值時，price 需為

將回傳的物件放入 modifyPrice 的方法中

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數             | 類別                                                                                                                    | 說明           |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------- | -------------- |
| account          | Account                                                                                                                 | 帳號           |
| modifiedPriceObj | [FutOptModifyPrice](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmodifyprice) | 修改價格的物件 |
| unblock          | bool (optional) (default = false)                                                                                       | 是否採用非阻塞 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別              | 說明                             |
| --------- | ----------------- | -------------------------------- |
| isSuccess | bool              | 是否成功                         |
| data      | FutOptOrderResult | 回傳修改資訊                     |
| message   | string            | 當isSuccess = False 回傳錯誤訊息 |

##### 修改資訊 FutOptOrderResult 欄位[​](#修改資訊-futoptorderresult-欄位 "Direct link to 修改資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                  | 說明                                                                                                                                            |
| --------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | int                                                                                                                   | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                | 委託書號                                                                                                                                        |
| assetType       | int                                                                                                                   | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | int                                                                                                                   | 單位數                                                                                                                                          |
| currency        | string                                                                                                                | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                | 到期日                                                                                                                                          |
| strikePrice     | double                                                                                                                | 履約價                                                                                                                                          |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | double                                                                                                                | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                                                |
| buySellLeg2     | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | double                                                                                                                | 價格                                                                                                                                            |
| lot             | int                                                                                                                   | 原始委託股口數                                                                                                                                  |
| timeInForce     | [TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#timeinforce)           | 委託條件別 : `Rod` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                  | 是否為預約單                                                                                                                                    |
| status          | int                                                                                                                   | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | double                                                                                                                | 有效委託價格                                                                                                                                    |
| afterLot        | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| filledLot       | int                                                                                                                   | 成交口數                                                                                                                                        |
| filledMoney     | double                                                                                                                | 成交價金                                                                                                                                        |
| beforeLot       | int                                                                                                                   | 改單前有效口數                                                                                                                                  |
| beforePrice     | double                                                                                                                | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                  | 委託歷程 (查詢OrderResultDetail or OrderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
var modify_price_obj = sdk.Futopt.MakeModifyPriceObj(order_result, "19900" , null );
var modify_price     = sdk.Futopt.ModifyPrice(account, modify_price_obj);
Console.WriteLine(modify_price);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess = True,
    message = ,
    data = FutOptOrderResult {
                functionType = 15,                        // 功能別 (int)
                date = 2024/03/25,                        // 交易日期 (string)
                seqNo = 00110212663,                      // 委託單流水序號 (string)
                branchNo = 15901,                         // 分公司代號 (string)
                account = 1234567,                        // 帳號 (string)
                orderNo = C0005,                          // 委託書號 (string)
                assetType = 1,                            // 資產類別 (int)
                market = TAIMEX,                          // 市場類型 (string)
                marketType = Future,                      // 盤別種類 (FutOptMarketType)
                unit = ,                                  // 單位數 (int)
                currency = TWD,                           // 幣別 (string)
                symbol = FITX,                            // 商品代號 (string)
                expiryDate = ,                            // 到期日 (string)
                strikePrice = ,                           // 履約價 (double)
                callPut = ,                               // 買賣權 (string)
                buySell = Buy,                            // 買賣別 (BsAction)
                symbolLeg2 = ,                            // 商品代號 - 複式第二隻腳 (string)
                expiryDateLeg2 = ,                        // 到期日 - 複式第二隻腳 (string)
                strikePriceLeg2 = ,                       // 履約價 - 複式第二隻腳 (double)
                callPutLeg2 = ,                           // 買賣權 - 複式第二隻腳 (string)
                buySellLeg2 = ,                           // 買賣別 - 複式第二隻腳 (BsAction)
                priceType = Limit,                        // 原始委託價格別 (FutOptPriceType)
                price = 20000,                            // 價格 (double)
                lot = 1,                                  // 原始委託股口數 (int)
                timeInForce = ROD,                        // 委託條件別 (TimeInforce)
                orderType = New,                          // 委託單類型 (FutOptOrderType)
                isPreOrder = false,                       // 是否為預約單 (bool)
                status = 10,                              // 委託單狀態 (int)
                afterPriceType = Limit,                   // 有效委託價格別 (FutOptPriceType)
                afterPrice = 19900,                       // 有效委託價格 (double)
                afterLot = 1,                             // 有效委託股口數 (int)
                filledLot = 0,                            // 成交股口數 (int)
                filledMoney = 0,                          // 成交價金 (int)
                beforeLot = 0,                            // 改單前有效股口數 (int)
                beforePrice = 20000,                      // 改單前有效價格 (double)
                userDef = ,                               // 自訂欄位 (string)
                lastTime = 13:39:05,                      // 最後異動時間 (string)
                detail = ,                                   // 委託歷程 (list)
                errorMessage =                            // 錯誤訊息 (string)
            }
}

```


---

### 修改委託單數量

ModifyQuantity

##### 先使用 MakeModifyLotObj 建立 FutOptModifyLot 物件[​](#先使用-makemodifylotobj-建立-futoptmodifylot-物件 "Direct link to 先使用 MakeModifyLotObj 建立 FutOptModifyLot 物件")

| 參數        | 類別                                                                                                                    | 說明                                                |
| ----------- | ----------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| orderResult | [FutOptOrderResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptorderresult) | 預計修改的委託單                                    |
| lots        | int                                                                                                                     | 修改後的委託量 ( 修改後數量包含此委託單已成交部份 ) |

將回傳的物件放入 ModifyLot 的方法中

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數         | 類別                                                                                                                | 說明           |
| ------------ | ------------------------------------------------------------------------------------------------------------------- | -------------- |
| account      | Account                                                                                                             | 帳號           |
| ModifyLotObj | [FutOptModifyLot](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmodifylot) | 修改的委託單   |
| unblock      | bool (optional) (default = false)                                                                                   | 是否採用非阻塞 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別              | 說明                             |
| --------- | ----------------- | -------------------------------- |
| isSuccess | bool              | 是否成功                         |
| data      | FutOptOrderResult | 回傳修改資訊                     |
| message   | string            | 當isSuccess = False 回傳錯誤訊息 |

##### 修改資訊 FutOptOrderResult 欄位[​](#修改資訊-futoptorderresult-欄位 "Direct link to 修改資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                  | 說明                                                                                                                                            |
| --------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | int                                                                                                                   | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                | 委託書號                                                                                                                                        |
| assetType       | int                                                                                                                   | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | int                                                                                                                   | 單位數                                                                                                                                          |
| currency        | string                                                                                                                | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                | 到期日                                                                                                                                          |
| strikePrice     | double                                                                                                                | 履約價                                                                                                                                          |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | double                                                                                                                | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                                                |
| buySellLeg2     | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | double                                                                                                                | 價格                                                                                                                                            |
| lot             | int                                                                                                                   | 原始委託股口數                                                                                                                                  |
| timeInForce     | [TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#timeinforce)           | 委託條件別 : `Rod` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                  | 是否為預約單                                                                                                                                    |
| status          | int                                                                                                                   | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | double                                                                                                                | 有效委託價格                                                                                                                                    |
| afterLot        | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| filledLot       | int                                                                                                                   | 成交口數                                                                                                                                        |
| filledMoney     | double                                                                                                                | 成交價金                                                                                                                                        |
| beforeLot       | int                                                                                                                   | 改單前有效口數                                                                                                                                  |
| beforePrice     | double                                                                                                                | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                  | 委託歷程 (查詢OrderResultDetail or OrderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
var modify_qty_obj  = sdk.FutOpt.MakeModifyLotObj(order_result, 2);
var modify_qty      = sdk.FutOpt.ModifyLot(account, modify_qty_obj, true);
Console.WriteLine(modify_qty);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess = True,
    message = ,
    data =   FutOptOrderResult{
                functionType = 20,                        // 功能別 (int)
                date = 2024/03/25,                        // 交易日期 (string)
                seqNo = 03100161319,                      // 委託單流水序號 (string)
                branchNo = 15901,                         // 分公司代號 (string)
                account = 1234567,                        // 帳號 (string)
                orderNo = l001D,                          // 委託書號 (string)
                assetType = 1,                            // 資產類別 (int)
                market = TAIMEX,                          // 市場類型 (string)
                marketType = FutureNight,                 // 盤別種類 (MarketType)
                unit =  =,                                // 單位數 (int)
                currency = TWD,                           // 幣別 (string)
                symbol = FIMTX,                           // 商品代號 (string)
                expiryDate =  =,                          // 到期日 (string)
                strikePrice =  =,                         // 履約價 (double)
                callPut =  =,                             // 買賣權 (Callput)
                buySell = Buy,                            // 買賣別 (BsAction)
                symbolLeg2 =  =,                          // 商品代號 - 複式第二隻腳 (string)
                expiryDateLeg2 =  =,                      // 到期日 - 複式第二隻腳 (string)
                strikePriceLeg2 =  =,                     // 履約價 - 複式第二隻腳 (double)
                callPutLeg2 =  =,                         // 買賣權 - 複式第二隻腳 (CallPut)
                buySellLeg2 =  =,                         // 買賣別 - 複式第二隻腳 (BsAction)
                priceType = Limit,                        // 原始委託價格別 (FutOptPriceType)
                price = 20000,                            // 價格 (double)
                lot = 3,                                  // 原始委託股口數 (int)
                timeInForce = ROD,                        // 委託條件別 (TimeInforce)
                orderType = New,                          // 委託單類型 (FutOptOrderType)
                isPreOrder = false,                       // 是否為預約單 (bool)
                status = 10,                              // 委託單狀態 (int)
                afterPriceType = Limit,                   // 有效委託價格別 (FutOptPriceType)
                afterPrice = 20000,                       // 有效委託價格 (double)
                afterLot = 2,                             // 有效委託股口數 (int)
                filledLot = 0,                            // 成交股口數 (int)
                filledMoney = 0,                          // 成交價金 (int)
                beforeLot = 0,                            // 改單前有效股口數 (int)
                beforePrice = 20000,                      // 改單前有效價格 (double)
                userDef =  =,                             // 自訂欄位 (string)
                lastTime = 18:24:40,                      // 最後異動時間 (string)
                detail = ,                                   // 委託歷程 (list)
                errorMessage =  =                         // 錯誤訊息 (string)
            }
}

```


---

### 查詢歷史委託

OrderHistory

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                                          | 說明                                                                                                      |
| ---------- | --------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)                                           | 帳號                                                                                                      |
| startDate  | string                                                                                                                                        | 查詢開始日                                                                                                |
| endDate    | string                                                                                                                                        | 查詢終止日                                                                                                |
| marketType | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmarkettype) (Optioanl : 不帶全盤別) | 盤別種類 : `Future` 期貨日盤 、 `FutureNight` 期貨夜盤 、 `Option` 選擇權日盤 、 `OptionNight` 選擇權夜盤 |

info

可查詢最近兩日之歷史資料

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別   | 說明                             |
| --------- | ------ | -------------------------------- |
| isSuccess | bool   | 是否成功                         |
| data      | List   | 回傳委託資訊                     |
| message   | string | 當isSuccess = False 回傳錯誤訊息 |

##### 委託資訊 FutOptOrderResult 欄位[​](#委託資訊-futoptorderresult-欄位 "Direct link to 委託資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                  | 說明                                                                                                                                            |
| --------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | int (Optional)                                                                                                        | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                | 委託書號                                                                                                                                        |
| assetType       | int                                                                                                                   | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | int                                                                                                                   | 單位數                                                                                                                                          |
| currency        | string                                                                                                                | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                | 到期日                                                                                                                                          |
| strikePrice     | double                                                                                                                | 履約價                                                                                                                                          |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | double                                                                                                                | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                                                |
| buySellLeg2     | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | double                                                                                                                | 價格                                                                                                                                            |
| lot             | int                                                                                                                   | 原始委託股口數                                                                                                                                  |
| timeInForce     | [TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#timeinforce)           | 委託條件別 : `Rod` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                  | 是否為預約單                                                                                                                                    |
| status          | int                                                                                                                   | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | double                                                                                                                | 有效委託價格                                                                                                                                    |
| afterLot        | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| filledLot       | int                                                                                                                   | 成交口數                                                                                                                                        |
| filledMoney     | double                                                                                                                | 成交價金                                                                                                                                        |
| beforeLot       | int                                                                                                                   | 改單前有效口數                                                                                                                                  |
| beforePrice     | double                                                                                                                | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                | 最後異動時間                                                                                                                                    |
| errorMessage    | string                                                                                                                | 錯誤訊息                                                                                                                                        |
| detail          | list                                                                                                                  | 委託歷程                                                                                                                                        |
| >> functionType | int                                                                                                                   | 功能別 : `10` 新單、`15` 改價、 `20` 改量、`30`刪單、`50` 成交 、`90`失敗                                                                       |
| >> modifiedTime | string                                                                                                                | 修改時間                                                                                                                                        |
| >> beforeLot    | int                                                                                                                   | 原始委託口數                                                                                                                                    |
| >> afterLot     | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| >> beforePrice  | double                                                                                                                | 原始委託價                                                                                                                                      |
| >> afterPrice   | double                                                                                                                | 有效委託價                                                                                                                                      |
| >> filledMoney  | double                                                                                                                | 成交價金                                                                                                                                        |
| >> errorMessage | string                                                                                                                | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs
Console.WriteLine(sdk.FutOpt.OrderHistory(account,"20230921","20230922"));

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{
    isSuccess = True,
    message = ,
    data =  [ FutOptOrderResult{
                functionType = ,                        // 功能別 (int)
                date = 2024/04/11,                      // 交易日期 (string)
                seqNo = 00230177314,                    // 委託單流水序號 (string)
                branchNo = 15000,                       // 分公司代號 (string)
                account = 9974825,                      // 帳號 (string)
                orderNo = C0020,                        // 委託書號 (string)
                assetType = 2,                          // 資產類別 :  `1` 期貨 、`2` 選擇權 (int)
                market = TAIMEX,                        // 市場類型 :  `TAIMEX` 期貨、選擇權 (string)
                marketType = Option,                    // 盤別種類  :  `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤 (FutOptMarketType)
                unit = ,                                // 單位數 (int)
                currency = ,                            // 幣別 (string)
                symbol = TXO,                           // 商品代號 (string)
                expiryDate = 202404,                    // 到期日 (string)
                strikePrice = 18600,                    // 履約價 (double)
                callPut = Call,                         // 買賣權 :  `Call` 買權、 `Put` 賣權 (string)
                buySell = Buy,                          // 買賣別 :  `Buy` 買 、 `Sell` 賣 (BsAction)
                symbolLeg2 = ,                          // 商品代號 - 複式第二隻腳 (string)
                expiryDateLeg2 = ,                      // 到期日 -  複式第二隻腳 (string)
                strikePriceLeg2 = ,                     // 履約價 - 複式第二隻腳 (double)
                callPutLeg2 = ,                         // 買賣權 - 複式第二隻腳 :  `Call` 買權、 `Put` 賣權 (string)
                buySellLeg2 = ,                         // 買賣別 - 複式第二隻腳:  `Buy` 買 、 `Sell` 賣 (BsAction)
                priceType = Limit,                      // 原始委託價格別  : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價 (FutOptPriceType)
                price = 2100,                           // 價格 (double)
                lot = 1,                                // 原始委託股口數 (int)
                timeInForce = ROD,                      // 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC (TimeInforce)
                orderType = New,                        // 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖 (FutOptOrderType)
                isPreOrder = false,                     // 是否為預約單 (bool)
                status = 50,                            // 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、`9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功  、 `50` 完全成交 、 `90` 失敗 (int)
                afterPriceType = ,                      // 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價 (FutOptPriceType)
                afterPrice = 2100,                      // 有效委託價格 (double)
                afterLot = 1,                           // 有效委託口數 (int)
                filledLot = 1,                          // 成交口數 (int)
                filledMoney = 2100,                     // 成交價金 (double)
                beforeLot = ,                           // 改單前有效口數 (int)
                beforePrice = ,                         // 改單前有效價 (double)
                userDef = ,                             // 自訂欄位 (string)
                lastTime = 10:41:46.760,                // 最後異動時間 (string)
                detail = [                                  // 委託歷程 (List)
                    OrderDetail{
                        functionType = 10,            // 功能別 (int)
                        modifiedTime = 10:20:27,      // 修改時間 (string)
                        beforeLot = 0,                // 原始委託口數 (int)
                        afterLot = 2,                 // 有效委託口數 (int)
                        beforePrice = 1822.6,         // 原始委託價格 (double)
                        afterPrice = 1822.6           // 有效委託價格 (double)
                        errorMessage =                 // 錯誤訊息 (string)
                    }
                ]
                errorMessage =                          // 錯誤訊息 (string)
            },
            FutOptOrderResult{
                functionType = ,                        // 功能別 (int)
                date = 2024/04/11,                      // 交易日期 (string)
                seqNo = 00230177315,                    // 委託單流水序號 (string)
                branchNo = 15000,                       // 分公司代號 (string)
                account = 9974825,                      // 帳號 (string)
                orderNo = C0021,                        // 委託書號 (string)
                assetType = 2,                          // 資產類別 :  `1` 期貨 、`2` 選擇權 (int)
                market = TAIMEX,                        // 市場類型 :  `TAIMEX` 期貨、選擇權 (string)
                marketType = Option,                    // 盤別種類  :  `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤 (FutOptMarketType)
                unit = ,                                // 單位數 (int)
                currency = ,                            // 幣別 (string)
                symbol = TXO,                           // 商品代號 (string)
                expiryDate = 202404,                    // 到期日 (string)
                strikePrice = 18500,                    // 履約價 (double)
                callPut = Call,                         // 買賣權 :  `Call` 買權、 `Put` 賣權 (string)
                buySell = Sell,                         // 買賣別 :  `Buy` 買 、 `Sell` 賣 (BsAction)
                symbolLeg2 = ,                          // 商品代號 - 複式第二隻腳 (string)
                expiryDateLeg2 = ,                      // 到期日 -  複式第二隻腳 (string)
                strikePriceLeg2 = ,                     // 履約價 - 複式第二隻腳 (double)
                callPutLeg2 = ,                         // 買賣權 - 複式第二隻腳 :  `Call` 買權、 `Put` 賣權 (string)
                buySellLeg2 = ,                         // 買賣別 - 複式第二隻腳:  `Buy` 買 、 `Sell` 賣 (BsAction)
                priceType = Limit,                      // 原始委託價格別  : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價 (FutOptPriceType)
                price = 2230,                           // 價格 (double)
                lot = 1,                                // 原始委託股口數 (int)
                timeInForce = ROD,                      // 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC (TimeInforce)
                orderType = New,                        // 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖 (FutOptOrderType)
                isPreOrder = false,                     // 是否為預約單 (bool)
                status = 50,                            // 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、`9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功  、 `50` 完全成交 、 `90` 失敗 (int)
                afterPriceType = ,                      // 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價 (FutOptPriceType)
                afterPrice = 2230,                       // 有效委託價格 (double)
                afterLot = 1,                            // 有效委託口數 (int)
                filledLot = 1,                           // 成交口數 (int)
                filledMoney = 2230,                     // 成交價金 (double)
                beforeLot = ,                           // 改單前有效口數 (int)
                beforePrice = ,                         // 改單前有效價 (double)
                userDef = ,                             // 自訂欄位 (string)
                lastTime = 10:41:46.760,               // 最後異動時間 (string)
                errorMessage =                         // 錯誤訊息 (string)
            },
            ...
            ]
} 

```


---

### 建立委託單

PlaceOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                        | 說明           |
| ----------- | ----------------------------------------------------------------------------------------------------------- | -------------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#account)         | 帳號           |
| orderObject | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#orderobject) | 委託內容       |
| unblock     | bool (optional) (default = False)                                                                           | 是否採用非阻塞 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別              | 說明                             |
| --------- | ----------------- | -------------------------------- |
| isSuccess | bool              | 是否成功                         |
| data      | FutOptOrderResult | 回傳委託資訊                     |
| message   | string            | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 FutOptOrderResult 欄位[​](#委託資訊-futoptorderresult-欄位 "Direct link to 委託資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                  | 說明                                                                                                                                            |
| --------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | int                                                                                                                   | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                | 委託書號                                                                                                                                        |
| assetType       | int                                                                                                                   | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | int                                                                                                                   | 單位數                                                                                                                                          |
| currency        | string                                                                                                                | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                | 到期日                                                                                                                                          |
| strikePrice     | double                                                                                                                | 履約價                                                                                                                                          |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | double                                                                                                                | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#callput)                   | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                                                |
| buySellLeg2     | [BsAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | double                                                                                                                | 價格                                                                                                                                            |
| lot             | int                                                                                                                   | 原始委託股口數                                                                                                                                  |
| timeInForce     | [TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#timeinforce)           | 委託條件別 : `Rod` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                  | 是否為預約單                                                                                                                                    |
| status          | int                                                                                                                   | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/csharp/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | double                                                                                                                | 有效委託價格                                                                                                                                    |
| afterLot        | int                                                                                                                   | 有效委託口數                                                                                                                                    |
| filledLot       | int                                                                                                                   | 成交口數                                                                                                                                        |
| filledMoney     | double                                                                                                                | 成交價金                                                                                                                                        |
| beforeLot       | int                                                                                                                   | 改單前有效口數                                                                                                                                  |
| beforePrice     | double                                                                                                                | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                  | 委託歷程 (查詢OrderResultDetail or OrderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```cs

// 單式
var order = new FutOptOrder(
    BsAction.Buy,
    "TXO20000E4",
    null,
    null,
    "500",
    1,
    FutOptMarketType.Option,
    FutOptPriceType.Limit,
    TimeInForce.Rod,
    FutOptOrderType.Auto,
    "FromCS" // optional field
)

sdk.FutOpt.PlaceOrder(account, order)

// 複式
var order = new FutOptOrder(
    BsAction.Sell,
    "TXO20000E4",
    BsAction.Buy,   // optional field
    "TXO19900E4",   // optional field
    "90",
    1,
    FutOptMarketType.Option,
    FutOptPriceType.Limit,
    TimeInForce.Ioc,
    FutOptOrderType.Auto,
    "FromCS"   // optional field
)


sdk.FutOpt.PlaceOrder(account, order)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```cs
{   //單式回覆
    isSuccess = True,
    message = ,
    data = FutOptOrderResult {
            functionType = 0,                            // 功能別 (int)
            date = 2024/03/25,                           // 交易日期 (string)
            seqNo = 00230177010,                         // 委託單流水序號 (string)
            branchNo = 15901,                            // 分公司代號 (string)
            account = 1234567,                           // 帳號 (string)
            orderNo = C0002,                             // 委託書號 (string)
            assetType = 2,                               // 資產類別 (int)
            market = TAIMEX,                             // 市場類型 (string)
            marketType = Option,                         // 盤別種類 (FutOptMarketType)
            unit = 1,                                    // 單位數 (int)
            currency = TWD,                              // 幣別 (string)
            symbol = TXO,                                // 商品代號 (string)
            expiryDate = 202404,                         // 到期日 (string)
            strikePrice = 20000,                        // 履約價 (double)
            callPut = Call,                              // 買賣權 (string)
            buySell = Buy,                               // 買賣別 (BsAction)
            symbolLeg2 =,                                // 商品代號 - 複式第二隻腳 (string)
            expiryDateLeg2 =,                             // 到期日 - 複式第二隻腳 (string)
            strikePriceLeg2 =,                            // 履約價 - 複式第二隻腳 (double)
            callPutLeg2 =,                                // 買賣權 - 複式第二隻腳 (string)
            buySellLeg2 =,                                // 買賣別 - 複式第二隻腳 (BsAction)
            priceType = Limit,                           // 原始委託價格別 (FutOptPriceType)
            price = 500,                                 // 價格 (double)
            lot = 1,                                     // 原始委託股口數 (int)
            timeInForce = Rod,                           // 委託條件別 (TimeInforce)
            orderType = Auto,                            // 委託單類型 (FutOptOrderType)
            isPreOrder = false,                          // 是否為預約單 (bool)
            status = 10,                                 // 委託單狀態 (int)
            afterPriceType = Limit,                      // 有效委託價格別 (FutOptPriceType)
            afterPrice = 500,                            // 有效委託價格 (double)
            afterLot = 1,                                // 有效委託股口數 (int)
            filledLot = 0,                               // 成交股口數 (int)
            filledMoney = 0,                             // 成交價金 (int)
            beforeLot = 0,                               // 改單前有效股口數 (int)
            beforePrice = 500,                           // 改單前有效價格 (double)
            userDef = From csharp,                       // 自訂欄位 (string)
            lastTime = 11:50:08,                         // 最後異動時間 (string)
            errorMessage =                                 // 錯誤訊息 (string)
}}



{   //複式回覆
    isSuccess = True,
    message =,
    data  = OrderResult {
            functionType = 0,                          // 功能別 (int)
            date = 2024/03/25,                         // 交易日期 (string)
            seqNo = 00230177020,                       // 委託單流水序號 (string)
            branchNo = 15901,                          // 分公司代號 (string)
            account = 1234567,                         // 帳號 (string)
            orderNo = C0004,                           // 委託書號 (string)
            assetType = 2,                             // 資產類別 (int)
            market = TAIMEX,                           // 市場類型 (string)
            marketType = Option,                       // 盤別種類 (FutOptMarketType)
            unit = 1,                                  // 單位數 (int)
            currency = TWD,                            // 幣別 (string)
            symbol = TXO,                              // 商品代號 (string)
            expiryDate = 202405,                       // 到期日 (string)
            strikePrice = 20000,                      // 履約價 (double)
            callPut = Call,                            // 買賣權 (string)
            buySell = Sell,                            // 買賣別 (BsAction)
            symbolLeg2 = TXO,                          // 商品代號 - 複式第二隻腳 (string)
            expiryDateLeg2 = 202405,                   // 到期日 - 複式第二隻腳 (string)
            strikePriceLeg2 = 19900,                  // 履約價 - 複式第二隻腳 (double)
            callPutLeg2 = Call,                        // 買賣權 - 複式第二隻腳 (string)
            buySellLeg2 = Buy,                         // 買賣別 - 複式第二隻腳 (BsAction)
            priceType = Limit,                         // 原始委託價格別 (FutOptPriceType)
            price = 90,                                // 價格 (double)
            lot = 1,                                   // 原始委託股口數 (int)
            timeInForce = IOC,                         // 委託條件別 (TimeInforce)
            orderType = New,                           // 委託單類型 (FutOptOrderType)
            isPreOrder = false,                        // 是否為預約單 (bool)
            status = 10,                               // 委託單狀態 (int)
            afterPriceType = Limit,                    // 有效委託價格別 (FutOptPriceType)
            afterPrice = 90,                           // 有效委託價格 (double)
            afterLot = 1,                              // 有效委託股口數 (int)
            filledLot = 0,                             // 成交股口數 (int)
            filledMoney = 0,                           // 成交價金 (int)
            beforeLot = 0,                             // 改單前有效股口數 (int)
            beforePrice = 90,                          // 改單前有效價格 (double)
            userDef = From csharp,                     // 自訂欄位 (string)
            lastTime = 11:57:41,                       // 最後異動時間 (string)
            errorMessage =                             // 錯誤訊息 (string)
}}


```


---

### 平倉查詢

closePositionRecord

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                | 說明     |
| ----------- | --------------------------------------------------------------------------------------------------- | -------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account) | 帳號     |
| start\_date | string                                                                                              | 查詢起日 |
| end\_date   | string                                                                                              | 查詢迄日 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳交割款資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 平倉 CloseRecord 欄位[​](#平倉-closerecord--欄位 "Direct link to 平倉 CloseRecord  欄位")

Return type : Object

| 參數           | 類別                                                                                                                | 說明                                                                       |
| -------------- | ------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| date           | string                                                                                                              | 資料日期                                                                   |
| branchNo       | string                                                                                                              | 分公司代號                                                                 |
| positionKind   | number                                                                                                              | 部位種類 : `1` 期貨 、`2` 選擇權                                           |
| account        | string                                                                                                              | 帳號                                                                       |
| orderNo        | string                                                                                                              | 委託書號                                                                   |
| market         | string                                                                                                              | 市場別 : `TAIMEX` 期貨、選擇權                                             |
| symbol         | string                                                                                                              | 商品代號                                                                   |
| expiryDate     | string                                                                                                              | 履約日                                                                     |
| strikePrice    | number                                                                                                              | 履約價                                                                     |
| callPut        | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                 | 權利別 : `Call` Call 、`Put` Put                                           |
| buySell        | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)               | 買賣別 : `Buy` 買、`Sell` 賣                                               |
| orderType      | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptordertype) | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖 |
| price          | number                                                                                                              | 成交價                                                                     |
| origLots       | number                                                                                                              | 原始口數                                                                   |
| transactionFee | number                                                                                                              | 交易手續費                                                                 |
| tax            | number                                                                                                              | 交易稅                                                                     |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
sdk.futoptAccounting.closePositionRecord(target_user,"20240410");

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data:[
            {
                date: "2024/04/10",                // 資料日期 (string)
                branchNo: "15000",                  // 分公司代號 (string)
                account: "9974825",                 // 帳號 (string)
                positionKind: 1,                    // 部位種類: `1` 期貨, `2` 選擇權 (number)
                orderNo: "15001-0000",             // 委託書號 (string)
                market: "TAIMEX",                   // 市場別 : `TAIMEX` (string)
                symbol: "FITX",                     // 商品代號 (string)
                expiryDate: '202404',                 // 履約日 (string)
                buySell: Buy,                       // Buy/Sell Type : `Buy`, `Sell` (BSAction)
                price: 20847.0,                     // 成交價 (number)
                origLots: 1,                        // 原始口數 (number)
                transactionFee: 40.0,               // 交易手續費 (number)
                tax: 83.0,                          // 交易稅 (number)
            },
             {
                date: "2024/04/10",                // 資料日期 (string)
                branchNo: "15000",                  // 分公司代號 (string)
                account: "9974825",                 // 帳號 (string)
                positionKind: 1,                    // 部位種類d : `1` 期貨, `2` 選擇權 (number)
                orderNo: "C0005-0000",             // 委託書號 (string)
                market: "TAIMEX",                   // 市場別 : `TAIMEX` 期貨, 選擇權 (string)
                symbol: "FITX",                     // 商品代號 (string)
                expiryDate: '202405',                 // 履約日 (string)
                buySell: Buy,                       // Buy/Sell Type : `Buy`, `Sell` (BSAction)
                price: 20890.0,                     // 成交價 (number)
                origLots: 1,                        // 原始口數 (number)
                transactionFee: 40.0,               // 交易手續費 (number)
                tax: 84.0,                          // 交易稅 (number)
            }
        ]
}

```


---

### 混合部位查詢

queryHybridPosition

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                                | 說明 |
| ------- | --------------------------------------------------------------------------------------------------- | ---- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account) | 帳號 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳部位資訊                     |
| message   | string ? (optional) | 當isSuccess : false 回傳錯誤訊息 |

##### 部位資訊 HybridPosition 欄位[​](#部位資訊--hybridposition-欄位 "Direct link to 部位資訊  HybridPosition 欄位")

Return type : Object

| 參數                 | 類別                                                                                                                | 說明                                              |
| -------------------- | ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| > date               | string                                                                                                              | 部位建立日期                                      |
| > branchNo           | string                                                                                                              | 分公司代號                                        |
| > account            | string                                                                                                              | 帳號                                              |
| > isSpread           | bool                                                                                                                | 是否為複式部位                                    |
| > positionKind       | int                                                                                                                 | 部位種類 : `1` 期貨 、`2` 選擇權                  |
| > symbol             | string                                                                                                              | 商品代號                                          |
| > expiryDate         | string                                                                                                              | 履約日                                            |
| > strikePrice        | double                                                                                                              | 履約價                                            |
| > callPut            | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                 | 權利別 : `1` Call 、`2` Put                       |
| > buySell            | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)               | 買賣別 : `Buy` 買、`Sell` 賣                      |
| > price              | double                                                                                                              | 成交價                                            |
| > origLots           | int                                                                                                                 | 原始口數                                          |
| > tradableLots       | int                                                                                                                 | 可交易口數                                        |
| > orderType          | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptordertype) | 委託別 : `New` 新倉、`Close`平倉、`FdayTrade`當沖 |
| > currency           | string                                                                                                              | 幣別                                              |
| > marketPrice        | string                                                                                                              | 即時價                                            |
| > initialMargin      | double                                                                                                              | 原始保證金                                        |
| > maintenanceMargin  | double                                                                                                              | 維持保證金                                        |
| > clearingMargin     | double                                                                                                              | 結算保證金                                        |
| > optValue           | double                                                                                                              | 選擇權市值                                        |
| > optLongValue       | double                                                                                                              | 選擇權買進市值                                    |
| > optShortValue      | double                                                                                                              | 選擇權賣出市值                                    |
| > profitOrLoss       | double                                                                                                              | 部位損益                                          |
| > premium            | double                                                                                                              | 權利金                                            |
| >> spread            | object                                                                                                              | 複式部位解析                                      |
| >> date              | string                                                                                                              | 部位建立日期                                      |
| >> branchNo          | string                                                                                                              | 分公司代號                                        |
| >> account           | string                                                                                                              | 帳號                                              |
| >> isSpread          | bool                                                                                                                | 是否為複式部位                                    |
| >> positionKind      | int                                                                                                                 | 部位種類 : `1` 期貨 、`2` 選擇權                  |
| >> symbol            | string                                                                                                              | 商品代號                                          |
| >> expiryDate        | string                                                                                                              | 履約日                                            |
| >> strikePrice       | double                                                                                                              | 履約價                                            |
| >> callPut           | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                 | 權利別 : `1` Call 、`2` Put                       |
| >> buySell           | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)               | 買賣別 : `Buy` 買、`Sell` 賣                      |
| >> price             | double                                                                                                              | 成交價                                            |
| >> origLots          | int                                                                                                                 | 原始口數                                          |
| >> tradableLots      | int                                                                                                                 | 可交易口數                                        |
| >> orderType         | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptordertype) | 委託別 : `New` 新倉、`Close`平倉、`FdayTrade`當沖 |
| >> currency          | string                                                                                                              | 幣別                                              |
| >> marketPrice       | string                                                                                                              | 即時價                                            |
| >> initialMargin     | double                                                                                                              | 原始保證金                                        |
| >> maintenanceMargin | double                                                                                                              | 維持保證金                                        |
| >> clearingMargin    | double                                                                                                              | 結算保證金                                        |
| >> optValue          | double                                                                                                              | 選擇權市值                                        |
| >> optLongValue      | double                                                                                                              | 選擇權買進市值                                    |
| >> optShortValue     | double                                                                                                              | 選擇權賣出市值                                    |
| >> profitOrLoss      | double                                                                                                              | 部位損益                                          |
| >> premium           | double                                                                                                              | 權利金                                            |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
sdk.futoptAccounting.queryHybridPosition(account);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data:[
       {
          date : "2024/04/08",                      // 部位建立日期 (string)
          branchNo : "15901",                       // 分公司代號 (string)
          account : "1234567",                      // 帳號 (string)
          isSpread : false,                         // 是否為複式部位 (boolean)
          positionKind : 1,                         // 部位種類 (number)
          symbol : "FITX",                          // 商品代號 (string)
          expiryDate : '202404',                    // 履約日 (string)
          buySell : Buy,                            // 買賣別 (number)
          price : 20325.3333,                       // 成交價 (number)
          origLots : 3,                             // 原始口數 (number)
          tradableLot : 3,                          // 可交易口數 (number)
          orderType : New,                          // 委託別 (FutOptOrderType)
          currency : "TWD",                         // 幣別 (string)
          marketPrice : "20351",                    // 即時價 (string)
          initialMargin : 0.0,                      // 原始保證金 (number)
          maintenanceMargin : 0.0,                  // 維持保證金 (number)
          clearingMargin : 0.0,                     // 結算保證金 (number)
          initialMarginAllSingle : 0.0,             // 原始保證金 (number)
          optValue : 0.0,                           // 選擇權市值 (number)
          optLongValue : 0.0,                       // 選擇權買進市值 (number)
          optShortValue : 0.0,                      // 選擇權賣出市值 (number)
          profitOrLoss : 0.0,                       // 部位損益 (number)
          premium : 0.0                             // 權利金 (number)
      },
      {
          date : "2024/04/08",                     // 部位建立日期 (string)
          branchNo : "15901",                       // 分公司代號 (string)
          account : "1234567",                      // 帳號 (string)
          isSpread : false,                         // 是否為複式部位 (boolean)
          positionKind : 2,                         // 部位種類 (number)
          symbol : "TX1",                           // 商品代號 (string)
          expiryDate : '202404',                      // 履約日 (string)
          strikePrice : 206000,                     // 履約價 (number)
          callPut : Call,                           // 權利別 (number)
          buySell : Buy,                            // 買賣別 (number)
          price : 10,                               // 成交價 (number)
          origLots : 4,                             // 原始口數 (number)
          tradableLot : 4,                          // 可交易口數 (number)
          orderType : New,                          // 委託別(FutOptOrderType)
          currency : "TWD",                         // 幣別 (string)
          marketPrice : "4.6",                      // 即時價 (string)
          initialMargin : 0.0,                      // 原始保證金 (number)
          maintenanceMargin : 0.0,                  // 維持保證金 (number)
          clearingMargin : 0.0,                     // 結算保證金 (number)
          initialMarginAllSingle : 0.0,             // 原始保證金 (number)
          optValue : 920.0,                         // 選擇權市值 (number)
          optLongValue : 920.0,                     // 選擇權買進市值 (number)
          optShortValue : 0.0,                      // 選擇權賣出市值 (number)
          profitOrLoss : -1080.0,                   // 部位損益 (number)
          premium : 0.0                            // 權利金 (number)
      },
       {
          date : "2024/04/08",                      // 部位建立日期 (string)
          branchNo : "15901",                       // 分公司代號 (string)
          account : "1234567",                      // 帳號 (string)
          isSpread : false,                         // 是否為複式部位 (boolean)
          positionKind : 2,                         // 部位種類 (number)
          symbol : "TXO",                           // 商品代號 (string)
          expiryDate : '202404',                      // 履約日 (string)
          strikePrice : 198000,                     // 履約價 (number)
          callPut : Call,                           // 權利別 (number)
          buySell : Buy,                            // 買賣別 (number)
          price : 243,                              // 成交價 (number)
          origLots : 2,                             // 原始口數 (number)
          tradableLot : 2,                          // 可交易口數 (number)
          orderType : New,                          // 委託別  (FutOptOrderType)
          currency : "TWD",                         // 幣別 (string)
          marketPrice : "46",                       // 即時價 (string)
          initialMargin : 0.0,                      // 原始保證金 (number)
          maintenanceMargin : 0.0,                  // 維持保證金 (number)
          clearingMargin : 0.0,                     // 結算保證金 (number)
          initialMarginAllSingle : 0.0,             // 原始保證金 (number)
          optValue : 4600.0,                        // 選擇權市值 (number)
          optLongValue : 4600.0,                    // 選擇權買進市值 (number)
          optShortValue : 0.0,                      // 選擇權賣出市值 (number)
          profitOrLoss : -19700.0,                  // 部位損益 (number)
          premium : 0.0                             // 權利金 (number)
      },
       {
          date : "2024/04/08",                     // 部位建立日期 (string)
          branchNo : "15901",                       // 分公司代號 (string)
          account : "1234567",                      // 帳號 (string)
          isSpread : false,                         // 是否為複式部位 (boolean)
          positionKind : 2,                         // 部位種類 (number)
          symbol : "TXO",                           // 商品代號 (string)
          expiryDate : '202404',                      // 履約日 (string)
          strikePrice : 200000,                     // 履約價 (number)
          callPut : Put,                            // 權利別 (number)
          buySell : Buy,                            // 買賣別 (number)
          price : 344,                              // 成交價 (number)
          origLots : 2,                             // 原始口數 (number)
          tradableLot : 2,                          // 可交易口數 (number)
          orderType : New,                          // 委託別 (FutOptOrderType)
          currency : "TWD",                         // 幣別 (string)
          marketPrice : "82",                       // 即時價 (string)
          initialMargin : 0.0,                      // 原始保證金 (number)
          maintenanceMargin : 0.0,                  // 維持保證金 (number)
          clearingMargin : 0.0,                     // 結算保證金 (number)
          initialMarginAllSingle : 0.0,             // 原始保證金 (number)
          optValue : 8200.0,                        // 選擇權市值 (number)
          optLongValue : 8200.0,                    // 選擇權買進市值 (number)
          optShortValue : 0.0,                      // 選擇權賣出市值 (number)
          profitOrLoss : -26200.0,                  // 部位損益 (number)
          premium : 0.0                            // 權利金 (number)
      },
       {
          date : "2024/04/08",                     // 部位建立日期 (string)
          branchNo : "15901",                       // 分公司代號 (string)
          account : "1234567",                      // 帳號 (string)
          isSpread : true,                          // 是否為複式部位 (boolean)
          positionKind : 2,                         // 部位種類(number)
          symbol : "TXO20100D4:20000P4",           // 商品代號 (string)
          expiryDate : 1,                           // 履約日 (string)
          strikePrice : 1,                          // 履約價 (number)
          buySell : Buy,                              // 買賣別 (number)
          origLots : 2,                             // 原始口數 (number)
          tradableLot : 2,                          // 可交易口數 (number)
          orderType : New,                          // 委託別(FutOptOrderType)
          currency : "TWD",                         // 幣別 (string)
          marketPrice : "0.0",                      // 即時價 (string)
          initialMargin : 0.0,                      // 原始保證金 (number)
          maintenanceMargin : 0.0,                  // 維持保證金 (number)
          clearingMargin : 0.0,                     // 結算保證金 (number)
          initialMarginAllSingle : 0.0,             // 原始保證金 (number)
          optValue : 0.0,                           // 選擇權市值 (number)
          optLongValue : 0.0,                       // 選擇權買進市值 (number)
          optShortValue : 0.0,                      // 選擇權賣出市值 (number)
          profitOrLoss : 0.0,                       // 部位損益 (number)
          premium : 0.0,                            // 權利金 (number)
          spreads : [                               // 複式部位 (SpreadPosition[])
               {
                  date : "2024/04/08",             // 部位建立日期 (string)
                  branchNo : "15901",               // 分公司代號 (string)
                  account : "1234567",              // 帳號 (string)
                  positionKind : 2,                 // 部位種類  (number)
                  symbol : "TXO",                  // 商品代號 (string)
                  symbolName : "臺指選擇權",        // 商品名稱 (string)
                  strikePrice : 201000,            // 履約價 (number)
                  callPut : Call,                     // 權利別 (number)
                  buySell : Buy,                     // 買賣別 (number)
                  price : 185,                     // 成交價 (number)
                  origLots : 2,                    // 原始口數 (number)
                  tradableLot : 2,                 // 可交易口數 (number)
                  currency : "TWD",                // 幣別 (string)
                  marketPrice : "365",             // 即時價 (string)
                  initialMargin : 0.0,             // 原始保證金 (number)
                  maintenanceMargin : 0.0,         // 維持保證金 (number)
                  clearingMargin : 0.0,            // 結算保證金 (number)
                  initialMarginAllSingle : 0.0,    // 原始保證金 (number)
                  optValue : 36500.0,              // 選擇權市值 (number)
                  optLongValue : 36500.0,          // 選擇權買進市值 (number)
                  optShortValue : 0.0,             // 選擇權賣出市值 (number)
                  profitOrLoss : 18000.0,          // 部位損益 (number)
                  premium : 0.0,                   // 權利金 (number)
              },
               {
                  date : "2024/04/08",             // 部位建立日期 (string)
                  branchNo : "15901",               // 分公司代號 (string)
                  account : "1234567",              // 帳號 (string)
                  positionKind : 2,                 // 部位種類 (number)
                  symbol : "TXO",                  // 商品代號 (string)
                  expiryDate : '202404',             // 履約日 (string)
                  strikePrice : 200000,            // 履約價 (number)
                  callPut : Put,                     // 權利別 (number)
                  buySell : Buy,                     // 買賣別 (number)
                  price : 354,                     // 成交價 (number)
                  origLots : 2,                    // 原始口數 (number)
                  tradableLot : 2,                 // 可交易口數 (number)
                  currency : "TWD",                // 幣別 (string)
                  marketPrice : "82",              // 即時價 (string)
                  initialMargin : 0.0,             // 原始保證金 (number)
                  maintenanceMargin : 0.0,         // 維持保證金 (number)
                  clearingMargin : 0.0,            // 結算保證金 (number)
                  initialMarginAllSingle : 0.0,    // 原始保證金 (number)
                  optValue : 8200.0,               // 選擇權市值 (number)
                  optLongValue : 8200.0,           // 選擇權買進市值 (number)
                  optShortValue : 0.0,             // 選擇權賣出市值 (number)
                  profitOrLoss : -27200.0,         // 部位損益 (number)
                  premium : 0.0,                   // 權利金 (number)
              },
          ],
      }
  ]
}

```


---

### 權益數查詢

queryMarginEquity

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                                | 說明 |
| ------- | --------------------------------------------------------------------------------------------------- | ---- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account) | 帳號 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳資訊                         |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

###### 權益數 Equity 欄位[​](#權益數-equity-欄位 "Direct link to 權益數 Equity 欄位")

Return type : Object

| 參數              | 類別   | 說明                                                                        |
| ----------------- | ------ | --------------------------------------------------------------------------- |
| date              | string | 查詢日期                                                                    |
| branchNo          | string | 分公司代號                                                                  |
| account           | string | 帳號                                                                        |
| currency          | string | 幣別 : `NTD` 約當台幣 、`TWD` 新台幣、`USD` 美元、`CNY` 人民幣 、`JPY` 日圓 |
| yesterdayBalance  | number | 昨日餘額                                                                    |
| todayBalance      | number | 今日餘額                                                                    |
| initialMargin     | number | 原始保證金                                                                  |
| maintenanceMargin | number | 維持保證金                                                                  |
| clearingMargin    | number | 結算保證金                                                                  |
| todayEquity       | number | 本日權益                                                                    |
| todayDeposit      | number | 今日入金                                                                    |
| todayWithdrawal   | number | 今日出金                                                                    |
| todayTradingFee   | number | 今日交易手續費                                                              |
| todayTradingTax   | number | 今日交易稅                                                                  |
| receivablePremium | number | 收取權利金                                                                  |
| payablePremium    | number | 付出權利金                                                                  |
| excessMargin      | number | 超額保證金                                                                  |
| availableMargin   | number | 可動用保證金                                                                |
| disgorgement      | number | 追繳金額                                                                    |
| optPnl            | number | 未沖銷選擇權浮動損益                                                        |
| optValue          | number | 選擇權市值                                                                  |
| optLongValue      | number | 未沖銷選擇權買方市值                                                        |
| optShortValue     | number | 未沖銷選擇權賣方市值                                                        |
| futRealizedPnl    | number | 期貨平倉損益                                                                |
| futUnrealizedPnl  | number | 期貨未平倉損益                                                              |
| buyLot            | number | 買進口數                                                                    |
| sellLot           | number | 賣出口數                                                                    |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
sdk.futoptAccounting.queryMarginEquity(account);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data:[
    {
        date: '2024/04/08',                    // 查詢日期 (string)
        branchNo: '15901',                      // 分公司代號 (string)
        account: '1234567',                     // 帳號 (string)
        currency: 'NTD',                        // 幣別 (string)
        yesterdayBalance: 22435152.4,          // 昨日餘額 (number)
        todayBalance: 22434910.4,              // 今日餘額 (number)
        initialMargin: 1114946.0,              // 原始保證金 (number)
        maintenanceMargin: 939214.0,           // 維持保證金 (number)
        clearingMargin: 915760.0,              // 結算保證金 (number)
        todayEquity: 22694910.4,               // 本日權益 (number)
        todayDeposit: 0.0,                     // 本日入金 (number)
        todayWithdrawal: 2102.0,               // 本日出金 (number)
        todayTradingFee: 16.0,                 // 本日交易手續費 (number)
        todayTradingTax: 0.0,                  // 本日交易稅 (number)
        receivablePremium: 0.0,                // 收取權利金 (number)
        payablePremium: 9250.0,                // 付出權利金 (number)
        excessMargin: 28744525.0,              // 超額保證金 (number)
        availableMargin: 21453562.4,           // 可動用保證金 (number)
        disgorgement: 0.0,                     // 追繳金額 (number)
        optPnl: -248600.0,                     // 未沖銷選擇權浮動損益 (number)
        optValue: -193100.0,                   // 選擇權市值 (number)
        optLongValue: 311900.0,                // 未沖銷選擇權買方市值 (number)
        optShortValue: 505000.0,               // 未沖銷選擇權賣方市值 (number)
        futRealizedPnl: 0.0,                   // 期貨平倉損益 (number)
        futUnrealizedPnl: 60700.0,             // 期貨未平倉損益 (number)
        buyLot: 22,                            // 買進口數 (number)
        sellLot: 7,                            // 賣出口數 (number)
      },
      {
        date: '2024/04/08',                    // 查詢日期 (string)
        branchNo: '15901',                      // 分公司代號 (string)
        account: '1234567',                      // 帳號 (string)
        currency: 'TWD',                         // 幣別 (string)
        yesterdayBalance: 19880310.0,           // 昨日餘額 (number)
        todayBalance: 19880068.0,               // 今日餘額 (number)
        initialMargin: 1114946.0,               // 原始保證金 (number)
        maintenanceMargin: 939214.0,            // 維持保證金 (number)
        clearingMargin: 915760.0,               // 結算保證金 (number)
        todayEquity: 20140068.0,                // 本日權益 (number)
        todayDeposit: 0.0,                     // 本日入金 (number)
        todayWithdrawal: 2102.0,               // 本日出金 (number)
        todayTradingFee: 16.0,                 // 本日交易手續費 (number)
        todayTradingTax: 0.0,                  // 本日交易稅 (number)
        receivablePremium: 0.0,                // 收取權利金 (number)
        payablePremium: 9250.0,                // 付出權利金 (number)
        excessMargin: 28744525.0,              // 超額保證金 (number)
        availableMargin: 18898720.0,            // 可動用保證金 (number)
        disgorgement: 0.0,                      // 追繳金額 (number)
        optPnl: -248600.0,                      // 未沖銷選擇權浮動損益 (number)
        optValue: -193100.0,                    // 選擇權市值 (number)
        optLongValue: 311900.0,                 // 未沖銷選擇權買方市值 (number)
        optShortValue: 505000.0,                // 未沖銷選擇權賣方市值 (number)
        futRealizedPnl: 0.0,                    // 期貨平倉損益 (number)
        futUnrealizedPnl: 60700.0,              // 期貨未平倉損益 (number)
        buyLot: 22,                             // 買進口數 (number)
        sellLot: 7,                             // 賣出口數 (number)
  },
    ...
  ]
}

```


---

### 單式部位查詢

querySinglePosition

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                                | 說明 |
| ------- | --------------------------------------------------------------------------------------------------- | ---- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account) | 帳號 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳未實現資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 部位 Position 欄位[​](#部位-position--欄位 "Direct link to 部位 Position  欄位")

Return type : Object

| 參數              | 類別                                                                                                                | 說明                                              |
| ----------------- | ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| date              | string                                                                                                              | 部位建立日期                                      |
| branchNo          | string                                                                                                              | 分公司代號                                        |
| account           | string                                                                                                              | 帳號                                              |
| isSpread          | bool                                                                                                                | 是否為複式部位                                    |
| positionKind      | number                                                                                                              | 部位種類 : `1` 期貨 、`2` 選擇權                  |
| symbol            | string                                                                                                              | 商品代號                                          |
| symbolName        | string                                                                                                              | 商品名稱                                          |
| expiryDate        | string                                                                                                              | 履約日                                            |
| strikePrice       | number                                                                                                              | 履約價                                            |
| callPut           | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                 | 權利別 : `1` Call 、`2` Put                       |
| buySell           | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)               | 買賣別 : `Buy` 買、`Sell` 賣                      |
| price             | number                                                                                                              | 成交價                                            |
| origLots          | number                                                                                                              | 原始口數                                          |
| tradableLots      | number                                                                                                              | 可交易口數                                        |
| orderType         | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptordertype) | 委託別 : `New` 新倉、`Close`平倉、`FdayTrade`當沖 |
| currency          | string                                                                                                              | 幣別                                              |
| marketPrice       | string                                                                                                              | 即時價                                            |
| initialMargin     | number                                                                                                              | 原始保證金                                        |
| maintenanceMargin | number                                                                                                              | 維持保證金                                        |
| clearingMargin    | number                                                                                                              | 結算保證金                                        |
| optValue          | number                                                                                                              | 選擇權市值                                        |
| optLongValue      | number                                                                                                              | 選擇權買進市值                                    |
| optShortValue     | number                                                                                                              | 選擇權賣出市值                                    |
| profitOrLoss      | number                                                                                                              | 部位損益                                          |
| premium           | number                                                                                                              | 權利金                                            |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
sdk.futoptAccounting.querySinglePosition(account);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data:[
         {
            date: "2024/04/08",                 // 部位建立日期 (string)
            branchNo: "15901",                   // 分公司代號 (string)
            account: "1234567",                  // 帳號 (string)
            orderNo: "l0001-0000",               // 訂單編號 (string)
            positionKind: 1,                     // 部位種類 (number)
            spreadKind: 0,                       // 是否為複式部位 (number)
            symbol: "FITX",                      // 商品代號 (string)
            expiryDate: '202404',                  // 履約日 (string)
            buySell: Buy,                        // 買賣別 (BSAction)
            price: 20362,                        // 成交價 (number)
            origLots: 2,                         // 原始口數 (number)
            tradableLot: 2,                      // 可交易口數 (number)
            orderType: "NEW",                    // 委託別 (FutOptOrderType)
            currency: "TWD",                     // 幣別 (string)
            marketPrice: "20521.0000",           // 即時價 (string)
            initialMargin: 358000.0,             // 原始保證金 (number)
            maintenanceMargin: 274000.0,         // 維持保證金 (number)
            clearingMargin: 264000.0,            // 結算保證金 (number)
            profitOrLoss: 63600.0,               // 部位損益 (number)
            premium: 0.0,                        // 權利金 (number)
        },
        {
            date: "2024/03/29",                  // 部位建立日期 (string)
            branchNo: "15901",                    // 分公司代號 (string)
            account: "1234567",                   // 帳號 (string)
            orderNo: "l0007-0000",                // 訂單編號 (string)
            positionKind: 2,                      // 部位種類 (number)
            spreadKind: 1,                        // 是否為複式部位 (number)
            symbol: "TX1",                        // 商品代號 (string)
            expiryDate: '202404',                   // 履約日 (string)
            strikePrice: 20600,                  // 履約價 (number)
            callPut: Call,                           // 權利別 (CallPut)
            buySell: Buy,                           // 買賣別 (BSAction)
            price: 10,                            // 成交價 (number)
            origLots: 2,                          // 原始口數 (number)
            tradableLot: 2,                       // 可交易口數 (number)
            orderType: 0,                         // 委託別 (FutOptOrderType)
            currency: "TWD",                      // 幣別 (string)
            marketPrice: "4.6000",                // 即時價 (string)
            initialMargin: 52660.0,              // 原始保證金 (number)
            maintenanceMargin: 36460.0,          // 維持保證金 (number)
            clearingMargin: 34460.0,             // 結算保證金 (number)
            profitOrLoss: -540.0,                // 部位損益 (number)
            premium: -1000.0,                    // 權利金 (number)
        },
        {
            date: "2024/03/29",                  // 部位建立日期 (string)
            branchNo: "15901",                    // 分公司代號 (string)
            account: "1234567",                   // 帳號 (string)
            orderNo: "l0007-0001",                // 訂單編號 (string)
            positionKind: 2,                      // 部位種類 (number)
            spreadKind: 1,                        // 是否為複式部位 (number)
            symbol: "TX1",                        // 商品代號 (string)
            expiryDate: 202404,                   // 履約日 (string)
            strikePrice: 20600,                  // 履約價 (number)
            callPut: Call,                           // 權利別 (CallPut)
            buySell: Buy,                           // 買賣別 (BSAction)
            price: 10,                            // 成交價 (number)
            origLots: 2,                          // 原始口數 (number)
            tradableLot: 2,                       // 可交易口數 (number)
            orderType: 0,                         // 委託別 (FutOptOrderType)
            currency: "TWD",                      // 幣別 (string)
            marketPrice: "4.6000",                // 即時價 (string)
            initialMargin: 52660.0,              // 原始保證金 (number)
            maintenanceMargin: 36460.0,          // 維持保證金 (number)
            clearingMargin: 34460.0,             // 結算保證金 (number)
            profitOrLoss: -540.0,                // 部位損益 (number)
            premium: -1000.0,                    // 權利金 (number)
        },
        {
            date: "2024/03/01",                   // 部位建立日期 (string)
            branchNo: "15901",                     // 分公司代號 (string)
            account: "1234567",                    // 帳號 (string)
            orderNo: "l0002-0000",                 // 訂單編號 (string)
            positionKind: 2,                       // 部位種類  (number)
            spreadKind: 1,                         // 是否為複式部位 (number)
            symbol: "TXO",                         // 商品代號 (string)
            expiryDate: 202404,                    // 履約日 (string)
            strikePrice: 18500,                   // 履約價 (number)
            callPut: Call,                            // 權利別  (CallPut)
            buySell: Sell,                            // 買賣別 (BSAction)
            price: 625,                            // 成交價 (number)
            origLots: 5,                           // 原始口數 (number)
            tradableLot: 4,                        // 可交易口數 (number)
            orderType: 0,                          // 委託別 (FutOptOrderType)
            currency: "TWD",                       // 幣別 (string)
            marketPrice: "2020.0000",              // 即時價 (string)
            initialMargin: 584000.0,               // 原始保證金 (number)
            maintenanceMargin: 544000.0,           // 維持保證金 (number)
            clearingMargin: 536000.0,              // 結算保證金 (number)
            profitOrLoss: -279000.0,               // 部位損益 (number)
            premium: 125000.0,                     // 權利金 (number)
        }
  ]
}

```


---

### 參數對照表

#### 類別[​](#類別 "Direct link to 類別")

Class

##### OrderObject[​](#orderobject "Direct link to OrderObject")

| Parameter          | Type             | Meaning                                                                                                                         |
| ------------------ | ---------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| buySell            | BSAction         | [買賣別](#bsaction) 可選用參數`Buy` 買 、 `Sell` 賣                                                                             |
| symbol             | string           | 商品代號                                                                                                                        |
| buySell2           | BSAction         | [買賣別](#bsaction) - 複式單                                                                                                    |
| symbol2            | string           | 商品代號 - 複式單                                                                                                               |
| price              | string           | 委託價格                                                                                                                        |
| lot                | int              | 委託數量                                                                                                                        |
| marketType         | FutOptMarketType | [盤別](#futoptmarkettype) 可選用參數`Future` 期貨日盤 、 `Option` 選擇權日盤、 `FutureNight` 期貨夜盤、`OptionNight` 選擇權夜盤 |
| priceType          | FutOptPriceType  | [價格旗標](#futoptpricetype) 可選用參數為 `Limit`限價 、 `Market`市價 、 `RangeMarket` 範圍市價 、 `Reference`參考價            |
| timeInForce        | timeInForce      | [委託條件](#timeinforce) 可選用參數為 `ROD`、`FOK`、`IOC`                                                                       |
| orderType          | FutOptOrderType  | [委託類別](#futoptordertype) 可選用參數為 `New` 新倉、`Close`平倉、`Auto`自動、`FdayTrade`當沖                                  |
| userDef (optional) | string           | 用戶自定義 (最長10個字元，不支援特殊字元)                                                                                       |

caution

priceType 爲 FutOptPriceType.Limit 時，需填入 price 欄位，其餘時候 price 欄位為空值或為null。

##### FutOptOrderResult[​](#futoptorderresult "Direct link to FutOptOrderResult")

委託列表，透過 [GetOrderResult(accounts)](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/trade/GetOrderResults.txt) 取得。

| 參數            | 類別             | 說明                                                                                                                           |
| --------------- | ---------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| functionType    | number           | 功能別 : `0` 新單 、 `10` 新單執行、`15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                 |
| date            | string           | 交易日期                                                                                                                       |
| seqNo           | string           | 委託單流水序號                                                                                                                 |
| branchNo        | string           | 分公司代號                                                                                                                     |
| account         | string           | 帳號                                                                                                                           |
| orderNo         | string           | 委託書號                                                                                                                       |
| assetType       | number           | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                               |
| market          | string           | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                               |
| marketType      | FutOptMarketType | [盤別種類](#futoptmarkettype) : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤 |
| unit            | number           | 單位數                                                                                                                         |
| currency        | string           | 幣別                                                                                                                           |
| symbol          | string           | 商品代號                                                                                                                       |
| expiryDate      | string           | 到期日                                                                                                                         |
| strikePrice     | number           | 履約價                                                                                                                         |
| callPut         | string           | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                              |
| buySell         | BSAction         | [買賣別](#bsaction) : `Buy` 買 、 `Sell` 賣                                                                                    |
| symbolLeg2      | string           | 商品代號 - 複式第二隻腳                                                                                                        |
| expiryDateLeg2  | string           | 到期日 - 複式第二隻腳                                                                                                          |
| strikePriceLeg2 | number           | 履約價 - 複式第二隻腳                                                                                                          |
| callPutLeg2     | string           | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                               |
| buySellLeg2     | BSAction         | [買賣別](#bsaction) - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                      |
| priceType       | FutOptPriceType  | [原始委託價格別](#futoptpricetype) : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價               |
| price           | number           | 價格                                                                                                                           |
| lot             | number           | 原始委託股口數                                                                                                                 |
| timeInForce     | TimeInforce      | [委託條件別](#timeinforce) : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                               |
| orderType       | FutOptOrderType  | [委託單類型](#futoptordertype) : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                 |
| isPreOrder      | bool             | 是否為預約單                                                                                                                   |
| status          | number           | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、`9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗   |
| afterPriceType  | FutOptPriceType  | [有效委託價格別](#futoptpricetype) : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價               |
| afterPrice      | number           | 有效委託價格                                                                                                                   |
| afterLot        | number           | 有效委託口數                                                                                                                   |
| filledLot       | number           | 成交口數                                                                                                                       |
| filledMoney     | number           | 成交價金                                                                                                                       |
| beforeLot       | number           | 改單前有效口數                                                                                                                 |
| beforePrice     | number           | 改單前有效價                                                                                                                   |
| userDef         | string           | 自訂欄位                                                                                                                       |
| lastTime        | string           | 最後異動時間                                                                                                                   |
| errorMessage    | string           | 錯誤訊息                                                                                                                       |
| detail          | list             | 委託歷程 (查詢orderResultDetail or orderHistory才有值)                                                                         |
| >> functionType | number           | 功能別 : `10` 新單、`15` 改價、 `20` 改量、`30`刪單、`50` 成交 、`90`失敗                                                      |
| >> modifiedTime | string           | 修改時間                                                                                                                       |
| >> beforeLot    | number           | 原始委託口數                                                                                                                   |
| >> afterLot     | number           | 有效委託口數                                                                                                                   |
| >> beforePrice  | number           | 原始委託價                                                                                                                     |
| >> filledMoney  | number           | 成交價金                                                                                                                       |
| >> afterPrice   | number           | 有效委託價                                                                                                                     |
| >> errorMessage | string           | 錯誤訊息                                                                                                                       |

##### BatchResult[​](#batchresult "Direct link to BatchResult")

批次委託列表，透過 [BatchOrderLists(accounts)](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/trade/batchOrder/BatchOrderList.txt) 取得。

| Parameter    | Type   | Meaning                                                                        |
| ------------ | ------ | ------------------------------------------------------------------------------ |
| functionType | number | 功能別 : `0` 新單 、 `10` 新單執行、`15` 改價、 `20` 改量、`30`刪單 、`90`失敗 |
| date         | string | 交易日期                                                                       |
| branchNo     | string | 分公司代號                                                                     |
| account      | string | 帳號                                                                           |
| batchSeqNo   | string | 批次單流水序號                                                                 |

##### FutoptModifyPrice[​](#futoptmodifyprice "Direct link to FutoptModifyPrice")

改價物件

| Parameter         | Type            | Meaning                                                                                        |
| ----------------- | --------------- | ---------------------------------------------------------------------------------------------- |
| FutOptOrderResult | Object          | [委託列表](#futoptorderresult)                                                                 |
| price             | string          | 改單後的價格                                                                                   |
| FutOptPriceType   | FutOptPriceType | 改單後的價格類型 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價 |

##### FutOptModifyLot[​](#futoptmodifylot "Direct link to FutOptModifyLot")

改量物件

| Parameter         | Type   | Meaning                        |
| ----------------- | ------ | ------------------------------ |
| FutOptOrderResult | Object | [委託列表](#futoptorderresult) |
| lot               | number | 改單後的委託量                 |

##### FutOptFilledData[​](#futoptfilleddata "Direct link to FutOptFilledData")

成交回報物件

| 參數            | 類別            | 說明                                                                                           |
| --------------- | --------------- | ---------------------------------------------------------------------------------------------- |
| date            | string          | 日期                                                                                           |
| branchNo        | string          | 分公司代號                                                                                     |
| account         | string          | 帳號                                                                                           |
| seqNo           | string          | 委託單流水序號 (只有主動回報才回傳此欄位)                                                      |
| orderNo         | string          | 委託書號                                                                                       |
| symbol          | string          | 商品代號                                                                                       |
| expiryDate      | string          | 到期日                                                                                         |
| strikePrice     | float           | 履約價                                                                                         |
| callPut         | CallPut         | [買賣權](#callput) : `Call` 買權、 `Put` 賣權                                                  |
| buySell         | BSAction        | \[買賣別] (#bsaction) : `Buy` 買 、 `Sell` 賣                                                  |
| symbolLeg2      | string          | 商品代號 - 複式第二隻腳                                                                        |
| expiryDateLeg2  | string          | 到期日 - 複式第二隻腳                                                                          |
| strikePriceLeg2 | float           | 履約價 - 複式第二隻腳                                                                          |
| callPutLeg2     | CallPut         | [買賣權](#callput) - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                   |
| buySellLeg2     | BSAction        | [買賣別](#bsaction) - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                      |
| orderType       | FutOptOrderType | [委託單類型](#futoptordertype) : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖 |
| filledNo        | string          | 成交流水號                                                                                     |
| filledAvgPrice  | float           | 成交均價                                                                                       |
| filledLot       | int             | 成交股數                                                                                       |
| filledPrice     | float           | 成交單價                                                                                       |
| filledTime      | string          | 成交時間                                                                                       |
| userDef         | string          | 用戶自定義 (只有主動回報才回傳此欄位)                                                          |

##### Account[​](#account "Direct link to Account")

帳號資訊

| Parameter   | Type   | Meaning                               |
| ----------- | ------ | ------------------------------------- |
| name        | string | 客戶姓名                              |
| account     | string | 帳號                                  |
| branchNo    | string | 分公司代號                            |
| accountType | string | 帳戶類型 : `stock` 證券 `futopt` 期貨 |

#### Constants ( 欄位對應數值 )[​](#constants--欄位對應數值- "Direct link to Constants ( 欄位對應數值 )")

##### BSAction[​](#bsaction "Direct link to BSAction")

買賣別 (buySell)

| Name | Meaning |
| ---- | ------- |
| Buy  | 買      |
| Sell | 賣      |

##### CallPut[​](#callput "Direct link to CallPut")

買賣別 (buySell)

| Name | Meaning |
| ---- | ------- |
| Call | Call    |
| Put  | Put     |

##### FutOptMarketType[​](#futoptmarkettype "Direct link to FutOptMarketType")

盤別

| Name        | Meaning    |
| ----------- | ---------- |
| Future      | 期貨日盤   |
| FutureNight | 期貨夜盤   |
| Option      | 選擇權日盤 |
| OptionNight | 選擇權夜盤 |

##### FutOptPriceType[​](#futoptpricetype "Direct link to FutOptPriceType")

價格類型 (priceType)

| Name        | Meaning  |
| ----------- | -------- |
| Limit       | 限價     |
| Market      | 市價     |
| RangeMarket | 範圍市價 |
| Reference   | 參考價   |

##### TimeInForce[​](#timeinforce "Direct link to TimeInForce")

委託條件 (TimeInForce)

| Name | Meaning                               |
| ---- | ------------------------------------- |
| ROD  | 當日有效(Rest of Day)                 |
| FOK  | 全部成交否則取消(Fill-or-Kill)        |
| IOC  | 立即成交否則取消(Immediate-or-Cancel) |

##### FutOptOrderType[​](#futoptordertype "Direct link to FutOptOrderType")

委託類別 (orderType)

| Name      | Meaning |
| --------- | ------- |
| New       | 新倉    |
| Close     | 平倉    |
| Auto      | 自動    |
| FdayTrade | 當沖    |

##### function\_type[​](#function_type "Direct link to function_type")

功能類別

| Name     | Value |
| -------- | ----- |
| 新單     | 0     |
| 新單執行 | 10    |
| 改價     | 15    |
| 改量     | 20    |
| 刪單     | 30    |
| 失敗     | 90    |

##### market[​](#market "Direct link to market")

市場

| Name       | Value  |
| ---------- | ------ |
| 期貨交易所 | TAIMEX |

##### status[​](#status "Direct link to status")

委託單狀態

| Name         | Value                                                      |
| ------------ | ---------------------------------------------------------- |
| 預約單       | 0                                                          |
| 中台收到委託 | 4 ( 請用GetOrderResult查詢狀態 )                           |
| 後台連線逾時 | 9 ( 請稍後再使用GetOrderResult查詢狀態 or 聯絡您的營業員 ) |
| 委託成功     | 10                                                         |
| 刪單成功     | 30                                                         |
| 完全成交     | 50                                                         |
| 改價失敗     | 19                                                         |
| 改量失敗     | 29                                                         |
| 刪單失敗     | 39                                                         |
| 失敗         | 90                                                         |

#### Month[​](#month "Direct link to Month")

月份代號

##### 期貨[​](#期貨 "Direct link to 期貨")

| 一月 | 二月 | 三月 | 四月 | 五月   | 六月   |
| ---- | ---- | ---- | ---- | ------ | ------ |
| A    | B    | C    | D    | E      | F      |
| 七月 | 八月 | 九月 | 十月 | 十一月 | 十二月 |
| G    | H    | I    | J    | K      | L      |

##### 選擇權[​](#選擇權 "Direct link to 選擇權")

Call

| 一月 | 二月 | 三月 | 四月 | 五月   | 六月   |
| ---- | ---- | ---- | ---- | ------ | ------ |
| A    | B    | C    | D    | E      | F      |
| 七月 | 八月 | 九月 | 十月 | 十一月 | 十二月 |
| G    | H    | I    | J    | K      | L      |

Put

| 一月 | 二月 | 三月 | 四月 | 五月   | 六月   |
| ---- | ---- | ---- | ---- | ------ | ------ |
| M    | N    | O    | P    | Q      | R      |
| 七月 | 八月 | 九月 | 十月 | 十一月 | 十二月 |
| S    | T    | U    | V    | W      | X      |


---

### 登入

apikeyLogin

版本資訊

v2.2.7 起新增功能

相關說明請參閱 [API Key 說明](https://www.fbs.com.tw/TradeAPI/docs/trading/api-key-apply.txt) 頁面

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別   | 說明           |
| ---------- | ------ | -------------- |
| personalID | String | 登入的ID       |
| key        | String | 申請的 API Key |
| certPath   | String | 憑證路徑       |
| certPass   | String | 憑證密碼       |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳帳號資訊                     |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 帳號資訊 Account 欄位[​](#帳號資訊-account-欄位 "Direct link to 帳號資訊 Account 欄位")

Return type : Object

| 參數        | 類別   | 說明                                     |
| ----------- | ------ | ---------------------------------------- |
| name        | String | 客戶姓名                                 |
| account     | String | 客戶帳號                                 |
| branchNo    | String | 分公司代號                               |
| accountType | string | 帳號類型 回傳 `stock` 證券 `futopt` 期貨 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
const { FubonSDK, BSAction, TimeInForce, OrderType, PriceType, MarketType } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Key","Your Cert Path","Your Cert Password");

Console.log(accounts);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data:
    {
        name: '富邦Bill',    // 客戶姓名 (string)
        account:  '28',      // 客戶帳號 (string)
        branchNo: '6460',     // 分公司代號  (string)
        accountType: 'futopt' // 帳號類型 (string)
    }
}


```


---

### 登入

login

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別   | 說明       |
| ---------- | ------ | ---------- |
| personalID | String | 登入的ID   |
| password   | String | 登入的密碼 |
| certPath   | String | 憑證路徑   |
| certPass   | String | 憑證密碼   |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳帳號資訊                     |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 帳號資訊 Account 欄位[​](#帳號資訊-account-欄位 "Direct link to 帳號資訊 Account 欄位")

Return type : Object

| 參數        | 類別   | 說明                                     |
| ----------- | ------ | ---------------------------------------- |
| name        | String | 客戶姓名                                 |
| account     | String | 客戶帳號                                 |
| branchNo    | String | 分公司代號                               |
| accountType | string | 帳號類型 回傳 `stock` 證券 `futopt` 期貨 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
const { FubonSDK, BSAction, TimeInForce, OrderType, PriceType, MarketType } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("Your ID", "Your Password","Your Cert Path","Your Cert Password");

Console.log(accounts);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data:
    {
        name: '富邦Bill',    // 客戶姓名 (string)
        account:  '28',      // 客戶帳號 (string)
        branchNo: '6460',     // 分公司代號  (string)
        accountType: 'futopt' // 帳號類型 (string)
    }
}


```


---

### 登出

logout

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別 | 說明     |
| --------- | ---- | -------- |
| isSuccess | bool | 是否成功 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
const { FubonSDK, BSAction, TimeInForce, OrderType, PriceType, MarketType } = require('fubon-neo');

const sdk = new FubonSDK();

sdk.logout();

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
true

```


---

### 刪除批次委託單

batchCancelOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                                    | 說明               |
| ----------- | ----------------------------------------------------------------------------------------------------------------------- | ------------------ |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                     | 帳號               |
| orderResult | [FutOptOrderResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptorderresult) | 批次取消委託單內容 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳修改資訊                     |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 修改資訊 FutOptOrderResult 欄位[​](#修改資訊-futoptorderresult-欄位 "Direct link to 修改資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                                                 | 說明                                                                                                                                            |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | number                                                                                                                                               | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                                               | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                                               | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                                               | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                                               | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                                               | 委託書號                                                                                                                                        |
| assetType       | number                                                                                                                                               | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                                               | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | js<!-- -->:string<!-- --> / ts:[FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | number                                                                                                                                               | 單位數                                                                                                                                          |
| currency        | string                                                                                                                                               | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                                               | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                                               | 到期日                                                                                                                                          |
| strikePrice     | number                                                                                                                                               | 履約價                                                                                                                                          |
| callPut         | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                                               | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                                               | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | number                                                                                                                                               | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySellLeg2     | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | number                                                                                                                                               | 價格                                                                                                                                            |
| lot             | number                                                                                                                                               | 原始委託股口數                                                                                                                                  |
| timeInForce     | js<!-- -->:string<!-- --> / ts:[TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#timeinforce)           | 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | js<!-- -->:string<!-- --> / ts:[FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                                                 | 是否為預約單                                                                                                                                    |
| status          | number                                                                                                                                               | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | number                                                                                                                                               | 有效委託價格                                                                                                                                    |
| afterLot        | number                                                                                                                                               | 有效委託口數                                                                                                                                    |
| filledLot       | number                                                                                                                                               | 成交口數                                                                                                                                        |
| filledMoney     | number                                                                                                                                               | 成交價金                                                                                                                                        |
| beforeLot       | number                                                                                                                                               | 改單前有效口數                                                                                                                                  |
| beforePrice     | number                                                                                                                                               | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                                               | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                                               | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                                                 | 委託歷程 (查詢orderResultDetail or orderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                                               | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js

// 批次改價(利用batch detail回傳的內容刪單)
cancel_object = [
    batch_results_detail.data[0],
    batch_results_detail.data[1],
]
sdk.futopt.batchCancelOrder(account, cancel_object)

// 批次改量(利用不同的單筆委託)
cancel_object = [
    orders[37],
    orders[35],
]

sdk.futopt.batchCancelOrder(account, cancel_object)


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data: 
  {
      functionType: 30,                       // 功能別 (number)
      date: "2024/03/25",                     // 交易日期 (string)
      seqNo: "00110212608",                   // 委託單流水序號 (string)
      branchNo: "15901",                      // 分公司代號 (string)
      account: "1234567",                     // 帳號 (string)
      orderNo: "C0001",                       // 委託書號 (string)
      assetType: 1,                           // 資產類別 (number)
      market: "TAIMEX",                       // 市場類型 (string)
      marketType: "Future",                   // 盤別種類 (FutOptMarketType)
      unit: 1,                                // 單位數 (number)
      currency: "TWD",                        // 幣別 (string)
      symbol: "FITF",                         // 商品代號 (string)
      expiryDate: "202404",                   // 到期日 (string)
      buySell: "Buy",                         // 買賣別 (BSAction)
      priceType: "Limit",                     // 原始委託價格別 (FutOptPriceType)
      price: 1822.6,                          // 價格 (number)
      lot: 2,                                 // 原始委託股口數 (number)
      timeInForce: "ROD",                     // 委託條件別 (TimeInforce)
      orderType: "Auto",                      // 委託單類型 (FutOptOrderType)
      isPreOrder: false,                      // 是否為預約單 (bool)
      status: 30,                             // 委託單狀態 (number)
      afterPrice: 1822.6,                     // 有效委託價格 (number)
      afterLot: 0,                            // 有效委託股口數 (number)
      filledLot: 0,                           // 成交股口數 (number)
      filledMoney: 0,                         // 成交價金 (number)
      lastTime: "13:21:34",                   // 最後異動時間 (string)
  },
  {
    functionType: 30,                       // 功能別 (number)
    date: "2024/03/25",                     // 交易日期 (string)
    seqNo: "00110212608",                   // 委託單流水序號 (string)
    branchNo: "15901",                      // 分公司代號 (string)
    account: "1234567",                     // 帳號 (string)
    orderNo: "C0001",                       // 委託書號 (string)
    assetType: 1,                           // 資產類別 (number)
    market: "TAIMEX",                       // 市場類型 (string)
    marketType: "Future",                   // 盤別種類 (FutOptMarketType)
    unit: 1,                                // 單位數 (number)
    currency: "TWD",                        // 幣別 (string)
    symbol: "FITF",                         // 商品代號 (string)
    expiryDate: "202404",                   // 到期日 (string)
    buySell: "Buy",                         // 買賣別 (BSAction)
    priceType: "Limit",                     // 原始委託價格別 (FutOptPriceType)
    price: 1822.6,                          // 價格 (number)
    lot: 2,                                 // 原始委託股口數 (number)
    timeInForce: "ROD",                     // 委託條件別 (TimeInforce)
    orderType: "Auto",                      // 委託單類型 (FutOptOrderType)
    isPreOrder: false,                      // 是否為預約單 (bool)
    status: 30,                             // 委託單狀態 (number)
    ... 
  }
}

```


---

### 批次修改委託價格

batchModifyPrice

##### 先使用makeModifyPriceObj 建立 FutOptModifyPrice 物件[​](#先使用makemodifypriceobj-建立-futoptmodifyprice-物件 "Direct link to 先使用makeModifyPriceObj 建立 FutOptModifyPrice 物件")

| 參數        | 類別                                                                                                                    | 說明             |
| ----------- | ----------------------------------------------------------------------------------------------------------------------- | ---------------- |
| orderResult | [FutOptOrderResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptorderresult) | 預計修改的委託單 |
| price       | string ?                                                                                                                | 修改後的價格     |
| priceType   | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)?    | 修改後的價格旗標 |

將回傳的物件放入 BatchModifyPrice 的方法中

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數           | 類別                                                                                                                                    | 說明               |
| -------------- | --------------------------------------------------------------------------------------------------------------------------------------- | ------------------ |
| account        | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                                     | 帳號               |
| ModifyPriceObj | [FutOptModifyPrice](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmodifyprice)(list of object) | 批次修改委託單內容 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳修改資訊                     |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 修改資訊 FutOptOrderResult 欄位[​](#修改資訊-futoptorderresult-欄位 "Direct link to 修改資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                                                 | 說明                                                                                                                                            |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | number                                                                                                                                               | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                                               | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                                               | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                                               | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                                               | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                                               | 委託書號                                                                                                                                        |
| assetType       | number                                                                                                                                               | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                                               | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | js<!-- -->:string<!-- --> / ts:[FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | number                                                                                                                                               | 單位數                                                                                                                                          |
| currency        | string                                                                                                                                               | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                                               | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                                               | 到期日                                                                                                                                          |
| strikePrice     | number                                                                                                                                               | 履約價                                                                                                                                          |
| callPut         | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                                               | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                                               | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | number                                                                                                                                               | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySellLeg2     | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | number                                                                                                                                               | 價格                                                                                                                                            |
| lot             | number                                                                                                                                               | 原始委託股口數                                                                                                                                  |
| timeInForce     | js<!-- -->:string<!-- --> / ts:[TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#timeinforce)           | 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | js<!-- -->:string<!-- --> / ts:[FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                                                 | 是否為預約單                                                                                                                                    |
| status          | number                                                                                                                                               | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | number                                                                                                                                               | 有效委託價格                                                                                                                                    |
| afterLot        | number                                                                                                                                               | 有效委託口數                                                                                                                                    |
| filledLot       | number                                                                                                                                               | 成交口數                                                                                                                                        |
| filledMoney     | number                                                                                                                                               | 成交價金                                                                                                                                        |
| beforeLot       | number                                                                                                                                               | 改單前有效口數                                                                                                                                  |
| beforePrice     | number                                                                                                                                               | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                                               | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                                               | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                                                 | 委託歷程 (查詢orderResultDetail or orderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                                               | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
// 批次改價 ( 利用 batch detail 回傳的內容改單)
modify_orders = [ 
    sdk.futopt.makeModifyPriceObj(batch_res.data[0], "19900"),
    sdk.futopt.makeModifyPriceObj(batch_res.data[1], "19900")
]

sdk.futopt.batchModifyPrice(account, modify_orders);


// 批次改價 ( 利用不同的單筆委託改單 )
modify_orders = [ 
    sdk.futopt.makeModifyPriceObj(orderResult.data[20], "19900"),
    sdk.futopt.makeModifyPriceObj(orderResult.data[21], "19900"),
    sdk.futopt.makeModifyPriceObj(orderResult.data[28], "19900")
]

sdk.futopt.batchModifyPrice(account, modify_orders);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data: [ // 批次改價 ( 利用 batch detail 回傳的內容改單) 
    {
      functionType: 15,                      // 功能別 (int)
      date: "2024/03/25",                    // 交易日期 (string)
      seqNo: "00110212663",                  // 委託單流水序號 (string)
      branchNo: "15901",                      // 分公司代號 (string)
      account: "1234567",                     // 帳號 (string)
      orderNo: "C0005",                       // 委託書號 (string)
      assetType: 1,                          // 資產類別 (int)
      market: "TAIMEX",                       // 市場類型 (string)
      marketType: "Future",                   // 盤別種類 (FutOptPriceType)
      currency: "TWD",                        // 幣別 (string)
      symbol: "FITX",                         // 商品代號 (string)
      expiryDate: "0",                        // 到期日 (string)
      strikePrice: 0,                         // 履約價 (float)
      buySell: "Buy",                         // 買賣別 (BSAction)
      priceType: "Limit",                     // 原始委託價格別 (FutOptPriceType)
      price: 20000,                           // 價格 (float)
      lot: 1,                                 // 原始委託股口數 (int)
      timeInForce: "ROD",                     // 委託條件別 (TimeInforce)
      orderType: "New",                       // 委託單類型 (FutOptOrderType)
      isPreOrder: false,                      // 是否為預約單 (bool)
      status: 10,                             // 委託單狀態 (int)
      afterPriceType: "Limit",                // 有效委託價格別 (FutOptPriceType)
      afterPrice: 19900,                      // 有效委託價格 (float)
      afterLot: 1,                            // 有效委託股口數 (int)
      filledLot: 0,                           // 成交股口數 (int)
      filledMoney: 0,                         // 成交價金 (int)
      beforeLot: 0,                           // 改單前有效股口數 (int)
      beforePrice: 20000,                     // 改單前有效價格 (float)
      lastTime: "13:39:05",                   // 最後異動時間 (string)
    },
    {
      functionType: 15,                      // 功能別 (int)
      date: "2024/03/25",                    // 交易日期 (string)
      seqNo: "00110212664",                  // 委託單流水序號 (string)
      branchNo: "15901",                      // 分公司代號 (string)
      account: "1234567",                     // 帳號 (string)
      orderNo: "C0006",                       // 委託書號 (string)
      assetType: 1,                          // 資產類別 (int)
      market: "TAIMEX",                       // 市場類型 (string)
      marketType: "Future",                   // 盤別種類 (FutOptPriceType)
      currency: "TWD",                        // 幣別 (string)
      symbol: "FITX",                         // 商品代號 (string)
      expiryDate: "0",                        // 到期日 (string)
      strikePrice: 0,                         // 履約價 (float)
      buySell: "Buy",                         // 買賣別 (BSAction)
      priceType: "Limit",                     // 原始委託價格別 (FutOptPriceType)
      price: 20000,                           // 價格 (float)
      lot: 1,                                 // 原始委託股口數 (int)
      timeInForce: "ROD",                     // 委託條件別 (TimeInforce)
      orderType: "New",                       // 委託單類型 (FutOptOrderType)
      isPreOrder: false,                      // 是否為預約單 (bool)
      status: 10,                             // 委託單狀態 (int)
      ...
    }
  ]
}

以下擷取data內容

[ // 批次改價 ( 利用不同的單筆委託改單 )
  {
      functionType: 15,                      // 功能別 (int)
      date: "2024/03/25",                    // 交易日期 (string)
      seqNo: "00110212763",                  // 委託單流水序號 (string)
      branchNo: "15901",                      // 分公司代號 (string)
      account: "1234567",                     // 帳號 (string)
      orderNo: "C0015",                       // 委託書號 (string)
      assetType: 1,                          // 資產類別 (int)
      market: "TAIMEX",                       // 市場類型 (string)
      marketType: "Future",                   // 盤別種類 (FutOptPriceType)
      currency: "TWD",                        // 幣別 (string)
      symbol: "FITX",                         // 商品代號 (string)
      expiryDate: "0",                        // 到期日 (string)
      strikePrice: 0,                         // 履約價 (float)
      buySell: "Buy",                         // 買賣別 (BSAction)
      priceType: "Limit",                     // 原始委託價格別 (FutOptPriceType)
      price: 20000,                           // 價格 (float)
      lot: 1,                                 // 原始委託股口數 (int)
      timeInForce: "ROD",                     // 委託條件別 (TimeInforce)
      orderType: "New",                       // 委託單類型 (FutOptOrderType)
      isPreOrder: false,                      // 是否為預約單 (bool)
      status: 10,                             // 委託單狀態 (int)
      afterPriceType: "Limit",                // 有效委託價格別 (FutOptPriceType)
      afterPrice: 19900,                      // 有效委託價格 (float)
      afterLot: 1,                            // 有效委託股口數 (int)
      filledLot: 0,                           // 成交股口數 (int)
      filledMoney: 0,                         // 成交價金 (int)
      beforeLot: 0,                           // 改單前有效股口數 (int)
      beforePrice: 20000,                     // 改單前有效價格 (float)
      lastTime: "13:39:35",                   // 最後異動時間 (string)
  },
  {
     functionType: 15,                      // 功能別 (int)
      date: "2024/03/25",                    // 交易日期 (string)
      seqNo: "00110212764",                  // 委託單流水序號 (string)
      branchNo: "15901",                      // 分公司代號 (string)
      account: "1234567",                     // 帳號 (string)
      orderNo: "C0016",                       // 委託書號 (string)
      assetType: 1,                          // 資產類別 (int)
      market: "TAIMEX",                       // 市場類型 (string)
      marketType: "Future",                   // 盤別種類 (FutOptPriceType)
      currency: "TWD",                        // 幣別 (string)
      symbol: "FITX",                         // 商品代號 (string)
      expiryDate: "0",                        // 到期日 (string)
      strikePrice: 0,                         // 履約價 (float)
      buySell: "Buy",                         // 買賣別 (BSAction)
      priceType: "Limit",                     // 原始委託價格別 (FutOptPriceType)
      price: 20000,                           // 價格 (float)
      lot: 1,                                 // 原始委託股口數 (int)
      timeInForce: "ROD",                     // 委託條件別 (TimeInforce)
      orderType: "New",                       // 委託單類型 (FutOptOrderType)
      isPreOrder: false,                      // 是否為預約單 (bool)
      status: 10,                             // 委託單狀態 (int)
      ...
    },
    ...
]

```


---

### 批次修改委託數量

batchModifyQuantity

##### 先使用 makeModifyLotObj 建立一個 FutoptModifyLot 物件[​](#先使用-makemodifylotobj-建立一個-futoptmodifylot-物件 "Direct link to 先使用 makeModifyLotObj 建立一個 FutoptModifyLot 物件")

| 參數        | 類別                                                                                                                    | 說明                                                |
| ----------- | ----------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| orderResult | [FutOptOrderResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptorderresult) | 預計修改的委託單                                    |
| lot         | number                                                                                                                  | 改單後的委託量 ( 修改後數量包含此委託單已成交部份 ) |

將回傳的物件放入batchModifyLot 方法中

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數           | 類別                                                                                                                                 | 說明               |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------------ | ------------------ |
| account        | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                                  | 帳號               |
| ModifiedLotObj | [FutoptModifyLot](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmodifylot) (list of object) | 批次改量委託單內容 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳修改資訊                     |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 修改資訊 FutOptOrderResult 欄位[​](#修改資訊-futoptorderresult-欄位 "Direct link to 修改資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                                                 | 說明                                                                                                                                            |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | number                                                                                                                                               | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                                               | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                                               | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                                               | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                                               | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                                               | 委託書號                                                                                                                                        |
| assetType       | number                                                                                                                                               | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                                               | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | js<!-- -->:string<!-- --> / ts:[FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | number                                                                                                                                               | 單位數                                                                                                                                          |
| currency        | string                                                                                                                                               | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                                               | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                                               | 到期日                                                                                                                                          |
| strikePrice     | number                                                                                                                                               | 履約價                                                                                                                                          |
| callPut         | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                                               | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                                               | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | number                                                                                                                                               | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySellLeg2     | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | number                                                                                                                                               | 價格                                                                                                                                            |
| lot             | number                                                                                                                                               | 原始委託股口數                                                                                                                                  |
| timeInForce     | js<!-- -->:string<!-- --> / ts:[TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#timeinforce)           | 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | js<!-- -->:string<!-- --> / ts:[FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                                                 | 是否為預約單                                                                                                                                    |
| status          | number                                                                                                                                               | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | number                                                                                                                                               | 有效委託價格                                                                                                                                    |
| afterLot        | number                                                                                                                                               | 有效委託口數                                                                                                                                    |
| filledLot       | number                                                                                                                                               | 成交口數                                                                                                                                        |
| filledMoney     | number                                                                                                                                               | 成交價金                                                                                                                                        |
| beforeLot       | number                                                                                                                                               | 改單前有效口數                                                                                                                                  |
| beforePrice     | number                                                                                                                                               | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                                               | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                                               | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                                                 | 委託歷程 (查詢orderResultDetail or orderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                                               | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
// 批次改量(利用batch detail回傳的內容改單)
modify_orders = [ 
    sdk.futopt.makeModifyLotObj(batch_res.data[0], 2),
    sdk.futopt.makeModifyLotObj(batch_res.data[1], 2)
]

sdk.futopt.batchModifyLot(account, modify_orders);


// # 批次改量(利用不同的單筆委託)
modify_orders = [ 
    sdk.futopt.makeModifyLotObj(orderResult.data[22], 2),
    sdk.futopt.makeModifyLotObj(orderResult.data[40], 2)
]

sdk.futopt.batchModifyLot(account, modify_orders);



```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data: [ // 批次改量(利用batch detail回傳的內容改單)
    {
          functionType: 20,                      // 功能別 (number)
          date: "2024/03/05",                    // 交易日期 (string)
          seqNo: "03100151319",                  // 委託單流水序號 (string)
          branchNo: "15901",                      // 分公司代號 (string)
          account: "1234567",                     // 帳號 (string)
          orderNo: "l001A",                       // 委託書號 (string)
          assetType: 1,                          // 資產類別 (number)
          market: "TAIMEX",                       // 市場類型 (string)
          marketType: "FutureNight",              // 盤別種類 (FutOptMarketType)
          currency: "TWD",                        // 幣別 (string)
          symbol: "FIMTX",                        // 商品代號 (string)
          buySell: "Buy",                         // 買賣別 (BSAction)
          priceType: "Limit",                     // 原始委託價格別 (FutOptPriceType)
          price: 20000,                           // 價格 (number)
          lot: 3,                                 // 原始委託股口數 (number)
          timeInForce: "ROD",                     // 委託條件別 (TimeInforce)
          orderType: "New",                       // 委託單類型 (FutOptOrderType)
          isPreOrder: false,                      // 是否為預約單 (bool)
          status: 10,                             // 委託單狀態 (number)
          afterPriceType: "Limit",                // 有效委託價格別 (FutOptPriceType)
          afterPrice: 20000,                      // 有效委託價格 (number)
          afterLot: 2,                            // 有效委託股口數 (number)
          filledLot: 0,                           // 成交股口數 (number)
          filledMoney: 0,                         // 成交價金 (number)
          beforeLot: 0,                           // 改單前有效股口數 (number)
          beforePrice: 20000,                     // 改單前有效價格 (number)
          lastTime: "18:24:40",                   // 最後異動時間 (string)
    },
    {
          functionType: 20,                      // 功能別 (number)
          date: "2024/03/05",                    // 交易日期 (string)
          seqNo: "03100151320",                  // 委託單流水序號 (string)
          branchNo: "15901",                      // 分公司代號 (string)
          account: "1234567",                     // 帳號 (string)
          orderNo: "l001B",                       // 委託書號 (string)
          assetType: 1,                          // 資產類別 (number)
          market: "TAIMEX",                       // 市場類型 (string)
          marketType: "FutureNight",              // 盤別種類 (FutOptMarketType)
          currency: "TWD",                        // 幣別 (string)
          symbol: "FIMTX",                        // 商品代號 (string)
          buySell: "Buy",                         // 買賣別 (BSAction)
          priceType: "Limit",                     // 原始委託價格別 (FutOptPriceType)
          price: 20000,                           // 價格 (number)
          lot: 3,                                 // 原始委託股口數 (number)
          timeInForce: "ROD",                     // 委託條件別 (TimeInforce)
          orderType: "New",                       // 委託單類型 (FutOptOrderType)
          isPreOrder: false,                      // 是否為預約單 (bool)
          status: 10,                             // 委託單狀態 (number)
          afterPriceType: "Limit",                // 有效委託價格別 (FutOptPriceType)
          afterPrice: 20000,                      // 有效委託價格 (number)
          afterLot: 2,                            // 有效委託股口數 (number)
          filledLot: 0,                           // 成交股口數 (number)
          filledMoney: 0,                         // 成交價金 (number)
          beforeLot: 0,                           // 改單前有效股口數 (number)
          beforePrice: 20000,                     // 改單前有效價格 (number)
          lastTime: "18:24:40",                   // 最後異動時間 (string)
    }
  ]
}

以下擷取data內容

[ // 批次改量(利用不同的單筆委託)
  {
          functionType: 20,                      // 功能別 (number)
          date: "2024/03/05",                    // 交易日期 (string)
          seqNo: "03100151419",                  // 委託單流水序號 (string)
          branchNo: "15901",                      // 分公司代號 (string)
          account: "1234567",                     // 帳號 (string)
          orderNo: "l001Q",                       // 委託書號 (string)
          assetType: 1,                          // 資產類別 (number)
          market: "TAIMEX",                       // 市場類型 (string)
          marketType: "FutureNight",              // 盤別種類 (FutOptMarketType)
          currency: "TWD",                        // 幣別 (string)
          symbol: "FIMTX",                        // 商品代號 (string)
          buySell: "Buy",                         // 買賣別 (BSAction)
          priceType: "Limit",                     // 原始委託價格別 (FutOptPriceType)
          price: 20000,                           // 價格 (number)
          lot: 3,                                 // 原始委託股口數 (number)
          timeInForce: "ROD",                     // 委託條件別 (TimeInforce)
          orderType: "New",                       // 委託單類型 (FutOptOrderType)
          isPreOrder: false,                      // 是否為預約單 (bool)
          status: 10,                             // 委託單狀態 (number)
          afterPriceType: "Limit",                // 有效委託價格別 (FutOptPriceType)
          afterPrice: 20000,                      // 有效委託價格 (number)
          afterLot: 2,                            // 有效委託股口數 (number)
          filledLot: 0,                           // 成交股口數 (number)
          filledMoney: 0,                         // 成交價金 (number)
          beforeLot: 0,                           // 改單前有效股口數 (number)
          beforePrice: 20000,                     // 改單前有效價格 (number)
          lastTime: "18:44:40",                   // 最後異動時間 (string)
    },
    {
          functionType: 20,                      // 功能別 (number)
          date: "2024/03/05",                    // 交易日期 (string)
          seqNo: "0310015142",                  // 委託單流水序號 (string)
          branchNo: "15901",                      // 分公司代號 (string)
          account: "1234567",                     // 帳號 (string)
          orderNo: "l001R",                       // 委託書號 (string)
          assetType: 1,                          // 資產類別 (number)
          market: "TAIMEX",                       // 市場類型 (string)
          marketType: "FutureNight",              // 盤別種類 (FutOptMarketType)
          currency: "TWD",                        // 幣別 (string)
          symbol: "FIMTX",                        // 商品代號 (string)
          buySell: "Buy",                         // 買賣別 (BSAction)
          priceType: "Limit",                     // 原始委託價格別 (FutOptPriceType)
          price: 20000,                           // 價格 (number)
          lot: 3,                                 // 原始委託股口數 (number)
          timeInForce: "ROD",                     // 委託條件別 (TimeInforce)
          orderType: "New",                       // 委託單類型 (FutOptOrderType)
          isPreOrder: false,                      // 是否為預約單 (bool)
          status: 10,                             // 委託單狀態 (number)
          afterPriceType: "Limit",                // 有效委託價格別 (FutOptPriceType)
          afterPrice: 20000,                      // 有效委託價格 (number)
          afterLot: 2,                            // 有效委託股口數 (number)
          filledLot: 0,                           // 成交股口數 (number)
          filledMoney: 0,                         // 成交價金 (number)
          beforeLot: 0,                           // 改單前有效股口數 (number)
          beforePrice: 20000,                     // 改單前有效價格 (number)
          lastTime: "18:44:40",                   // 最後異動時間 (string)
    }
]

```


---

### 取得批次委託明細

batchOrderDetail

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數           | 類別                                                                                                        | 說明         |
| -------------- | ----------------------------------------------------------------------------------------------------------- | ------------ |
| account        | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)         | 帳號         |
| BatchOrderList | [BatchResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#batchresult) | 批次委託列表 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳委託資訊                     |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 FutOptOrderResult 欄位[​](#委託資訊-futoptorderresult-欄位 "Direct link to 委託資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                                                 | 說明                                                                                                                                            |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | number (Optional)                                                                                                                                    | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                                               | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                                               | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                                               | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                                               | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                                               | 委託書號                                                                                                                                        |
| assetType       | number                                                                                                                                               | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                                               | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | js<!-- -->:string<!-- --> / ts:[FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | number                                                                                                                                               | 單位數                                                                                                                                          |
| currency        | string                                                                                                                                               | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                                               | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                                               | 到期日                                                                                                                                          |
| strikePrice     | number                                                                                                                                               | 履約價                                                                                                                                          |
| callPut         | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                                               | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                                               | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | number                                                                                                                                               | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySellLeg2     | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | number                                                                                                                                               | 價格                                                                                                                                            |
| lot             | number                                                                                                                                               | 原始委託股口數                                                                                                                                  |
| timeInForce     | js<!-- -->:string<!-- --> / ts:[TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#timeinforce)           | 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | js<!-- -->:string<!-- --> / ts:[FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                                                 | 是否為預約單                                                                                                                                    |
| status          | number                                                                                                                                               | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | number                                                                                                                                               | 有效委託價格                                                                                                                                    |
| afterLot        | number                                                                                                                                               | 有效委託口數                                                                                                                                    |
| filledLot       | number                                                                                                                                               | 成交口數                                                                                                                                        |
| filledMoney     | number                                                                                                                                               | 成交價金                                                                                                                                        |
| beforeLot       | number                                                                                                                                               | 改單前有效口數                                                                                                                                  |
| beforePrice     | number                                                                                                                                               | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                                               | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                                               | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                                                 | 委託歷程 (查詢orderResultDetail or orderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                                               | 錯誤訊息                                                                                                                                        |

caution

此功能僅供查詢批次送單執行結果，欲取得委託單最新狀態請使用單筆委託單查詢功能

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
const batch_order_list = sdk.futopt.batchOrderLists(account)
sdk.futopt.batchOrderDetail(account, batch_order_list.data[0])

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data: [
   {
          date: "2024/03/25",                      // 交易日期 (string)
          seqNo: "00110212608",                    // 委託單流水序號 (string)
          branchNo: "15901",                        // 分公司代號 (string)
          account: "1234567",                       // 帳號 (string)
          orderNo: "C0001",                         // 委託書號 (string)
          assetType: 1,                             // 資產類別 (number)
          market: "TAIMEX",                         // 市場類型 (string)
          marketType: "Future",                     // 盤別種類 (FutOptMarketType)
          unit: 1,                                  // 單位數 (number)
          currency: "TWD",                          // 幣別 (string)
          symbol: "FITF",                           // 商品代號 (string)
          expiryDate: "202404",                     // 到期日 (string)
          buySell: "Buy",                           // 買賣別 (BSAction)
          priceType: "Limit",                       // 原始委託價格別 (FutOptPriceType)
          price: 1822.6,                            // 價格 (number)
          lot: 2,                                   // 原始委託股口數 (number)
          timeInForce: "ROD",                       // 委託條件別 (TimeInforce)
          orderType: "Auto",                        // 委託單類型 (FutOptOrderType)
          isPreOrder: false,                        // 是否為預約單 (bool)
          status: 10,                               // 委託單狀態 (number)
          afterPrice: 1822.6,                       // 有效委託價格 (number)
          afterLot: 2,                              // 有效委託股口數 (number)
          filledLot: 0,                             // 成交股口數 (number)
          filledMoney: 0,                           // 成交價金 (number)
          lastTime: "10:20:27",                     // 最後異動時間 (string)
      },
    ...
  ] 
}

```


---

### 取得批次委託列表

batchOrderLists

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                                | 說明 |
| ------- | --------------------------------------------------------------------------------------------------- | ---- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account) | 帳號 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳批次單資訊                   |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 批次單 BatchResult 欄位[​](#批次單-batchresult-欄位 "Direct link to 批次單 BatchResult 欄位")

Return type : Object

| 參數         | 類別   | 說明                                                                           |
| ------------ | ------ | ------------------------------------------------------------------------------ |
| functionType | number | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗 |
| date         | string | 交易日期                                                                       |
| branchNo     | string | 分公司代號                                                                     |
| account      | string | 帳號                                                                           |
| batchSeqNo   | string | 批次單流水序號                                                                 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
sdk.futopt.batchOrderLists(target_user)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data: [
    {
      functionType: 0,
      date: '2023/10/24',
      branchNo: '6460',
      account: '26',
      batchSeqNo: '11EE72270B4D79F48000000C29304663'
    },
    ...
  ]
}

```


---

### 建立批次委託單

batchPlaceOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                                         | 說明     |
| ----------- | ---------------------------------------------------------------------------------------------------------------------------- | -------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                          | 帳號     |
| OrderObject | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#orderobject) (list of object) | 委託內容 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳委託資訊                     |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 FutOptOrderResult 欄位[​](#委託資訊-futoptorderresult-欄位 "Direct link to 委託資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                                                 | 說明                                                                                                                                            |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | number                                                                                                                                               | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                                               | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                                               | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                                               | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                                               | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                                               | 委託書號                                                                                                                                        |
| assetType       | number                                                                                                                                               | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                                               | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | js<!-- -->:string<!-- --> / ts:[FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | number                                                                                                                                               | 單位數                                                                                                                                          |
| currency        | string                                                                                                                                               | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                                               | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                                               | 到期日                                                                                                                                          |
| strikePrice     | number                                                                                                                                               | 履約價                                                                                                                                          |
| callPut         | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                                               | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                                               | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | number                                                                                                                                               | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySellLeg2     | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | number                                                                                                                                               | 價格                                                                                                                                            |
| lot             | number                                                                                                                                               | 原始委託股口數                                                                                                                                  |
| timeInForce     | js<!-- -->:string<!-- --> / ts:[TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#timeinforce)           | 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | js<!-- -->:string<!-- --> / ts:[FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                                                 | 是否為預約單                                                                                                                                    |
| status          | number                                                                                                                                               | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | number                                                                                                                                               | 有效委託價格                                                                                                                                    |
| afterLot        | number                                                                                                                                               | 有效委託口數                                                                                                                                    |
| filledLot       | number                                                                                                                                               | 成交口數                                                                                                                                        |
| filledMoney     | number                                                                                                                                               | 成交價金                                                                                                                                        |
| beforeLot       | number                                                                                                                                               | 改單前有效口數                                                                                                                                  |
| beforePrice     | number                                                                                                                                               | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                                               | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                                               | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                                                 | 委託歷程 (查詢orderResultDetail or orderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                                               | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
const orders = [{
 buySell: BSAction.Buy,
    symbol: "TXO20000E4",
    price: "530",
    lot: 1,
    marketType: FutOptMarketType.Option,
    priceType: FutOptPriceType.Limit,
    timeInForce: TimeInForce.ROD,
    orderType: FutOptOrderType.Auto,
    userDef: "nodejs" // optional field
  },{
    buySell: BSAction.Buy,
    symbol: "TXO20000E4",
    price: "530",
    lot: 1,
    marketType: FutOptMarketType.Option,
    priceType: FutOptPriceType.Limit,
    timeInForce: TimeInForce.ROD,
    orderType: FutOptOrderType.Auto,
    userDef: "nodejs" // optional field
  }
];

const batch_order_results = sdk.futopt.batchPlaceOrder(account,orders);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data:[
    {
                functionType: 0,                    // 功能別 (number)
                date: "2024/03/15",                 // 交易日期 (string)
                seqNo: "00230177010",               // 委託單流水序號 (string)
                branchNo: "15901",                  // 分公司代號 (string)
                account: "1234567",                 // 帳號 (string)
                orderNo: "C0002",                   // 委託書號 (string)
                assetType: 2,                       // 資產類別 (number)
                market: "TAIMEX",                   // 市場類型 (string)
                marketType: Option,                 // 盤別種類 (FutOptMarketType)
                unit: 1,                            // 單位數 (number)
                currency: "TWD",                    // 幣別 (string)
                symbol: "TXO",                      // 商品代號 (string)
                expiryDate: "202404",               // 到期日 (string)
                strikePrice: 20000,                // 履約價 (number)
                callPut: "Call",                    // 買賣權 (string)
                buySell: Buy,                       // 買賣別 (BSAction)
                priceType: Limit,                   // 原始委託價格別 (FutOptPriceType)
                price: 500,                         // 價格 (number)
                lot: 1,                             // 原始委託股口數 (number)
                timeInForce: ROD,                   // 委託條件別 (TimeInforce)
                orderType: Auto,                    // 委託單類型 (FutOptOrderType)
                isPreOrder: false,                  // 是否為預約單 (bool)
                status: 10,                         // 委託單狀態 (number)
                afterPriceType: Limit,              // 有效委託價格別 (FutOptPriceType)
                afterPrice: 500,                    // 有效委託價格 (number)
                afterLot: 1,                        // 有效委託股口數 (number)
                filledLot: 0,                       // 成交股口數 (number)
                filledMoney: 0,                     // 成交價金 (number)
                beforeLot: 0,                       // 改單前有效股口數 (number)
                beforePrice: 500,                   // 改單前有效價格 (number)
                userDef: "nodejs",                  // 自訂欄位 (string)
                lastTime: "11:50:08",               // 最後異動時間 (string)
    },
    {
                functionType: 0,                    // 功能別 (number)
                date: "2024/03/15",                 // 交易日期 (string)
                seqNo: "00230177011",               // 委託單流水序號 (string)
                branchNo: "15901",                  // 分公司代號 (string)
                account: "1234567",                 // 帳號 (string)
                orderNo: "C0003",                   // 委託書號 (string)
                assetType: 2,                       // 資產類別 (number)
                market: "TAIMEX",                   // 市場類型 (string)
                marketType: Option,                 // 盤別種類 (FutOptMarketType)
                unit: 1,                            // 單位數 (number)
                currency: "TWD",                    // 幣別 (string)
                symbol: "TXO",                      // 商品代號 (string)
                expiryDate: "202404",               // 到期日 (string)
                strikePrice: 20000,                // 履約價 (number)
                callPut: "Call",                    // 買賣權 (string)
                buySell: Buy,                       // 買賣別 (BSAction)
                priceType: Limit,                   // 原始委託價格別 (FutOptPriceType)
                price: 500,                         // 價格 (number)
                lot: 1,                             // 原始委託股口數 (number)
                timeInForce: ROD,                   // 委託條件別 (TimeInforce)
                orderType: Auto,                    // 委託單類型 (FutOptOrderType)
                isPreOrder: false,                  // 是否為預約單 (bool)
                status: 10,                         // 委託單狀態 (number)
                afterPriceType: Limit,              // 有效委託價格別 (FutOptPriceType)
                afterPrice: 500,                    // 有效委託價格 (number)
                afterLot: 1,                        // 有效委託股口數 (number)
                filledLot: 0,                       // 成交股口數 (number)
                filledMoney: 0,                     // 成交價金 (number)
                beforeLot: 0,                       // 改單前有效股口數 (number)
                beforePrice: 500,                   // 改單前有效價格 (number)
                userDef: "nodejs",                  // 自訂欄位 (string)
                lastTime: "11:50:08",               // 最後異動時間 (string)
    }
  ]
}

```


---

### 刪除委託單

cancelOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                                    | 說明               |
| ----------- | ----------------------------------------------------------------------------------------------------------------------- | ------------------ |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                     | 帳號               |
| orderResult | [FutOprOrderResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptorderresult) | 欲取消的委託單物件 |
| unblock     | bool? (default = false)                                                                                                 | 是否採用非阻塞     |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳修改資訊                     |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 修改資訊 FutOptOrderResult 欄位[​](#修改資訊-futoptorderresult-欄位 "Direct link to 修改資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                                                 | 說明                                                                                                                                            |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | number                                                                                                                                               | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                                               | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                                               | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                                               | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                                               | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                                               | 委託書號                                                                                                                                        |
| assetType       | number                                                                                                                                               | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                                               | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | js<!-- -->:string<!-- --> / ts:[FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | number                                                                                                                                               | 單位數                                                                                                                                          |
| currency        | string                                                                                                                                               | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                                               | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                                               | 到期日                                                                                                                                          |
| strikePrice     | number                                                                                                                                               | 履約價                                                                                                                                          |
| callPut         | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                                               | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                                               | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | number                                                                                                                                               | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySellLeg2     | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | number                                                                                                                                               | 價格                                                                                                                                            |
| lot             | number                                                                                                                                               | 原始委託股口數                                                                                                                                  |
| timeInForce     | js<!-- -->:string<!-- --> / ts:[TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#timeinforce)           | 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | js<!-- -->:string<!-- --> / ts:[FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                                                 | 是否為預約單                                                                                                                                    |
| status          | number                                                                                                                                               | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | number                                                                                                                                               | 有效委託價格                                                                                                                                    |
| afterLot        | number                                                                                                                                               | 有效委託口數                                                                                                                                    |
| filledLot       | number                                                                                                                                               | 成交口數                                                                                                                                        |
| filledMoney     | number                                                                                                                                               | 成交價金                                                                                                                                        |
| beforeLot       | number                                                                                                                                               | 改單前有效口數                                                                                                                                  |
| beforePrice     | number                                                                                                                                               | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                                               | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                                               | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                                                 | 委託歷程 (查詢orderResultDetail or orderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                                               | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請�求範例 "Direct link to 請求範例")

```js
sdk.futopt.cancelOrder(account, cancel_order)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data: {
    functionType: 30,                       // 功能別 (number)
    date: "2024/03/25",                     // 交易日期 (string)
    seqNo: "00110212608",                   // 委託單流水序號 (string)
    branchNo: "15901",                      // 分公司代號 (string)
    account: "1234567",                     // 帳號 (string)
    orderNo: "C0001",                       // 委託書號 (string)
    assetType: 1,                           // 資產類別 (number)
    market: "TAIMEX",                       // 市場類型 (string)
    marketType: "Future",                   // 盤別種類 (FutOptMarketType)
    unit: 1,                                // 單位數 (number)
    currency: "TWD",                        // 幣別 (string)
    symbol: "FITF",                         // 商品代號 (string)
    expiryDate: "202404",                   // 到期日 (string)
    buySell: "Buy",                         // 買賣別 (BSAction)
    priceType: "Limit",                     // 原始委託價格別 (FutOptPriceType)
    price: 1822.6,                          // 價格 (number)
    lot: 2,                                 // 原始委託股口數 (number)
    timeInForce: "ROD",                     // 委託條件別 (TimeInforce)
    orderType: "Auto",                      // 委託單類型 (FutOptOrderType)
    isPreOrder: false,                      // 是否為預約單 (bool)
    status: 30,                             // 委託單狀態 (number)
    afterPrice: 1822.6,                     // 有效委託價格 (number)
    afterLot: 0,                            // 有效委託股口數 (number)
    filledLot: 0,                           // 成交股口數 (number)
    filledMoney: 0,                         // 成交價金 (number)
    lastTime: "13:21:34",                   // 最後異動時間 (string)
}
}

```


---

### 商品代號轉換

convertSymbol

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                             | 說明                              |
| ----------- | ---------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| symbol      | string                                                                                                           | 帳務商品代號                      |
| expiryDate  | string                                                                                                           | 履約日                            |
| strikePrice | number ( Optional )                                                                                              | 履約價                            |
| callPut     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput) ( Optional ) | 買賣權 : `Call` Call 、 `Put` Put |

info

月份代號可參閱[參數對照表](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#month)

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數   | 類別   | 說明               |
| ------ | ------ | ------------------ |
| symbol | string | 行情與下單商品代號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
//期貨
sdk.futopt.convertSymbol("FITX","202404")

//選擇權
sdk.futopt.convertSymbol("TXO","202404",20000,CallPut.Call)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js

//期貨
"TXFD4"

//選擇權
"TXO20000D4"

```


---

### 商品保證金查詢

queryEstimateMargin

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                        | 說明     |
| ----------- | ----------------------------------------------------------------------------------------------------------- | -------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)         | 帳號     |
| orderObject | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#orderobject) | 委託物件 |

#### Result 回傳[​](#result-��回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳配額資訊                     |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### EstimateMargin 欄位[​](#estimatemargin-欄位 "Direct link to EstimateMargin 欄位")

Return type : Object

| 參數           | 類別   | 說明       |
| -------------- | ------ | ---------- |
| date           | string | 查詢日期   |
| currency       | string | 幣別       |
| estimateMargin | number | 預估保證金 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
order = FutOptOrder(
    BSAction.Buy,
    "TXFE4",
    "20900",
    1,
    FutOptMarketType.Future,
    FutOptPriceType.Limit,
    TimeInForce.ROD,
    FutOptOrderType.Auto,
    "Fromjs" // optional field
)


sdk.futopt.queryEstimateMargin(account, order)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data:
    {
      date: "2024/04/10",             // 日期 (string)
      currency: "TWD",                // 幣別 (string)
      estimateMargin: 179000          // 預估保證金 (number)
    }
}

```


---

### 查詢歷史成交

filledHistory

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                  | 說明                                                                                                      |
| ---------- | --------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                   | 帳號                                                                                                      |
| marketType | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `FutureNight` 期貨夜盤 、 `Option` 選擇權日盤 、 `OptionNight` 選擇權夜盤 |
| startDate  | string                                                                                                                | 查詢開始日                                                                                                |
| endDate    | string? (空值預設與開始日相同)                                                                                        | 查詢終止日                                                                                                |

info

可查詢最近兩日之歷史資料

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳成交資訊                     |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 成交資訊 FutOptFilledData 欄位[​](#成交資訊-futoptfilleddata-欄位 "Direct link to 成交資訊 FutOptFilledData 欄位")

Return type : Object

| 參數            | 類別                                                                                                                                               | 說明                                                        |
| --------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
| date            | string                                                                                                                                             | 成交日期                                                    |
| branchNo        | string                                                                                                                                             | 分公司代號                                                  |
| account         | string                                                                                                                                             | 帳號                                                        |
| seqNo           | string ?                                                                                                                                           | 委託單流水序號 (只有主動回報才回傳此欄位)                   |
| orderNo         | string                                                                                                                                             | 委託書號                                                    |
| symbol          | string                                                                                                                                             | 商品代號                                                    |
| expiryDate      | string                                                                                                                                             | 到期日                                                      |
| strikePrice     | number                                                                                                                                             | 履約價                                                      |
| callPut         | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                 | 買賣權 : `Call` 買權、 `Put` 賣權                           |
| buySell         | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)               | 買賣別 : `Buy` 買 、 `Sell` 賣                              |
| symbolLeg2      | string                                                                                                                                             | 商品代號 - 複式第二隻腳                                     |
| expiryDateLeg2  | string                                                                                                                                             | 到期日 - 複式第二隻腳                                       |
| strikePriceLeg2 | number                                                                                                                                             | 履約價 - 複式第二隻腳                                       |
| callPutLeg2     | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                 | 買賣權- 複式第二隻腳: `Call` 買權、 `Put` 賣權              |
| buySellLeg2     | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)               | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                |
| orderType       | js<!-- -->:string<!-- --> / ts:[FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptordertype) | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `FdayTrade` 當沖 |
| filledNo        | string                                                                                                                                             | 成交流水號                                                  |
| filledAvgPrice  | number                                                                                                                                             | 成交均價                                                    |
| filledLot       | number                                                                                                                                             | 成交股數                                                    |
| filledPrice     | number                                                                                                                                             | 成交單價                                                    |
| filledTime      | string                                                                                                                                             | 成交時間                                                    |
| userDef         | string ?                                                                                                                                           | 用戶自定義 (只有主動回報才回傳此欄位)                       |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
const filledHistory = sdk.futopt.filledHistory(account,FutOptMarketType.Future,"20230921","20230922");
console.log(filledHistory)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data:[
  {
    date: "2023/09/15",             // 成交日期 (string)
    branchNo: "6460",               // 分公司代號 (string)
    account: "26",                  // 帳號 (string)
    orderNo: "bA422",               // 委託書號 (string)
    symbol: "FITX",                 // 商品代號 (string)
    expiryDate: "202404",           // 履約日 (string)
    buySell: "Buy",                 // 買賣別 (BSAction)
    filledNo: "00000000001",        // 成交流水號 (string)
    filledAvgPrice: 20890.0,        // 成交均價 (float)
    filledLots: 1,                  // 成交股數 (int)
    filledPrice: 20890.0,           // 成交單價 (float)
    orderType: "New",               // 委託單類型 (FutOptOrderType)
    filledTime: "10:31:00.931",     // 成交時間 (string)
  },
    ...
  ]
}


```


---

### 取得委託單結果

getOrderResults

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                                          | 說明                                                                                                      |
| ---------- | --------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                                           | 帳號                                                                                                      |
| marketType | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmarkettype) (Optional : 不帶全盤別) | 盤別種類 : `Future` 期貨日盤 、 `FutureNight` 期貨夜盤 、 `Option` 選擇權日盤 、 `OptionNight` 選擇權夜盤 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳委託資訊                     |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 FutOptOrderResult 欄位[​](#委託資訊-futoptorderresult-欄位 "Direct link to 委託資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                                                 | 說明                                                                                                                                            |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | number (Optional)                                                                                                                                    | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                                               | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                                               | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                                               | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                                               | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                                               | 委託書號                                                                                                                                        |
| assetType       | number                                                                                                                                               | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                                               | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | js<!-- -->:string<!-- --> / ts:[FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | number                                                                                                                                               | 單位數                                                                                                                                          |
| currency        | string                                                                                                                                               | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                                               | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                                               | 到期日                                                                                                                                          |
| strikePrice     | number                                                                                                                                               | 履約價                                                                                                                                          |
| callPut         | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                                               | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                                               | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | number                                                                                                                                               | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySellLeg2     | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | number                                                                                                                                               | 價格                                                                                                                                            |
| lot             | number                                                                                                                                               | 原始委託股口數                                                                                                                                  |
| timeInForce     | js<!-- -->:string<!-- --> / ts:[TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#timeinforce)           | 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | js<!-- -->:string<!-- --> / ts:[FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                                                 | 是否為預約單                                                                                                                                    |
| status          | number                                                                                                                                               | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | number                                                                                                                                               | 有效委託價格                                                                                                                                    |
| afterLot        | number                                                                                                                                               | 有效委託口數                                                                                                                                    |
| filledLot       | number                                                                                                                                               | 成交口數                                                                                                                                        |
| filledMoney     | number                                                                                                                                               | 成交價金                                                                                                                                        |
| beforeLot       | number                                                                                                                                               | 改單前有效口數                                                                                                                                  |
| beforePrice     | number                                                                                                                                               | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                                               | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                                               | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                                                 | 委託歷程 (查詢orderResultDetail or orderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                                               | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
const orders = sdk.futopt.getOrderResults(accounts,FutOptMarketType.Future);
console.log(orders);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data:[
        {
          date: "2024/03/25",                      // 交易日期 (string)
          seqNo: "00110212608",                    // 委託單流水序號 (string)
          branchNo: "15901",                        // 分公司代號 (string)
          account: "1234567",                       // 帳號 (string)
          orderNo: "C0001",                         // 委託書號 (string)
          assetType: 1,                             // 資產類別 (number)
          market: "TAIMEX",                         // 市場類型 (string)
          marketType: "Future",                     // 盤別種類 (FutOptMarketType)
          unit: 1,                                  // 單位數 (number)
          currency: "TWD",                          // 幣別 (string)
          symbol: "FITF",                           // 商品代號 (string)
          expiryDate: "202404",                     // 到期日 (string)
          buySell: "Buy",                           // 買賣別 (BSAction)
          priceType: "Limit",                       // 原始委託價格別 (FutOptPriceType)
          price: 1822.6,                            // 價格 (number)
          lot: 2,                                   // 原始委託股口數 (number)
          timeInForce: "ROD",                       // 委託條件別 (TimeInforce)
          orderType: "Auto",                        // 委託單類型 (FutOptOrderType)
          isPreOrder: false,                        // 是否為預約單 (bool)
          status: 10,                               // 委託單狀態 (number)
          afterPrice: 1822.6,                       // 有效委託價格 (number)
          afterLot: 2,                              // 有效委託股口數 (number)
          filledLot: 0,                             // 成交股口數 (number)
          filledMoney: 0,                           // 成交價金 (number)
          lastTime: "10:20:27",                     // 最後異動時間 (string)
      },
    ...
  ]
}

```


---

### 取得委託單結果 (含歷程)

getOrderResultsDetail

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數       | 類別                                                                                                                                          | 說明                                                                                                      |
| ---------- | --------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| account    | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                                           | 帳號                                                                                                      |
| marketType | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmarkettype) (Optioanl : 不帶全盤別) | 盤別種類 : `Future` 期貨日盤 、 `FutureNight` 期貨夜盤 、 `Option` 選擇權日盤 、 `OptionNight` 選擇權夜盤 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳委託資訊                     |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 FutOptOrderResult 欄位[​](#委託資訊-futoptorderresult-欄位 "Direct link to 委託資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                                                 | 說明                                                                                                                                            |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | number (Optional)                                                                                                                                    | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                                               | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                                               | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                                               | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                                               | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                                               | 委託書號                                                                                                                                        |
| assetType       | number                                                                                                                                               | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                                               | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | js<!-- -->:string<!-- --> / ts:[FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | number                                                                                                                                               | 單位數                                                                                                                                          |
| currency        | string                                                                                                                                               | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                                               | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                                               | 到期日                                                                                                                                          |
| strikePrice     | number                                                                                                                                               | 履約價                                                                                                                                          |
| callPut         | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                                               | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                                               | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | number                                                                                                                                               | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySellLeg2     | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | number                                                                                                                                               | 價格                                                                                                                                            |
| lot             | number                                                                                                                                               | 原始委託股口數                                                                                                                                  |
| timeInForce     | js<!-- -->:string<!-- --> / ts:[TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#timeinforce)           | 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | js<!-- -->:string<!-- --> / ts:[FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                                                 | 是否為預約單                                                                                                                                    |
| status          | number                                                                                                                                               | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | number                                                                                                                                               | 有效委託價格                                                                                                                                    |
| afterLot        | number                                                                                                                                               | 有效委託口數                                                                                                                                    |
| filledLot       | number                                                                                                                                               | 成交口數                                                                                                                                        |
| filledMoney     | number                                                                                                                                               | 成交價金                                                                                                                                        |
| beforeLot       | number                                                                                                                                               | 改單前有效口數                                                                                                                                  |
| beforePrice     | number                                                                                                                                               | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                                               | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                                               | 最後異動時間                                                                                                                                    |
| errorMessage    | string                                                                                                                                               | 錯誤訊息                                                                                                                                        |
| detail          | list                                                                                                                                                 | 委託歷程                                                                                                                                        |
| >> functionType | number                                                                                                                                               | 功能別 : `10` 新單、`15` 改價、 `20` 改量、`30`刪單、`50` 成交 、 `90` 失敗                                                                     |
| >> modifiedTime | string                                                                                                                                               | 修改時間                                                                                                                                        |
| >> beforeLot    | number                                                                                                                                               | 原始委託口數                                                                                                                                    |
| >> afterLot     | number                                                                                                                                               | 有效委託口數                                                                                                                                    |
| >> beforePrice  | number                                                                                                                                               | 原始委託價                                                                                                                                      |
| >> afterPrice   | number                                                                                                                                               | 有效委託價                                                                                                                                      |
| >> errorMessage | string                                                                                                                                               | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
var account = accounts.data[0];
const orders = sdk.futopt.getOrderResultsDetail(account);
console.log(orders);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data:[
        {
          date: "2024/03/25",                      // 交易日期 (string)
          seqNo: "00110212608",                    // 委託單流水序號 (string)
          branchNo: "15901",                        // 分公司代號 (string)
          account: "1234567",                       // 帳號 (string)
          orderNo: "C0001",                         // 委託書號 (string)
          assetType: 1,                             // 資產類別 (number)
          market: "TAIMEX",                         // 市場類型 (string)
          marketType: "Future",                     // 盤別種類 (FutOptMarketType)
          unit: 1,                                  // 單位數 (number)
          currency: "TWD",                          // 幣別 (string)
          symbol: "FITF",                           // 商品代號 (string)
          expiryDate: "202404",                     // 到期日 (string)
          buySell: "Buy",                           // 買賣別 (BSAction)
          priceType: "Limit",                       // 原始委託價格別 (FutOptPriceType)
          price: 1822.6,                            // 價格 (number)
          lot: 2,                                   // 原始委託股口數 (number)
          timeInForce: "ROD",                       // 委託條件別 (TimeInforce)
          orderType: "Auto",                        // 委託單類型 (FutOptOrderType)
          isPreOrder: false,                        // 是否為預約單 (bool)
          status: 10,                               // 委託單狀態 (number)
          afterPrice: 1822.6,                       // 有效委託價格 (number)
          afterLot: 2,                              // 有效委託股口數 (number)
          filledLot: 0,                             // 成交股口數 (number)
          filledMoney: 0,                           // 成交價金 (number)
          lastTime: "10:20:27",                     // 最後異動時間 (string)
          details:[
            functionType: 10,                       // 功能別 (number)
            modifiedTime: "10:20:27",              // 修改時間 (string)
            beforeLot: 0,                          // 原始委託口數 (number)
            afterLot: 2,                           // 有效委託口數 (number)
            beforePrice: 1822.6,                   // 原始委託價格 (number)
            afterPrice: 1822.6,                    // 有效委託價格 (number)
            status: 10                             // 委託單狀態 (number)
          ]
      },
    ...
  ]
}

```


---

### 修改委託價格

modifyPrice

##### 先使用makeModifyPriceObj 建立 FutOptModifyPrice 物件[​](#先使用makemodifypriceobj-建立-futoptmodifyprice-物件 "Direct link to 先使用makeModifyPriceObj 建立 FutOptModifyPrice 物件")

| 參數        | 類別                                                                                                                    | 說明             |
| ----------- | ----------------------------------------------------------------------------------------------------------------------- | ---------------- |
| orderResult | [FutOptOrderResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptorderresult) | 預計修改的委託單 |
| price       | string?                                                                                                                 | 新的委託價格     |
| priceType   | [FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)?    | 新的價格旗標     |

caution

當 price 欄位有填入值時，priceType 欄位為空值或為null ； 當 priceType 欄位有填入值時，price 欄位為空值或為null

將回傳的物件放入 modifyPrice 的方法中

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數              | 類別                                                                                                                    | 說明           |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------- | -------------- |
| account           | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                     | 帳號           |
| FutOptModifyPrice | [FutOptModifyPrice](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmodifyprice) | 修改價格物件   |
| unblock           | bool ? (default = false)                                                                                                | 是否採用非阻塞 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳修改資訊                     |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 修改資訊 FutOptOrderResult 欄位[​](#修改資訊-futoptorderresult-欄位 "Direct link to 修改資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                                                 | 說明                                                                                                                                            |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | number                                                                                                                                               | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                                               | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                                               | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                                               | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                                               | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                                               | 委託書號                                                                                                                                        |
| assetType       | number                                                                                                                                               | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                                               | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | js<!-- -->:string<!-- --> / ts:[FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | number                                                                                                                                               | 單位數                                                                                                                                          |
| currency        | string                                                                                                                                               | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                                               | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                                               | 到期日                                                                                                                                          |
| strikePrice     | number                                                                                                                                               | 履約價                                                                                                                                          |
| callPut         | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                                               | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                                               | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | number                                                                                                                                               | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySellLeg2     | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | number                                                                                                                                               | 價格                                                                                                                                            |
| lot             | number                                                                                                                                               | 原始委託股口數                                                                                                                                  |
| timeInForce     | js<!-- -->:string<!-- --> / ts:[TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#timeinforce)           | 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | js<!-- -->:string<!-- --> / ts:[FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                                                 | 是否為預約單                                                                                                                                    |
| status          | number                                                                                                                                               | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | number                                                                                                                                               | 有效委託價格                                                                                                                                    |
| afterLot        | number                                                                                                                                               | 有效委託口數                                                                                                                                    |
| filledLot       | number                                                                                                                                               | 成交口數                                                                                                                                        |
| filledMoney     | number                                                                                                                                               | 成交價金                                                                                                                                        |
| beforeLot       | number                                                                                                                                               | 改單前有效口數                                                                                                                                  |
| beforePrice     | number                                                                                                                                               | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                                               | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                                               | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                                                 | 委託歷程 (查詢orderResultDetail or orderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                                               | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
// 單筆阻塞改價
const modifyPriceObj = sdk.futopt.makeModifyPriceObj(target_order, "19900");
sdk.futopt.modifyPrice(accounts.data[0], modifyPriceObj, false)


```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data:{
    functionType: 15,                      // 功能別 (int)
    date: "2024/03/25",                    // 交易日期 (string)
    seqNo: "00110212663",                  // 委託單流水序號 (string)
    branchNo: "15901",                      // 分公司代號 (string)
    account: "1234567",                     // 帳號 (string)
    orderNo: "C0005",                       // 委託書號 (string)
    assetType: 1,                          // 資產類別 (int)
    market: "TAIMEX",                       // 市場類型 (string)
    marketType: "Future",                   // 盤別種類 (FutOptMarketType)
    currency: "TWD",                        // 幣別 (string)
    symbol: "FITX",                         // 商品代號 (string)
    expiryDate: "0",                        // 到期日 (string)
    strikePrice: 0,                         // 履約價 (float)
    buySell: "Buy",                         // 買賣別 (BSAction)
    priceType: "Limit",                     // 原始委託價格別 (FutOptPriceType)
    price: 20000,                           // 價格 (float)
    lot: 1,                                 // 原始委託股口數 (int)
    timeInForce: "ROD",                     // 委託條件別 (TimeInforce)
    orderType: "New",                       // 委託單類型 (FutOptOrderType)
    isPreOrder: false,                      // 是否為預約單 (bool)
    status: 10,                             // 委託單狀態 (int)
    afterPriceType: "Limit",                // 有效委託價格別 (FutOptPriceType)
    afterPrice: 19900,                      // 有效委託價格 (float)
    afterLot: 1,                            // 有效委託股口數 (int)
    filledLot: 0,                           // 成交股口數 (int)
    filledMoney: 0,                         // 成交價金 (int)
    beforeLot: 0,                           // 改單前有效股口數 (int)
    beforePrice: 20000,                     // 改單前有效價格 (float)
    lastTime: "13:39:05",                   // 最後異動時間 (string)
  }
}

```


---

### 修改委託單數量

modifyLot

##### 先使用 makeModifyLotObj 建立一個 FutoptModifyLot 物件[​](#先使用-makemodifylotobj-建立一個-futoptmodifylot-物件 "Direct link to 先使用 makeModifyLotObj 建立一個 FutoptModifyLot 物件")

| 參數        | 類別                                                                                                                    | 說明                                                |
| ----------- | ----------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| orderResult | [FutOptOrderResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptorderresult) | 預計修改的委託單                                    |
| quantity    | number                                                                                                                  | 改單後的委託量 ( 修改後數量包含此委託單已成交部份 ) |

將回傳的物件放入modifyLot 方法中

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數            | 類別                                                                                                                | 說明           |
| --------------- | ------------------------------------------------------------------------------------------------------------------- | -------------- |
| account         | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                 | 帳號           |
| FutoptModifyLot | [FutoptModifyLot](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmodifylot) | 修改的委託單   |
| unblock         | bool ? (default = false)                                                                                            | 是否採用非阻塞 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳修改資訊                     |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 修改資訊 FutOptOrderResult 欄位[​](#修改資訊-futoptorderresult-欄位 "Direct link to 修改資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                                                 | 說明                                                                                                                                            |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | number                                                                                                                                               | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                                               | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                                               | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                                               | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                                               | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                                               | 委託書號                                                                                                                                        |
| assetType       | number                                                                                                                                               | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                                               | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | js<!-- -->:string<!-- --> / ts:[FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | number                                                                                                                                               | 單位數                                                                                                                                          |
| currency        | string                                                                                                                                               | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                                               | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                                               | 到期日                                                                                                                                          |
| strikePrice     | number                                                                                                                                               | 履約價                                                                                                                                          |
| callPut         | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                                               | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                                               | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | number                                                                                                                                               | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySellLeg2     | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | number                                                                                                                                               | 價格                                                                                                                                            |
| lot             | number                                                                                                                                               | 原始委託股口數                                                                                                                                  |
| timeInForce     | js<!-- -->:string<!-- --> / ts:[TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#timeinforce)           | 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | js<!-- -->:string<!-- --> / ts:[FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                                                 | 是否為預約單                                                                                                                                    |
| status          | number                                                                                                                                               | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | number                                                                                                                                               | 有效委託價格                                                                                                                                    |
| afterLot        | number                                                                                                                                               | 有效委託口數                                                                                                                                    |
| filledLot       | number                                                                                                                                               | 成交口數                                                                                                                                        |
| filledMoney     | number                                                                                                                                               | 成交價金                                                                                                                                        |
| beforeLot       | number                                                                                                                                               | 改單前有效口數                                                                                                                                  |
| beforePrice     | number                                                                                                                                               | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                                               | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                                               | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                                                 | 委託歷程 (查詢orderResultDetail or orderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                                               | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
const modifyLotObj = sdk.futopt.makeModifyLotObj(target_order, 2);
sdk.futopt.modifyLot(accounts.data[0], modifyLotObj)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{
  isSuccess: true,
  data:
      {
          functionType: 20,                      // 功能別 (number)
          date: "2024/03/25",                    // 交易日期 (string)
          seqNo: "03100161319",                  // 委託單流水序號 (string)
          branchNo: "15901",                      // 分公司代號 (string)
          account: "1234567",                     // 帳號 (string)
          orderNo: "l001D",                       // 委託書號 (string)
          assetType: 1,                          // 資產類別 (number)
          market: "TAIMEX",                       // 市場類型 (string)
          marketType: "FutureNight",              // 盤別種類 (FutOptMarketType)
          currency: "TWD",                        // 幣別 (string)
          symbol: "FIMTX",                        // 商品代號 (string)
          buySell: "Buy",                         // 買賣別 (BSAction)
          priceType: "Limit",                     // 原始委託價格別 (FutOptPriceType)
          price: 20000,                           // 價格 (number)
          lot: 3,                                 // 原始委託股口數 (number)
          timeInForce: "ROD",                     // 委託條件別 (TimeInforce)
          orderType: "New",                       // 委託單類型 (FutOptOrderType)
          isPreOrder: false,                      // 是否為預約單 (bool)
          status: 10,                             // 委託單狀態 (number)
          afterPriceType: "Limit",                // 有效委託價格別 (FutOptPriceType)
          afterPrice: 20000,                      // 有效委託價格 (number)
          afterLot: 2,                            // 有效委託股口數 (number)
          filledLot: 0,                           // 成交股口數 (number)
          filledMoney: 0,                         // 成交價金 (number)
          beforeLot: 0,                           // 改單前有效股口數 (number)
          beforePrice: 20000,                     // 改單前有效價格 (number)
          lastTime: "18:24:40",                   // 最後異動時間 (string)
      }  
}

```


---

### 查詢歷史委託

orderHistory

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數         | 類別                                                                                                                                          | 說明                                                                                                      |
| ------------ | --------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| account      | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)                                           | 帳號                                                                                                      |
| startDate    | string                                                                                                                                        | 查詢開始日                                                                                                |
| endDate      | string                                                                                                                                        | 查詢終止日                                                                                                |
| market\_type | [FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptmarkettype) (Optional : 不帶全盤別) | 盤別種類 : `Future` 期貨日盤 、 `FutureNight` 期貨夜盤 、 `Option` 選擇權日盤 、 `OptionNight` 選擇權夜盤 |

info

可查詢最近兩日之歷史資料

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳委託資訊                     |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 FutOptOrderResult 欄位[​](#委託資訊-futoptorderresult-欄位 "Direct link to 委託資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                                                 | 說明                                                                                                                                            |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | number (Optional)                                                                                                                                    | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                                               | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                                               | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                                               | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                                               | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                                               | 委託書號                                                                                                                                        |
| assetType       | number                                                                                                                                               | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                                               | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | js<!-- -->:string<!-- --> / ts:[FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | number                                                                                                                                               | 單位數                                                                                                                                          |
| currency        | string                                                                                                                                               | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                                               | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                                               | 到期日                                                                                                                                          |
| strikePrice     | number                                                                                                                                               | 履約價                                                                                                                                          |
| callPut         | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                                               | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                                               | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | number                                                                                                                                               | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySellLeg2     | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | number                                                                                                                                               | 價格                                                                                                                                            |
| lot             | number                                                                                                                                               | 原始委託股口數                                                                                                                                  |
| timeInForce     | js<!-- -->:string<!-- --> / ts:[TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#timeinforce)           | 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | js<!-- -->:string<!-- --> / ts:[FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                                                 | 是否為預約單                                                                                                                                    |
| status          | number                                                                                                                                               | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | number                                                                                                                                               | 有效委託價格                                                                                                                                    |
| afterLot        | number                                                                                                                                               | 有效委託口數                                                                                                                                    |
| filledLot       | number                                                                                                                                               | 成交口數                                                                                                                                        |
| filledMoney     | number                                                                                                                                               | 成交價金                                                                                                                                        |
| beforeLot       | number                                                                                                                                               | 改單前有效口數                                                                                                                                  |
| beforePrice     | number                                                                                                                                               | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                                               | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                                               | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                                                 | 委託歷程                                                                                                                                        |
| >> functionType | number                                                                                                                                               | 功能別 : `10` 新單、`15` 改價、 `20` 改量、`30`刪單、`50` 成交、`90`失敗                                                                        |
| >> modifiedTime | string                                                                                                                                               | 修改時間                                                                                                                                        |
| >> beforeLot    | number                                                                                                                                               | 原始委託口數                                                                                                                                    |
| >> afterLot     | number                                                                                                                                               | 有效委託口數                                                                                                                                    |
| >> beforePrice  | number                                                                                                                                               | 原始委託價                                                                                                                                      |
| >> afterPrice   | number                                                                                                                                               | 有效委託價                                                                                                                                      |
| >> errorMessage | string                                                                                                                                               | 錯誤訊息                                                                                                                                        |
| errorMessage    | string                                                                                                                                               | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js
const orderHistory = sdk.stock.orderHistory(account,"20230921","20231020");
console.log(orderHistory);

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範�例")

```js
{
  isSuccess: true,
  data:[
   {
      date: "2024/04/11",                      // 交易日期 (string)
      seqNo: "00230177314",                    // 委託單流水序號 (string)
      branchNo: "15000",                        // 分公司代號 (string)
      account: "9974825",                       // 帳號 (string)
      orderNo: "C0020",                         // 委託書號 (string)
      assetType: 2,                             // 資產類別 :  `1` 期貨 、`2` 選擇權 (number)
      market: "TAIMEX",                         // 市場類型 :  `TAIMEX` 期貨、選擇權 (string)
      marketType: "Option",                     // 盤別種類  :  `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤 (FutOptMarketType)
      symbol: "TXO",                            // 商品代號 (string)
      expiryDate: "202404",                     // 到期日 (string)
      strikePrice: 18600,                       // 履約價 (number)
      callPut: "Call",                          // 買賣權 :  `Call` 買權、 `Put` 賣權 (string)
      buySell: "Buy",                           // 買賣別 :  `Buy` 買 、 `Sell` 賣 (BSAction)
      priceType: "Limit",                       // 原始委託價格別  : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價 (PriceType)
      price: 2100,                              // 價格 (number)
      lot: 1,                                   // 原始委託股口數 (number)
      timeInForce: "ROD",                       // 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC (TimeInforce)
      orderType: "New",                         // 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖 (FutOptOrderType)
      isPreOrder: false,                        // 是否為預約單 (bool)
      status: 50,                               // 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、`9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功  、 `50` 完全成交 、 `90` 失敗 (number)
      afterPrice: 2100,                         // 有效委託價格 (number)
      afterLot: 1,                              // 有效委託口數 (number)
      filledLot: 1,                             // 成交口數 (number)
      filledMoney: 2100,                        // 成交價金 (number)
      lastTime: "10:41:46.760",                // 最後異動時間 (string)
      details:[
            functionType: 10,                       // 功能別 (number)
            modifiedTime: "10:20:27",              // 修改時間 (string)
            beforeLot: 0,                          // 原始委託口數 (number)
            afterLot: 1,                           // 有效委託口數 (number)
            beforePrice: 2100,                   // 原始委託價格 (number)
            afterPrice: 2100,                    // 有效委託價格 (number)
          ]
          ....
    },
      
  {
      date: "2024/04/11",                      // 交易日期 (string)
      seqNo: "00230177315",                    // 委託單流水序號 (string)
      branchNo: "15000",                        // 分公司代號 (string)
      account: "9974825",                       // 帳號 (string)
      orderNo: "C0021",                         // 委託書號 (string)
      assetType: 2,                             // 資產類別 :  `1` 期貨 、`2` 選擇權 (number)
      market: "TAIMEX",                         // 市場類型 :  `TAIMEX` 期貨、選擇權 (string)
      marketType: "Option",                     // 盤別種類  :  `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤 (FutOptMarketType)
      symbol: "TXO",                            // 商品代號 (string)
      expiryDate: "202404",                     // 到期日 (string)
      strikePrice: 18500,                       // 履約價 (number)
      callPut: "Call",                          // 買賣權 :  `Call` 買權、 `Put` 賣權 (string)
      buySell: "Sell",                          // 買賣別 :  `Buy` 買 、 `Sell` 賣 (BSAction)
      priceType: "Limit",                       // 原始委託價格別  : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價 (PriceType)
      price: 2230,                              // 價格 (number)
      lot: 1,                                   // 原始委託股口數 (number)
      timeInForce: "ROD",                       // 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC (TimeInforce)
      orderType: "New",                         // 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖 (FutOptOrderType)
      isPreOrder: false,                        // 是否為預約單 (bool)
      status: 50,                               // 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、`9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功  、 `50` 完全成交 、 `90` 失敗 (number)
      afterPrice: 2230,                         // 有效委託價格 (number)
      afterLot: 1,                              // 有效委託口數 (number)
      filledLot: 1,                             // 成交口數 (number)
      filledMoney: 2230,                        // 成交價金 (number)
      lastTime: "10:41:46.760",                // 最後異動時間 (string)
  }
  ]
}

```


---

### 建立委託單

placeOrder

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                        | 說明           |
| ----------- | ----------------------------------------------------------------------------------------------------------- | -------------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#account)         | 帳號           |
| OrderObject | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#orderobject) | 委託內容       |
| unblock     | bool ? (default = false)                                                                                    | 是否採用非阻塞 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數      | 類別                | 說明                             |
| --------- | ------------------- | -------------------------------- |
| isSuccess | bool                | 是否成功                         |
| data      | Object              | 回傳委託資訊                     |
| message   | string ? (optional) | 當isSuccess = false 回傳錯誤訊息 |

##### 委託資訊 FutOptOrderResult 欄位[​](#委託資訊-futoptorderresult-欄位 "Direct link to 委託資訊 FutOptOrderResult 欄位")

Return type : Object

| 參數            | 類別                                                                                                                                                 | 說明                                                                                                                                            |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| functionType    | number                                                                                                                                               | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                                  |
| date            | string                                                                                                                                               | 交易日期                                                                                                                                        |
| seqNo           | string                                                                                                                                               | 委託單流水序號                                                                                                                                  |
| branchNo        | string                                                                                                                                               | 分公司代號                                                                                                                                      |
| account         | string                                                                                                                                               | 帳號                                                                                                                                            |
| orderNo         | string                                                                                                                                               | 委託書號                                                                                                                                        |
| assetType       | number                                                                                                                                               | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                                                |
| market          | string                                                                                                                                               | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                                                |
| marketType      | js<!-- -->:string<!-- --> / ts:[FutOptMarketType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptmarkettype) | 盤別種類 : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤                                       |
| unit            | number                                                                                                                                               | 單位數                                                                                                                                          |
| currency        | string                                                                                                                                               | 幣別                                                                                                                                            |
| symbol          | string                                                                                                                                               | 商品代號                                                                                                                                        |
| expiryDate      | string                                                                                                                                               | 到期日                                                                                                                                          |
| strikePrice     | number                                                                                                                                               | 履約價                                                                                                                                          |
| callPut         | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySell         | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 : `Buy` 買 、 `Sell` 賣                                                                                                                  |
| symbolLeg2      | string                                                                                                                                               | 商品代號 - 複式第二隻腳                                                                                                                         |
| expiryDateLeg2  | string                                                                                                                                               | 到期日 - 複式第二隻腳                                                                                                                           |
| strikePriceLeg2 | number                                                                                                                                               | 履約價 - 複式第二隻腳                                                                                                                           |
| callPutLeg2     | js<!-- -->:string<!-- --> / ts:[CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#callput)                   | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                                               |
| buySellLeg2     | js<!-- -->:string<!-- --> / ts:[BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#bsaction)                 | 買賣別 - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                                                    |
| priceType       | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 原始委託價格別 : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價                                                    |
| price           | number                                                                                                                                               | 價格                                                                                                                                            |
| lot             | number                                                                                                                                               | 原始委託股口數                                                                                                                                  |
| timeInForce     | js<!-- -->:string<!-- --> / ts:[TimeInforce](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#timeinforce)           | 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                                                                |
| orderType       | js<!-- -->:string<!-- --> / ts:[FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptordertype)   | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                                                      |
| isPreOrder      | bool                                                                                                                                                 | 是否為預約單                                                                                                                                    |
| status          | number                                                                                                                                               | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、 `8` 後台傳送中 、 `9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗 |
| afterPriceType  | js<!-- -->:string<!-- --> / ts:[FutOptPriceType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/nodejs/EnumMatrix.txt#futoptpricetype)   | 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價                                                    |
| afterPrice      | number                                                                                                                                               | 有效委託價格                                                                                                                                    |
| afterLot        | number                                                                                                                                               | 有效委託口數                                                                                                                                    |
| filledLot       | number                                                                                                                                               | 成交口數                                                                                                                                        |
| filledMoney     | number                                                                                                                                               | 成交價金                                                                                                                                        |
| beforeLot       | number                                                                                                                                               | 改單前有效口數                                                                                                                                  |
| beforePrice     | number                                                                                                                                               | 改單前有效價                                                                                                                                    |
| userDef         | string                                                                                                                                               | 自訂欄位                                                                                                                                        |
| lastTime        | string                                                                                                                                               | 最後異動時間                                                                                                                                    |
| detail          | list                                                                                                                                                 | 委託歷程 (查詢orderResultDetail or orderHistory才有值)                                                                                          |
| errorMessage    | string                                                                                                                                               | 錯誤訊息                                                                                                                                        |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```js

// 單式
futoptorder = {
    buySell: BSAction.Buy,
    symbol: "TXO20000E4",
    price: "530",
    lot: 1,
    marketType: FutOptMarketType.Option,
    priceType: FutOptPriceType.Limit,
    timeInForce: TimeInForce.ROD,
    orderType: FutOptOrderType.Auto,
    userDef: "nodejs" // optional field
}

sdk.futopt.placeOrder(account, futoptorder)

// 複式
futoptorder = {
    buySell: BSAction.Sell,
    symbol: "TXO20000E4",
    buySell2: BSAction.Buy,   // optional field
    symbol2: "TXO19900E4",   // optional field
    price: "90",
    lot: 1,
    marketType: FutOptMarketType.Option,
    priceType: FutOptPriceType.Limit,
    timeInForce: TimeInForce.IOC,
    orderType: FutOptOrderType.Auto,
    userDef: "nodejs"   // optional field
}


sdk.futopt.placeOrder(account, futoptorder)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```js
{   //單式回覆
    isSuccess : true,
    data = {
                functionType: 0,                    // 功能別 (number)
                date: "2024/03/25",                 // 交易日期 (string)
                seqNo: "00230177010",               // 委託單流水序號 (string)
                branchNo: "15901",                  // 分公司代號 (string)
                account: "1234567",                 // 帳號 (string)
                orderNo: "C0002",                   // 委託書號 (string)
                assetType: 2,                       // 資產類別 (number)
                market: "TAIMEX",                   // 市場類型 (string)
                marketType: Option,                 // 盤別種類 (FutOptMarketType)
                unit: 1,                            // 單位數 (number)
                currency: "TWD",                    // 幣別 (string)
                symbol: "TXO",                      // 商品代號 (string)
                expiryDate: "202404",               // 到期日 (string)
                strikePrice: 20000,                // 履約價 (number)
                callPut: "Call",                    // 買賣權 (string)
                buySell: Buy,                       // 買賣別 (BSAction)
                priceType: Limit,                   // 原始委託價格別 (FutOptPriceType)
                price: 500,                         // 價格 (number)
                lot: 1,                             // 原始委託股口數 (number)
                timeInForce: ROD,                   // 委託條件別 (TimeInforce)
                orderType: Auto,                    // 委託單類型 (FutOptOrderType)
                isPreOrder: false,                  // 是否為預約單 (bool)
                status: 10,                         // 委託單狀態 (number)
                afterPriceType: Limit,              // 有效委託價格別 (FutOptPriceType)
                afterPrice: 500,                    // 有效委託價格 (number)
                afterLot: 1,                        // 有效委託股口數 (number)
                filledLot: 0,                       // 成交股口數 (number)
                filledMoney: 0,                     // 成交價金 (number)
                beforeLot: 0,                       // 改單前有效股口數 (number)
                beforePrice: 500,                   // 改單前有效價格 (number)
                userDef: "nodejs",                  // 自訂欄位 (string)
                lastTime: "11:50:08",               // 最後異動時間 (string)
}}



{   //複式回覆
    isSuccess : true,
    data  :  {
                functionType: 0,                            // 功能別 (number)
                date: "2024/03/25",                         // 交易日期 (string)
                seqNo: "00230177020",                       // 委託單流水序號 (string)
                branchNo: "15901",                          // 分公司代號 (string)
                account: "1234567",                         // 帳號 (string)
                orderNo: "C0004",                           // 委託書號 (string)
                assetType: 2,                               // 資產類別 (number)
                market: "TAIMEX",                           // 市場類型 (string)
                marketType: Option,                         // 盤別種類 (FutOptMarketType)
                unit: 1,                                    // 單位數 (number)
                currency: "TWD",                            // 幣別 (string)
                symbol: "TXO",                              // 商品代號 (string)
                expiryDate: "202405",                       // 到期日 (string)
                strikePrice: 20000,                        // 履約價 (number)
                callPut: Call,                              // 買賣權 (string)
                buySell: Sell,                              // 買賣別 (BSAction)
                symbolLeg2: "TXO",                          // 商品代號 - 複式第二隻腳 (string)
                expiryDateLeg2: "202405",                   // 到期日 - 複式第二隻腳 (string)
                strikePriceLeg2: 199000,                    // 履約價 - 複式第二隻腳 (number)
                callPutLeg2: Call,                          // 買賣權 - 複式第二隻腳 (string)
                buySellLeg2: Buy,                           // 買賣別 - 複式第二隻腳 (BSAction)
                priceType: Limit,                           // 原始委託價格別 (FutOptPriceType)
                price: 90,                                  // 價格 (number)
                lot: 1,                                     // 原始委託股口數 (number)
                timeInForce: IOC,                           // 委託條件別 (TimeInforce)
                orderType: Auto,                             // 委託單類型 (FutOptOrderType)
                isPreOrder: false,                          // 是否為預約單 (bool)
                status: 10,                                 // 委託單狀態 (number)
                afterPriceType: Limit,                      // 有效委託價格別 (FutOptPriceType)
                afterPrice: 90,                             // 有效委託價格 (number)
                afterLot: 1,                                // 有效委託股口數 (number)
                filledLot: 0,                               // 成交股口數 (number)
                filledMoney: 0,                             // 成交價金 (number)
                beforeLot: 0,                               // 改單前有效股口數 (number)
                beforePrice: 90,                            // 改單前有效價格 (number)
                userDef: "nodejs",                          // 自訂欄位 (string)
                lastTime: "11:57:41",                       // 最後異動時間 (string)
  }
}

```


---

### 平倉查詢

close\_position\_record

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數        | 類別                                                                                                | 說明     |
| ----------- | --------------------------------------------------------------------------------------------------- | -------- |
| account     | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account) | 帳號     |
| start\_date | string                                                                                              | 查詢起日 |
| end\_date   | string                                                                                              | 查詢迄日 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別   | 說明                               |
| ----------- | ------ | ---------------------------------- |
| is\_success | bool   | 是否成功                           |
| data        | List   | 回傳平倉資訊                       |
| message     | string | 當is\_success = false 回傳錯誤訊息 |

##### 已實現彙總 CloseRecord 欄位[​](#已實現彙總-closerecord-欄位 "Direct link to 已實現彙總 CloseRecord 欄位")

Return type : Object

| 參數             | 類別                                                                                                                | 說明                                                                       |
| ---------------- | ------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| date             | string                                                                                                              | 資料日期                                                                   |
| branch\_no       | string                                                                                                              | 分公司代號                                                                 |
| position\_kind   | int                                                                                                                 | 部位種類 : `1` 期貨 、`2` 選擇權                                           |
| account          | string                                                                                                              | 帳號                                                                       |
| order\_no        | string                                                                                                              | 委託書號                                                                   |
| market           | string                                                                                                              | 市場別 : `TAIMEX` 期貨、選擇權                                             |
| symbol           | string                                                                                                              | 商品代號                                                                   |
| expiry\_date     | string                                                                                                              | 履約日                                                                     |
| strike\_price    | float                                                                                                               | 履約價                                                                     |
| call\_put        | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput)                 | 權利別 : `Call` Call 、`Put` Put                                           |
| buy\_sell        | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#bsaction)               | 買賣別 : `Buy` 買、`Sell` 賣                                               |
| order\_type      | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptordertype) | 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖 |
| price            | float                                                                                                               | 成交價                                                                     |
| orig\_lots       | int                                                                                                                 | 原始口數                                                                   |
| transaction\_fee | float                                                                                                               | 交易手續費                                                                 |
| tax              | float                                                                                                               | 交易稅                                                                     |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
sdk.futopt_accounting.close_position_record(accounts,"20240310","20240410")

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
    is_success: True,
    message: None,
    data : [
                CloseRecord {
                    date: "2024/04/10",             # 資料日期(string)
                    branch_no: "15000",             # 分公司代號(string)
                    account: "9974825",             # 帳號 (string)
                    position_kind: 1,               # 部位種類: `1` 期貨, `2` 選擇權 (int)
                    order_no: "15001-0000",         # 委託書號 (string)
                    market: "TAIMEX",               # 市場別 : `TAIMEX` (string)
                    symbol: "FITX",                 # 商品代號 (string)
                    expiry_date: "202404",          # 履約日 (string)
                    strike_price: None,             # 履約價 (float)
                    call_put: None,                 # Call/Put Type : `Call` Call, `Put` Put (CallPut)
                    buy_sell: Buy,                  # Buy/Sell Type : `Buy`, `Sell` (BSAction)
                    price: 20847.0,                 # 成交價 (float)
                    orig_lots: 1,                   # 原始口數 (int)
                    transaction_fee: 40.0,          # 交易手續費 (float)
                    tax: 83.0,                      # 交易稅 (float)
                },
                CloseRecord {
                    date: "2024/04/10",            # 資料日期 (string)
                    branch_no: "15000",             # 分公司代號 (string)
                    account: "9974825",             # 帳號 (string)
                    position_kind: 1,               # 部位種類d : `1` 期貨, `2` 選擇權 (int)
                    order_no: "C0005-0000",         # 委託書號 (string)
                    market: "TAIMEX",               # 市場別 : `TAIMEX` 期貨, 選擇權 (string)
                    symbol: "FITX",                 # 商品代號 (string)
                    expiry_date: "202405",          # 履約日 (string)
                    strike_price: None,             # 履約價 (float)
                    call_put: None,                 # Call/Put Type : `Call` Call, `Put` Put (CallPut)
                    buy_sell: Buy,                  # Buy/Sell Type : `Buy`, `Sell` (BSAction)
                    price: 20890.0,                 # 成交價 (float)
                    orig_lots: 1,                   # 原始口數 (int)
                    transaction_fee: 40.0,          # 交易手續費 (float)
                    tax: 84.0,                      # 交易稅 (float)
                }
            ]
}

```


---

### 混合部位查詢

query\_hybrid\_position

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                                | 說明 |
| ------- | --------------------------------------------------------------------------------------------------- | ---- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account) | 帳號 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別   | 說明                               |
| ----------- | ------ | ---------------------------------- |
| is\_success | bool   | 是否成功                           |
| data        | List   | 回傳混和部位資訊                   |
| message     | string | 當is\_success : false 回傳錯誤訊息 |

##### 部位 HybridPosition 欄位[​](#部位-hybridposition-欄位 "Direct link to 部位 HybridPosition 欄位")

Return type : Object

| 參數                   | 類別                                                                                                                | 說明                                              |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| > date                 | string                                                                                                              | 部位建立日期                                      |
| > branch\_no           | string                                                                                                              | 分公司代號                                        |
| > account              | string                                                                                                              | 帳號                                              |
| > is\_spread           | bool                                                                                                                | 是否為複式部位                                    |
| > position\_kind       | int                                                                                                                 | 部位種類 : `1` 期貨 、`2` 選擇權                  |
| > symbol               | string                                                                                                              | 商品代號                                          |
| > expiry\_date         | string                                                                                                              | 履約日                                            |
| > strike\_price        | float                                                                                                               | 履約價                                            |
| > call\_put            | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput)                 | 權利別 : `Call` 、`Put`                           |
| > buy\_sell            | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#bsaction)               | 買賣別 : `Buy` 買、`Sell` 賣                      |
| > price                | float                                                                                                               | 成交價                                            |
| > orig\_lots           | int                                                                                                                 | 原始口數                                          |
| > tradable\_lots       | int                                                                                                                 | 可交易口數                                        |
| > order\_type          | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptordertype) | 委託別 : `New` 新倉、`Close`平倉、`FdayTrade`當沖 |
| > currency             | string                                                                                                              | 幣別                                              |
| > market\_price        | string                                                                                                              | 即時價                                            |
| > initial\_margin      | float                                                                                                               | 原始保證金                                        |
| > maintenance\_margin  | float                                                                                                               | 維持保證金                                        |
| > clearing\_margin     | float                                                                                                               | 結算保證金                                        |
| > opt\_value           | float                                                                                                               | 選擇權市值                                        |
| > opt\_long\_value     | float                                                                                                               | 選擇權買進市值                                    |
| > opt\_short\_value    | float                                                                                                               | 選擇權賣出市值                                    |
| > profit\_or\_loss     | float                                                                                                               | 部位損益                                          |
| > premium              | float                                                                                                               | 權利金                                            |
| >> spread              | object                                                                                                              | 複式部位解析                                      |
| >> date                | string                                                                                                              | 部位建立日期                                      |
| >> branch\_no          | string                                                                                                              | 分公司代號                                        |
| >> account             | string                                                                                                              | 帳號                                              |
| >> is\_spread          | bool                                                                                                                | 是否為複式部位                                    |
| >> position\_kind      | int                                                                                                                 | 部位種類 : `1` 期貨 、`2` 選擇權                  |
| >> symbol              | string                                                                                                              | 商品代號                                          |
| >> expiry\_date        | string                                                                                                              | 履約日                                            |
| >> strike\_price       | float                                                                                                               | 履約價                                            |
| >> call\_put           | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput)                 | 權利別 : `Call` 、`Put`                           |
| >> buy\_sell           | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#bsaction)               | 買賣別 : `Buy` 買、`Sell` 賣                      |
| >> price               | float                                                                                                               | 成交價                                            |
| >> orig\_lots          | int                                                                                                                 | 原始口數                                          |
| >> tradable\_lots      | int                                                                                                                 | 可交易口數                                        |
| >> order\_type         | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptordertype) | 委託別 : `New` 新倉、`Close`平倉、`FdayTrade`當沖 |
| >> currency            | string                                                                                                              | 幣別                                              |
| >> market\_price       | string                                                                                                              | 即時價                                            |
| >> initial\_margin     | float                                                                                                               | 原始保證金                                        |
| >> maintenance\_margin | float                                                                                                               | 維持保證金                                        |
| >> clearing\_margin    | float                                                                                                               | 結算保證金                                        |
| >> opt\_value          | float                                                                                                               | 選擇權市值                                        |
| >> opt\_long\_value    | float                                                                                                               | 選擇權買進市值                                    |
| >> opt\_short\_value   | float                                                                                                               | 選擇權賣出市值                                    |
| >> profit\_or\_loss    | float                                                                                                               | 部位損益                                          |
| >> premium             | float                                                                                                               | 權利金                                            |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
sdk.futopt_accounting.query_hybrid_position(account)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
    is_success: True,
    message: None,
    data : [HybridPosition {
                date : "2024/04/08",                     # 部位建立日期 (str)
                branch_no : "15901",                      # 分公司代號 (str)
                account : "1234567",                      # 帳號 (str)
                is_spread : False,                        # 是否為複式部位 (bool)
                position_kind : 1,                        # 部位種類  (int)
                symbol : "FITX",                          # 商品代號 (str)
                expiry_date : 202404,                     # 履約日 (string)
                strike_price : None,                      # 履約價 (int or None)
                call_put : None,                          # 權利別 (int or None)
                buy_sell : Buy,                             # 買賣別  (BSAction)
                price : 20325.3333,                       # 成交價 (float)
                orig_lots : 3,                            # 原始口數 (int)
                tradable_lot : 3,                         # 可交易口數 (int)
                order_type : New,                           # 委託別 (FutOptOrderType)
                currency : "TWD",                         # 幣別 (str)
                market_price : "20351",                   # 即時價 (str)
                initial_margin : 0.0,                     # 原始保證金 (float)
                maintenance_margin : 0.0,                 # 維持保證金 (float)
                clearing_margin : 0.0,                    # 結算保證金 (float)
                initial_margin_all_single : 0.0,          # 原始保證金 (float)
                opt_value : 0.0,                          # 選擇權市值 (float)
                opt_long_value : 0.0,                     # 選擇權買進市值 (float)
                opt_short_value : 0.0,                    # 選擇權賣出市值 (float)
                profit_or_loss : 0.0,                     # 部位損益 (float)
                premium : 0.0,                            # 權利金 (float)
                spreads : None,                           # 複式部位 (List[SpreadPosition])
            },
            HybridPosition {
                date : "2024/04/08",                     # 部位建立日期 (str)
                branch_no : "15901",                      # 分公司代號 (str)
                account : "1234567",                      # 帳號 (str)
                is_spread : False,                        # 是否為複式部位 (bool)
                position_kind : 2,                        # 部位種類  (int))
                symbol : "TX1",                           # 商品代號 (str)
                expiry_date : 202404,                     # 履約日 (string)
                strike_price : 206000,                    # 履約價 (float)
                call_put : Call,                             # 權利別  (CallPut)
                buy_sell : Buy,                             # 買賣別  (BSAction)
                price : 10,                               # 成交價 (int)
                orig_lots : 4,                            # 原始口數 (int)
                tradable_lot : 4,                         # 可交易口數 (int)
                order_type : New,                           # 委託別 (FutOptOrderType)
                currency : "TWD",                         # 幣別 (str)
                market_price : "4.6",                     # 即時價 (str)
                initial_margin : 0.0,                     # 原始保證金 (float)
                maintenance_margin : 0.0,                 # 維持保證金 (float)
                clearing_margin : 0.0,                    # 結算保證金 (float)
                initial_margin_all_single : 0.0,          # 原始保證金 (float)
                opt_value : 920.0,                        # 選擇權市值 (float)
                opt_long_value : 920.0,                   # 選擇權買進市值 (float)
                opt_short_value : 0.0,                    # 選擇權賣出市值 (float)
                profit_or_loss : -1080.0,                 # 部位損益 (float)
                premium : 0.0,                            # 權利金 (float)
                spreads : None,                           # 複式部位 (List[SpreadPosition])
            },
            HybridPosition {
                date : "2024/04/08",                     # 部位建立日期 (str)
                branch_no : "15901",                      # 分公司代號 (str)
                account : "1234567",                      # 帳號 (str)
                is_spread : False,                        # 是否為複式部位 (bool)
                position_kind : 2,                        # 部位種類  (int)
                symbol : "TXO",                           # 商品代號 (str)
                expiry_date : 202404,                     # 履約日 (string)
                strike_price : 198000,                    # 履約價 (float)
                call_put : Call,                             # 權利別  (CallPut)
                buy_sell : Sell,                             # 買賣別  (BSAction)
                price : 243,                              # 成交價 (int)
                orig_lots : 2,                            # 原始口數 (int)
                tradable_lot : 2,                         # 可交易口數 (int)
                order_type : New,                           # 委託別  (int)
                currency : "TWD",                         # 幣別 (str)
                market_price : "46",                      # 即時價 (str)
                initial_margin : 0.0,                     # 原始保證金 (float)
                maintenance_margin : 0.0,                 # 維持保證金 (float)
                clearing_margin : 0.0,                    # 結算保證金 (float)
                initial_margin_all_single : 0.0,          # 原始保證金 (float)
                opt_value : 4600.0,                       # 選擇權市值 (float)
                opt_long_value : 4600.0,                  # 選擇權買進市值 (float)
                opt_short_value : 0.0,                    # 選擇權賣出市值 (float)
                profit_or_loss : -19700.0,                # 部位損益 (float)
                premium : 0.0,                            # 權利金 (float)
                spreads : None,                           # 複式部位 (List[SpreadPosition])
            },
            HybridPosition {
                date : "2024/04/08",                     # 部位建立日期 (str)
                branch_no : "15901",                      # 分公司代號 (str)
                account : "1234567",                      # 帳號 (str)
                is_spread : False,                        # 是否為複式部位 (bool)
                position_kind : 2,                        # 部位種類  (int)
                symbol : "TXO",                           # 商品代號 (str)
                expiry_date : 202404,                     # 履約日 (string)
                strike_price : 200000,                    # 履約價 (float)
                call_put : Call,                             # 權利別  (CallPut)
                buy_sell : Buy,                             # 買賣別  (BSAction)
                price : 500,                              # 成交價 (int)
                orig_lots : 2,                            # 原始口數 (int)
                tradable_lot : 2,                         # 可交易口數 (int)
                order_type : New,                         # 委託別 (FutOptOrderType)
                currency : "TWD",                         # 幣別 (str)
                market_price : "430",                     # 即時價 (str)
                initial_margin : 0.0,                     # 原始保證金 (float)
                maintenance_margin : 0.0,                 # 維持保證金 (float)
                clearing_margin : 0.0,                    # 結算保證金 (float)
                initial_margin_all_single : 0.0,          # 原始保證金 (float)
                opt_value : 43000.0,                      # 選擇權市值 (float)
                opt_long_value : 43000.0,                 # 選擇權買進市值 (float)
                opt_short_value : 0.0,                    # 選擇權賣出市值 (float)
                profit_or_loss : -7000.0,                 # 部位損益 (float)
                premium : 0.0,                            # 權利金 (float)
                spreads : None,                           # 複式部位 (List[SpreadPosition])
            },
            HybridPosition {
                date : "2024/04/08",                     # 部位建立日期 (str)
                branch_no : "15901",                      # 分公司代號 (str)
                account : "1234567",                      # 帳號 (str)
                is_spread : False,                        # 是否為複式部位 (bool)
                position_kind : 2,                        # 部位種類  (int)
                symbol : "TXO",                           # 商品代號 (str)
                expiry_date : 202404,                     # 履約日 (string)
                strike_price : 200000,                    # 履約價 (float)
                call_put : Put,                             # 權利別  (CallPut)
                buy_sell : Buy,                             # 買賣別  (BSAction)
                price : 344,                              # 成交價 (int)
                orig_lots : 2,                            # 原始口數 (int)
                tradable_lot : 2,                         # 可交易口數 (int)
                order_type : New,                           # 委託別 (FutOptOrderType)
                currency : "TWD",                         # 幣別 (str)
                market_price : "82",                      # 即時價 (str)
                initial_margin : 0.0,                     # 原始保證金 (float)
                maintenance_margin : 0.0,                 # 維持保證金 (float)
                clearing_margin : 0.0,                    # 結算保證金 (float)
                initial_margin_all_single : 0.0,          # 原始保證金 (float)
                opt_value : 8200.0,                       # 選擇權市值 (float)
                opt_long_value : 8200.0,                  # 選擇權買進市值 (float)
                opt_short_value : 0.0,                    # 選擇權賣出市值 (float)
                profit_or_loss : -26200.0,                # 部位損益 (float)
                premium : 0.0,                            # 權利金 (float)
                spreads : None,                           # 複式部位 (List[SpreadPosition])
            },
            HybridPosition {
                date : "2024/04/08",                     # 部位建立日期 (str)
                branch_no : "15901",                      # 分公司代號 (str)
                account : "1234567",                      # 帳號 (str)
                is_spread : False,                        # 是否為複式部位 (bool)
                position_kind : 2,                        # 部位種類  (int)
                symbol : "TXO",                           # 商品代號 (str)
                expiry_date : 202405,                     # 履約日 (string)
                strike_price : 199000,                    # 履約價 (float)
                call_put : Call,                             # 權利別  (CallPut)
                buy_sell : Buy,                             # 買賣別  (BSAction)
                price : 610,                              # 成交價 (int)
                orig_lots : 1,                            # 原始口數 (int)
                tradable_lot : 1,                         # 可交易口數 (int)
                order_type : New,                           # 委託別 (FutOptOrderType)
                currency : "TWD",                         # 幣別 (str)
                market_price : "670",                     # 即時價 (str)
                initial_margin : 0.0,                     # 原始保證金 (float)
                maintenance_margin : 0.0,                 # 維持保證金 (float)
                clearing_margin : 0.0,                    # 結算保證金 (float)
                initial_margin_all_single : 0.0,          # 原始保證金 (float)
                opt_value : 33500.0,                      # 選擇權市值 (float)
                opt_long_value : 33500.0,                 # 選擇權買進市值 (float)
                opt_short_value : 0.0,                    # 選擇權賣出市值 (float)
                profit_or_loss : 3000.0,                  # 部位損益 (float)
                premium : 0.0,                            # 權利金 (float)
                spreads : None,                           # 複式部位 (List[SpreadPosition])
            },
            HybridPosition {
                date : "2024/04/08",                      # 部位建立日期 (str)
                branch_no : "15901",                      # 分公司代號 (str)
                account : "1234567",                      # 帳號 (str)
                is_spread : True,                         # 是否為複式部位 (bool)
                position_kind : 2,                        # 部位種類  (int)
                symbol : "TXO20100D4:20000P4",            # 商品代號 (str)
                expiry_date : 1,                          # 履約日 (string)
                strike_price : 1,                         # 履約價 (float)
                call_put : None,                          # 權利別 (int or None)
                buy_sell : Buy,                           # 買賣別  (BSAction)
                price : None,                             # 成交價 (int or None)
                orig_lots : 2,                            # 原始口數 (int)
                tradable_lot : 2,                         # 可交易口數 (int)
                order_type : New,                         # 委託別 (FutOptOrderType)
                currency : "TWD",                         # 幣別 (str)
                market_price : "0.0",                     # 即時價 (str)
                initial_margin : 0.0,                     # 原始保證金 (float)
                maintenance_margin : 0.0,                 # 維持保證金 (float)
                clearing_margin : 0.0,                    # 結算保證金 (float)
                initial_margin_all_single : 0.0,          # 原始保證金 (float)
                opt_value : 0.0,                          # 選擇權市值 (float)
                opt_long_value : 0.0,                     # 選擇權買進市值 (float)
                opt_short_value : 0.0,                    # 選擇權賣出市值 (float)
                profit_or_loss : 0.0,                     # 部位損益 (float)
                premium : 0.0,                            # 權利金 (float)
                spreads : [                               # 複式部位 (List[SpreadPosition])
                    SpreadPosition {
                        date : "2024/04/08",              # 部位建立日期 (str)
                        branch_no : "15901",              # 分公司代號 (str)
                        account : "1234567",              # 帳號 (str)
                        position_kind : 2,                # 部位種類  (int)
                        symbol : "TXO",                   # 商品代號 (str)
                        expiry_date : 202404,             # 履約日 (string)
                        strike_price : 201000,            # 履約價 (float)
                        call_put : Call,                  # 權利別  (CallPut)
                        buy_sell : Buy,                   # 買賣別  (BSAction)
                        price : 185,                      # 成交價 (int)
                        orig_lots : 2,                    # 原始口數 (int)
                        tradable_lot : 2,                 # 可交易口數 (int)
                        order_type : None,                # 委託別 (int or None)
                        currency : "TWD",                 # 幣別 (str)
                        market_price : "365",             # 即時價 (str)
                        initial_margin : 0.0,             # 原始保證金 (float)
                        maintenance_margin : 0.0,         # 維持保證金 (float)
                        clearing_margin : 0.0,            # 結算保證金 (float)
                        initial_margin_all_single : 0.0,  # 原始保證金 (float)
                        opt_value : 36500.0,              # 選擇權市值 (float)
                        opt_long_value : 36500.0,         # 選擇權買進市值 (float)
                        opt_short_value : 0.0,            # 選擇權賣出市值 (float)
                        profit_or_loss : 18000.0,         # 部位損益 (float)
                        premium : 0.0,                    # 權利金 (float)
                    },
                    SpreadPosition {
                        date : "2024/04/08",              # 部位建立日期 (str)
                        branch_no : "15901",              # 分公司代號 (str)
                        account : "1234567",              # 帳號 (str)
                        position_kind : 2,                # 部位種類 (int)
                        symbol : "TXO",                   # 商品代號 (str)
                        expiry_date : 202404,             # 履約日 (string)
                        strike_price : 200000,            # 履約價 (float)
                        call_put : Put,                   # 權利別  (CallPut)
                        buy_sell : Buy,                   # 買賣別  (BSAction)
                        price : 354,                      # 成交價 (int)
                        orig_lots : 2,                    # 原始口數 (int)
                        tradable_lot : 2,                 # 可交易口數 (int)
                        order_type : None,                # 委託別 (int or None)
                        currency : "TWD",                 # 幣別 (str)
                        market_price : "82",              # 即時價 (str)
                        initial_margin : 0.0,             # 原始保證金 (float)
                        maintenance_margin : 0.0,         # 維持保證金 (float)
                        clearing_margin : 0.0,            # 結算保證金 (float)
                        initial_margin_all_single : 0.0,  # 原始保證金 (float)
                        opt_value : 8200.0,               # 選擇權市值 (float)
                        opt_long_value : 8200.0,          # 選擇權買進市值 (float)
                        opt_short_value : 0.0,            # 選擇權賣出市值 (float)
                        profit_or_loss : -27200.0,        # 部位損益 (float)
                        premium : 0.0,                    # 權利金 (float)
                    },
                ],
            }
          ]
    }

```


---

### 權益數查詢

query\_margin\_equity

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                                | 說明 |
| ------- | --------------------------------------------------------------------------------------------------- | ---- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account) | 帳號 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別   | 說明                               |
| ----------- | ------ | ---------------------------------- |
| is\_success | bool   | 是否成功                           |
| data        | List   | 回傳權益數資訊                     |
| message     | string | 當is\_success = false 回傳錯誤訊息 |

###### 權益數 Equity 欄位[​](#權益數-equity-欄位 "Direct link to 權益數 Equity 欄位")

Return type : Object

| 參數                 | 類別   | 說明                                                                        |
| -------------------- | ------ | --------------------------------------------------------------------------- |
| date                 | string | 查詢日期                                                                    |
| branch\_no           | string | 分公司代號                                                                  |
| account              | string | 帳號                                                                        |
| currency             | string | 幣別 : `NTD` 約當台幣 、`TWD` 新台幣、`USD` 美元、`CNY` 人民幣 、`JPY` 日圓 |
| yesterday\_balance   | float  | 昨日餘額                                                                    |
| today\_balance       | float  | 今日餘額                                                                    |
| initial\_margin      | float  | 原始保證金                                                                  |
| maintenance\_margin  | float  | 維持保證金                                                                  |
| clearing\_margin     | float  | 結算保證金                                                                  |
| today\_equity        | float  | 本日權益                                                                    |
| today\_deposit       | float  | 今日入金                                                                    |
| today\_withdrawal    | float  | 今日出金                                                                    |
| today\_trading\_fee  | float  | 今日交易手續費                                                              |
| today\_trading\_tax  | float  | 今日交易稅                                                                  |
| receivable\_premium  | float  | 收取權利金                                                                  |
| payable\_premium     | float  | 付出權利金                                                                  |
| excess\_margin       | float  | 超額保證金                                                                  |
| available\_margin    | float  | 可動用保證金                                                                |
| disgorgement         | float  | 追繳金額                                                                    |
| opt\_pnl             | float  | 未沖銷選擇權浮動損益                                                        |
| opt\_value           | float  | 選擇權市值                                                                  |
| opt\_long\_value     | float  | 未沖銷選擇權買方市值                                                        |
| opt\_short\_value    | float  | 未沖銷選擇權賣方市值                                                        |
| fut\_realized\_pnl   | float  | 期貨平倉損益                                                                |
| fut\_unrealized\_pnl | float  | 期貨期貨未平倉損益                                                          |
| buy\_lot             | int    | 買進口數                                                                    |
| sell\_lot            | int    | 賣出口數                                                                    |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
sdk.futopt_accounting.query_margin_equity(accounts)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
    is_success: True,
    message: None,
    data : [Equity({
              date: "2024/04/08",                   # 查詢日期 (string)
              branch_no: "15901",                   # 分公司代號 (string)
              account: "1234567",                   # 帳號 (string)
              currency: "NTD",                      # 幣別 (string)
              yesterday_balance: 22435152.4,        # 昨日餘額 (float)
              today_balance: 22434910.4,            # 今日餘額 (float)
              initial_margin: 1114946.0,            # 原始保證金 (float)
              maintenance_margin: 939214.0,         # 維持保證金 (float)
              clearing_margin: 915760.0,            # 結算保證金 (float)
              today_equity: 22694910.4,             # 本日權益 (float)
              today_deposit : 0.0,                  # 本日入金 (float)
              today_withdrawal: 2102.0,             # 本日出金 (float)
              today_trading_fee: 16.0,              # 本日交易手續費 (float)
              today_trading_tax: 0.0,               # 本日交易稅 (float)
              receivable_premium: 0.0,              # 收取權利金 (float)
              payable_premium: 9250.0,              # 付出權利金 (float)
              excess_margin: 28744525.0,            # 超額保證金 (float)
              available_margin: 21453562.4,         # 可動用保證金 (float)
              disgorgement: 0.0,                    # 追繳金額 (float)
              opt_pnl: -248600.0,                   # 未沖銷選擇權浮動損益 (float)
              opt_value: -193100.0,                 # 選擇權市值 (float)
              opt_long_value: 311900.0,             # 未沖銷選擇權買方市值 (float)
              opt_short_value: 505000.0,            # 未沖銷選擇權賣方市值 (float)
              fut_realized_pnl: 0.0,                # 期貨平倉損益 (float)
              fut_unrealized_pnl: 60700.0,          # 期貨期貨未平倉損益 (float)
              buy_lot: 22,                          # 買進口數 (int)
              sell_lot: 7                           # 賣出口數 (int)
            }),
            Equity({
              date: "2024/04/08",                   # 查詢日期 (string)
              branch_no: "15901",                   # 分公司代號 (string)
              account: "1234567",                   # 帳號 (string)
              currency: "TWD",                      # 幣別 (string)
              yesterday_balance: 19880310.0,        # 昨日餘額 (float)
              today_balance: 19880068.0,            # 今日餘額 (float)
              initial_margin: 1114946.0,            # 原始保證金 (float)
              maintenance_margin: 939214.0,         # 維持保證金 (float)
              clearing_margin: 915760.0,            # 結算保證金 (float)
              today_equity: 20140068.0,             # 本日權益 (float)
              today_deposit : 0.0,                  # 本日入金 (float)
              today_withdrawal: 2102.0,             # 本日出金 (float)
              today_trading_fee: 16.0,              # 本日交易手續費 (float)
              today_trading_tax: 0.0,               # 本日交易稅 (float)
              receivable_premium: 0.0,              # 收取權利金 (float)
              payable_premium: 9250.0,              # 付出權利金 (float)
              excess_margin: 28744525.0,            # 超額保證金 (float)
              available_margin: 18898720.0,         # 可動用保證金 (float)
              disgorgement: 0.0,                    # 追繳金額 (float)
              opt_pnl: -248600.0,                   # 未沖銷選擇權浮動損益 (float)
              opt_value: -193100.0,                 # 選擇權市值 (float)
              opt_long_value: 311900.0,             # 未沖銷選擇權買方市值 (float)
              opt_short_value: 505000.0,            # 未沖銷選擇權賣方市值 (float)
              fut_realized_pnl: 0.0,                # 期貨平倉損益 (float)
              fut_unrealized_pnl: 60700.0,          # 期貨期貨未平倉損益 (float)
              buy_lot: 22,                          # 買進口數 (int)
              sell_lot: 7,                          # 賣出口數 (int)
            }),
            ...
            ]
}

```


---

### 單式部位查詢

query\_single\_position

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                                | 說明 |
| ------- | --------------------------------------------------------------------------------------------------- | ---- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account) | 帳號 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別   | 說明                               |
| ----------- | ------ | ---------------------------------- |
| is\_success | bool   | 是否成功                           |
| data        | List   | 回傳部位資訊                       |
| message     | string | 當is\_success = false 回傳錯誤訊息 |

##### Position 欄位[​](#position-欄位 "Direct link to Position 欄位")

Return type : Object

| 參數                | 類別                                                                                                                | 說明                                              |
| ------------------- | ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| date                | string                                                                                                              | 部位建立日期                                      |
| branch\_no          | string                                                                                                              | 分公司代號                                        |
| account             | string                                                                                                              | 帳號                                              |
| is\_spread          | bool                                                                                                                | 是否為複式部位                                    |
| position\_kind      | int                                                                                                                 | 部位種類 : `1` 期貨 、`2` 選擇權                  |
| symbol              | string                                                                                                              | 商品代號                                          |
| expiry\_date        | string                                                                                                              | 履約日                                            |
| strike\_price       | float                                                                                                               | 履約價                                            |
| call\_put           | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput)                 | 權利別 : `Call` 、 `Put`                          |
| buy\_sell           | [BSAction](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#bsaction)               | 買賣別 : `Buy` 買、`Sell` 賣                      |
| price               | float                                                                                                               | 成交價                                            |
| orig\_lots          | int                                                                                                                 | 原始口數                                          |
| tradable\_lots      | int                                                                                                                 | 可交易口數                                        |
| order\_type         | [FutOptOrderType](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptordertype) | 委託別 : `New` 新倉、`Close`平倉、`FdayTrade`當沖 |
| currency            | string                                                                                                              | 幣別                                              |
| market\_price       | string                                                                                                              | 即時價                                            |
| initial\_margin     | float                                                                                                               | 原始保證金                                        |
| maintenance\_margin | float                                                                                                               | 維持保證金                                        |
| clearing\_margin    | float                                                                                                               | 結算保證金                                        |
| opt\_value          | float                                                                                                               | 選擇權市值                                        |
| opt\_long\_value    | float                                                                                                               | 選擇權買進市值                                    |
| opt\_short\_value   | float                                                                                                               | 選擇權賣出市值                                    |
| profit\_or\_loss    | float                                                                                                               | 部位損益                                          |
| premium             | float                                                                                                               | 權利金                                            |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
sdk.futopt_accounting.query_single_position(accounts)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
    is_success: True,
    message: None,
    data: [
        Position {
            date: "2024/04/08",                 # 部位建立日期 (string)
            branch_no: "15901",                  # 分公司代號 (string)
            account: "1234567",                  # 帳號 (string)
            order_no: "l0001-0000",              # 訂單編號 (string)
            position_kind: 1,                    # 部位種類  (int)
            symbol: "FITX",                      # 商品代號 (string)
            expiry_date: 202404,                 # 履約日 (string)
            strike_price: None,                  # 履約價 (float)
            call_put: None,                      # 權利別  (CallPut)
            buy_sell: Buy,                       # 買賣別 (BSAction)
            price: 20362,                        # 成交價 (float)
            orig_lots: 2,                        # 原始口數 (int)
            tradable_lot: 2,                     # 可交易口數 (int)
            order_type: New,                     # 委託別  (FutOptOrderType)
            currency: "TWD",                     # 幣別 (string)
            market_price: "20521.0000",          # 即時價 (string)
            initial_margin: 358000.0,            # 原始保證金 (float)
            maintenance_margin: 274000.0,        # 維持保證金 (float)
            clearing_margin: 264000.0,           # 結算保證金 (float)
            profit_or_loss: 63600.0,             # 部位損益 (float)
            premium: 0.0,                        # 權利金 (float)
        },
        Position {
            date: "2024/03/29",                  # 部位建立日期 (string)
            branch_no: "15901",                   # 分公司代號 (string)
            account: "1234567",                   # 帳號 (string)
            order_no: "l0007-0000",               # 訂單編號 (string)
            position_kind: 2,                     # 部位種類 (int)
            symbol: "TX1",                        # 商品代號 (string)
            expiry_date: 202404,                  # 履約日 (string)
            strike_price: 20600,                 # 履約價 (float)
            call_put: Call,                       # 權利別 (CallPut)
            buy_sell: Buy,                        # 買賣別 (BSAction)
            price: 10,                            # 成交價 (float)
            orig_lots: 2,                         # 原始口數 (int)
            tradable_lot: 2,                      # 可交易口數 (int)
            order_type: NEW,                      # 委託別 (FutOptOrderType)
            currency: "TWD",                      # 幣別 (string)
            market_price: "4.6000",               # 即時價 (string)
            initial_margin: 52660.0,              # 原始保證金 (float)
            maintenance_margin: 36460.0,          # 維持保證金 (float)
            clearing_margin: 34460.0,             # 結算保證金 (float)
            profit_or_loss: -540.0,               # 部位損益 (float)
            premium: -1000.0,                     # 權利金 (float)
        },
        Position {
            date: "2024/03/29",                   # 部位建立日期 (string)
            branch_no: "15901",                    # 分公司代號 (string)
            account: "1234567",                    # 帳號 (string)
            order_no: "l0007-0001",                # 訂單編號 (string)
            position_kind: 2,                      # 部位種類  (int)
            symbol: "TX1",                         # 商品代號 (string)
            expiry_date: 202404,                   # 履約日 (string)
            strike_price: 20600,                  # 履約價 (float)
            call_put: Call,                        # 權利別  (CallPut)
            buy_sell: Buy,                         # 買賣別  (BSAction)
            price: 10,                             # 成交價 (float)
            orig_lots: 2,                          # 原始口數 (int)
            tradable_lot: 2,                       # 可交易口數 (int)
            order_type: NEW,                       # 委託別  (FutOptOrderType)
            currency: "TWD",                       # 幣別 (string)
            market_price: "4.6000",                # 即時價 (string)
            initial_margin: 52660.0,              # 原始保證金 (float)
            maintenance_margin: 36460.0,          # 維持保證金 (float)
            clearing_margin: 34460.0,             # 結算保證金 (float)
            profit_or_loss: -540.0,               # 部位損益 (float)
            premium: -1000.0,                     # 權利金 (float)
        },
        Position {
            date: "2024/03/01",                   # 部位建立日期 (string)
            branch_no: "15901",                    # 分公司代號 (string)
            account: "1234567",                    # 帳號 (string)
            order_no: "l0002-0000",                # 訂單編號 (string)
            position_kind: 2,                      # 部位種類  (int)
            symbol: "TXO",                         # 商品代號 (string)
            expiry_date: 202404,                   # 履約日 (string)
            strike_price: 18500,                  # 履約價 (float)
            call_put: Call,                        # 權利別 (CallPut)
            buy_sell: Sell,                        # 買賣別 (BSAction)
            price: 625,                            # 成交價 (float)
            orig_lots: 5,                          # 原始口數 (int)
            tradable_lot: 4,                       # 可交易口數 (int)
            order_type: NEW,                       # 委託別  (FutOptOrderType)
            currency: "TWD",                       # 幣別 (string)
            market_price: "2020.0000",             # 即時價 (string)
            initial_margin: 584000.0,              # 原始保證金 (float)
            maintenance_margin: 544000.0,          # 維持保證金 (float)
            clearing_margin: 536000.0,             # 結算保證金 (float)
            profit_or_loss: -279000.0,             # 部位損益 (float)
            premium: 125000.0,                     # 權利金 (float)
        }
    ]
}

```


---

### 參數對照表

#### 類別[​](#類別 "Direct link to 類別")

Class

##### OrderObject[​](#orderobject "Direct link to OrderObject")

| Parameter             | Type             | Meaning                                                                                                                     |
| --------------------- | ---------------- | --------------------------------------------------------------------------------------------------------------------------- |
| buy\_sell             | BSAction         | [買賣別](#bsaction) 可選用參數`Buy` 買 、 `Sell` 賣                                                                         |
| symbol                | string           | 商品代號                                                                                                                    |
| buy\_sell2 (optional) | BSAction         | [買賣別](#bsaction) - 複式第二隻腳                                                                                          |
| symbol2 (optional)    | string           | 商品代號 - 複式第二隻腳                                                                                                     |
| price                 | string           | 委託價格                                                                                                                    |
| lot                   | int              | 委託口數                                                                                                                    |
| market\_type          | FutOptMarketType | [盤別](#futoptmarkettype) 可選用參數`Future` 期貨日盤、`Option`選擇權日盤、`FutureNight` 期貨夜盤、`OptionNight` 選擇權夜盤 |
| price\_type           | FutOptPriceType  | [價格旗標](#futoptpricetype) 可選用參數為 `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價       |
| time\_in\_force       | TimeInForce      | [委託條件](#timeinforce) 可選用參數為 `ROD`、`FOK`、`IOC`                                                                   |
| order\_type           | FutOptOrderType  | [委託類別](#futoptordertype) 可選用參數為 `New` 新倉、`Close`平倉、`Auto`自動、`FdayTrade`當沖                              |
| user\_def (optional)  | string           | 用戶自定義 (最長10個字元，不支援特殊字元)                                                                                   |

caution

priceType 爲 FutOptPriceType.Limit 時，需填入 price 欄位，其餘時候 price 欄位為空值或為None。

##### FutOptOrderResult[​](#futoptorderresult "Direct link to FutOptOrderResult")

委託列表，透過 [get\_order\_result(accounts)](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/trade/GetOrderResults.txt) 取得。

| 參數                | 類別             | 說明                                                                                                                           |
| ------------------- | ---------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| function\_type      | int              | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗                                                 |
| date                | string           | 交易日期                                                                                                                       |
| seq\_no             | string           | 委託單流水序號                                                                                                                 |
| branch\_no          | string           | 分公司代號                                                                                                                     |
| account             | string           | 帳號                                                                                                                           |
| order\_no           | string           | 委託書號                                                                                                                       |
| asset\_type         | int              | 資產類別 : `1` 期貨 、`2` 選擇權                                                                                               |
| market              | string           | 市場類型 : `TAIMEX` 期貨、選擇權                                                                                               |
| market\_type        | FutOptMarketType | [盤別種類](#futoptmarkettype) : `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤 |
| unit                | int              | 單位數                                                                                                                         |
| currency            | string           | 幣別                                                                                                                           |
| symbol              | string           | 商品代號                                                                                                                       |
| expiry\_date        | string           | 到期日                                                                                                                         |
| strike\_price       | float            | 履約價                                                                                                                         |
| call\_put           | string           | 買賣權 : `Call` 買權、 `Put` 賣權                                                                                              |
| buy\_sell           | BSAction         | [買賣別](#bsaction) : `Buy` 買 、 `Sell` 賣                                                                                    |
| symbol\_leg2        | string           | 商品代號 - 複式第二隻腳                                                                                                        |
| expiry\_date\_leg2  | string           | 到期日 - 複式第二隻腳                                                                                                          |
| strike\_price\_leg2 | float            | 履約價 - 複式第二隻腳                                                                                                          |
| call\_put\_leg2     | string           | 買賣權 - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                                                               |
| buy\_sell\_leg2     | BSAction         | [買賣別](#bsaction) - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                                                      |
| price\_type         | FutOptPriceType  | [原始委託價格別](#futoptpricetype) : `Limit` 限價 、 `LimitUp` 漲停 、 `LimitDown` 跌停 、 `Market` 市價 、 `Reference` 參考價 |
| price               | float            | 價格                                                                                                                           |
| lot                 | int              | 原始委託股口數                                                                                                                 |
| time\_in\_force     | TimeInforce      | [委託條件別](#timeinforce) : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC                                                               |
| order\_type         | FutOptOrderType  | [委託單類型](#futoptordertype) : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖                                 |
| is\_pre\_order      | bool             | 是否為預約單                                                                                                                   |
| status              | int              | 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、`9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功 、 `50` 完全成交 、 `90` 失敗   |
| after\_price\_type  | FutOptPriceType  | [有效委託價格別](#futoptpricetype) : `Limit`限價 、 `Market`市價 、`RangeMarket` 範圍市價 、 `Reference`參考價                 |
| after\_price        | float            | 有效委託價格                                                                                                                   |
| after\_lot          | int              | 有效委託口數                                                                                                                   |
| filled\_lot         | int              | 成交口數                                                                                                                       |
| filled\_money       | float            | 成交價金                                                                                                                       |
| before\_lot         | int              | 改單前有效口數                                                                                                                 |
| before\_price       | float            | 改單前有效價                                                                                                                   |
| user\_def           | string           | 自訂欄位                                                                                                                       |
| last\_time          | string           | 最後異動時間                                                                                                                   |
| error\_message      | string           | 錯誤訊息                                                                                                                       |
| detail              | list             | 委託歷程 (查詢order\_result\_detail or order\_history才有值)                                                                   |
| >> function\_type   | int              | 功能別 : `10` 新單、 `15` 改價、 `20` 改量、`30`刪單、`50`成交                                                                 |
| >> modified\_time   | string           | 修改時間                                                                                                                       |
| >> before\_lot      | int              | 原始委託口數                                                                                                                   |
| >> after\_lot       | int              | 有效委託口數                                                                                                                   |
| >> before\_price    | float            | 原始委託價                                                                                                                     |
| >> after\_price     | float            | 有效委託價                                                                                                                     |
| >> filled\_money    | float            | 成交價金                                                                                                                       |
| >> error\_message   | string           | 錯誤訊息                                                                                                                       |

##### BatchResult[​](#batchresult "Direct link to BatchResult")

批次委託列表，透過 [batch\_order\_lists(accounts)](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/trade/batchOrder/BatchOrderList.txt) 取得。

| Parameter      | Type   | Meaning                                                                        |
| -------------- | ------ | ------------------------------------------------------------------------------ |
| function\_type | int    | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗 |
| date           | string | 交易日期                                                                       |
| branch\_no     | string | 分公司代號                                                                     |
| account        | string | 帳號                                                                           |
| batch\_seq\_no | string | 批次單流水序號                                                                 |

##### FutoptModifyPrice[​](#futoptmodifyprice "Direct link to FutoptModifyPrice")

改價物件

| Parameter     | Type            | Meaning                                                                                                          |
| ------------- | --------------- | ---------------------------------------------------------------------------------------------------------------- |
| order\_result | Object          | [委託列表](#futoptorderresult)                                                                                   |
| price         | string          | 改單後的價格                                                                                                     |
| price\_type   | FutOptPriceType | [改單後的價格類型](#futoptpricetype) : `Limit`限價 、 `Market`市價 、`RangeMarket` 範圍市價 、 `Reference`參考價 |

##### FutOptModifyLot[​](#futoptmodifylot "Direct link to FutOptModifyLot")

改量物件

| Parameter     | Type   | Meaning                        |
| ------------- | ------ | ------------------------------ |
| order\_result | Object | [委託列表](#futoptorderresult) |
| lot           | int    | 改單後的委託量                 |

##### FutOptFilledData[​](#futoptfilleddata "Direct link to FutOptFilledData")

成交回報物件

| 參數                | 類別            | 說明                                                                                           |
| ------------------- | --------------- | ---------------------------------------------------------------------------------------------- |
| date                | string          | 日期                                                                                           |
| branch\_no          | string          | 分公司代號                                                                                     |
| account             | string          | 帳號                                                                                           |
| seq\_no             | string          | 委託單流水序號 (只有主動回報才回傳此欄位)                                                      |
| order\_no           | string          | 委託書號                                                                                       |
| symbol              | string          | 商品代號                                                                                       |
| expiry\_date        | string          | 到期日                                                                                         |
| strike\_price       | float           | 履約價                                                                                         |
| call\_put           | CallPut         | [買賣權](#callput) : `Call` 買權、 `Put` 賣權                                                  |
| buy\_sell           | BSAction        | \[買賣別] (#bsaction) : `Buy` 買 、 `Sell` 賣                                                  |
| symbol\_leg2        | string          | 商品代號 - 複式第二隻腳                                                                        |
| expiry\_date\_leg2  | string          | 到期日 - 複式第二隻腳                                                                          |
| strike\_price\_leg2 | float           | 履約價 - 複式第二隻腳                                                                          |
| call\_put\_leg2     | CallPut         | [買賣權](#callput) - 複式第二隻腳 : `Call` 買權、 `Put` 賣權                                   |
| buy\_sell\_leg2     | BSAction        | [買賣別](#bsaction) - 複式第二隻腳: `Buy` 買 、 `Sell` 賣                                      |
| order\_type         | FutOptOrderType | [委託單類型](#futoptordertype) : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖 |
| filled\_no          | string          | 成交流水號                                                                                     |
| filled\_avg\_price  | float           | 成交均價                                                                                       |
| filled\_lot         | int             | 成交股數                                                                                       |
| filled\_price       | float           | 成交單價                                                                                       |
| filled\_time        | string          | 成交時間                                                                                       |
| user\_def           | string          | 用戶自定義 (只有主動回報才回傳此欄位)                                                          |

##### Account[​](#account "Direct link to Account")

帳號資訊

| Parameter     | Type   | Meaning                                    |
| ------------- | ------ | ------------------------------------------ |
| name          | string | 客戶姓名                                   |
| account       | string | 帳號                                       |
| branch\_no    | string | 分公司代號                                 |
| account\_type | string | 帳號類型 ，回傳 `stock`證券 、`futopt`期貨 |

#### Constants ( 欄位對應數值 )[​](#constants--欄位對應數值- "Direct link to Constants ( 欄位對應數值 )")

##### BSAction[​](#bsaction "Direct link to BSAction")

買賣別 (buySell)

| Name | Meaning |
| ---- | ------- |
| Buy  | 買      |
| Sell | 賣      |

##### CallPut[​](#callput "Direct link to CallPut")

買賣別 (buySell)

| Name | Meaning |
| ---- | ------- |
| Call | Call    |
| Put  | Put     |

##### FutOptMarketType[​](#futoptmarkettype "Direct link to FutOptMarketType")

盤別

| Name        | Meaning    |
| ----------- | ---------- |
| Future      | 期貨日盤   |
| FutureNight | 期貨夜盤   |
| Option      | 選擇權日盤 |
| OptionNight | 選擇權夜盤 |

##### FutOptPriceType[​](#futoptpricetype "Direct link to FutOptPriceType")

價格類型 (priceType)

| Name        | Meaning  |
| ----------- | -------- |
| Limit       | 限價     |
| Market      | 市價     |
| RangeMarket | 範圍市價 |
| Reference   | 參考價   |

##### TimeInForce[​](#timeinforce "Direct link to TimeInForce")

委託條件 (TimeInForce)

| Name | Meaning                               |
| ---- | ------------------------------------- |
| ROD  | 當日有效(Rest of Day)                 |
| FOK  | 全部成交否則取消(Fill-or-Kill)        |
| IOC  | 立即成交否則取消(Immediate-or-Cancel) |

##### FutOptOrderType[​](#futoptordertype "Direct link to FutOptOrderType")

委託類別 (orderType)

| Name      | Meaning |
| --------- | ------- |
| New       | 新倉    |
| Close     | 平倉    |
| Auto      | 自動    |
| FdayTrade | 當沖    |

##### function\_type[​](#function_type "Direct link to function_type")

功能類別

| Name     | Value |
| -------- | ----- |
| 新單     | 0     |
| 新單執行 | 10    |
| 改價     | 15    |
| 改量     | 20    |
| 刪單     | 30    |
| 失敗     | 90    |

##### market[​](#market "Direct link to market")

市場

| Name       | Value  |
| ---------- | ------ |
| 期貨交易所 | TAIMEX |

##### status[​](#status "Direct link to status")

委託單狀態

| Name         | Value                                                      |
| ------------ | ---------------------------------------------------------- |
| 預約單       | 0                                                          |
| 中台收到委託 | 4 ( 請用GetOrderResult查詢狀態 )                           |
| 後台連線逾時 | 9 ( 請稍後再使用GetOrderResult查詢狀態 or 聯絡您的營業員 ) |
| 委託成功     | 10                                                         |
| 刪單成功     | 30                                                         |
| 完全成交     | 50                                                         |
| 改價失敗     | 19                                                         |
| 改量失敗     | 29                                                         |
| 刪單失敗     | 39                                                         |
| 失敗         | 90                                                         |

#### Month[​](#month "Direct link to Month")

月份代號

##### 期貨[​](#期貨 "Direct link to 期貨")

| 一月 | 二月 | 三月 | 四月 | 五月   | 六月   |
| ---- | ---- | ---- | ---- | ------ | ------ |
| A    | B    | C    | D    | E      | F      |
| 七月 | 八月 | 九月 | 十月 | 十一月 | 十二月 |
| G    | H    | I    | J    | K      | L      |

##### 選擇權[​](#選擇權 "Direct link to 選擇權")

Call

| 一月 | 二月 | 三月 | 四月 | 五月   | 六月   |
| ---- | ---- | ---- | ---- | ------ | ------ |
| A    | B    | C    | D    | E      | F      |
| 七月 | 八月 | 九月 | 十月 | 十一月 | 十二月 |
| G    | H    | I    | J    | K      | L      |

Put

| 一月 | 二月 | 三月 | 四月 | 五月   | 六月   |
| ---- | ---- | ---- | ---- | ------ | ------ |
| M    | N    | O    | P    | Q      | R      |
| 七月 | 八月 | 九月 | 十月 | 十一月 | 十二月 |
| S    | T    | U    | V    | W      | X      |


---

### 登入

apikey\_login

版本資訊

v2.2.7 起新增功能

相關說明請參閱 [API Key 說明](https://www.fbs.com.tw/TradeAPI/docs/trading/api-key-apply.txt) 頁面

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數         | 類別   | 說明           |
| ------------ | ------ | -------------- |
| personal\_id | String | 登入的ID       |
| Key          | String | 申請的 API Key |
| cert\_path   | String | 憑證路徑       |
| cert\_pass   | String | 憑證密碼       |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別   | 說明                               |
| ----------- | ------ | ---------------------------------- |
| is\_success | bool   | 是否成功                           |
| data        | List   | 回傳帳號資訊                       |
| message     | string | 當is\_success = false 回傳錯誤訊息 |

##### 帳號資訊 Account 欄位[​](#帳號資訊--account-欄位 "Direct link to 帳號資訊  Account 欄位")

Return type: Object

| 參數          | 類別   | 說明                                     |
| ------------- | ------ | ---------------------------------------- |
| name          | String | 客戶姓名                                 |
| account       | String | 客戶帳號                                 |
| branch\_no    | String | 分公司代號                               |
| account\_type | string | 帳號類型 回傳 `stock` 證券 `futopt` 期貨 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
from fubon_neo.sdk import FubonSDK, Order
from fubon_neo.constant import TimeInForce, OrderType, PriceType, MarketType, BSAction

sdk = FubonSDK()

accounts = sdk.apikey_login("Your ID", "Your KEY","Your Cert Path","Your Cert Password")

print(accounts) #若有多帳號，則回傳多個

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
    is_success: True,
    message: None,
    data : Account{
        name : "富邦Bill",      # 客戶姓名 (string)
        account :  "28",       # 客戶帳號 (string)
        branch_no : "6460",     # 分公司代號  (string)
        account_type : "futopt"  # 帳號類型 (string)
    }
}

```


---

### 登入

login

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數         | 類別   | 說明       |
| ------------ | ------ | ---------- |
| personal\_id | String | 登入的ID   |
| password     | String | 登入的密碼 |
| cert\_path   | String | 憑證路徑   |
| cert\_pass   | String | 憑證密碼   |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別   | 說明                               |
| ----------- | ------ | ---------------------------------- |
| is\_success | bool   | 是否成功                           |
| data        | List   | 回傳帳號資訊                       |
| message     | string | 當is\_success = false 回傳錯誤訊息 |

##### 帳號資訊 Account 欄位[​](#帳號資訊--account-欄位 "Direct link to 帳號資訊  Account 欄位")

Return type: Object

| 參數          | 類別   | 說明                                     |
| ------------- | ------ | ---------------------------------------- |
| name          | String | 客戶姓名                                 |
| account       | String | 客戶帳號                                 |
| branch\_no    | String | 分公司代號                               |
| account\_type | string | 帳號類型 回傳 `stock` 證券 `futopt` 期貨 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
from fubon_neo.sdk import FubonSDK, Order
from fubon_neo.constant import TimeInForce, OrderType, PriceType, MarketType, BSAction

sdk = FubonSDK()

accounts = sdk.login("Your ID", "Your Password","Your Cert Path","Your Cert Password")

print(accounts) #若有多帳號，則回傳多個

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
    is_success: True,
    message: None,
    data : Account{
        name : "富邦Bill",      # 客戶姓名 (string)
        account :  "28",       # 客戶帳號 (string)
        branch_no : "6460",     # 分公司代號  (string)
        account_type : "futopt"  # 帳號類型 (string)
    }
}

```


---

### 登出

logout

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別 | 說明     |
| ----------- | ---- | -------- |
| is\_success | bool | 是否成功 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
from fubon_neo.sdk import FubonSDK, Order
from fubon_neo.constant import TimeInForce, OrderType, PriceType, MarketType, BSAction

sdk = FubonSDK()

sdk.logout()

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
True

```


---

### 刪除批次委託單

batch\_cancel\_order

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數           | 類別                                                                                                                    | 說明               |
| -------------- | ----------------------------------------------------------------------------------------------------------------------- | ------------------ |
| account        | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                     | 帳號               |
| cancel\_object | [FutOptOrderResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptorderresult) | 批次取消委託單內容 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別   | 說明                               |
| ----------- | ------ | ---------------------------------- |
| is\_success | bool   | 是否成功                           |
| data        | List   | 回傳修改資訊                       |
| message     | string | 當is\_success = false 回傳錯誤訊息 |

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
# 批次刪單(利用batch detail回傳的內容刪單)
cancel_object = [
    batch_results_detail.data[0],
    batch_results_detail.data[1],
]
sdk.futopt.batch_cancel_order(account, cancel_object)

# 批次刪單(利用不同的單筆委託)
cancel_object = [
    orders.data[37],
    orders.data[35],
]

sdk.futopt.batch_cancel_order(account, cancel_object)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
    is_success: True,
    message: None,
    data :[  FutOptOrderResult { # 利用batchDetail 回傳內容
                function_type: 30,                   # 功能別 (int)
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
                price_type: Limit,                   # 原始委託價格別 (FutOptPriceType)
                price: 1822.6,                       # 價格 (float)
                lot: 2,                              # 原始委託股口數 (int)
                time_in_force: ROD,                  # 委託條件別 (TimeInforce)
                order_type: Auto,                    # 委託單類型 (FutOptOrderType)
                is_pre_order: False,                 # 是否為預約單 (bool)
                status: 30,                          # 委託單狀態 (int)
                after_price_type: None,              # 有效委託價格別 (FutOptPriceType)
                after_price: 1822.6,                 # 有效委託價格 (float)
                after_lot: 0,                        # 有效委託股口數 (int)
                filled_lot: 0,                       # 成交股口數 (int)
                filled_money: 0,                     # 成交價金 (int)
                before_lot: None,                    # 改單前有效股口數 (int)
                before_price: None,                  # 改單前有效價格 (float)
                user_def: None,                      # 自訂欄位 (string)
                last_time: "13:21:34",               # 最後異動時間 (string)
                details: None,                        # 委託歷程 (list)
                error_message: None,                 # 錯誤訊息 (string)
    },
          FutOptOrderResult {
                function_type: 30,                   # 功能別 (int)
                date: "2024/03/25",                  # 交易日期 (string)
                seq_no: "00110212609",               # 委託單流水序號 (string)
                branch_no: "15901",                  # 分公司代號 (string)
                account: "1234567",                  # 帳號 (string)
                order_no: "C0002",                   # 委託書號 (string)
                asset_type: 1,                       # 資產類別 (int)
                market: "TAIMEX",                    # 市場類型 (string)
                market_type: Future,                 # 盤別種類 (FutOptMarketType)
                unit: 1,                             # 單位數 (int)
                currency: "TWD",                     # 幣別 (string)
                symbol: "FITF",                      # 商品代號 (string)
                expiry_date: "202404",               # 到期日 (string)
                strike_price: None,                  # 履約價 (float)
                call_put: None,                      # 買賣權 (CallPut)
                ...
    }
]}

以下範例擷取data內容

[FutOptOrderResult{  # 批次刪單(利用不同的單筆委託)
                 function_type: 30,                   # 功能別 (int)
                date: "2024/03/25",                  # 交易日期 (string)
                seq_no: "00110212610",               # 委託單流水序號 (string)
                branch_no: "15901",                  # 分公司代號 (string)
                account: "1234567",                  # 帳號 (string)
                order_no: "C0003",                   # 委託書號 (string)
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
                price_type: Limit,                   # 原始委託價格別 (FutOptPriceType)
                price: 1822.6,                       # 價格 (float)
                lot: 2,                              # 原始委託股口數 (int)
                time_in_force: ROD,                  # 委託條件別 (TimeInforce)
                order_type: Auto,                    # 委託單類型 (FutOptOrderType)
                is_pre_order: False,                 # 是否為預約單 (bool)
                status: 30,                          # 委託單狀態 (int)
                after_price_type: None,              # 有效委託價格別 (FutOptPriceType)
                after_price: 1822.6,                 # 有效委託價格 (float)
                after_lot: 0,                        # 有效委託股口數 (int)
                filled_lot: 0,                       # 成交股口數 (int)
                filled_money: 0,                     # 成交價金 (int)
                before_lot: None,                    # 改單前有效股口數 (int)
                before_price: None,                  # 改單前有效價格 (float)
                user_def: None,                      # 自訂欄位 (string)
                last_time: "13:21:28",               # 最後異動時間 (string)
                error_message: None,                 # 錯誤訊息 (string)
    },
FutOptOrderResult{
                function_type: 30,                   # 功能別 (int)
                date: "2024/03/25",                  # 交易日期 (string)
                seq_no: "00110212611",               # 委託單流水序號 (string)
                branch_no: "15901",                  # 分公司代號 (string)
                account: "1234567",                  # 帳號 (string)
                order_no: "C0004",                   # 委託書號 (string)
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
                price_type: Limit,                   # 原始委託價格別 (FutOptPriceType)
                price: 1822.6,                       # 價格 (float)
                lot: 2,                              # 原始委託股口數 (int)
                time_in_force: ROD,                  # 委託條件別 (TimeInforce)
                order_type: Auto,                    # 委託單類型 (FutOptOrderType)
                is_pre_order: False,                 # 是否為預約單 (bool)
                status: 30,                          # 委託單狀態 (int)
                
}] 

```


---

### 批次修改委託價格

batch\_modify\_price

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

| 參數           | 類別                                                                                                                           | 說明               |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------ | ------------------ |
| account        | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                            | 帳號               |
| ModifyPriceObj | [FutOptModifyPrice](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptmodifyprice) (list) | 批次修改委託單內容 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別   | 說明                               |
| ----------- | ------ | ---------------------------------- |
| is\_success | bool   | 是否成功                           |
| data        | List   | 回傳修改資訊                       |
| message     | string | 當is\_success = false 回傳錯誤訊息 |

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
# 批次改價(利用batch detail回傳的內容改單)
modify_objects = [
    sdk.futopt.make_modify_price_obj(batch_results_detail.data[0], "19900"),
    sdk.futopt.make_modify_price_obj(batch_results_detail.data[1], "19900"),
]
sdk.futopt.batch_modify_price(target_user, modify_objects)


# 批次改價(利用不同的單筆委託)
modify_objects = [
    sdk.futopt.make_modify_price_obj(orders.data[37], "19900"),
    sdk.futopt.make_modify_price_obj(orders.data[35], "19900"),
]

sdk.futopt.batch_modify_price(target_user, modify_objects)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
    is_success: True,
    message: None,
    data : [ FutOptOrderResult{# 批次改價(利用batch detail回傳的內容改單)
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
          },
          FutOptOrderResult{
                function_type: 15,                   # 功能別 (int)
                date: "2024/03/25",                  # 交易日期 (string)
                seq_no: "00110212664",               # 委託單流水序號 (string)
                branch_no: "15901",                  # 分公司代號 (string)
                account: "1234567",                  # 帳號 (string)
                order_no: "C0006",                   # 委託書號 (string)
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
                ...
        }]
}


以下範例擷取data內容

[   FutOptOrderResult{# 批次改價(利用不同的單筆委託)
                    function_type: 15,                   # 功能別 (int)
                    date: "2024/03/25",                  # 交易日期 (string)
                    seq_no: "00110212665",               # 委託單流水序號 (string)
                    branch_no: "15901",                  # 分公司代號 (string)
                    account: "1234567",                  # 帳號 (string)
                    order_no: "C0006",                   # 委託書號 (string)
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
                    last_time: "13:39:10",               # 最後異動時間 (string)
                    error_message: None,                 # 錯誤訊息 (string)
    },
    FutOptOrderResult{
                 function_type: 15,                   # 功能別 (int)
                date: "2024/03/25",                  # 交易日期 (string)
                seq_no: "00110212665",               # 委託單流水序號 (string)
                branch_no: "15901",                  # 分公司代號 (string)
                account: "1234567",                  # 帳號 (string)
                order_no: "C0007",                   # 委託書號 (string)
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
                ...
}]

```


---

### 批次修改委託數量

batch\_modify\_lot

##### 先使用make\_modify\_lot\_obj 建立 FutOptModifyLot 物件[​](#先使用make_modify_lot_obj-建立-futoptmodifylot-物件 "Direct link to 先使用make_modify_lot_obj 建立 FutOptModifyLot 物件")

| 參數        | 類別                                                                                                                    | 說明                                                |
| ----------- | ----------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| orderResult | [FutOptOrderResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptorderresult) | 預計修改的委託單                                    |
| lots        | int                                                                                                                     | 修改後的委託量 ( 修改後數量包含此委託單已成交部份 ) |

將回傳的物件放入batch\_modify\_lot 方法中

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數         | 類別                                                                                                                       | 說明               |
| ------------ | -------------------------------------------------------------------------------------------------------------------------- | ------------------ |
| account      | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                        | 帳號               |
| ModifyLotObj | [FutOptModifyLot](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptmodifylot) (list) | 批次改量委託單內容 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別   | 說明                               |
| ----------- | ------ | ---------------------------------- |
| is\_success | bool   | 是否成功                           |
| data        | List   | 回傳修改資訊                       |
| message     | string | 當is\_success = false 回傳錯誤訊息 |

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
# 批次改量(利用batch detail回傳的內容改單)
modify_objects = [
    sdk.futopt.make_modify_lot_obj(batch_results_detail.data[0], 2),
    sdk.futopt.make_modify_lot_obj(batch_results_detail.data[1], 2),
]
sdk.futopt.batch_modify_lot(target_user, modify_objects)

# 批次改量(利用不同的單筆委託)
modify_objects = [
    sdk.futopt.make_modify_lot_obj(orders.data[37], 2),
    sdk.futopt.make_modify_lot_obj(orders.data[35], 2),
]

sdk.futopt.batch_modify_lot(target_user, modify_objects)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
    is_success: True,
    message: None,
    data : [ FutOptOrderResult{ # 批次改量(利用batch detail回傳的內容改單)
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
                before_lot: 3,                      # 改單前有效股口數 (int)
                before_price: None,                 # 改單前有效價格 (float)
                user_def: None,                     # 自訂欄位 (string)
                last_time: "18:24:40",              # 最後異動時間 (string)
                details: None,                        # 委託歷程 (list)
                error_message: None,                # 錯誤訊息 (string)
       },
        FutOptOrderResult{ # 批次改量(利用batch detail回傳的內容改單)
            function_type: 20,                  # 功能別 (int)
            date: "2024/03/25",                 # 交易日期 (string)
            seq_no: "03100161320",              # 委託單流水序號 (string)
            branch_no: "15901",                 # 分公司代號 (string)
            account: "1234567",                 # 帳號 (string)
            order_no: "l001E",                  # 委託書號 (string)
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
            ...
        }]
}


以下範例擷取data內容

[FutOptOrderResult{ # 批次改量(利用不同的單筆委託)
                function_type: 20,                  # 功能別 (int)
                date: "2024/03/25",                 # 交易日期 (string)
                seq_no: "03100161321",              # 委託單流水序號 (string)
                branch_no: "15901",                 # 分公司代號 (string)
                account: "1234567",                 # 帳號 (string)
                order_no: "l001F",                  # 委託書號 (string)
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
                before_lot: 3,                      # 改單前有效股口數 (int)
                before_price: None,                 # 改單前有效價格 (float)
                user_def: None,                     # 自訂欄位 (string)
                last_time: "18:24:40",              # 最後異動時間 (string)
                error_message: None,                # 錯誤訊息 (string)
        },
        FutOptOrderResult{
                function_type: 20,                  # 功能別 (int)
                date: "2024/03/25",                 # 交易日期 (string)
                seq_no: "03100161322",              # 委託單流水序號 (string)
                branch_no: "15901",                 # 分公司代號 (string)
                account: "1234567",                 # 帳號 (string)
                order_no: "l001G",                  # 委託書號 (string)
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
            ...
    }]

```


---

### 取得批次委託明細

batch\_order\_detail

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數               | 類別                                                                                                        | 說明         |
| ------------------ | ----------------------------------------------------------------------------------------------------------- | ------------ |
| account            | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)         | 帳號         |
| batch\_order\_list | [BatchResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#batchresult) | 批次委託列表 |

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
| detail              | list                                                                                                                  | 委託歷程 (查詢order\_result\_detail or order\_history才有值)                                                                                    |
| error\_message      | string                                                                                                                | 錯誤訊息                                                                                                                                        |

caution

此功能僅供查詢批次送單執行結果，欲取得委託單最新狀態請使用單筆委託單查詢功能

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
batch_results = sdk.futopt.batch_order_lists(account)
sdk.futopt.batch_order_detail(account, batch_results.data[0])

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
    is_success: True,
    message: None,
    data : [FutOptOrderResult {
                date: "2024/04/11",                     # 交易日期 (string)
                seq_no: "00230177314",                  # 委託單流水序號 (string)
                branch_no: "15000",                      # 分公司代號 (string)
                account: "9974825",                      # 帳號 (string)
                order_no: "C0020",                       # 委託書號 (string)
                asset_type: 2,                           # 資產類別 :  `1` 期貨 、`2` 選擇權 (int)
                market: "TAIMEX",                        # 市場類型 :  `TAIMEX` 期貨、選擇權 (string)
                market_type: Option,                     # 盤別種類  :  `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤 (MarketType)
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
                price_type: Limit,                       # 原始委託價格別  : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價 (PriceType)
                price: 2100,                             # 價格 (float)
                lot: 1,                                  # 原始委託股口數 (int)
                time_in_force: ROD,                      # 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC (TimeInforce)
                order_type: New,                         # 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖 (OrderType)
                is_pre_order: False,                     # 是否為預約單 (bool)
                status: 50,                              # 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、`9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功  、 `50` 完全成交 、 `90` 失敗 (int)
                after_price_type: None,                   # 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價 (PriceType)
                after_price: 2100,                       # 有效委託價格 (float)
                after_lot: 1,                            # 有效委託口數 (int)
                filled_lot: 1,                           # 成交口數 (int)
                filled_money: 2100,                     # 成交價金 (float)
                before_lot: None,                        # 改單前有效口數 (int)
                before_price: None,                      # 改單前有效價 (float)
                user_def: None,                          # 自訂欄位 (string)
                last_time: "10:41:46.760",               # 最後異動時間 (string)
                details: None,                        # 委託歷程 (list)
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
                market_type: Option,                     # 盤別種類  :  `Future` 期貨日盤 、 `Option` 選擇權日盤 、 `FutureNight` 期貨夜盤 、 `OptionNight` 選擇權夜盤 (MarketType)
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
                price_type: Limit,                       # 原始委託價格別  : `Limit` 限價 、 `Market` 市價 、 `RangeMarket` 範圍市價、`Reference` 參考價 (PriceType)
                price: 2230,                             # 價格 (float)
                lot: 1,                                  # 原始委託股口數 (int)
                time_in_force: ROD,                      # 委託條件別 : `ROD` ROD 、 `FOK` FOK 、 `IOC` IOC (TimeInforce)
                order_type: New,                         # 委託單類型 : `New` 新倉 、 `Close` 平倉 、 `Auto` 自動 、 `FdayTrade` 當沖 (OrderType)
                is_pre_order: False,                     # 是否為預約單 (bool)
                status: 50,                              # 委託單狀態 : `0` 預約單 、 `4` 中台收到委託 、`9` 後台連線逾時 、`10` 委託成功 、 `30`刪單成功  、 `50` 完全成交 、 `90` 失敗 (int)
                after_price_type: None,                   # 有效委託價格別 : `Limit` 限價 、 `Market` 市價 、`RangeMarket` 範圍市價、 `Reference` 參考價 (PriceType)
                after_price: 2230,                       # 有效委託價格 (float)
                after_lot: 1,                            # 有效委託口數 (int)
                filled_lot: 1,                           # 成交口數 (int)
                filled_money: 2230,                     # 成交價金 (float)
                before_lot: None,                        # 改單前有效口數 (int)
                before_price: None,                      # 改單前有效價 (float)
                user_def: None,                          # 自訂欄位 (string)
                last_time: "10:41:46.760",               # 最後異動時間 (string)
                error_message: None                      # 錯誤訊息 (string)
            }]}

```


---

### 取得批次委託列表

batch\_order\_lists

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數    | 類別                                                                                                | 說明 |
| ------- | --------------------------------------------------------------------------------------------------- | ---- |
| account | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account) | 帳號 |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別   | 說明                               |
| ----------- | ------ | ---------------------------------- |
| is\_success | bool   | 是否成功                           |
| data        | List   | 回傳批次單資訊                     |
| message     | string | 當is\_success = false 回傳錯誤訊息 |

##### 批次單 BatchResult 欄位[​](#批次單-batchresult-欄位 "Direct link to 批次單 BatchResult 欄位")

Return type : Object

| 參數           | 類別   | 說明                                                                           |
| -------------- | ------ | ------------------------------------------------------------------------------ |
| function\_type | int    | 功能別 : `0` 新單 、`10` 新單執行、 `15` 改價、 `20` 改量、`30`刪單 、`90`失敗 |
| date           | string | 交易日期                                                                       |
| branch\_no     | string | 分公司代號                                                                     |
| account        | string | 帳號                                                                           |
| batch\_seq\_no | string | 批次單流水序號                                                                 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
sdk.futopt.batch_order_lists(account)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
    is_success: True,
    message: None,
    data : [BatchResult{
                function_type: 0,    # 功能種類 (int)
                date: "2023/10/16",  # 交易日期 (string)
                branch_no: "15901",  # 分公司代號 (string)
                account: "1234567",      # 帳號 (string)
                batch_seq_no: "11EE6BC3B85670DE8000000C29304663" # 批次單流水序號 (string)
            },
            BatchResult{
                    function_type: 15,   # 功能種類 (int)
                    date: "2023/10/16",  # 交易日期 (string)
                    branch_no: "15901",  # 分公司代號 (string)
                    account: "1234567",      # 帳號 (string)
                    batch_seq_no: "11EE6BC3E189F02A8000000C29304663" # 批次單流水序號 (string)
            },
            ...
    ]
} 

```


---

### 建立批次委託單

batch\_place\_order

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數          | 類別                                                                                                                         | 說明     |
| ------------- | ---------------------------------------------------------------------------------------------------------------------------- | -------- |
| account       | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                          | 帳號     |
| order\_object | [OrderObject](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#orderobject) (list of object) | 委託內容 |

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
orders = [
   FutOptOrder(
    buy_sell = BSAction.Buy,
    symbol = "TXO20000E4",
    price =  "530",
    lot =  1,
    market_type = FutOptMarketType.Option,
    price_type = FutOptPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = FutOptOrderType.Auto,
    user_def = "FromPy" # optional field
), FutOptOrder(
    buy_sell = BSAction.Buy,
    symbol = "TXO20000E4",
    price =  "530",
    lot =  1,
    market_type = FutOptMarketType.Option,
    price_type = FutOptPriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = FutOptOrderType.Auto,
    user_def = "FromPy" # optional field
) ]

sdk.futopt.batch_place_order(target_user, orders)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
    is_success: True,
    message: None,
    data : [ FutOptOrderResult{
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
            },
            FutOptOrderResult{
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
}]}

```


---

### 刪除委託單

cancel\_order

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數          | 類別                                                                                                                    | 說明               |
| ------------- | ----------------------------------------------------------------------------------------------------------------------- | ------------------ |
| account       | [Account](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#account)                     | 帳號               |
| order\_result | [FutOptOrderResult](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#futoptorderresult) | 欲取消的委託單物件 |
| unblock       | bool (optional) (default = False)                                                                                       | 是否採用非阻塞     |

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數        | 類別              | 說明                               |
| ----------- | ----------------- | ---------------------------------- |
| is\_success | bool              | 是否成功                           |
| data        | FutOptOrderResult | 回傳更新委託資訊                   |
| message     | string            | 當is\_success = false 回傳錯誤訊息 |

##### 修改資訊 FutOptOrderResult 欄位[​](#修改資訊-futoptorderresult--欄位 "Direct link to 修改資訊 FutOptOrderResult  欄位")

Return type: Object

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
sdk.futopt.cancel_order(account, cancel_order)

```

#### 回傳範例[​](#回傳範例 "Direct link to 回傳範例")

```py
Result {
    is_success: True,
    message: None,
    data : FutOptOrderResult {
                function_type: 30,                   # 功能別 (int)
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
                status: 30,                          # 委託單狀態 (int)
                after_price_type: None,              # 有效委託價格別 (PriceType)
                after_price: 1822.6,                 # 有效委託價格 (float)
                after_lot: 0,                        # 有效委託股口數 (int)
                filled_lot: 0,                       # 成交股口數 (int)
                filled_money: 0,                     # 成交價金 (int)
                before_lot: None,                    # 改單前有效股口數 (int)
                before_price: None,                  # 改單前有效價格 (float)
                user_def: None,                      # 自訂欄位 (string)
                last_time: "13:21:34",               # 最後異動時間 (string)
                details: None,                        # 委託歷程 (list)
                error_message: None,                 # 錯誤訊息 (string)
    }
}

```


---

### 商品代號轉換

convert\_symbol

#### 輸入參數[​](#輸入參數 "Direct link to 輸入參數")

| 參數          | 類別                                                                                                             | 說明                              |
| ------------- | ---------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| symbol        | string                                                                                                           | 帳務商品代號                      |
| expiry\_date  | string                                                                                                           | 履約日                            |
| strike\_price | float ( Optional )                                                                                               | 履約價                            |
| call\_put     | [CallPut](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#callput) ( Optional ) | 買賣權 : `Call` Call 、 `Put` Put |

info

月份代號可參閱[參數對照表](https://www.fbs.com.tw/TradeAPI/docs/trading-future/library/python/EnumMatrix.txt#month)

#### Result 回傳[​](#result-回傳 "Direct link to Result 回傳")

| 參數   | 類別   | 說明               |
| ------ | ------ | ------------------ |
| symbol | string | 行情與下單商品代號 |

#### 請求範例[​](#請求範例 "Direct link to 請求範例")

```py
#期貨
sdk.futopt.convert_symbol("FITX","202404")

