### 事前準備

caution

在開始富邦新一代 API 條件單前，您必須完成簽署條件單智慧單

#### 簽署條件單風險預告書[​](#簽署條件單風險預告書 "Direct link to 簽署條件單風險預告書")

登入富邦證券簽署中心，選擇期貨，簽署期貨條件單

![線上簽署](/TradeAPI/assets/images/futurecondition-28d403275883db4916ae8c3d249373c3.png)

#### 引入套件 ( 僅Python、JS需要，C#不需額外操作)[​](#引入套件--僅pythonjs需要c不需額外操作 "Direct link to 引入套件 ( 僅Python、JS需要，C#不需額外操作)")

引入條件單相關套件

* Python
* Node.js

```py
from fubon_neo.sdk import FubonSDK, Condition, FutOptConditionOrder
from fubon_neo.constant import ( 
    TriggerContent, TradingType, Operator, FutOptTPSLOrder, FutOptTPSLWrapper, SplitDescription,
    StopSign, TimeSliceOrderType, FutOptConditionPriceType, FutOptConditionOrderType, FutOptConditionMarketType, TrailOrder, Direction, FutOptTrailOrder, ConditionStatus, HistoryStatus
)


```

```js
const { FubonSDK, TriggerContent, TradingType, Operator, FutOptTPSLOrder, FutOptTPSLWrapper, SplitDescription,
    StopSign, TimeSliceOrderType, FutOptConditionPriceType, FutOptConditionOrderType, FutOptConditionMarketType, TrailOrder, Direction, FutOptTrailOrder, ConditionStatus, HistoryStatus } = require('fubon-neo');


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

更多訊息請洽富邦證券官網[「網路下單系統無法正常使用之應變措施」](https://www.fbs.com.tw/Beginner/TradingNote)


---

### API Key 說明

版本限制

2.2.7版本新增功能

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

1. 進入[金鑰申請及管理](https://www.fbs.com.tw/TradeAPI/docs/key.txt)

![key\_login\_step1](/TradeAPI/assets/images/key_login_step1-d6356e971b19e5a35f3a1b089bf2a6a7.png)

2. 輸入您的身分證字號、電子平台密碼 ![key\_login\_step2](/TradeAPI/assets/images/key_login_step2-516ea7a99d80eba338fd760b77a2bc93.png)

3. 申請網頁版憑證，收取 OTP ![OTP](/TradeAPI/assets/images/OTP-3771c5ab5c12c603f48994c14e130c77.png)

4. 申請完憑證後，即可進行憑證匯出與新增金鑰 ![key\_login\_step3](/TradeAPI/assets/images/key_login_step3-48c99a5a670c60ce55e5d09a4fb097eb.png)

使用預設密碼憑證

在系統提示輸入憑證密碼時，請使用您的登入 ID

5. 進行金鑰申請

   * 點擊新增金鑰<br />![add\_key](/TradeAPI/assets/images/add_key-ce15b3f601484f2a96499f2a85150398.png)

   * 設置控制權限 ( IP或有效期限若不輸入，則為無限制 ) ![add\_key\_step2](/TradeAPI/assets/images/add_key_step2-ee688b483e6d020bb1ad988c42cd2325.png)

   * 設置成功，顯示 Secret Key ( Secret Key 關閉後即不再顯示 ) ![add\_key\_step3](/TradeAPI/assets/images/add_key_step3-6913c4ffe9e7e563c740c6ae15f80cbe.png)

6. 可檢視先前申請的 Key 內容或停用 ( Key 最多申請 30 把 ) ![key\_list](/TradeAPI/assets/images/key_list-7e65381ef6623e6c61a53a7cef01bf94.png)


---

### 帳務

***

我們可以透過成交及庫存查詢來確認損益等資訊。

info

查詢發送的次數為每秒5次，若超出上限，請稍後再試試！

#### 庫存查詢[​](#庫存查詢 "Direct link to 庫存查詢")

* Python
* Node.js
* C#

```python
inventories = sdk.accounting.inventories(accounts.data[0])
print(inventories)

```

```py
Result {
    is_success: true,
    message: None,
    data : [
        Inventory{
          date: "2023/09/20",    # 查詢庫存日期 (string)
          account: "26",         # 帳號 (string)
          branch_no: "6460",     # 分公司代號 (string)
          stock_no: "1101",      # 股票代號 (string)
          order_type: Stock,     # 委託類別 (OrderType)
          lastday_qty: 2000,     # 昨日整股餘額 (int)
          buy_qty: 0,            # 整股委買股數 (int)
          buy_filled_qty: 0,     # 整股買進成交股數 (int)
          buy_value: 0,     # 整股買進成交價金 (int)
          today_qty: 2000,  # 整股餘額 (int)
          tradable_qty: 2000, #可委託整股庫存數 (int)
          sell_qty: 0,     # 整股委賣股數 (int)
          sell_filled_qty: 0, # 整股賣出成交股數 (int)
          sell_value: 0,  # 整股賣出成交價金 (int)
          odd: InventoryOdd{ # 零股
            lastday_qty: 0, # 昨日零股餘額 (int)
            buy_qty: 0,     # 零股委買股數 (int)
            buy_filled_qty: 0, # 零股買進成交股數 (int)
            buy_value: 0,   # 零股買進成交價金 (int)
            today_qty: 0,   # 零股餘額 (int)
            tradable_qty: 0, # 可委託零股餘額 (int)
            sell_qty: 0,    # 零股委賣股數 (int)
            sell_filled_qty: 0, # 零股賣出成交股數 (int)
            sell_value: 0    # 零股賣出成交價金 (int)
        }}, 
        ...
      ]
}

```

```js
const inventory = sdk.accounting.inventories(accounts.data[0])
console.log(inventory)

```

```js
{
  isSuccess: true,
  data:[
    {
      date: '2023/10/13',
      account: '26',
      branchNo: '6460',
      stockNo: '1101',
      orderType: Stock,
      lastdayQty: 2000,
      buyQty: 0,
      buyFilledQty: 0,
      buyValue: 0,
      todayQty: 2000,
      tradableQty: 2000,
      sellQty: 0,
      sellFilledQty: 0,
      sellValue: 0,
      odd: {
        lastdayQty: 0,
        buyQty: 0,
        buyFilledQty: 0,
        buyValue: 0,
        todayQty: 0,
        tradableQty: 0,
        sellQty: 0,
        sellFilledQty: 0,
        sellValue: 0
      }
    },
    ...
  ]
}

```

```cs
var inventories = sdk.Accounting.Inventoies(accounts.data[0]);
foreach (var inventory in inventories.data)
{
  Console.WriteLine(inventory);
}

```

```cs
{
    date = 2023/09/21,    // 查詢庫存日期 (string)
    account = 26,         // 帳號 (string)
    branchNo = 6460,      // 分公司代號 (string)
    stockNo = 1101,       // 股票代號 (string)
    orderType = Stock,    // 委託類別 (OrderType))
    lastdayQty = 2000,    // 昨日整股餘額 (int)
    buyQty = 0,           // 整股委買股數 (int)
    buyFilledQty = 0,     // 整股買進成交股數 (int)
    buyValue = 0,         // 整股買進成交價金 (int)
    todayQty = 2000,      // 整股餘額 (int)
    tradableQty = 2000,   // 可委託整股庫存數 (int)
    sellQty = 0,          // 整股委賣股數 (int)
    sellFilledQty = 0,    // 整股賣出成交股數 (int)
    sellValue = 0,        // 整股賣出成交價金 (int)
     odd = InventoryOdd {  // 零股
        lastdayQty = 0,    // 昨日零股餘額 (int)
        buyQty = 0,        // 零股委買股數 (int)
        buyFilledQty = 0,  // 零股買進成交股數 (int)
        buyValue = 0,      // 零股買進成交價金 (int)
        todayQty = 0,      // 零股餘額 (int)
        tradableQty = 0,   // 可委託零股餘額 (int)
        sellQty = 0,       // 零股委賣股數 (int)
        sellFilledQty = 0, // 零股賣出成交股數 (int)
        sellValue = 0      // 零股賣出成交價金 (int)
     }
}

```

* 以下範例回傳僅擷取data內容

#### 未實現損益查詢[​](#未實現損益查詢 "Direct link to 未實現損益查詢")

* Python
* Node.js
* C#

```python
unrealized_pnl = sdk.accounting.unrealized_gains_and_loses(accounts.data[0])
print(unrealized_pnl.data)

```

```py
[UnrealizedData({
    date: "2023/05/23", # 查詢當天日期 (string)
    account: "482",     # 帳號 (string)
    branch_no: "6460",  # 分公司代號 (string)
    stock_no: "2442",   # 股票代號 (string)
    buy_sell: Buy,      # 買賣別 (BSAction)
    order_type: Margin, # 委託類別 (OrderType)
    cost_price: 19.95,  # 成本價 (int)
    tradable_qty: 6000, # 可委託庫存數 (int)
    unrealized_profit: 0, # 未實現獲利 (int)
    unrealized_loss: 10002 # 未實現損失 (int)
}),
  ...
]

```

```js
const unrealizedPNL = sdk.accounting.unrealizedGainsAndLoses(accounts.data[0])
console.log(unrealizedPNL.data)

```

```js
[
  {
    date: '2021/08/09',
    account: '26',
    branchNo: '6460',
    stockNo: '2303',
    buySell: Buy,
    orderType: Margin,
    costPrice: 50,
    tradableQty: 1000,
    todayQty: 1000,
    unrealizedProfit: 47000,
    unrealizedLoss: 0
  },
  ...
]

```

```cs
var unrealizedPNL =  sdk.Accounting.UnrealizedGainsAndLoses(accounts.data[0]);
foreach (var unrealized in unrealizedPNL.data)
{
  Console.WriteLine(unrealized);
}

```

```cs
{ 
    date = 2021/08/09,   // 查詢當天日期 (string)
    account = 26,        // 帳號 (string)
    branchNo = 6460,     // 分公司代號 (string)
    stockNo = 2303,      // 股票代號 (string)
    buySell = Buy,       // 買賣別 (BsAction)
    orderType = Margin,  // 委託類別 (OrderType)
    costPrice = 50,      // 成本價 (double)
    tradableQty = 1000,   // 可委託庫存數 (int)
    unrealizedProfit = 45600,  // 未實現獲利 (int)
    unrealizedLoss = 0         // 未實現損失 (int)
}

```

#### 交割資訊[​](#交割資訊 "Direct link to 交割資訊")

可以查詢交割資訊，確認我們今日或近三日的應收付金額。

* Python
* Node.js
* C#

```python
settlement = sdk.accounting.query_settlement(accounts.data[0],"0d")
print(settlement.data)

```

```js
const settlement = sdk.accounting.querySettlement(accounts.data[0],"0d")
console.log(settlement.data)

```

```cs
var settlement = sdk.Accounting.QuerySettlement(accounts.data[0],"0d");
Console.WriteLine(settlement.data);
Console.WriteLine(settlement.data.settle);

```

以下查詢的結果代表9/12時應付1,429,513元交割款：

* Python
* Node.js
* C#

```py
SettlementData{
  account: AccountRes{    # 帳號資訊
    branch_no: "6460",    # 帳號 (string)
    account: "26"         # 分公司代號 (string)
  },
  details: [  # 交割資訊
    {
      date: "2023/09/08", # 查詢日期 (string)
      settlement_date: "2023/09/12", # 交割日期 (string)
      buy_value: 735500,                  # 買進金額 (int)
      buy_fee: 313,                       # 買進手續費 (int)
      buy_settlement: -1429513,           # 買進應收付款 (int)
      buy_tax: 0,                         # 買進交易稅 (int)
      sell_value: 770500,                 # 賣出金額 (int)
      sell_fee: 320,                      # 賣出手續費 (int)
      sell_settlement: 0,                 # 賣出應收付款 (int)
      sell_tax: 2309,                     # 賣出交易稅 (int)
      total_bs_value: 1506000,            # 合計買賣金額 (int)
      total_fee: 633,                     # 合計手續費 (int)
      total_tax: 2309,                    # 合計交易稅 (int)
      total_settlement_amount: -1429513,  # 合計交割金額 (int)
      currency: "TWD",                    # 幣別 (string)
    }
  ]
}

```

```js
{
   account: { // 帳號資訊
    branch_no: '6460',    // 帳號 (string)
    account: '26'         // 分公司代號 (string)
  },
  details: [  // 交割資訊
    {
      date: '2023/09/08',            // 查詢日期 (string)
      settlement_date: '2023/09/12', // 交割日期 (string)
      buyValue: 735500,              // 買進金額 (number)
      buyFee: 313,                   // 買進手續費 (int)
      buySettlement: -1429513,       // 買進應收付款 (int)
      buyTax: 0,                     // 買進交易稅 (int)
      sellValue: 770500,             // 賣出金額 (int)
      sellFee: 320,                  // 賣出手續費 (int)
      sellSettlement: 0,             // 賣出應收付款 (int)
      sellTax: 2309,                 // 賣出交易稅 (int)
      totalBsValue: 1506000,         // 合計買賣金額 (int)
      totalFee: 633,                   // 合計手續費 (int)
      totalSettlementAmount: -1429513, // 合計交割金額 (int)
      totalTax: 2309,                  // 合計交易稅 (int)
      currency: "TWD"                  // 幣別 (string)
    }
  ]
}

```

```cs
{ 
    account = AccountRes {  // 帳號資訊
      account = 26,         // 帳號 (string)
      branchNo = 6460       // 分公司代號 (string)
      }, 
    details = [                // 交割資訊
          date = 2023/09/08,             // 查詢日期 (string)
          settlementDate = 2023/09/12,  // 交割日期 (string)
          buyValue = 735500,           // 買進金額             (int)
          buyFee = 313,                // 買進費用             (int)
          buySettlement = -1429513,    // 買進應收付款         (int)
          buyTax = 0,                  // 買進交易稅           (int)
          sellValue = 770500,          // 賣出金額             (int)
          sellFee = 320,               // 賣出費用             (int)
          sellSettlement = 0,          // 賣出應收付款         (int)
          sellTax = 2309,              // 賣出交易稅           (int)
          totalBsValue = 1506000,      // 合計買賣金額         (int)
          totalFee = 633,              // 合計手續費           (int)
          totalSettlementAmount = -1429513, // 合計交割款金額  (int)
          totalTax = 2309,                  // 合計交易稅      (int)
          currency = TWD                    // 幣別           (string)
    ]
}

```


---

### 非阻塞下單

首先，我們先理解阻塞與非阻塞的概念， 阻塞（Block）和非阻塞（Unblock）是用來描述事件、操作或通信方式的兩種不同方式，以下是它們的基本概念和區別：

info

非阻塞模式一般搭配主動回報進行委託單管理

info

阻塞（Block）模式亦可搭配並行（concurrency）程式設計，達到同時送出多筆指令的效果

請參考 [**並行（concurrency）程式設計下單範例**](#%E4%B8%A6%E8%A1%8Cconcurrency%E7%A8%8B%E5%BC%8F%E8%A8%AD%E8%A8%88%E4%B8%8B%E5%96%AE%E7%AF%84%E4%BE%8B)

##### 阻塞（Block）：[​](#阻塞block "Direct link to 阻塞（Block）：")

阻塞操作是指事件或操作按照預定的順序進行，並且一個操作完成後，才會回覆結果。

![sync](/TradeAPI/assets/images/normal-24db51cb3aa3d4c74d40c79d4ab38ac8.png)

##### 非阻塞（Unblock）：[​](#非阻塞unblock "Direct link to 非阻塞（Unblock）：")

非阻塞操作是指事件或操作不必按照固定的順序進行，可以並行執行，且一個操作不需要等待另一個操作的完成。 當API server接收到您發出的Request後就直接回覆，即為非阻塞。

![async](/TradeAPI/assets/images/async-f1632167c6d8f31259864d28445e3ffc.png)

##### 使用阻塞機制下單[​](#使用阻塞機制下單 "Direct link to 使用阻塞機制下單")

委託後，回覆的Order Response即會帶回完整的資料內容。

* Python
* Node.js
* C#

```py
#建立委託單內容
order = Order(
    buy_sell = BSAction.Buy,
    symbol = "2881",
    price = "66",
    quantity = 2000,
    market_type = MarketType.Common,
    price_type = PriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = OrderType.Stock,
    user_def = "FromPy" # optional field
) 

sdk.stock.place_order(accounts.data[0], order)  #下單委託

#或採用
#sdk.stock.place_order(accounts.data[0], order, False)


```

```js
const order = {
  buySell: BSAction.Buy,
  symbol: "2881",
  price: "66",
  quantity: 2000,
  marketType: MarketType.Common,
  priceType: PriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: OrderType.Stock,
  userDef: "fromJs"
};

sdk.stock.placeOrder(accounts.data[0], order); 

// 或採用
//sdk.stock.placeOrder(accounts.data[0], order, false);


```

```cs
var order = new Order(
    BsAction.Buy,
    "2881",
    "66",
    2000,
    MarketType.Common,
    PriceType.Limit,
    TimeInForce.Rod,
    OrderType.Stock,
    null
);

sdk.Stock.PlaceOrder(accounts.data[0] ,order); // 使用阻塞委託下單
// 或採用
//sdk.stock.placeOrder(accounts.data[0], order, false);


```

##### 使用非阻塞機制下單[​](#使用非阻塞機制下單 "Direct link to 使用非阻塞機制下單")

委託後，回覆的Order Response可能會少帶出委託書號···等資訊。

* Python
* Node.js
* C#

```py
#建立委託單內容
order = Order(
    buy_sell = BSAction.Buy,
    symbol = "2881",
    price = "66",
    quantity = 2000,
    market_type = MarketType.Common,
    price_type = PriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = OrderType.Stock,
    user_def = "FromPy" # optional field
) 

sdk.stock.place_order(accounts.data[0], order, True)  #下單委託


```

```js
const order = {
  buySell: BSAction.Buy,
  symbol: "2881",
  price: "66",
  quantity: 2000,
  marketType: MarketType.Common,
  priceType: PriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: OrderType.Stock,
  userDef: "fromJs"
};

sdk.stock.placeOrder(accounts.data[0], order, true);


```

```cs
var order = new Order(
    BsAction.Buy,
    "2881",
    "66",
    2000,
    MarketType.Common,
    PriceType.Limit,
    TimeInForce.Rod,
    OrderType.Stock,
    null
);

sdk.Stock.PlaceOrder(accounts.data[0] ,order, true); // 使用非阻塞委託下單


```

以下為支援非阻塞委託的函數

* PlaceOrder - 下單委託
* ModifyPrice - 修改委託價格
* ModifyQuantity - 修改委託數量
* CancelOrder - 取消委託

##### 並行（concurrency）程式設計下單範例[​](#並行concurrency程式設計下單範例 "Direct link to 並行（concurrency）程式設計下單範例")

* Python
* Node.js
* C#

```py
import concurrent.futures

#Create Order Object
order = Order(
    buy_sell = BSAction.Buy,
    symbol = "2881",
    price = "66",
    quantity = 2000,
    market_type = MarketType.Common,
    price_type = PriceType.Limit,
    time_in_force = TimeInForce.ROD,
    order_type = OrderType.Stock,
    user_def = "FromPy" # optional
)

# 下單

def my_place_order():
  sdk.stock.place_order(accounts.data[0], order)  #Place Order

with concurrent.futures.ThreadPoolExecutor() as executor:
  # Submit 20 tasks
  futures = [executor.submit(my_place_order) for _ in range(20)]

  # Wait for all tasks to complete
  concurrent.futures.wait(futures)


```

```js
//Create Order Object
const order = {
  buySell: BSAction.Buy,
  symbol: "2881",
  price: "66",
  quantity: 2000,
  marketType: MarketType.Common,
  priceType: PriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: OrderType.Stock,
  userDef: "fromJs"
};

// 下單

function my_place_order() {
  return new Promise((resolve) => {
    sdk.stock.placeOrder(accounts.data[0], order);
    resolve();
  });
}

async function main() {
  const tasks = [];

  for (let i = 0; i < 20; i++) {
    tasks.push(my_place_order());
  }

  await Promise.all(tasks); // Wait for all tasks to complete
}

main().then(() => {
  console.log("All tasks completed")
});


```

```cs
using System;
using System.Threading.Tasks;

// Create Order Object
var order = new Order(
    BsAction.Buy,
    "2881",
    "66",
    2000,
    MarketType.Common,
    PriceType.Limit,
    TimeInForce.Rod,
    OrderType.Stock,
    null
);

// 下單

Task[] tasks = new Task[20];

for (int i = 0; i < 20; i++)
{
  tasks[i] = Task.Run(() => sdk.Stock.PlaceOrder(accounts.data[0], order));
}

Task.WaitAll(tasks);


```


---

### 主動回報範例（輕量風控版）

info

輕量風控為大戶下單服務，需另外申請開通，詳情請洽所屬營業員

以下列舉主動回報範例，包含阻塞式 (non-async, unblock=False) 及 非阻塞式 (async, unblock=True) 功能

（ 關於阻塞式與非阻塞式功能，請參考 [非阻塞下單](https://www.fbs.com.tw/TradeAPI/docs/trading/guide/advance/asyn_order.txt) ）

caution

此文件僅提供常見使用場景對應範例，不保證包含所有例外情況

#### 單筆下單[​](#單筆下單 "Direct link to 單筆下單")

##### 單筆新單[​](#單筆新單 "Direct link to 單筆新單")

###### 阻塞式 (non-async, unblock=False)[​](#阻塞式-non-async-unblockfalse "Direct link to 阻塞式 (non-async, unblock=False)")

1. 下單成功

   * 主動回報（兩筆）:

     第一筆 status 顯示為 8，表示後台準備送出委託單；第二筆 status 顯示為 10，下單成功

     ```py
     ==下單主動回報==
     Code None
     內容 OrderResult {
         function_type: 0,
         date: "2024/10/17",
         seq_no: "00098000023",
         branch_no: "20603",
         account: "9809789",
         order_no: None,
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: true,
         status: 8,
         after_price_type: Limit,
         after_price: 43,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 0,
         before_price: 43,
         user_def: "Test45",
         last_time: "10:44:05.796",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==下單主動回報==
     Code None
     內容 OrderResult {
         function_type: 10,
         date: "2024/10/17",
         seq_no: "00098000023",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ008",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 10,
         after_price_type: Limit,
         after_price: 43,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 0,
         before_price: 43,
         user_def: "Test45",
         last_time: "10:44:05.797",
         details: None,
         error_message: None,
     }
     ========

     ```

   * 函式回傳值 (return):

     ```py
     Result {
         is_success: True,
         message: None,
         data: OrderResult {
             function_type: 0,
             date: "2024/10/17",
             seq_no: "00098000023",
             branch_no: "20603",
             account: "9809789",
             order_no: "KQ008",
             asset_type: 0,
             market: "TAIEX",
             market_type: Common,
             stock_no: "1102",
             buy_sell: Buy,
             price_type: Limit,
             price: 43,
             quantity: 5000,
             time_in_force: ROD,
             order_type: Stock,
             is_pre_order: false,
             status: 10,
             after_price_type: Limit,
             after_price: 43,
             unit: 1000,
             after_qty: 5000,
             filled_qty: 0,
             filled_money: 0,
             before_qty: 0,
             before_price: 43,
             user_def: "Test45",
             last_time: "10:44:05.798",
             details: None,
             error_message: None,
         }
     }

     ```

2. 下單失敗

   * 主動回報（兩筆）:

     第一筆 status 顯示為 8，表示後台準備送出委託單；第二筆 status 顯示為 90，下單異常

     ```py
     ==下單主動回報==
     內容 OrderResult {
         function_type: 0,
         date: "2024/10/17",
         seq_no: "00098000024",
         branch_no: "20603",
         account: "9809789",
         order_no: None,
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 20,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: true,
         status: 8,
         after_price_type: Limit,
         after_price: 20,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 0,
         before_price: 20,
         user_def: "Test97",
         last_time: "10:45:04.122",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==下單主動回報==
     內容 OrderResult {
         function_type: 90,
         date: "2024/10/17",
         seq_no: "00098000024",
         branch_no: "20603",
         account: "9809789",
         order_no: None,
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 20,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 90,
         after_price_type: Limit,
         after_price: 20,
         unit: 1000,
         after_qty: 0,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 0,
         before_price: 20,
         user_def: "Test97",
         last_time: "10:45:04.124",
         details: None,
         error_message: "超過跌停價[5656-後檯]",
     }
     ========

     ```

   * 函式回傳值 (return):

     下單失敗，無委託單資料回傳

     ```py
     Result {
         is_success: False,
         message: 超過跌停價,
         data: None
     }

     ```

###### 非阻塞 (async, unblock=True)[​](#非阻塞-async-unblocktrue "Direct link to 非阻塞 (async, unblock=True)")

1. 下單成功

   * 主動回報（三筆）:

     第一筆為ACK，表示系統將執行請求（status 4），第二筆表示後台準備送單（status 8），此時皆無 order\_no（尚未確認下單成功），可用 seq\_no 進行後續比對；最後確認下單狀態（成功為 status 10）

     ```py
     ==下單主動回報==
     內容 OrderResult {
         function_type: 0,
         date: "2024/10/17",
         seq_no: "00098000025",
         branch_no: "20603",
         account: "9809789",
         order_no: None,
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: true,
         status: 4,
         after_price_type: Limit,
         after_price: 43,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 0,
         before_price: 43,
         user_def: "Test47",
         last_time: "10:45:44.303",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==下單主動回報==
     內容 OrderResult {
         function_type: 10,
         date: "2024/10/17",
         seq_no: "00098000025",
         branch_no: "20603",
         account: "9809789",
         order_no: None,
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 8,
         after_price_type: Limit,
         after_price: 43,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 0,
         before_price: 43,
         user_def: "Test47",
         last_time: "10:45:44.305",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==下單主動回報==
     內容 OrderResult {
         function_type: 10,
         date: "2024/10/17",
         seq_no: "00098000025",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ009",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 10,
         after_price_type: Limit,
         after_price: 43,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 0,
         before_price: 43,
         user_def: "Test47",
         last_time: "10:45:44.305",
         details: None,
         error_message: None,
     }
     ========

     ```

   * 函式回傳值 (return):

     因使用非阻塞，函式收到 ACK 即回傳 (status 4)

     ```py
     Result {
         is_success: True,
         message: None,
         data: OrderResult {
             function_type: 0,
             date: "2024/10/17",
             seq_no: "00098000025",
             branch_no: "20603",
             account: "9809789",
             order_no: None,
             asset_type: 0,
             market: "TAIEX",
             market_type: Common,
             stock_no: "1102",
             buy_sell: Buy,
             price_type: Limit,
             price: 43,
             quantity: 5000,
             time_in_force: ROD,
             order_type: Stock,
             is_pre_order: true,
             status: 4,
             after_price_type: Limit,
             after_price: 43,
             unit: 1000,
             after_qty: 5000,
             filled_qty: 0,
             filled_money: 0,
             before_qty: 0,
             before_price: 43,
             user_def: "Test47",
             last_time: "10:45:44.303",
             details: None,
             error_message: None,
         }
     }

     ```

2. 下單失敗

   * 主動回報（三筆）:

     第一筆為ACK，表示系統將執行請求（status 4），第二筆表示後台準備送單（status 8），此時皆無 order\_no（尚未確認下單成功），可用 seq\_no 進行後續比對；最後確認下單狀態（失敗為 status 90）

     ```py
     ==下單主動回報==
     內容 OrderResult {
         function_type: 0,
         date: "2024/10/17",
         seq_no: "00098000026",
         branch_no: "20603",
         account: "9809789",
         order_no: None,
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 430,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: true,
         status: 4,
         after_price_type: Limit,
         after_price: 430,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 0,
         before_price: 430,
         user_def: "Test77",
         last_time: "10:46:24.101",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==下單主動回報==
     內容 OrderResult {
         function_type: 10,
         date: "2024/10/17",
         seq_no: "00098000026",
         branch_no: "20603",
         account: "9809789",
         order_no: None,
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 430,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 8,
         after_price_type: Limit,
         after_price: 430,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 0,
         before_price: 430,
         user_def: "Test77",
         last_time: "10:46:24.102",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==下單主動回報==
     內容 OrderResult {
         function_type: 90,
         date: "2024/10/17",
         seq_no: "00098000026",
         branch_no: "20603",
         account: "9809789",
         order_no: None,
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 430,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 90,
         after_price_type: Limit,
         after_price: 430,
         unit: 1000,
         after_qty: 0,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 0,
         before_price: 430,
         user_def: "Test77",
         last_time: "10:46:24.103",
         details: None,
         error_message: "超過漲停價[5655-後檯]",
     }
     ========

     ```

   * 函式回傳值 (return):

     因使用非阻塞，函式收到 ACK 即回傳 (status 4)

     ```py
     Result {
         is_success: True,
         message: None,
         data: OrderResult {
             function_type: 0,
             date: "2024/10/17",
             seq_no: "00098000026",
             branch_no: "20603",
             account: "9809789",
             order_no: None,
             asset_type: 0,
             market: "TAIEX",
             market_type: Common,
             stock_no: "1102",
             buy_sell: Buy,
             price_type: Limit,
             price: 430,
             quantity: 5000,
             time_in_force: ROD,
             order_type: Stock,
             is_pre_order: true,
             status: 4,
             after_price_type: Limit,
             after_price: 430,
             unit: 1000,
             after_qty: 5000,
             filled_qty: 0,
             filled_money: 0,
             before_qty: 0,
             before_price: 430,
             user_def: "Test77",
             last_time: "10:46:24.101",
             details: None,
             error_message: None,
         }
     }

     ```

##### 修改委託單價格[​](#修改委託單價格 "Direct link to 修改委託單價格")

info

改單無論使用阻塞模式或非阻塞模式，若由系統擋單（例如改價同原委託價），無 status 4 回報（表示系統將執行請求）；若由後台擋單（例如超過漲跌停價），則有 status 4 回報

阻塞模式與非阻塞模式差異為，若有 status 4 回報時（系統向後送單），非阻塞模式之函式收到該筆回報即回傳，而阻塞模式之函式將等到最終成功或失敗確認才回傳

###### 阻塞[​](#阻塞 "Direct link to 阻塞")

1. 改價成功

   * 主動回報（兩筆）:

     改價 function\_type 15，回報第一筆為系統將執行請求（status 4）；第二筆改價成功 status 10

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 15,
         date: "2024/10/17",
         seq_no: "00098000025",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ009",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 4,
         after_price_type: Limit,
         after_price: 43,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: None,
         before_price: 43,
         user_def: "Test47",
         last_time: "10:47:24.816",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 15,
         date: "2024/10/17",
         seq_no: "00098000025",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ009",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 10,
         after_price_type: Limit,
         after_price: 42,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: None,
         before_price: 43,
         user_def: "Test47",
         last_time: "10:47:24.817",
         details: None,
         error_message: None,
     }
     ========

     ```

   * 函式回傳 (return):

     ```py
     Result {
         is_success: True,
         message: None,
         data: OrderResult {
             function_type: 15,
             date: "2024/10/17",
             seq_no: "00098000025",
             branch_no: "20603",
             account: "9809789",
             order_no: "KQ009",
             asset_type: 0,
             market: "TAIEX",
             market_type: Common,
             stock_no: "1102",
             buy_sell: Buy,
             price_type: Limit,
             price: 43,
             quantity: 5000,
             time_in_force: ROD,
             order_type: Stock,
             is_pre_order: false,
             status: 10,
             after_price_type: Limit,
             after_price: 42,
             unit: 1000,
             after_qty: 5000,
             filled_qty: 0,
             filled_money: 0,
             before_qty: None,
             before_price: 43,
             user_def: "Test47",
             last_time: "10:47:24.817",
             details: None,
             error_message: None,
         }
     }

     ```

2. 改價失敗

   * 主動回報（兩筆）:

     第一筆系統將執行請求 status 4；第二筆改價失敗回報 status 19

     info

     此範例由後台擋回請求，因此有 status 4

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 15,
         date: "2024/10/17",
         seq_no: "00098000025",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ009",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 4,
         after_price_type: Limit,
         after_price: 42,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: None,
         before_price: 42,
         user_def: "Test47",
         last_time: "10:48:00.682",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==改單主動回報==
     Code [DT3_10104]超過漲停價
     內容 OrderResult {
         function_type: 15,
         date: "2024/10/17",
         seq_no: "00098000025",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ009",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 19,
         after_price_type: Limit,
         after_price: 42,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: None,
         before_price: 42,
         user_def: "Test47",
         last_time: "10:48:00.683",
         details: None,
         error_message: "超過漲停價",
     }
     ========

     ```

   * 函式回傳 (return):

     改價失敗，無委託單資料回傳

     ```py
     Result {
         is_success: False,
         message: 超過漲停價,
         data: None
     }

     ```

###### 非阻塞[​](#非阻塞 "Direct link to 非阻塞")

1. 改價成功

   * 主動回報（兩筆）:

     改價 function\_type 15，回報第一筆為系統將執行請求（status 4）；第二筆改價成功 status 10

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 15,
         date: "2024/10/17",
         seq_no: "00098000025",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ009",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 4,
         after_price_type: Limit,
         after_price: 42,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: None,
         before_price: 42,
         user_def: "Test47",
         last_time: "10:50:55.480",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 15,
         date: "2024/10/17",
         seq_no: "00098000025",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ009",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 10,
         after_price_type: Limit,
         after_price: 43,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: None,
         before_price: 42,
         user_def: "Test47",
         last_time: "10:50:55.481",
         details: None,
         error_message: None,
     }
     ========

     ```

   * 函式回傳 (return):

     因為使用非阻塞，函式收到 ACK（status 4）即回傳

     ```py
     Result {
         is_success: True,
         message: None,
         data: OrderResult {
             function_type: 15,
             date: "2024/10/17",
             seq_no: "00098000025",
             branch_no: "20603",
             account: "9809789",
             order_no: "KQ009",
             asset_type: 0,
             market: "TAIEX",
             market_type: Common,
             stock_no: "1102",
             buy_sell: Buy,
             price_type: Limit,
             price: 43,
             quantity: 5000,
             time_in_force: ROD,
             order_type: Stock,
             is_pre_order: false,
             status: 4,
             after_price_type: Limit,
             after_price: 42,
             unit: 1000,
             after_qty: 5000,
             filled_qty: 0,
             filled_money: 0,
             before_qty: None,
             before_price: 42,
             user_def: "Test47",
             last_time: "10:50:55.479",
             details: None,
             error_message: None,
         }
     }

     ```

2. 改價失敗

   * 主動回報（一筆）:

     改價失敗 status 19

     info

     此範例由系統直接擋回請求，因此**無** status 4

     ```py
     ==改單主動回報==
     Code [154]證券改價限價格格和原價格相同==>[00098000025]
     內容 OrderResult {
         function_type: 15,
         date: "2024/10/17",
         seq_no: "00098000025",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ009",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 19,
         after_price_type: Limit,
         after_price: 43,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: None,
         before_price: None,
         user_def: "Test47",
         last_time: "10:51:31.697",
         details: None,
         error_message: "證券改價限價格格和原價格相同==>[00098000025]",
     }
     ========

     ```

   * 函式回傳 (return):

     改價失敗，無委託單資料回傳

     ```py
     Result {
         is_success: False,
         message: 證券改價限價格格和原價格相同==>[00098000025],
         data: None
     }

     ```

##### 修改委託單數量[​](#修改委託單數量 "Direct link to 修改委託單數量")

info

改單無論使用阻塞模式或非阻塞模式，若由系統擋單（例如改價同原委託價），無 status 4 回報（表示系統將執行請求）；若由後台擋單（例如超過漲跌停價），則有 status 4 回報

阻塞模式與非阻塞模式差異為，若有 status 4 回報時（系統向後送單），非阻塞模式之函式收到該筆回報即回傳，而阻塞模式之函式將等到最終成功或失敗確認才回傳

###### 阻塞[​](#阻塞-1 "Direct link to 阻塞")

1. 改量成功

   * 主動回報（兩筆）:

     改量 function\_type 20，回報第一筆為系統將執行請求（status 4）；第二筆改價成功 status 10

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 20,
         date: "2024/10/17",
         seq_no: "00098000025",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ009",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 4,
         after_price_type: None,
         after_price: 43,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 5000,
         before_price: None,
         user_def: "Test47",
         last_time: "10:53:11.924",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 20,
         date: "2024/10/17",
         seq_no: "00098000025",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ009",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 10,
         after_price_type: None,
         after_price: 43,
         unit: 1000,
         after_qty: 3000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 5000,
         before_price: None,
         user_def: "Test47",
         last_time: "10:53:11.925",
         details: None,
         error_message: None,
     }
     ========

     ```

   * 函式回傳 (return):

     ```py
     Result {
         is_success: True,
         message: None,
         data: OrderResult {
             function_type: 20,
             date: "2024/10/17",
             seq_no: "00098000025",
             branch_no: "20603",
             account: "9809789",
             order_no: "KQ009",
             asset_type: 0,
             market: "TAIEX",
             market_type: Common,
             stock_no: "1102",
             buy_sell: Buy,
             price_type: Limit,
             price: 43,
             quantity: 5000,
             time_in_force: ROD,
             order_type: Stock,
             is_pre_order: false,
             status: 10,
             after_price_type: None,
             after_price: 43,
             unit: 1000,
             after_qty: 3000,
             filled_qty: 0,
             filled_money: 0,
             before_qty: 5000,
             before_price: None,
             user_def: "Test47",
             last_time: "10:53:11.926",
             details: None,
             error_message: None,
         }
     }

     ```

2. 改量失敗

   * 主動回報（一筆）:

     改量 function\_type 20，回報失敗 status 29 （由系統直接擋回請求，因此無 status 4）

     ```py
     ==改單主動回報==
     Code [164]證券減量使用剩餘數量大於有效單位數量==>[00098000025]
     內容 OrderResult {
         function_type: 20,
         date: "2024/10/17",
         seq_no: "00098000025",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ009",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 29,
         after_price_type: None,
         after_price: 43,
         unit: 1000,
         after_qty: 3000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: None,
         before_price: None,
         user_def: "Test47",
         last_time: "10:53:48.940",
         details: None,
         error_message: "證券減量使用剩餘數量大於有效單位數量==>[00098000025]",
     }
     ========

     ```

   * 函式回傳 (return):

     改量失敗，無委託單資料回傳

     ```py
     Result {
         is_success: False,
         message: 證券減量使用剩餘數量大於有效單位數量==>[00098000025],
         data: None
     } 

     ```

###### 非阻塞[​](#非阻塞-1 "Direct link to 非阻塞")

1. 改量成功

   * 主動回報（兩筆）:

     改量 function\_type 20，回報第一筆為系統將執行請求（status 4）；第二筆改價成功 status 10

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 20,
         date: "2024/10/17",
         seq_no: "00098000025",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ009",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 4,
         after_price_type: None,
         after_price: 43,
         unit: 1000,
         after_qty: 3000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 3000,
         before_price: None,
         user_def: "Test47",
         last_time: "10:54:17.774",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 20,
         date: "2024/10/17",
         seq_no: "00098000025",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ009",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 10,
         after_price_type: None,
         after_price: 43,
         unit: 1000,
         after_qty: 2000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 3000,
         before_price: None,
         user_def: "Test47",
         last_time: "10:54:17.775",
         details: None,
         error_message: None,
     }
     ========

     ```

   * 函式回傳 (return):

     因使用非阻塞，函式收到 ACK (status 4) 即回傳

     ```py
     Result {
         is_success: True,
         message: None,
         data: OrderResult {
             function_type: 20,
             date: "2024/10/17",
             seq_no: "00098000025",
             branch_no: "20603",
             account: "9809789",
             order_no: "KQ009",
             asset_type: 0,
             market: "TAIEX",
             market_type: Common,
             stock_no: "1102",
             buy_sell: Buy,
             price_type: Limit,
             price: 43,
             quantity: 5000,
             time_in_force: ROD,
             order_type: Stock,
             is_pre_order: false,
             status: 4,
             after_price_type: None,
             after_price: 43,
             unit: 1000,
             after_qty: 3000,
             filled_qty: 0,
             filled_money: 0,
             before_qty: 3000,
             before_price: None,
             user_def: "Test47",
             last_time: "10:54:17.773",
             details: None,
             error_message: None,
         }
     }

     ```

2. 改量失敗

   * 主動回報（一筆）:

     改量 function\_type 20，回報失敗 status 29 （由系統直接擋回請求，因此無 status 4）

     ```py
     ==改單主動回報==
     Code [164]證券減量使用剩餘數量大於有效單位數量==>[00098000025]
     內容 OrderResult {
         function_type: 20,
         date: "2024/10/17",
         seq_no: "00098000025",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ009",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 29,
         after_price_type: None,
         after_price: 43,
         unit: 1000,
         after_qty: 2000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: None,
         before_price: None,
         user_def: "Test47",
         last_time: "10:54:54.455",
         details: None,
         error_message: "證券減量使用剩餘數量大於有效單位數量==>[00098000025]",
     }
     ========

     ```

   * 函式回傳 (return):

     改量失敗，無委託單資料回傳

     ```py
     response: Result {
         is_success: False,
         message: 證券減量使用剩餘數量大於有效單位數量==>[00098000025],
         data: None
     }

     ```

##### 刪除委託單[​](#刪除委託單 "Direct link to 刪除委託單")

caution

若委託單已**部分成交**, 刪單成功狀態代碼為 40, 其餘為 30

info

改單無論使用阻塞模式或非阻塞模式，若由系統擋單（例如改價同原委託價），無 status 4 回報（表示系統將執行請求）；若由後台擋單（例如超過漲跌停價），則有 status 4 回報

阻塞模式與非阻塞模式差異為，若有 status 4 回報時（系統向後送單），非阻塞模式之函式收到該筆回報即回傳，而阻塞模式之函式將等到最終成功或失敗確認才回傳

###### 阻塞[​](#阻塞-2 "Direct link to 阻塞")

1. 刪單成功

   * 主動回報（兩筆）:

     刪單 function\_type 30，回報第一筆為系統將執行請求（status 4）；第二筆刪單成功 status 30 （若已有部分成交 status 40）

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 30,
         date: "2024/10/17",
         seq_no: "00098000025",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ009",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 4,
         after_price_type: None,
         after_price: 43,
         unit: 1000,
         after_qty: 2000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 2000,
         before_price: None,
         user_def: "Test47",
         last_time: "10:55:39.567",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 30,
         date: "2024/10/17",
         seq_no: "00098000025",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ009",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 30,
         after_price_type: None,
         after_price: 43,
         unit: 1000,
         after_qty: 0,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 2000,
         before_price: None,
         user_def: "Test47",
         last_time: "10:55:39.568",
         details: None,
         error_message: None,
     }
     ========

     ```

   * 函式回傳 (return):

     ```py
     Result {
         is_success: True,
         message: None,
         data: OrderResult {
             function_type: 30,
             date: "2024/10/17",
             seq_no: "00098000025",
             branch_no: "20603",
             account: "9809789",
             order_no: "KQ009",
             asset_type: 0,
             market: "TAIEX",
             market_type: Common,
             stock_no: "1102",
             buy_sell: Buy,
             price_type: Limit,
             price: 43,
             quantity: 5000,
             time_in_force: ROD,
             order_type: Stock,
             is_pre_order: false,
             status: 30,
             after_price_type: None,
             after_price: 43,
             unit: 1000,
             after_qty: 0,
             filled_qty: 0,
             filled_money: 0,
             before_qty: 2000,
             before_price: None,
             user_def: "Test47",
             last_time: "10:55:39.569",
             details: None,
             error_message: None,
         }
     }

     ```

2. 刪單失敗

   * 主動回報（一筆）:

     刪單 function\_type 30，回報刪單失敗 status 39 （由系統直接擋回請求，因此無 status 4）

     ```py
     ==改單主動回報==
     Code [115]證券委託目前狀態部分成交單已不允許取消交易
     內容 OrderResult {
         function_type: 30,
         date: "2024/10/17",
         seq_no: "00090000080",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ00A",
         asset_type: 1,
         market: None,
         market_type: UnSupported,
         stock_no: "1102",
         buy_sell: None,
         price_type: None,
         price: None,
         quantity: None,
         time_in_force: None,
         order_type: None,
         is_pre_order: false,
         status: 39,
         after_price_type: None,
         after_price: None,
         unit: None,
         after_qty: None,
         filled_qty: None,
         filled_money: None,
         before_qty: None,
         before_price: None,
         user_def: None,
         last_time: "10:59:25.982",
         details: None,
         error_message: "證券委託目前狀態部分成交單已不允許取消交易",
     }
     ========

     ```

   * 函式回傳 (return):

     刪單失敗，無委託單資料回傳

     ```py
     Result {
         is_success: False,
         message: 證券委託目前狀態部分成交單已不允許取消交易,
         data: None
     }

     ```

###### 非阻塞[​](#非阻塞-2 "Direct link to 非阻塞")

1. 刪單成功

   * 主動回報（兩筆）:

     刪單 function\_type 30，回報第一筆為系統將執行請求（status 4）；第二筆刪單成功 status 30 （若已有部分成交 status 40）

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 30,
         date: "2024/10/17",
         seq_no: "00098000023",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ008",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 4,
         after_price_type: None,
         after_price: 43,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 5000,
         before_price: None,
         user_def: "Test45",
         last_time: "11:00:06.415",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 30,
         date: "2024/10/17",
         seq_no: "00098000023",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ008",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 30,
         after_price_type: None,
         after_price: 43,
         unit: 1000,
         after_qty: 0,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 5000,
         before_price: None,
         user_def: "Test45",
         last_time: "11:00:06.416",
         details: None,
         error_message: None,
     }
     ========

     ```

   * 函式回傳 (return):

     因為使用非阻塞，函式回傳ACK (status 4)

     ```py
     Result {
         is_success: True,
         message: None,
         data: OrderResult {
             function_type: 30,
             date: "2024/10/17",
             seq_no: "00098000023",
             branch_no: "20603",
             account: "9809789",
             order_no: "KQ008",
             asset_type: 0,
             market: "TAIEX",
             market_type: Common,
             stock_no: "1102",
             buy_sell: Buy,
             price_type: Limit,
             price: 43,
             quantity: 5000,
             time_in_force: ROD,
             order_type: Stock,
             is_pre_order: false,
             status: 4,
             after_price_type: None,
             after_price: 43,
             unit: 1000,
             after_qty: 5000,
             filled_qty: 0,
             filled_money: 0,
             before_qty: 5000,
             before_price: None,
             user_def: "Test45",
             last_time: "11:00:06.414",
             details: None,
             error_message: None,
         }
     }

     ```

2. 刪單失敗

   * 主動回報（一筆）:

     刪單 function\_type 30，回報刪單失敗 status 39 （由系統直接擋回請求，因此無 status 4）

     ```py
     ==改單主動回報==
     Code [115]證券委託目前狀態取消單已不允許取消交易
     內容 OrderResult {
         function_type: 30,
         date: "2024/10/17",
         seq_no: "00098000023",
         branch_no: "20603",
         account: "9809789",
         order_no: "KQ008",
         asset_type: 1,
         market: None,
         market_type: UnSupported,
         stock_no: "1102",
         buy_sell: None,
         price_type: None,
         price: None,
         quantity: None,
         time_in_force: None,
         order_type: None,
         is_pre_order: false,
         status: 39,
         after_price_type: None,
         after_price: None,
         unit: None,
         after_qty: None,
         filled_qty: None,
         filled_money: None,
         before_qty: None,
         before_price: None,
         user_def: None,
         last_time: "11:00:35.797",
         details: None,
         error_message: "證券委託目前狀態取消單已不允許取消交易",
     }
     ========

     ```

   * 函式回傳 (return):

     刪單失敗，無委託單資料回傳

     ```py
     Result {
         is_success: False,
         message: 證券委託目前狀態取消單已不允許取消交易,
         data: None
     }

     ```

#### 批次下單[​](#批次下單 "Direct link to 批次下單")

**批次功能由系統收到請求後以多線程 (threads) 方式運作**

後續批次範例以下委託單列表為例:

```py
[Order {
     buy_sell: Buy,
     symbol: "1102",
     price: "42",
     quantity: 2000,
     market_type: Common,
     price_type: Limit,
     time_in_force: ROD,
     order_type: Stock,
     user_def: "batch1",
 },
 Order {
     buy_sell: Buy,
     symbol: "1101",
     price: "32",
     quantity: 1000,
     market_type: Common,
     price_type: Limit,
     time_in_force: ROD,
     order_type: Stock,
     user_def: "batch2",
 },
 Order {
     buy_sell: Buy,
     symbol: "2330",
     price: None,
     quantity: 1000,
     market_type: Common,
     price_type: Market,
     time_in_force: ROD,
     order_type: Stock,
     user_def: "batch3",
 }]

```

##### 新委託單下單[​](#新委託單下單 "Direct link to 新委託單下單")

* 主動回報（六筆）:

  包含三筆後台準備送出委託單回報（status 8）；三筆下單成功確認（status 10）

  ```py
  ==下單主動回報==
  Code None
  內容 OrderResult {
      function_type: 0,
      date: "2024/10/18",
      seq_no: "00098000010",
      branch_no: "20706",
      account: "9809268",
      order_no: None,
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1102",
      buy_sell: Buy,
      price_type: Limit,
      price: 42,
      quantity: 2000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: true,
      status: 8,
      after_price_type: Limit,
      after_price: 42,
      unit: 1000,
      after_qty: 2000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 0,
      before_price: 42,
      user_def: "batch1",
      last_time: "16:21:19",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==下單主動回報==
  Code None
  內容 OrderResult {
      function_type: 0,
      date: "2024/10/18",
      seq_no: "00098000011",
      branch_no: "20706",
      account: "9809268",
      order_no: None,
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1101",
      buy_sell: Buy,
      price_type: Limit,
      price: 32,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: true,
      status: 8,
      after_price_type: Limit,
      after_price: 32,
      unit: 1000,
      after_qty: 1000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 0,
      before_price: 32,
      user_def: "batch2",
      last_time: "16:21:19",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==下單主動回報==
  Code None
  內容 OrderResult {
      function_type: 0,
      date: "2024/10/18",
      seq_no: "00098000012",
      branch_no: "20706",
      account: "9809268",
      order_no: None,
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "2330",
      buy_sell: Buy,
      price_type: Market,
      price: None,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: true,
      status: 8,
      after_price_type: Market,
      after_price: None,
      unit: 1000,
      after_qty: 1000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 0,
      before_price: None,
      user_def: "batch3",
      last_time: "16:21:19",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==下單主動回報==
  Code None
  內容 OrderResult {
      function_type: 10,
      date: "2024/10/18",
      seq_no: "00098000011",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0006",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1101",
      buy_sell: Buy,
      price_type: Limit,
      price: 32,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 10,
      after_price_type: Limit,
      after_price: 32,
      unit: 1000,
      after_qty: 1000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 0,
      before_price: 32,
      user_def: "batch2",
      last_time: "16:21:18.962",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==下單主動回報==
  Code None
  內容 OrderResult {
      function_type: 10,
      date: "2024/10/18",
      seq_no: "00098000010",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0005",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1102",
      buy_sell: Buy,
      price_type: Limit,
      price: 42,
      quantity: 2000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 10,
      after_price_type: Limit,
      after_price: 42,
      unit: 1000,
      after_qty: 2000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 0,
      before_price: 42,
      user_def: "batch1",
      last_time: "16:21:18.963",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==下單主動回報==
  Code None
  內容 OrderResult {
      function_type: 10,
      date: "2024/10/18",
      seq_no: "00098000012",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0007",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "2330",
      buy_sell: Buy,
      price_type: Market,
      price: 0,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 10,
      after_price_type: Market,
      after_price: 0,
      unit: 1000,
      after_qty: 1000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 0,
      before_price: 0,
      user_def: "batch3",
      last_time: "16:21:19",
      details: None,
      error_message: None,
  }
  ========

  ```

* 函式回傳 (return):

  批次委託新單，回傳值皆為 ACK (status 4)，並無委託書號 (order\_no)，後續可用流水號 (seq\_no) 進行比對

  ```py
  Result {
      is_success: True,
      message: None,
      data: [OrderResult {
          function_type: 0,
          date: "2024/10/18",
          seq_no: "00098000010",
          branch_no: "20706",
          account: "9809268",
          order_no: None,
          asset_type: 0,
          market: "TAIEX",
          market_type: Common,
          stock_no: "1102",
          buy_sell: Buy,
          price_type: Limit,
          price: 42,
          quantity: 2000,
          time_in_force: ROD,
          order_type: Stock,
          is_pre_order: true,
          status: 4,
          after_price_type: Limit,
          after_price: 42,
          unit: 1000,
          after_qty: 2000,
          filled_qty: 0,
          filled_money: 0,
          before_qty: 0,
          before_price: 42,
          user_def: "batch1",
          last_time: "16:21:18.999",
          details: None,
          error_message: None,
      }, OrderResult {
          function_type: 0,
          date: "2024/10/18",
          seq_no: "00098000011",
          branch_no: "20706",
          account: "9809268",
          order_no: None,
          asset_type: None,
          market: None,
          market_type: Common,
          stock_no: "1101",
          buy_sell: Buy,
          price_type: Limit,
          price: 32,
          quantity: 1000,
          time_in_force: ROD,
          order_type: Stock,
          is_pre_order: true,
          status: 4,
          after_price_type: Limit,
          after_price: 32,
          unit: None,
          after_qty: 1000,
          filled_qty: 0,
          filled_money: 0,
          before_qty: 0,
          before_price: 32,
          user_def: "batch2",
          last_time: "16:21:18.999",
          details: None,
          error_message: None,
      }, OrderResult {
          function_type: 0,
          date: "2024/10/18",
          seq_no: "00098000012",
          branch_no: "20706",
          account: "9809268",
          order_no: None,
          asset_type: None,
          market: None,
          market_type: Common,
          stock_no: "2330",
          buy_sell: Buy,
          price_type: Market,
          price: None,
          quantity: 1000,
          time_in_force: ROD,
          order_type: Stock,
          is_pre_order: true,
          status: 4,
          after_price_type: Market,
          after_price: None,
          unit: None,
          after_qty: 1000,
          filled_qty: 0,
          filled_money: 0,
          before_qty: 0,
          before_price: None,
          user_def: "batch3",
          last_time: "16:21:18.999",
          details: None,
          error_message: None,
      }]
  }

  ```

##### 修改委託單價格[​](#修改委託單價格-1 "Direct link to 修改委託單價格")

**委託單列表中不可含市價單，若含市價單則整批請求不執行，並回傳錯誤訊息**

* 主動回報（四筆，此處無納入市價委託單）:

  兩筆 ACK （status 4）；兩筆確認 status 10

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 15,
      date: "2024/10/18",
      seq_no: "00098000011",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0006",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1101",
      buy_sell: Buy,
      price_type: Limit,
      price: 32,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 4,
      after_price_type: Limit,
      after_price: 32,
      unit: 1000,
      after_qty: 1000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: None,
      before_price: 32,
      user_def: "batch2",
      last_time: "16:23:11.710",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 15,
      date: "2024/10/18",
      seq_no: "00098000010",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0005",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1102",
      buy_sell: Buy,
      price_type: Limit,
      price: 42,
      quantity: 2000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 4,
      after_price_type: Limit,
      after_price: 42,
      unit: 1000,
      after_qty: 2000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: None,
      before_price: 42,
      user_def: "batch1",
      last_time: "16:23:11.710",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 15,
      date: "2024/10/18",
      seq_no: "00098000011",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0006",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1101",
      buy_sell: Buy,
      price_type: Limit,
      price: 32,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 10,
      after_price_type: Limit,
      after_price: 33,
      unit: 1000,
      after_qty: 1000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: None,
      before_price: 32,
      user_def: "batch2",
      last_time: "16:23:11.673",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 15,
      date: "2024/10/18",
      seq_no: "00098000010",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0005",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1102",
      buy_sell: Buy,
      price_type: Limit,
      price: 42,
      quantity: 2000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 10,
      after_price_type: Limit,
      after_price: 43,
      unit: 1000,
      after_qty: 2000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: None,
      before_price: 42,
      user_def: "batch1",
      last_time: "16:23:11.683",
      details: None,
      error_message: None,
  }
  ========

  ```

* 函式回傳 (return):

  批次委託回傳值皆為 ACK (status 4)

  ```py
  Result {
      is_success: True,
      message: None,
      data: [OrderResult {
          function_type: 15,
          date: "2024/10/18",
          seq_no: "00098000011",
          branch_no: "20706",
          account: "9809268",
          order_no: "x0006",
          asset_type: 0,
          market: "TAIEX",
          market_type: Common,
          stock_no: "1101",
          buy_sell: Buy,
          price_type: Limit,
          price: 32,
          quantity: 1000,
          time_in_force: ROD,
          order_type: Stock,
          is_pre_order: false,
          status: 4,
          after_price_type: Limit,
          after_price: 33,
          unit: 1000,
          after_qty: 1000,
          filled_qty: 0,
          filled_money: 0,
          before_qty: None,
          before_price: 32,
          user_def: "batch2",
          last_time: "16:23:11.708",
          details: None,
          error_message: None,
      }, OrderResult {
          function_type: 15,
          date: "2024/10/18",
          seq_no: "00098000010",
          branch_no: "20706",
          account: "9809268",
          order_no: "x0005",
          asset_type: 0,
          market: "TAIEX",
          market_type: Common,
          stock_no: "1102",
          buy_sell: Buy,
          price_type: Limit,
          price: 42,
          quantity: 2000,
          time_in_force: ROD,
          order_type: Stock,
          is_pre_order: false,
          status: 4,
          after_price_type: Limit,
          after_price: 43,
          unit: 1000,
          after_qty: 2000,
          filled_qty: 0,
          filled_money: 0,
          before_qty: None,
          before_price: 42,
          user_def: "batch1",
          last_time: "16:23:11.708",
          details: None,
          error_message: None,
      }]
  }

  ```

##### 修改數量及刪單[​](#修改數量及刪單 "Direct link to 修改數量及刪單")

此處部分委託單改量為 0，視同刪單；若使用刪單功能，則主動回報及函式回傳值與改量為 0 同

* 主動回報（六筆）:

  三筆 ACK （status 4）；改量 function\_type 20, 改量成功 status 10; 刪單 function\_type 30, 刪單成功 status 30

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 30,
      date: "2024/10/18",
      seq_no: "00098000011",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0006",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1101",
      buy_sell: Buy,
      price_type: Limit,
      price: 32,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 4,
      after_price_type: None,
      after_price: 33,
      unit: 1000,
      after_qty: 1000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 1000,
      before_price: None,
      user_def: "batch2",
      last_time: "16:24:16.494",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 30,
      date: "2024/10/18",
      seq_no: "00098000012",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0007",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "2330",
      buy_sell: Buy,
      price_type: Market,
      price: 0,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 4,
      after_price_type: None,
      after_price: 0,
      unit: 1000,
      after_qty: 1000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 1000,
      before_price: None,
      user_def: "batch3",
      last_time: "16:24:16.494",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 20,
      date: "2024/10/18",
      seq_no: "00098000010",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0005",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1102",
      buy_sell: Buy,
      price_type: Limit,
      price: 42,
      quantity: 2000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 4,
      after_price_type: None,
      after_price: 43,
      unit: 1000,
      after_qty: 2000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 2000,
      before_price: None,
      user_def: "batch1",
      last_time: "16:24:16.494",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 20,
      date: "2024/10/18",
      seq_no: "00098000010",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0005",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1102",
      buy_sell: Buy,
      price_type: Limit,
      price: 42,
      quantity: 2000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 10,
      after_price_type: None,
      after_price: 43,
      unit: 1000,
      after_qty: 1000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 2000,
      before_price: None,
      user_def: "batch1",
      last_time: "16:24:16.554",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 30,
      date: "2024/10/18",
      seq_no: "00098000011",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0006",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1101",
      buy_sell: Buy,
      price_type: Limit,
      price: 32,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 30,
      after_price_type: None,
      after_price: 33,
      unit: 1000,
      after_qty: 0,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 1000,
      before_price: None,
      user_def: "batch2",
      last_time: "16:24:16.558",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 30,
      date: "2024/10/18",
      seq_no: "00098000012",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0007",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "2330",
      buy_sell: Buy,
      price_type: Market,
      price: 0,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 30,
      after_price_type: None,
      after_price: 0,
      unit: 1000,
      after_qty: 0,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 1000,
      before_price: None,
      user_def: "batch3",
      last_time: "16:24:16.558",
      details: None,
      error_message: None,
  }
  ========

  ```

* 函式回傳 (return):

  批次委託回傳值皆為 ACK (status 4)

  ```py
  Result {
      is_success: True,
      message: None,
      data: [OrderResult {
          function_type: 30,
          date: "2024/10/18",
          seq_no: "00098000012",
          branch_no: "20706",
          account: "9809268",
          order_no: "x0007",
          asset_type: 0,
          market: "TAIEX",
          market_type: Common,
          stock_no: "2330",
          buy_sell: Buy,
          price_type: Market,
          price: 0,
          quantity: 1000,
          time_in_force: ROD,
          order_type: Stock,
          is_pre_order: false,
          status: 4,
          after_price_type: None,
          after_price: 0,
          unit: 1000,
          after_qty: 0,
          filled_qty: 0,
          filled_money: 0,
          before_qty: 1000,
          before_price: None,
          user_def: "batch3",
          last_time: "16:24:16.492",
          details: None,
          error_message: None,
      }, OrderResult {
          function_type: 30,
          date: "2024/10/18",
          seq_no: "00098000011",
          branch_no: "20706",
          account: "9809268",
          order_no: "x0006",
          asset_type: 0,
          market: "TAIEX",
          market_type: Common,
          stock_no: "1101",
          buy_sell: Buy,
          price_type: Limit,
          price: 32,
          quantity: 1000,
          time_in_force: ROD,
          order_type: Stock,
          is_pre_order: false,
          status: 4,
          after_price_type: None,
          after_price: 33,
          unit: 1000,
          after_qty: 0,
          filled_qty: 0,
          filled_money: 0,
          before_qty: 1000,
          before_price: None,
          user_def: "batch2",
          last_time: "16:24:16.492",
          details: None,
          error_message: None,
      }, OrderResult {
          function_type: 20,
          date: "2024/10/18",
          seq_no: "00098000010",
          branch_no: "20706",
          account: "9809268",
          order_no: "x0005",
          asset_type: 0,
          market: "TAIEX",
          market_type: Common,
          stock_no: "1102",
          buy_sell: Buy,
          price_type: Limit,
          price: 42,
          quantity: 2000,
          time_in_force: ROD,
          order_type: Stock,
          is_pre_order: false,
          status: 4,
          after_price_type: None,
          after_price: 43,
          unit: 1000,
          after_qty: 1000,
          filled_qty: 0,
          filled_money: 0,
          before_qty: 2000,
          before_price: None,
          user_def: "batch1",
          last_time: "16:24:16.492",
          details: None,
          error_message: None,
      }]
  }

  ```

#### 成交主動回報[​](#成交主動回報 "Direct link to 成交主動回報")

成交主動回報包含委託書號 (order\_no) 及流水號 (seq\_no) 以供比對

```py
==成交主動回報==
Code None
FilledData {
    date: "2024/10/17",
    branch_no: "20603",
    account: "9809789",
    order_no: "KQ00B",
    stock_no: "1102",
    buy_sell: Sell,
    order_type: Stock,
    seq_no: "00090000081",
    filled_no: "00000000002",
    filled_avg_price: 45.0,
    filled_qty: 2000,
    filled_price: 45.0,
    filled_time: "10:57:49.331",
    user_def: None,
}
========

```


---

### 主動回報範例（標準版）

以下列舉主動回報範例，包含阻塞式 (non-async, unblock=False) 及 非阻塞式 (async, unblock=True) 功能

（ 關於阻塞式與非阻塞式功能，請參考 [非阻塞下單](https://www.fbs.com.tw/TradeAPI/docs/trading/guide/advance/asyn_order.txt) ）

caution

此文件僅提供常見使用場景對應範例，不保證包含所有例外情況

#### 單筆下單[​](#單筆下單 "Direct link to 單筆下單")

##### 單筆新單[​](#單筆新單 "Direct link to 單筆新單")

###### 阻塞式 (non-async, unblock=False)[​](#阻塞式-non-async-unblockfalse "Direct link to 阻塞式 (non-async, unblock=False)")

1. 下單成功

   * 主動回報（一筆）:

     下單成功，status 顯示為 10

     ```py
     ==下單主動回報==
     Code None
     內容 OrderResult {
         function_type: 10,
         date: "2024/10/17",
         seq_no: "00000394159",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0002",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 10,
         after_price_type: Limit,
         after_price: 43,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 0,
         before_price: 43,
         user_def: "Test50",
         last_time: "11:24:40.378",
         details: None,
         error_message: None,
     }
     ========

     ```

   * 函式回傳值 (return):

     ```py
     Result {
         is_success: True,
         message: None,
         data: OrderResult {
             function_type: 0,
             date: "2024/10/17",
             seq_no: "00000394159",
             branch_no: "20706",
             account: "9809268",
             order_no: "x0002",
             asset_type: 0,
             market: "TAIEX",
             market_type: Common,
             stock_no: "1102",
             buy_sell: Buy,
             price_type: Limit,
             price: 43,
             quantity: 5000,
             time_in_force: ROD,
             order_type: Stock,
             is_pre_order: false,
             status: 10,
             after_price_type: Limit,
             after_price: 43,
             unit: 1000,
             after_qty: 5000,
             filled_qty: 0,
             filled_money: 0,
             before_qty: 0,
             before_price: 43,
             user_def: "Test50",
             last_time: "11:24:40.378",
             details: None,
             error_message: None,
         }
     }

     ```

2. 下單失敗

   * 主動回報（一筆）:

     下單異常，status 顯示為 90

     ```py
     ==下單主動回報==
     Code [4385715]單價輸入錯誤[4385715]
     內容 OrderResult {
         function_type: 90,
         date: "2024/10/17",
         seq_no: "00000394160",
         branch_no: "20706",
         account: "9809268",
         order_no: None,
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 430,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 90,
         after_price_type: Limit,
         after_price: 430,
         unit: 1000,
         after_qty: 0,
         filled_qty: None,
         filled_money: None,
         before_qty: 0,
         before_price: 430,
         user_def: "Test25",
         last_time: "11:25:25.670",
         details: None,
         error_message: "單價輸入錯誤[4385715]",
     }
     ========

     ```

   * 函式回傳值 (return):

     下單失敗，無委託單資料回傳

     ```py
     Result {
         is_success: False,
         message: 單價輸入錯誤[4385715],
         data: None
     }

     ```

###### 非阻塞 (async, unblock=True)[​](#非阻塞-async-unblocktrue "Direct link to 非阻塞 (async, unblock=True)")

1. 下單成功

   * 主動回報（三筆）:

     第一筆為ACK，表示系統將執行請求（status 4），第二筆表示後台準備送單（status 8），此時皆無 order\_no（尚未確認下單成功），可用 seq\_no 進行後續比對；最後確認下單狀態（成功為 status 10）

     ```py
     ==下單主動回報==
     Code None
     內容 OrderResult {
         function_type: 0,
         date: "2024/10/17",
         seq_no: "00098000031",
         branch_no: "20706",
         account: "9809268",
         order_no: None,
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: true,
         status: 4,
         after_price_type: Limit,
         after_price: 43,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 0,
         before_price: 43,
         user_def: "Test70",
         last_time: "11:25:54.188",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==下單主動回報==
     Code None
     內容 OrderResult {
         function_type: 10,
         date: "2024/10/17",
         seq_no: "00098000031",
         branch_no: "20706",
         account: "9809268",
         order_no: None,
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 8,
         after_price_type: Limit,
         after_price: 43,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 0,
         before_price: 43,
         user_def: "Test70",
         last_time: "11:25:54.189",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==下單主動回報==
     Code None
     內容 OrderResult {
         function_type: 10,
         date: "2024/10/17",
         seq_no: "00098000031",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0003",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 10,
         after_price_type: Limit,
         after_price: 43,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 0,
         before_price: 43,
         user_def: "Test70",
         last_time: "11:25:54.163",
         details: None,
         error_message: None,
     }
     ========

     ```

   * 函式回傳值 (return):

     因使用非阻塞，函式收到 ACK 即回傳 (status 4)

     ```py
     Result {
         is_success: True,
         message: None,
         data: OrderResult {
             function_type: 0,
             date: "2024/10/17",
             seq_no: "00098000031",
             branch_no: "20706",
             account: "9809268",
             order_no: None,
             asset_type: 0,
             market: "TAIEX",
             market_type: Common,
             stock_no: "1102",
             buy_sell: Buy,
             price_type: Limit,
             price: 43,
             quantity: 5000,
             time_in_force: ROD,
             order_type: Stock,
             is_pre_order: true,
             status: 4,
             after_price_type: Limit,
             after_price: 43,
             unit: 1000,
             after_qty: 5000,
             filled_qty: 0,
             filled_money: 0,
             before_qty: 0,
             before_price: 43,
             user_def: "Test70",
             last_time: "11:25:54.188",
             details: None,
             error_message: None,
         }
     }

     ```

2. 下單失敗

   * 主動回報（三筆）:

     第一筆為ACK，表示系統將執行請求（status 4），第二筆表示後台準備送單（status 8），此時皆無 order\_no（尚未確認下單成功），可用 seq\_no 進行後續比對；最後確認下單狀態（異常為 status 90）

     ```py
     ==下單主動回報==
     Code None
     內容 OrderResult {
         function_type: 0,
         date: "2024/10/17",
         seq_no: "00098000032",
         branch_no: "20706",
         account: "9809268",
         order_no: None,
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 20,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: true,
         status: 4,
         after_price_type: Limit,
         after_price: 20,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 0,
         before_price: 20,
         user_def: "Test64",
         last_time: "11:26:59.651",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==下單主動回報==
     Code None
     內容 OrderResult {
         function_type: 10,
         date: "2024/10/17",
         seq_no: "00098000032",
         branch_no: "20706",
         account: "9809268",
         order_no: None,
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 20,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 8,
         after_price_type: Limit,
         after_price: 20,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 0,
         before_price: 20,
         user_def: "Test64",
         last_time: "11:26:59.652",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==下單主動回報==
     Code [4385715]單價輸入錯誤[4385715]
     內容 OrderResult {
         function_type: 90,
         date: "2024/10/17",
         seq_no: "00098000032",
         branch_no: "20706",
         account: "9809268",
         order_no: None,
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 20,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 90,
         after_price_type: Limit,
         after_price: 20,
         unit: 1000,
         after_qty: 0,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 0,
         before_price: 20,
         user_def: "Test64",
         last_time: "11:26:59.658",
         details: None,
         error_message: "單價輸入錯誤[4385715]",
     }
     ========

     ```

   * 函式回傳值 (return):

     因使用非阻塞，函式收到 ACK 即回傳 (status 4)

     ```py
     Result {
         is_success: True,
         message: None,
         data: OrderResult {
             function_type: 0,
             date: "2024/10/17",
             seq_no: "00098000032",
             branch_no: "20706",
             account: "9809268",
             order_no: None,
             asset_type: 0,
             market: "TAIEX",
             market_type: Common,
             stock_no: "1102",
             buy_sell: Buy,
             price_type: Limit,
             price: 20,
             quantity: 5000,
             time_in_force: ROD,
             order_type: Stock,
             is_pre_order: true,
             status: 4,
             after_price_type: Limit,
             after_price: 20,
             unit: 1000,
             after_qty: 5000,
             filled_qty: 0,
             filled_money: 0,
             before_qty: 0,
             before_price: 20,
             user_def: "Test64",
             last_time: "11:26:59.651",
             details: None,
             error_message: None,
         }
     }

     ```

##### 修改委託單價格[​](#修改委託單價格 "Direct link to 修改委託單價格")

info

改單無論使用阻塞模式或非阻塞模式，若由系統擋單（例如改價同原委託價），無 status 4 回報（表示系統將執行請求）；若由後台擋單（例如超過漲跌停價），則有 status 4 回報

阻塞模式與非阻塞模式差異為，若有 status 4 回報時（系統向後送單），非阻塞模式之函式收到該筆回報即回傳，而阻塞模式之函式將等到最終成功或失敗確認才回傳

###### 阻塞[​](#阻塞 "Direct link to 阻塞")

1. 改價成功

   * 主動回報（兩筆）:

     改價 function\_type 15，回報第一筆為系統將執行請求（status 4）；第二筆改價成功 status 10

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 15,
         date: "2024/10/17",
         seq_no: "00098000031",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0003",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 4,
         after_price_type: Limit,
         after_price: 43,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: None,
         before_price: 43,
         user_def: "Test70",
         last_time: "11:27:47.512",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 15,
         date: "2024/10/17",
         seq_no: "00098000031",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0003",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 10,
         after_price_type: Limit,
         after_price: 42,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: None,
         before_price: 43,
         user_def: "Test70",
         last_time: "11:27:47.407",
         details: None,
         error_message: None,
     }
     ========

     ```

   * 函式回傳 (return):

     ```py
     Result {
         is_success: True,
         message: None,
         data: OrderResult {
             function_type: 15,
             date: "2024/10/17",
             seq_no: "00098000031",
             branch_no: "20706",
             account: "9809268",
             order_no: "x0003",
             asset_type: 0,
             market: "TAIEX",
             market_type: Common,
             stock_no: "1102",
             buy_sell: Buy,
             price_type: Limit,
             price: 43,
             quantity: 5000,
             time_in_force: ROD,
             order_type: Stock,
             is_pre_order: false,
             status: 10,
             after_price_type: Limit,
             after_price: 42,
             unit: 1000,
             after_qty: 5000,
             filled_qty: 0,
             filled_money: 0,
             before_qty: 5000,
             before_price: 43,
             user_def: "Test70",
             last_time: "11:27:47.407",
             details: None,
             error_message: None,
         }
     }

     ```

2. 改價失敗

   * 主動回報（兩筆，後台擋單）:

     改價 function\_type 15，回報第一筆為系統將執行請求（status 4）；第二筆改價失敗 status 19

     info

     此案例由後台擋單，因此有第一筆回報 status 4

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 15,
         date: "2024/10/17",
         seq_no: "00098000031",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0003",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 4,
         after_price_type: Limit,
         after_price: 42,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: None,
         before_price: 42,
         user_def: "Test70",
         last_time: "11:28:46.753",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==改單主動回報==
     Code [4385715]單價輸入錯誤[4385715]
     內容 OrderResult {
         function_type: 15,
         date: "2024/10/17",
         seq_no: "00098000031",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0003",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 19,
         after_price_type: Limit,
         after_price: 42,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: None,
         before_price: 42,
         user_def: "Test70",
         last_time: "11:28:46.754",
         details: None,
         error_message: "單價輸入錯誤[4385715]",
     }
     ========

     ```

   * 函式回傳 (return):

     改價失敗，無委託單資料回傳

     ```py
     Result {
         is_success: False,
         message: 單價輸入錯誤[4385715],
         data: None
     }   

     ```

###### 非阻塞[​](#非阻塞 "Direct link to 非阻塞")

1. 改價成功

   * 主動回報（兩筆）:

     改價 function\_type 15，回報第一筆為系統將執行請求（status 4）；第二筆改價成功 status 10

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 15,
         date: "2024/10/17",
         seq_no: "00098000031",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0003",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 4,
         after_price_type: Limit,
         after_price: 42,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: None,
         before_price: 42,
         user_def: "Test70",
         last_time: "11:29:43.593",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 15,
         date: "2024/10/17",
         seq_no: "00098000031",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0003",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 10,
         after_price_type: Limit,
         after_price: 43,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: None,
         before_price: 42,
         user_def: "Test70",
         last_time: "11:29:43.682",
         details: None,
         error_message: None,
     }
     ========

     ```

   * 函式回傳 (return):

     因使用非阻塞，函式收到 ACK 即回傳 (status 4)

     ```py
     Result {
         is_success: True,
         message: None,
         data: OrderResult {
             function_type: 15,
             date: "2024/10/17",
             seq_no: "00098000031",
             branch_no: "20706",
             account: "9809268",
             order_no: "x0003",
             asset_type: 0,
             market: "TAIEX",
             market_type: Common,
             stock_no: "1102",
             buy_sell: Buy,
             price_type: Limit,
             price: 43,
             quantity: 5000,
             time_in_force: ROD,
             order_type: Stock,
             is_pre_order: false,
             status: 4,
             after_price_type: Limit,
             after_price: 42,
             unit: 1000,
             after_qty: 5000,
             filled_qty: 0,
             filled_money: 0,
             before_qty: None,
             before_price: 42,
             user_def: "Test70",
             last_time: "11:29:43.592",
             details: None,
             error_message: None,
         }
     }

     ```

2. 改價失敗

   * 主動回報（一筆）:

     改價 function\_type 15，改價失敗 status 19

     info

     此案例由系統擋單，因此**無** status 4 回報

     ```py
     ==改單主動回報==
     Code [154]證券改價限價格格和原價格相同==>[00098000031]
     內容 OrderResult {
         function_type: 15,
         date: "2024/10/17",
         seq_no: "00098000031",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0003",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 19,
         after_price_type: Limit,
         after_price: 43,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: None,
         before_price: None,
         user_def: "Test70",
         last_time: "11:30:20.330",
         details: None,
         error_message: "證券改價限價格格和原價格相同==>[00098000031]",
     }
     ========

     ```

   * 函式回傳 (return):

     改價失敗，無委託單資料回傳

     ```py
     response: Result {
         is_success: False,
         message: 證券改價限價格格和原價格相同==>[00098000031],
         data: None
     }

     ```

##### 修改委託單數量[​](#修改委託單數量 "Direct link to 修改委託單數量")

info

改單無論使用阻塞模式或非阻塞模式，若由系統擋單（例如改價同原委託價），無 status 4 回報（表示系統將執行請求）；若由後台擋單（例如超過漲跌停價），則有 status 4 回報

阻塞模式與非阻塞模式差異為，若有 status 4 回報時（系統向後送單），非阻塞模式之函式收到該筆回報即回傳，而阻塞模式之函式將等到最終成功或失敗確認才回傳

###### 阻塞[​](#阻塞-1 "Direct link to 阻塞")

1. 改量成功

   * 主動回報（兩筆）:

     改量 function\_type 20，回報第一筆為系統將執行請求（status 4）；第二筆改量成功 status 10

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 20,
         date: "2024/10/17",
         seq_no: "00098000031",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0003",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 4,
         after_price_type: None,
         after_price: 43,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 5000,
         before_price: None,
         user_def: "Test70",
         last_time: "11:31:00.211",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 20,
         date: "2024/10/17",
         seq_no: "00098000031",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0003",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 10,
         after_price_type: None,
         after_price: 43,
         unit: 1000,
         after_qty: 3000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 5000,
         before_price: None,
         user_def: "Test70",
         last_time: "11:31:00.332",
         details: None,
         error_message: None,
     }
     ========

     ```

   * 函式回傳 (return):

     ```py
     Result {
         is_success: True,
         message: None,
         data: OrderResult {
             function_type: 20,
             date: "2024/10/17",
             seq_no: "00098000031",
             branch_no: "20706",
             account: "9809268",
             order_no: "x0003",
             asset_type: 0,
             market: "TAIEX",
             market_type: Common,
             stock_no: "1102",
             buy_sell: Buy,
             price_type: Limit,
             price: 43,
             quantity: 5000,
             time_in_force: ROD,
             order_type: Stock,
             is_pre_order: false,
             status: 10,
             after_price_type: Limit,
             after_price: 43,
             unit: 1000,
             after_qty: 3000,
             filled_qty: 0,
             filled_money: 0,
             before_qty: 5000,
             before_price: 43,
             user_def: "Test70",
             last_time: "11:31:00.332",
             details: None,
             error_message: None,
         }
     }

     ```

2. 改量失敗

   * 主動回報（一筆）:

     改量 function\_type 20，失敗回報 status 29（系統擋單，因此無 status 4 回報）

     ```py
     ==改單主動回報==
     Code [164]證券減量使用剩餘數量大於有效單位數量==>[00098000031]
     內容 OrderResult {
         function_type: 20,
         date: "2024/10/17",
         seq_no: "00098000031",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0003",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 29,
         after_price_type: None,
         after_price: 43,
         unit: 1000,
         after_qty: 3000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: None,
         before_price: None,
         user_def: "Test70",
         last_time: "11:31:51.980",
         details: None,
         error_message: "證券減量使用剩餘數量大於有效單位數量==>[00098000031]",
     }
     ========

     ```

   * 函式回傳 (return):

     改量失敗，無委託單資料回傳

     ```py
     Result {
         is_success: False,
         message: 證券減量使用剩餘數量大於有效單位數量==>[00098000031],
         data: None
     }

     ```

###### 非阻塞[​](#非阻塞-1 "Direct link to 非阻塞")

1. 改量成功

   * 主動回報（兩筆）:

     改量 function\_type 20，回報第一筆為系統將執行請求（status 4）；第二筆改量成功 status 10

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 20,
         date: "2024/10/17",
         seq_no: "00098000031",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0003",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 4,
         after_price_type: None,
         after_price: 43,
         unit: 1000,
         after_qty: 3000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 3000,
         before_price: None,
         user_def: "Test70",
         last_time: "11:32:33.047",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 20,
         date: "2024/10/17",
         seq_no: "00098000031",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0003",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 10,
         after_price_type: None,
         after_price: 43,
         unit: 1000,
         after_qty: 1000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 3000,
         before_price: None,
         user_def: "Test70",
         last_time: "11:32:33.215",
         details: None,
         error_message: None,
     }
     ========

     ```

   * 函式回傳 (return):

     因使用非阻塞，函式收到 ACK 即回傳 (status 4)

     ```py
     Result {
     is_success: True,
     message: None,
     data: OrderResult {
             function_type: 20,
             date: "2024/10/17",
             seq_no: "00098000031",
             branch_no: "20706",
             account: "9809268",
             order_no: "x0003",
             asset_type: 0,
             market: "TAIEX",
             market_type: Common,
             stock_no: "1102",
             buy_sell: Buy,
             price_type: Limit,
             price: 43,
             quantity: 5000,
             time_in_force: ROD,
             order_type: Stock,
             is_pre_order: false,
             status: 4,
             after_price_type: None,
             after_price: 43,
             unit: 1000,
             after_qty: 3000,
             filled_qty: 0,
             filled_money: 0,
             before_qty: 3000,
             before_price: None,
             user_def: "Test70",
             last_time: "11:32:33.045",
             details: None,
             error_message: None,
         }
     }

     ```

2. 改量失敗

   * 主動回報（一筆）:

     改量 function\_type 20，失敗回報 status 29 （系統擋單，因此無 status 4 回報）

     ```py
     ==改單主動回報==
     Code [164]證券減量使用剩餘數量大於有效單位數量==>[00098000031]
     內容 OrderResult {
         function_type: 20,
         date: "2024/10/17",
         seq_no: "00098000031",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0003",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 29,
         after_price_type: None,
         after_price: 43,
         unit: 1000,
         after_qty: 1000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: None,
         before_price: None,
         user_def: "Test70",
         last_time: "11:33:18.679",
         details: None,
         error_message: "證券減量使用剩餘數量大於有效單位數量==>[00098000031]",
     }
     ========

     ```

   * 函式回傳 (return):

     由系統擋單改量失敗，無委託單資料回傳

     ```py
     Result {
         is_success: False,
         message: 證券減量使用剩餘數量大於有效單位數量==>[00098000031],
         data: None
     }

     ```

##### 刪除委託單[​](#刪除委託單 "Direct link to 刪除委託單")

caution

若委託單已**部分成交**, 刪單成功狀態代碼為 40, 其餘為 30

info

改單無論使用阻塞模式或非阻塞模式，若由系統擋單（例如改價同原委託價），無 status 4 回報（表示系統將執行請求）；若由後台擋單（例如超過漲跌停價），則有 status 4 回報

阻塞模式與非阻塞模式差異為，若有 status 4 回報時（系統向後送單），非阻塞模式之函式收到 status 4 回報即回傳，而阻塞模式之函式將等到最終成功或失敗確認才回傳

###### 阻塞[​](#阻塞-2 "Direct link to 阻塞")

1. 刪單成功

   * 主動回報（兩筆）:

     刪單 function\_type 30，回報第一筆為系統將執行請求（status 4）；第二筆刪單成功 status 30 （若已有部分成交 status 40）

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 30,
         date: "2024/10/17",
         seq_no: "00098000031",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0003",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 4,
         after_price_type: None,
         after_price: 43,
         unit: 1000,
         after_qty: 1000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 1000,
         before_price: None,
         user_def: "Test70",
         last_time: "11:36:14.561",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 30,
         date: "2024/10/17",
         seq_no: "00098000031",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0003",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 43,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 30,
         after_price_type: None,
         after_price: 43,
         unit: 1000,
         after_qty: 0,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 1000,
         before_price: None,
         user_def: "Test70",
         last_time: "11:36:14.499",
         details: None,
         error_message: None,
     }
     ========

     ```

   * 函式回傳 (return):

     ```py
     Result {
     is_success: True,
     message: None,
     data: OrderResult {
             function_type: 30,
             date: "2024/10/17",
             seq_no: "00098000031",
             branch_no: "20706",
             account: "9809268",
             order_no: "x0003",
             asset_type: 0,
             market: "TAIEX",
             market_type: Common,
             stock_no: "1102",
             buy_sell: Buy,
             price_type: Limit,
             price: 43,
             quantity: 5000,
             time_in_force: ROD,
             order_type: Stock,
             is_pre_order: false,
             status: 30,
             after_price_type: Limit,
             after_price: 43,
             unit: 1000,
             after_qty: 0,
             filled_qty: 0,
             filled_money: 0,
             before_qty: 1000,
             before_price: 43,
             user_def: "Test70",
             last_time: "11:36:14.499",
             details: None,
             error_message: None,
         }
     }

     ```

2. 刪單失敗

   * 主動回報（一筆）:

     刪單 function\_type 30，刪單失敗 status 39（系統擋單，因此無 status 4 回報）

     ```py
     ==改單主動回報==
     Code [115]證券委託目前狀態部分成交單已不允許取消交易
     內容 OrderResult {
         function_type: 30,
         date: "2024/10/17",
         seq_no: "00000394165",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0007",
         asset_type: 1,
         market: None,
         market_type: UnSupported,
         stock_no: "1102",
         buy_sell: None,
         price_type: None,
         price: None,
         quantity: None,
         time_in_force: None,
         order_type: None,
         is_pre_order: false,
         status: 39,
         after_price_type: None,
         after_price: None,
         unit: None,
         after_qty: None,
         filled_qty: None,
         filled_money: None,
         before_qty: None,
         before_price: None,
         user_def: None,
         last_time: "11:39:57.468",
         details: None,
         error_message: "證券委託目前狀態部分成交單已不允許取消交易",
     }
     ========

     ```

   * 函式回傳 (return):

     系統擋單而刪單失敗，無委託單資料回傳

     ```py
     Result {
         is_success: False,
         message: 證券委託目前狀態部分成交單已不允許取消交易,
         data: None
     }

     ```

###### 非阻塞[​](#非阻塞-2 "Direct link to 非阻塞")

1. 刪單成功

   * 主動回報（兩筆）:

     刪單 function\_type 30，回報第一筆為系統將執行請求（status 4）；第二筆刪單成功 status 30 （若已有部分成交 status 40）

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 30,
         date: "2024/10/17",
         seq_no: "00098000033",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0008",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 41,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 4,
         after_price_type: None,
         after_price: 41,
         unit: 1000,
         after_qty: 5000,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 5000,
         before_price: None,
         user_def: "Test4",
         last_time: "11:42:24.961",
         details: None,
         error_message: None,
     }
     ========

     ```

     ```py
     ==改單主動回報==
     Code None
     內容 OrderResult {
         function_type: 30,
         date: "2024/10/17",
         seq_no: "00098000033",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0008",
         asset_type: 0,
         market: "TAIEX",
         market_type: Common,
         stock_no: "1102",
         buy_sell: Buy,
         price_type: Limit,
         price: 41,
         quantity: 5000,
         time_in_force: ROD,
         order_type: Stock,
         is_pre_order: false,
         status: 30,
         after_price_type: None,
         after_price: 41,
         unit: 1000,
         after_qty: 0,
         filled_qty: 0,
         filled_money: 0,
         before_qty: 5000,
         before_price: None,
         user_def: "Test4",
         last_time: "11:42:25.125",
         details: None,
         error_message: None,
     }
     ========

     ```

   * 函式回傳 (return):

     因使用非阻塞，函式收到 ACK 即回傳 (status 4)

     ```py
     Result {
         is_success: True,
         message: None,
         data: OrderResult {
                 function_type: 30,
                 date: "2024/10/17",
                 seq_no: "00098000033",
                 branch_no: "20706",
                 account: "9809268",
                 order_no: "x0008",
                 asset_type: 0,
                 market: "TAIEX",
                 market_type: Common,
                 stock_no: "1102",
                 buy_sell: Buy,
                 price_type: Limit,
                 price: 41,
                 quantity: 5000,
                 time_in_force: ROD,
                 order_type: Stock,
                 is_pre_order: false,
                 status: 4,
                 after_price_type: None,
                 after_price: 41,
                 unit: 1000,
                 after_qty: 5000,
                 filled_qty: 0,
                 filled_money: 0,
                 before_qty: 5000,
                 before_price: None,
                 user_def: "Test4",
                 last_time: "11:42:24.960",
                 details: None,
                 error_message: None,
             }
     }

     ```

2. 刪單失敗

   * 主動回報（一筆）:

     刪單 function\_type 30，回報刪單失敗 status 39 （由系統直接擋回請求，因此無 status 4）

     ```py
     ==改單主動回報==
     Code [115]證券委託目前狀態取消單已不允許取消交易
     內容 OrderResult {
         function_type: 30,
         date: "2024/10/17",
         seq_no: "00098000033",
         branch_no: "20706",
         account: "9809268",
         order_no: "x0008",
         asset_type: 1,
         market: None,
         market_type: UnSupported,
         stock_no: "1102",
         buy_sell: None,
         price_type: None,
         price: None,
         quantity: None,
         time_in_force: None,
         order_type: None,
         is_pre_order: false,
         status: 39,
         after_price_type: None,
         after_price: None,
         unit: None,
         after_qty: None,
         filled_qty: None,
         filled_money: None,
         before_qty: None,
         before_price: None,
         user_def: None,
         last_time: "11:48:06.850",
         details: None,
         error_message: "證券委託目前狀態取消單已不允許取消交易",
     }
     ========

     ```

   * 函式回傳 (return):

     系統擋單而刪單失敗，無委託單資料回傳

     ```py
     Result {
         is_success: False,
         message: 證券委託目前狀態取消單已不允許取消交易,
         data: None
     }

     ```

#### 批次下單[​](#批次下單 "Direct link to 批次下單")

**批次功能由系統收到請求後以多線程 (threads) 方式運作**

後續批次範例以下委託單列表為例:

```py
[Order {
     buy_sell: Buy,
     symbol: "1102",
     price: "42",
     quantity: 2000,
     market_type: Common,
     price_type: Limit,
     time_in_force: ROD,
     order_type: Stock,
     user_def: "batch1",
 },
 Order {
     buy_sell: Buy,
     symbol: "1101",
     price: "32",
     quantity: 1000,
     market_type: Common,
     price_type: Limit,
     time_in_force: ROD,
     order_type: Stock,
     user_def: "batch2",
 },
 Order {
     buy_sell: Buy,
     symbol: "2330",
     price: None,
     quantity: 1000,
     market_type: Common,
     price_type: Market,
     time_in_force: ROD,
     order_type: Stock,
     user_def: "batch3",
 }]

```

##### 新委託單下單[​](#新委託單下單 "Direct link to 新委託單下單")

* 主動回報（六筆）:

  包含三筆後台準備送出委託單回報（status 8）；三筆下單成功確認（status 10）

  ```py
  ==下單主動回報==
  內容 OrderResult {
      function_type: 0,
      date: "2024/10/17",
      seq_no: "00098000041",
      branch_no: "20706",
      account: "9809268",
      order_no: None,
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1101",
      buy_sell: Buy,
      price_type: Limit,
      price: 32,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: true,
      status: 8,
      after_price_type: Limit,
      after_price: 32,
      unit: 1000,
      after_qty: 1000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 0,
      before_price: 32,
      user_def: "batch2",
      last_time: "11:50:24.656",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==下單主動回報==
  內容 OrderResult {
      function_type: 0,
      date: "2024/10/17",
      seq_no: "00098000040",
      branch_no: "20706",
      account: "9809268",
      order_no: None,
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1102",
      buy_sell: Buy,
      price_type: Limit,
      price: 42,
      quantity: 2000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: true,
      status: 8,
      after_price_type: Limit,
      after_price: 42,
      unit: 1000,
      after_qty: 2000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 0,
      before_price: 42,
      user_def: "batch1",
      last_time: "11:50:24.656",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==下單主動回報==
  內容 OrderResult {
      function_type: 0,
      date: "2024/10/17",
      seq_no: "00098000042",
      branch_no: "20706",
      account: "9809268",
      order_no: None,
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "2330",
      buy_sell: Buy,
      price_type: Market,
      price: None,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: true,
      status: 8,
      after_price_type: Market,
      after_price: None,
      unit: 1000,
      after_qty: 1000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 0,
      before_price: None,
      user_def: "batch3",
      last_time: "11:50:24.656",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==下單主動回報==
  內容 OrderResult {
      function_type: 10,
      date: "2024/10/17",
      seq_no: "00098000040",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0014",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1102",
      buy_sell: Buy,
      price_type: Limit,
      price: 42,
      quantity: 2000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 10,
      after_price_type: Limit,
      after_price: 42,
      unit: 1000,
      after_qty: 2000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 0,
      before_price: 42,
      user_def: "batch1",
      last_time: "11:50:24.796",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==下單主動回報==
  內容 OrderResult {
      function_type: 10,
      date: "2024/10/17",
      seq_no: "00098000041",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0013",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1101",
      buy_sell: Buy,
      price_type: Limit,
      price: 32,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 10,
      after_price_type: Limit,
      after_price: 32,
      unit: 1000,
      after_qty: 1000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 0,
      before_price: 32,
      user_def: "batch2",
      last_time: "11:50:24.796",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==下單主動回報==
  內容 OrderResult {
      function_type: 10,
      date: "2024/10/17",
      seq_no: "00098000042",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0015",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "2330",
      buy_sell: Buy,
      price_type: Market,
      price: 0,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 10,
      after_price_type: Market,
      after_price: 0,
      unit: 1000,
      after_qty: 1000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 0,
      before_price: 0,
      user_def: "batch3",
      last_time: "11:50:24.806",
      details: None,
      error_message: None,
  }
  ========

  ```

* 函式回傳 (return):

  批次委託新單，回傳值皆為 ACK (status 4)，並無委託書號 (order\_no)，後續可用流水號 (seq\_no) 進行比對

  ```py
  Result {
      is_success: True,
      message: None,
      data: [OrderResult {
              function_type: 0,
              date: "2024/10/17",
              seq_no: "00098000040",
              branch_no: "20706",
              account: "9809268",
              order_no: None,
              asset_type: 0,
              market: "TAIEX",
              market_type: Common,
              stock_no: "1102",
              buy_sell: Buy,
              price_type: Limit,
              price: 42,
              quantity: 2000,
              time_in_force: ROD,
              order_type: Stock,
              is_pre_order: true,
              status: 4,
              after_price_type: Limit,
              after_price: 42,
              unit: 1000,
              after_qty: 2000,
              filled_qty: 0,
              filled_money: 0,
              before_qty: 0,
              before_price: 42,
              user_def: "batch1",
              last_time: "11:50:24.655",
              details: None,
              error_message: None,
          }, OrderResult {
              function_type: 0,
              date: "2024/10/17",
              seq_no: "00098000041",
              branch_no: "20706",
              account: "9809268",
              order_no: None,
              asset_type: None,
              market: None,
              market_type: Common,
              stock_no: "1101",
              buy_sell: Buy,
              price_type: Limit,
              price: 32,
              quantity: 1000,
              time_in_force: ROD,
              order_type: Stock,
              is_pre_order: true,
              status: 4,
              after_price_type: Limit,
              after_price: 32,
              unit: None,
              after_qty: 1000,
              filled_qty: 0,
              filled_money: 0,
              before_qty: 0,
              before_price: 32,
              user_def: "batch2",
              last_time: "11:50:24.655",
              details: None,
              error_message: None,
          }, OrderResult {
              function_type: 0,
              date: "2024/10/17",
              seq_no: "00098000042",
              branch_no: "20706",
              account: "9809268",
              order_no: None,
              asset_type: None,
              market: None,
              market_type: Common,
              stock_no: "2330",
              buy_sell: Buy,
              price_type: Market,
              price: None,
              quantity: 1000,
              time_in_force: ROD,
              order_type: Stock,
              is_pre_order: true,
              status: 4,
              after_price_type: Market,
              after_price: None,
              unit: None,
              after_qty: 1000,
              filled_qty: 0,
              filled_money: 0,
              before_qty: 0,
              before_price: None,
              user_def: "batch3",
              last_time: "11:50:24.655",
              details: None,
              error_message: None,
          }]
  }

  ```

##### 修改委託單價格[​](#修改委託單價格-1 "Direct link to 修改委託單價格")

**委託單列表中不可含市價單，若含市價單則整批請求不執行，並回傳錯誤訊息**

* 主動回報（四筆，此處無納入市價委託單）:

  兩筆 ACK （status 4）；兩筆確認 status 10

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 15,
      date: "2024/10/17",
      seq_no: "00098000041",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0013",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1101",
      buy_sell: Buy,
      price_type: Limit,
      price: 32,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 4,
      after_price_type: Limit,
      after_price: 32,
      unit: 1000,
      after_qty: 1000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: None,
      before_price: 32,
      user_def: "batch2",
      last_time: "11:52:24.985",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 15,
      date: "2024/10/17",
      seq_no: "00098000040",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0014",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1102",
      buy_sell: Buy,
      price_type: Limit,
      price: 42,
      quantity: 2000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 4,
      after_price_type: Limit,
      after_price: 42,
      unit: 1000,
      after_qty: 2000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: None,
      before_price: 42,
      user_def: "batch1",
      last_time: "11:52:24.985",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 15,
      date: "2024/10/17",
      seq_no: "00098000041",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0013",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1101",
      buy_sell: Buy,
      price_type: Limit,
      price: 32,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 10,
      after_price_type: Limit,
      after_price: 33,
      unit: 1000,
      after_qty: 1000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: None,
      before_price: 32,
      user_def: "batch2",
      last_time: "11:52:25.156",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 15,
      date: "2024/10/17",
      seq_no: "00098000040",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0014",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1102",
      buy_sell: Buy,
      price_type: Limit,
      price: 42,
      quantity: 2000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 10,
      after_price_type: Limit,
      after_price: 43,
      unit: 1000,
      after_qty: 2000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: None,
      before_price: 42,
      user_def: "batch1",
      last_time: "11:52:25.162",
      details: None,
      error_message: None,
  }
  ========

  ```

* 函式回傳 (return):

  批次委託回傳值皆為 ACK (status 4)

  ```py
  Result {
      is_success: True,
      message: None,
      data: [OrderResult {
              function_type: 15,
              date: "2024/10/17",
              seq_no: "00098000041",
              branch_no: "20706",
              account: "9809268",
              order_no: "x0013",
              asset_type: 0,
              market: "TAIEX",
              market_type: Common,
              stock_no: "1101",
              buy_sell: Buy,
              price_type: Limit,
              price: 32,
              quantity: 1000,
              time_in_force: ROD,
              order_type: Stock,
              is_pre_order: false,
              status: 4,
              after_price_type: Limit,
              after_price: 33,
              unit: 1000,
              after_qty: 1000,
              filled_qty: 0,
              filled_money: 0,
              before_qty: None,
              before_price: 32,
              user_def: "batch2",
              last_time: "11:52:24.984",
              details: None,
              error_message: None,
          }, OrderResult {
              function_type: 15,
              date: "2024/10/17",
              seq_no: "00098000040",
              branch_no: "20706",
              account: "9809268",
              order_no: "x0014",
              asset_type: 0,
              market: "TAIEX",
              market_type: Common,
              stock_no: "1102",
              buy_sell: Buy,
              price_type: Limit,
              price: 42,
              quantity: 2000,
              time_in_force: ROD,
              order_type: Stock,
              is_pre_order: false,
              status: 4,
              after_price_type: Limit,
              after_price: 43,
              unit: 1000,
              after_qty: 2000,
              filled_qty: 0,
              filled_money: 0,
              before_qty: None,
              before_price: 42,
              user_def: "batch1",
              last_time: "11:52:24.984",
              details: None,
              error_message: None,
          }]
  }

  ```

##### 修改數量及刪單[​](#修改數量及刪單 "Direct link to 修改數量及刪單")

此處部分委託單改量為 0，視同刪單；若使用刪單功能，則主動回報及函式回傳值與改量為 0 同

* 主動回報（六筆）:

  三筆 ACK （status 4）；改量 function\_type 20, 改量成功 status 10; 刪單 function\_type 30, 刪單成功 status 30

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 30,
      date: "2024/10/17",
      seq_no: "00098000042",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0015",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "2330",
      buy_sell: Buy,
      price_type: Market,
      price: 0,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 4,
      after_price_type: None,
      after_price: 0,
      unit: 1000,
      after_qty: 1000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 1000,
      before_price: None,
      user_def: "batch3",
      last_time: "11:53:37.765",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 20,
      date: "2024/10/17",
      seq_no: "00098000040",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0014",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1102",
      buy_sell: Buy,
      price_type: Limit,
      price: 42,
      quantity: 2000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 4,
      after_price_type: None,
      after_price: 43,
      unit: 1000,
      after_qty: 2000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 2000,
      before_price: None,
      user_def: "batch1",
      last_time: "11:53:37.765",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 30,
      date: "2024/10/17",
      seq_no: "00098000041",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0013",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1101",
      buy_sell: Buy,
      price_type: Limit,
      price: 32,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 4,
      after_price_type: None,
      after_price: 33,
      unit: 1000,
      after_qty: 1000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 1000,
      before_price: None,
      user_def: "batch2",
      last_time: "11:53:37.765",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 30,
      date: "2024/10/17",
      seq_no: "00098000042",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0015",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "2330",
      buy_sell: Buy,
      price_type: Market,
      price: 0,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 30,
      after_price_type: None,
      after_price: 0,
      unit: 1000,
      after_qty: 0,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 1000,
      before_price: None,
      user_def: "batch3",
      last_time: "11:53:37.790",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 30,
      date: "2024/10/17",
      seq_no: "00098000041",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0013",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1101",
      buy_sell: Buy,
      price_type: Limit,
      price: 32,
      quantity: 1000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 30,
      after_price_type: None,
      after_price: 33,
      unit: 1000,
      after_qty: 0,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 1000,
      before_price: None,
      user_def: "batch2",
      last_time: "11:53:37.789",
      details: None,
      error_message: None,
  }
  ========

  ```

  ```py
  ==改單主動回報==
  Code None
  內容 OrderResult {
      function_type: 20,
      date: "2024/10/17",
      seq_no: "00098000040",
      branch_no: "20706",
      account: "9809268",
      order_no: "x0014",
      asset_type: 0,
      market: "TAIEX",
      market_type: Common,
      stock_no: "1102",
      buy_sell: Buy,
      price_type: Limit,
      price: 42,
      quantity: 2000,
      time_in_force: ROD,
      order_type: Stock,
      is_pre_order: false,
      status: 10,
      after_price_type: None,
      after_price: 43,
      unit: 1000,
      after_qty: 1000,
      filled_qty: 0,
      filled_money: 0,
      before_qty: 2000,
      before_price: None,
      user_def: "batch1",
      last_time: "11:53:37.790",
      details: None,
      error_message: None,
  }
  ========

  ```

* 函式回傳 (return):

  批次委託回傳值皆為 ACK (status 4)

  ```py
  Result {
      is_success: True,
      message: None,
      data: [OrderResult {
          function_type: 30,
          date: "2024/10/17",
          seq_no: "00098000042",
          branch_no: "20706",
          account: "9809268",
          order_no: "x0015",
          asset_type: 0,
          market: "TAIEX",
          market_type: Common,
          stock_no: "2330",
          buy_sell: Buy,
          price_type: Market,
          price: 0,
          quantity: 1000,
          time_in_force: ROD,
          order_type: Stock,
          is_pre_order: false,
          status: 4,
          after_price_type: None,
          after_price: 0,
          unit: 1000,
          after_qty: 0,
          filled_qty: 0,
          filled_money: 0,
          before_qty: 1000,
          before_price: None,
          user_def: "batch3",
          last_time: "11:53:37.763",
          details: None,
          error_message: None,
      }, OrderResult {
          function_type: 30,
          date: "2024/10/17",
          seq_no: "00098000041",
          branch_no: "20706",
          account: "9809268",
          order_no: "x0013",
          asset_type: 0,
          market: "TAIEX",
          market_type: Common,
          stock_no: "1101",
          buy_sell: Buy,
          price_type: Limit,
          price: 32,
          quantity: 1000,
          time_in_force: ROD,
          order_type: Stock,
          is_pre_order: false,
          status: 4,
          after_price_type: None,
          after_price: 33,
          unit: 1000,
          after_qty: 0,
          filled_qty: 0,
          filled_money: 0,
          before_qty: 1000,
          before_price: None,
          user_def: "batch2",
          last_time: "11:53:37.763",
          details: None,
          error_message: None,
      }, OrderResult {
          function_type: 20,
          date: "2024/10/17",
          seq_no: "00098000040",
          branch_no: "20706",
          account: "9809268",
          order_no: "x0014",
          asset_type: 0,
          market: "TAIEX",
          market_type: Common,
          stock_no: "1102",
          buy_sell: Buy,
          price_type: Limit,
          price: 42,
          quantity: 2000,
          time_in_force: ROD,
          order_type: Stock,
          is_pre_order: false,
          status: 4,
          after_price_type: None,
          after_price: 43,
          unit: 1000,
          after_qty: 1000,
          filled_qty: 0,
          filled_money: 0,
          before_qty: 2000,
          before_price: None,
          user_def: "batch1",
          last_time: "11:53:37.763",
          details: None,
          error_message: None,
      }]
  }

  ```

#### 成交主動回報[​](#成交主動回報 "Direct link to 成交主動回報")

成交主動回報包含委託書號 (order\_no) 及流水號 (seq\_no) 以供比對

```py
==成交主動回報==
Code None
FilledData {
    date: "2024/10/17",
    branch_no: "20706",
    account: "9809268",
    order_no: "x0007",
    stock_no: "1102",
    buy_sell: Sell,
    order_type: Stock,
    seq_no: "00000394165",
    filled_no: "00000000061",
    filled_avg_price: 43.0,
    filled_qty: 2000,
    filled_price: 43.0,
    filled_time: "11:38:08.535",
    user_def: None,
}
========

```


---

### 連線參數修改

新一代 API SDK 使用 WebSocket 安全連線方式與富邦伺服器主機建立連線，使用標準的 ping/pong control frames 進行連線確認。預設每 30 秒發送 ping frame 給伺服器，若達連續兩次無收到伺服器 pong 回應即判斷連線品質較弱，並進行斷線與提示。

為能更廣泛適用於不同使用者場景，自 `2.2.1` 版以後，使用者可自行設定此部分參數。在建立連線時可選擇自定參數，例如設定每 300 秒發送 ping frame，並若連續三次無回應即斷線，則連線功能呼叫方式如下：

* Python
* Node.js
* C#

```python
sdk = FubonSDK(300, 3)

```

```js
var sdk = new FubonSDK(300, 3);

```

```cs
var sdk = new FubonSDK(300, 3, "wss://neoapi.fbs.com.tw/TASP/XCPXWS");

```

其中第一個參數為間隔秒數，第二個參數為最大可忍受無回應次數。

參數設定建議

因網際網路難免時有雍塞情形，參數設定建議不小於預設值 (30,2)，以免對於連線品質判斷過於敏感，導致不必要的連線中斷。

測試環境連線

使用測試環境需以下進行連線（`2.2.1`版以後）：

```python
# "wss://neoapitest.fbs.com.tw/TASP/XCPXWS" 為測試環境連線 url

sdk = FubonSDK(30, 3, "wss://neoapitest.fbs.com.tw/TASP/XCPXWS")

# C# 或 JavaScript, 使用
# var sdk = new FubonSDK(30, 3, "wss://neoapitest.fbs.com.tw/TASP/XCPXWS");


```

註：前兩個參數之值僅為範例參考，可依需求自由調整。


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
from fubon_neo.sdk import FubonSDK
import threading

# === 請填入你的真實憑證資料 ===
USER_ID = "您的身分證字號"
USER_PW = "您的登入密碼"
CERT_PATH = "您的憑證位置"
CERT_PW = "您的憑證密碼"

# --- 若有行情 WebSocket 需要重連，可以在這裡實作 ---
def handle_marketdata_ws_disconnect(code, msg):
    print(f"[行情WS] 重新建立連線，原因: {code}, {msg}")

# === 全域鎖定，避免同時重複觸發重連 ===
relogin_lock = threading.Lock()

# === 建立 FubonSDK 實例（全域使用，重複利用） ===
sdk = FubonSDK()
accounts = None    # 登入後會儲存帳號資訊

# === 下單與成交等事件的處理函式（可自訂） ===
def on_order(code, msg): pass
def on_order_changed(code, msg): pass
def on_filled(code, msg): pass

def re_login():
    """
    當連線異常時，自動重新登入並重設事件 callback。
    用 lock 確保同時間只會有一個重連動作。
    """
    if not relogin_lock.acquire(blocking=False):
        print("[自動重連] 目前已有重連程序執行中，這次略過。")
        return

    try:
        global accounts, sdk
        print("[自動重連] 登出並重新登入...")
        try:
            sdk.logout()
        except Exception as e:
            print("[自動重連] 登出時發生例外：", e)

        try:
            sdk = FubonSDK()
            accounts = sdk.login(USER_ID, USER_PW, CERT_PATH, CERT_PW)
        except Exception as e:
            print("[自動重連] 登入時發生例外：", e)
            return

        if accounts.is_success:
            print("[自動重連] 重新登入成功，重新設定所有事件 callback。")
            # 很重要：重新登入後一定要重設事件 callback！
            sdk.set_on_event(on_event)
            sdk.set_on_order(on_order)
            sdk.set_on_order_changed(on_order_changed)
            sdk.set_on_filled(on_filled)

            # 若有行情 WS 也需要重連，可自行呼叫
            handle_marketdata_ws_disconnect(-9000, "交易WS已重連")
        else:
            print("[自動重連] 重新登入失敗。")
    finally:
        relogin_lock.release()

def on_event(code, content):
    """
    FubonSDK 的事件通知 callback。
    """
    print("[事件通知] 代碼:", code, "| 內容:", content)
    if code == "300":
        print("[事件通知] 偵測到斷線（代碼300），啟動自動重連。")
        re_login()

# === 一開始就要先登入並註冊各種 callback ===
accounts = sdk.login(USER_ID, USER_PW, CERT_PATH, CERT_PW)
if accounts.is_success:
    print("[主程式] 登入成功，開始註冊事件 callback...")
    sdk.set_on_event(on_event)
    sdk.set_on_order(on_order)
    sdk.set_on_order_changed(on_order_changed)
    sdk.set_on_filled(on_filled)

    # 範例：可以指定預設帳號（如果有多個）
    acc = accounts.data[0] if hasattr(accounts, "data") else None
else:
    print("[主程式] 登入失敗！")

# === 範例：手動模擬斷線事件，測試自動重連機制 ===
on_event("300", "測試：WebSocket 已斷線")
on_event("300", "測試：再觸發一次斷線，理論上 lock 會防止重複重連")

# --- 範例結束 ---


```

```js
const { FubonSDK } = require('fubon-neo');

// === 請填入你的真實憑證資料 ===
const USER_ID = "您的身分證字號";
const USER_PW = "您的登入密碼";
const CERT_PATH = "您的憑證路徑";
const CERT_PW = "您的憑證密碼";

// === 全域鎖，避免同時重複觸發重連 ===
let reloginLock = false;

// === 建立 SDK 實例（全域用）===
let sdk = new FubonSDK();
let accounts = null;

// === 下單與成交等事件的處理函式（可依需求擴充） ===
function onOrder(code, msg) {}
function onOrderChanged(code, msg) {}
function onFilled(code, msg) {}

// --- 若有行情 WS 需要重連，可在這裡擴充 ---
function handleMarketdataWsDisconnect(code, msg) {
    console.log(`[行情WS] 重新建立連線，原因: ${code}, ${msg}`);
}

function reLogin() {
    // 用 flag 防止同時多次重連
    if (reloginLock) {
        console.log("[自動重連] 目前已有重連程序執行中，這次略過。");
        return;
    }
    reloginLock = true;

    console.log("[自動重連] 登出並重新登入...");
    try {
        // 取消 callback 註冊
        sdk.setOnEvent(() => {});
        sdk.setOnOrder(() => {});
        sdk.setOnOrderChanged(() => {});
        sdk.setOnFilled(() => {});
        sdk.logout(); // 若沒登入也沒關係
    } catch (e) {
        console.log("[自動重連] 登出時發生例外：", e);
    }

    try {
        // 重新建立 SDK 實例是好習慣
        sdk = new FubonSDK();
        accounts = sdk.login(USER_ID, USER_PW, CERT_PATH, CERT_PW);
    } catch (e) {
        console.log("[自動重連] 登入時發生例外：", e);
        reloginLock = false;
        return;
    }

    if (accounts && accounts.isSuccess) {
        console.log("[自動重連] 重新登入成功，重新設定所有事件 callback。");
        // 重新註冊所有事件 callback（非常重要）
        sdk.setOnEvent(onEvent);
        sdk.setOnOrder(onOrder);
        sdk.setOnOrderChanged(onOrderChanged);
        sdk.setOnFilled(onFilled);

        // 如需行情WS，也可在這裡呼叫
        handleMarketdataWsDisconnect(-9000, "交易WS已重連");
    } else {
        console.log("[自動重連] 重新登入失敗。");
    }

    reloginLock = false;
}

function onEvent(code, content) {
    // 事件通知 callback
    console.log("[事件通知] 代碼:", code, "| 內容:", content);
    if (code === "300") {
        console.log("[事件通知] 偵測到斷線（代碼300），啟動自動重連。");
        reLogin();
    }
}

// === 一開始先登入並註冊 callback ===
accounts = sdk.login(USER_ID, USER_PW, CERT_PATH, CERT_PW);
if (accounts && accounts.isSuccess) {
    console.log("[主程式] 登入成功，開始註冊事件 callback...");
    sdk.setOnEvent(onEvent);
    sdk.setOnOrder(onOrder);
    sdk.setOnOrderChanged(onOrderChanged);
    sdk.setOnFilled(onFilled);

    // 範例：可取得預設帳號
    let acc = (accounts.data && accounts.data.length > 0) ? accounts.data[0] : null;
} else {
    console.log("[主程式] 登入失敗！");
}

// === 範例：手動觸發斷線事件，測試自動重連 ===
Promise.all([
    Promise.resolve().then(() => onEvent("300", "測試：WebSocket 已斷線")),
]);

// --- 範例結束 ---

```

```cs
using System;
using System.Threading;
using FubonNeo.Sdk;

class Program
{
    // === 請填入你的真實憑證資料 ===
    static string USER_ID = "您的身分證字號";
    static string USER_PW = "您的登入密碼";
    static string CERT_PATH = "您的憑證位置";
    static string CERT_PW = "您的憑證密碼";

    // === 全域 lock，避免同時重複觸發重連 ===
    static object reloginLock = new object();
    static bool isReloginRunning = false;

    // === 全域 SDK 物件（重複利用）===
    static FubonSDK sdk = new FubonSDK();
    static dynamic accounts = null; // SDK 回傳型別需依實際狀況確認

    // === 下單與成交等事件處理器（可自行擴充）===
    static void OnOrder(string code, OrderResult data) { }
    static void OnOrderChanged(string code, OrderResult data) { }
    static void OnFilled(string code, FilledData data) { }

    // --- 若有行情 WS 需要重連，可在這裡擴充 ---
    static void HandleMarketdataWsDisconnect(int code, string msg)
    {
        Console.WriteLine($"[行情WS] 重新建立連線，原因: {code}, {msg}");
    }

    static void ReLogin()
    {
        // 避免同時重複重連，用 lock 與旗標
        lock (reloginLock)
        {
            if (isReloginRunning)
            {
                Console.WriteLine("[自動重連] 目前已有重連程序執行中，這次略過。");
                return;
            }
            isReloginRunning = true;
        }

        try
        {
            Console.WriteLine("[自動重連] 登出並重新登入...");
            try
            {
                sdk.Logout();
            }
            catch (Exception e)
            {
                Console.WriteLine("[自動重連] 登出時發生例外：" + e.Message);
            }

            try
            {
                sdk = new FubonSDK();
                accounts = sdk.Login(USER_ID, USER_PW, CERT_PATH, CERT_PW);
            }
            catch (Exception e)
            {
                Console.WriteLine("[自動重連] 登入時發生例外：" + e.Message);
                return;
            }

            if (accounts != null && accounts.isSuccess)
            {
                Console.WriteLine("[自動重連] 重新登入成功，重新設定所有事件 callback。");
                // 重新設定 callback
                sdk.OnEvent = OnEvent;
                sdk.OnOrder = OnOrder;
                sdk.OnOrderChanged = OnOrderChanged;
                sdk.OnFilled = OnFilled;

                // 行情WS重連（如需）
                HandleMarketdataWsDisconnect(-9000, "交易WS已重連");
            }
            else
            {
                Console.WriteLine("[自動重連] 重新登入失敗。");
            }
        }
        finally
        {
            lock (reloginLock)
            {
                isReloginRunning = false;
            }
        }
    }

    static void OnEvent(string code, string content)
    {
        Console.WriteLine("[事件通知] 代碼:" + code + " | 內容:" + content);
        if (code == "300")
        {
            Console.WriteLine("[事件通知] 偵測到斷線（代碼300），啟動自動重連。");
            ReLogin();
        }
    }

    static void Main(string[] args)
    {
        // === 一開始登入並註冊 callback ===
        accounts = sdk.Login(USER_ID, USER_PW, CERT_PATH, CERT_PW);
        if (accounts != null && accounts.isSuccess)
        {
            Console.WriteLine("[主程式] 登入成功，開始註冊事件 callback...");
            sdk.OnEvent = OnEvent;
            sdk.OnOrder = OnOrder;
            sdk.OnOrderChanged = OnOrderChanged;
            sdk.OnFilled = OnFilled;

            // 取得預設帳號（如果有多個）
            var acc = (accounts.data != null && accounts.data.Count > 0) ? accounts.data[0] : null;
        }
        else
        {
            Console.WriteLine("[主程式] 登入失敗！");
        }

        // === 範例：手動觸發斷線事件，測試自動重連 ===
        OnEvent("300", "測試：WebSocket 已斷線");
        OnEvent("300", "測試：再觸發一次斷線，lock 應該會防止重複重連");

        // 讓主程式暫停，觀察結果（可依需求移除）
        Console.WriteLine("Press any key to exit...");
        Console.ReadKey();
    }
}


```


---

### 錯誤碼與狀態碼對照表

本頁面彙整了富邦新一代 API（證券交易）中常見的狀態碼、功能代碼與事件代碼，方便開發者在遇到問題或進行狀態判斷時快速檢索。

#### 委託單狀態 (Status)[​](#委託單狀態-status "Direct link to 委託單狀態 (Status)")

委託單的當前狀態，可透過 `GetOrderResult` 查詢或由主動回報取得。

| 狀態名稱           | 狀態碼 (Value)   | 說明                                                     |
| ------------------ | ---------------- | -------------------------------------------------------- |
| 預約單             | `0`              | 預約單                                                   |
| 系統將委託送往後台 | `4`              | 請用 `GetOrderResult` 查詢狀態                           |
| 後台傳送中         | `8`              | 請用 `GetOrderResult` 查詢狀態                           |
| 連線逾時           | `9`              | 請稍後再使用 `GetOrderResult` 查詢狀態 or 聯絡您的營業員 |
| 委託成功           | `10`             | 委託成功                                                 |
| 未成交刪單成功     | `30`             | 未成交刪單成功                                           |
| 部分成交，剩餘取消 | `40`             | 部分成交，剩餘取消                                       |
| 完全成交           | `50`             | 完全成交                                                 |
| 失敗               | `90`             | 失敗                                                     |
| 委託單歷程查詢標示 | `14`, `24`, `34` | `14` - 改價ACK、`24` - 改量ACK、`34` - 刪單ACK           |
| 改價成功           | `15`             | 歷史委託單查詢                                           |
| 改量成功           | `20`             | 歷史委託單查詢                                           |
| 改價失敗           | `19`             | 主動回報                                                 |
| 改量失敗           | `29`             | 主動回報                                                 |
| 刪單失敗           | `39`             | 主動回報                                                 |

#### 功能類別 (Function Type)[​](#功能類別-function-type "Direct link to 功能類別 (Function Type)")

表示該筆回報或查詢結果所屬的操作類型。

| 功能名稱 | 代碼 (Value) | 說明               |
| -------- | ------------ | ------------------ |
| 新單     | `0`          | 新單               |
| 新單執行 | `10`         | 新單執行           |
| 改價     | `15`         | 改價               |
| 改量     | `20`         | 改量               |
| 刪單     | `30`         | 刪單               |
| 完全成交 | `50`         | for 委託單歷程查詢 |
| 失敗     | `90`         | 失敗               |

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

主動回報範例

詳細回傳內容，可參考[主動回報範例](https://www.fbs.com.tw/TradeAPI/docs/trading/guide/advance/callback/callback_example_standard.txt)；物件說明可參照[SDK Reference 參數對照表](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#orderresult)

#### 訂閱委託回報[​](#訂閱委託回報 "Direct link to 訂閱委託回報")

* Python
* Node.js
* C#
* C++
* Go

```py
# A callback to receive order data
def on_order(code, content):
    print("==Order==")
    print(code)
    print(content)
    # print(content.seq_no)  # 印出委託單流水號
    print("========")

sdk.set_on_order(on_order) 

```

info

回傳物件說明，可參照[OrderResult Object](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#orderresult)

```js
//Callback to receive order data
sdk.setOnOrder(function(code, content) 
    { console.log("====order===\n",code, content)});

```

info

回傳物件說明，可參照[OrderResult Object](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#orderresult)

```cs
public void OnOrder(string code, OrderResult data)
{
    if(data != null)
    {
        response = data.ToString();
        Console.WriteLine("On Order" + response);
    }

}

```

或使用下方方法

```cs
sdk.OnOrder += (code, ordeResult) =>
{
    Console.WriteLine(code + ordeResult.ToString());
}

```

info

回傳物件說明，可參照[OrderResult Object](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#orderresult)

```cpp
void on_order(std::optional<std::string> err, const OrderResult& data) override {
    if (err.has_value()) {
        std::cerr << "Error in on_order: " << err.value() << ", status: " << data.status.value() << std::endl;
        return;
    }

    if (data.order_no.has_value()) { // 或者更簡潔的 if (data.order_no)
        std::cout << "on order: order_no - " << data.order_no.value() << ". status - " << data.status.value() << std::endl;
    }
    else {
        std::cout << "on order: order_no is not available. status - " << data.status.value() << std::endl;
    }
};

```

info

回傳物件說明，可參照[OrderResult Object](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#orderresult)

```go
sdk.OnOrder = func(err *string, data fubon.OrderResult) {
	s, _ := json.MarshalIndent(data, "", "  ")
	fmt.Println("On Order: ", string(s))
	if err != nil {
		fmt.Printf("   Error: %s\n", *err)
	}
}

```

info

詳細回傳內容，可參照[OrderResult Object](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#orderresult)

#### 訂閱改價/改量/刪單回報[​](#訂閱改價改量刪單回報 "Direct link to 訂閱改價/改量/刪單回報")

* Python
* Node.js
* C#
* C++
* Go

```py
def on_order_changed(code, content):
    print("=Modified==")
    print(code)
    print(content)
    # print(content.seq_no)  # 印出委託單流水號
    print("========")
    
    
sdk.set_on_order_changed(on_order_changed) 

```

info

回傳物件說明，可參照[OrderResult Object](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#orderresult)

```js
//Callback to receive Modified data
sdk.setOnOrderChanged(function(code, content) 
    { console.log("===Modified===\n", code, content)});

```

info

回傳物件說明，可參照[OrderResult Object](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#orderresult)

```cs
public void OnOrderChanged(string code, OrderResult data)
{
    if(data != null)
    {
        response = data.ToString();

        Console.WriteLine(code);
        Console.WriteLine("Modified" + response);
    }   

}

```

或使用下方方法

```cs
sdk.OnOrderChanged += (code, ordeResult) =>
{
    Console.WriteLine(code + ordeResult.ToString());
}

```

info

詳細回傳內容，可參照[OrderResult Object](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#orderresult)

```cpp
void on_order_changed(std::optional<std::string> err, const OrderResult& data) override {
    if (data.error_message.has_value()) {
        std::cout << "on_order_changed: " << data.order_no.value() << ", status: " << data.status.value() << ", message: " << data.error_message.value() << std::endl;
    }
    else {
        std::cout << "on_order_changed: " << data.order_no.value() << ", status: " << data.status.value() << std::endl;
    }
};

```

info

回傳物件說明，可參照[OrderResult Object](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#orderresult)

```go
sdk.OnOrderChanged = func(err *string, data fubon.OrderResult) {
	s, _ := json.MarshalIndent(data, "", " ")
	fmt.Println("On Change ", string(s))
	if err != nil {
		fmt.Printf("   Error: %s\n", *err)
	}
}

```

info

詳細回傳內容，可參照[OrderResult Object](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#orderresult)

#### 訂閱成交回報[​](#訂閱成交回報 "Direct link to 訂閱成交回報")

* Python
* Node.js
* C#
* C++
* Go

```py
def on_filled(code, content):
    print("==Filled==")
    print(code)
    print(content)
    # print(content.filled_no)  # 印出成交流水號
    print("========")
    
sdk.set_on_filled(on_filled)

```

info

回傳物件說明，可參照[FilledData Object](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#filleddata)

```js
sdk.setOnFilled(function(code, content) 
    { console.log("===Filled===\n",code, content)})

```

info

回傳物件說明，可參照[FilledData Object](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#filleddata)

```cs
public void OnFilled(string code, FilledData data)
{
    if(data != null)
    {
        response = data.ToString();

        Console.WriteLine(code);
        Console.WriteLine("Filled" + response);
    }

}

```

或使用下方方法

```cs
sdk.OnFilled += (code, filledData) =>
{
    Console.WriteLine(code + filledData.ToString());
}

```

info

回傳物件說明，可參照[FilledData Object](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#filleddata)

```cpp
void on_filled(std::optional<std::string> err, const FilledData& data) override {
    if (err.has_value()) { 
        std::cerr << "Error in on_order: " << err.value() << std::endl;
        return; 
    }

    if (data.order_no.has_value()) { 
        std::cout << "on order: " << data.order_no.value() << std::endl;
    }
    else {
        std::cout << "on order: order_no is not available." << std::endl;
    }
};

```

info

詳細回傳內容，可參照[FilledData Object](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#filleddata)

```go
sdk.OnFilled = func(err *string, data fubon.FilledData) {
	s, _ := json.MarshalIndent(data, "", " ")
	fmt.Println("On Filled ", string(s))
	if err != nil {
		fmt.Printf("   Error: %s\n", *err)
	}
}

```

info

詳細回傳內容，可參照[FilledData Object](https://www.fbs.com.tw/TradeAPI/docs/trading/library/cpp/EnumMatrix.txt#filleddata)

#### 訂閱事件通知[​](#訂閱事件通知 "Direct link to 訂閱事件通知")

* Python
* Node.js
* C#
* C++
* Go

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
    response = data.ToString();
    Console.WriteLine(code);
    Console.WriteLine("Event" + response);
}

```

或使用下方方法

```cs
sdk.OnEvent += (code, msg) =>
{
    Console.WriteLine(code + msg );
};

```

```cpp
void on_event(const std::string& code, const std::string& data) override {
    std::cout << code << " " << data << std::endl;
}

```

```go
sdk.OnEvent = func(data string, eventType string) {
    fmt.Printf("📡 OnEvent - Type: %s, Data: %s\n", eventType, data)
    if eventType == "disconnect" {
        fmt.Println("⚠️  Connection lost!")
    }
}

```

事件包含以下情況回傳

| 回傳代碼 | 意義                                               |
| -------- | -------------------------------------------------- |
| 100      | 連線建立成功                                       |
| 200      | 登入成功                                           |
| 201      | 登入警示 , Ex : 90天未更換密碼                     |
| 300      | 斷線                                               |
| 301      | 未收到連線pong回傳                                 |
| 302      | 用戶執行登出 , 並斷線                              |
| 304      | API Key 異動 (Revoked), 已強制登出 (2.2.7版本新增) |
| 500      | 錯誤                                               |

#### 訂閱範例[​](#訂閱範例 "Direct link to 訂閱範例")

使用者可訂閱不同的callback，來接收系統主動發送的委託及成交通知。

* Python
* Node.js
* C#
* C++
* Go

```py
# A callback to receive order data
def on_order(code, content):
    print("==Order==")
    print(code)
    print(content)
    print("========")
    
sdk.set_on_order(on_order) 

# A callback to receive Modified data
def on_order_changed(code, content):
    print("=Modified==")
    print(code)
    print(content)
    print("========")
    
sdk.set_on_order_changed(on_order_changed) 

# A callback to receive Filled data
def on_filled(code, content):
    print("==Filled==")
    print(code)
    print(content)
    print("========")
    
sdk.set_on_filled(on_filled)

# A callback to receive Event data
def on_event(code, content):
    print("===event=====")
    print(code)
    print(content)
    print("========")
    
sdk.set_on_event(on_event) 

```

```js
//Callback to receive order data
sdk.setOnOrder(function(code, content) 
    { console.log("====order===\n",code, content)});

//Callback to receive Modified data
sdk.setOnOrderChanged(function(code, content) 
    { console.log("===Modified===\n", code, content)});

//Callback to receive Filled data
sdk.setOnFilled(function(code, content) 
    { console.log("===Filled===\n",code, content)})

//Callback to receive Event data
sdk.setOnEvent(function(code, content) 
    { console.log("===Event===\n",code, content)})

```

```cs
public class MyCallback : Callback
{
    public string code ="";
    public string response = "";

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

```

或使用下方方法分別訂閱

```cs
sdk.OnEvent += (code, msg) =>
{
    Console.WriteLine(code + msg );
};


sdk.OnOrder += (code, ordeResult) =>
{
    Console.WriteLine(code + ordeResult.ToString());
}

sdk.OnOrderChanged += (code, ordeResult) =>
{
    Console.WriteLine(code + ordeResult.ToString());
}

sdk.OnFilled += (code, filledData) =>
{
    Console.WriteLine(code + filledData.ToString());
}

```

```cpp
struct CustomCallback : public fubon::Callback {
    void on_event(const std::string& code, const std::string& data) override {
        std::cout << code << " " << data << std::endl;
    }

    void on_order(std::optional<std::string> err, const OrderResult& data) override {
        if (err.has_value()) {
            std::cerr << "Error in on_order: " << err.value() << ", status: " << data.status.value() << std::endl;
            return;
        }

        if (data.order_no.has_value()) { // 或者更簡潔的 if (data.order_no)
            std::cout << "on order: order_no - " << data.order_no.value() << ". status - " << data.status.value() << std::endl;
        }
        else {
            std::cout << "on order: order_no is not available. status - " << data.status.value() << std::endl;
        }
    };

    void on_order_changed(std::optional<std::string> err, const OrderResult& data) override {
        if (data.error_message.has_value()) {
            std::cout << "on_order_changed: " << data.order_no.value() << ", status: " << data.status.value() << ", message: " << data.error_message.value() << std::endl;
        }
        else {
            std::cout << "on_order_changed: " << data.order_no.value() << ", status: " << data.status.value() << std::endl;
        }
    };

    void on_filled(std::optional<std::string> err, const FilledData& data) override {
        std::cout << data.order_no << " " << std::endl;
    };

    void on_futopt_order(std::optional<std::string> err, const FutOptOrderResult& data) override {
        return;
        //if (err.has_value()) {
        //    std::cerr << "Error in on_order: " << err.value() << std::endl;
        //    return;
        //}

        //if (data.order_no.has_value()) { // 或者更簡潔的 if (data.order_no)
        //    std::cout << "on order: " << data.order_no.value() << std::endl;
        //}
        //else {
        //    std::cout << "on order: order_no is not available." << std::endl;
        //}
    };

    void on_futopt_order_changed(std::optional<std::string> err, const FutOptOrderResult& data) override {
        /*std::cout << data.order_no.value() << " " << std::endl;*/
        return;
    };

    void on_futopt_filled(std::optional<std::string> err, const FutOptFilledData& data) override {
        //std::cout << data.order_no << " " << std::endl;
        return;
    };
};


int main()
{
    auto callback = std::make_shared<CustomCallback>();
    auto sdk = new FubonSDK();
    sdk->register_callback(callback);

    /// 以下略 ...
    /// ...
    
    return 0;
}

```

```go
import (
	"encoding/json"
	"fmt"
	"fubon"
	"strings"
)

func main(){

	sdk.OnOrder = func(err *string, data fubon.OrderResult) {
		s, _ := json.MarshalIndent(data, "", "  ")
		fmt.Println("On Order: ", string(s))
		if err != nil {
			fmt.Printf("   Error: %s\n", *err)
		}
	}

	sdk.OnOrderChanged = func(err *string, data fubon.OrderResult) {
		s, _ := json.MarshalIndent(data, "", " ")
		fmt.Println("On Change ", string(s))
		if err != nil {
			fmt.Printf("   Error: %s\n", *err)
		}
	}

	sdk.OnFilled = func(err *string, data fubon.FilledData) {
		s, _ := json.MarshalIndent(data, "", " ")
		fmt.Println("On Filled ", string(s))
		if err != nil {
			fmt.Printf("   Error: %s\n", *err)
		}
	}

	sdk.OnEvent = func(data string, eventType string) {
		fmt.Printf("📡 OnEvent - Type: %s, Data: %s\n", eventType, data)
		if eventType == "disconnect" {
			fmt.Println("⚠️  Connection lost!")
		}
	}

}

```


---

### 交易

***

<!-- -->

此篇教學將示範如何進行一個完整的買賣流程

#### 買入股票[​](#買入股票 "Direct link to 買入股票")

假設今天開盤後，我們想以66.00元買進2張富邦金，我們可以這樣撰寫程式並執行：

* Python
* Node.js
* C#

```python
from fubon_neo.sdk import FubonSDK, Order
from fubon_neo.constant import TimeInForce, OrderType, PriceType, MarketType, BSAction

sdk = FubonSDK()
   
accounts = sdk.login("您的身分證字號", "您的登入密碼", "您的憑證位置", "您的憑證密碼") #若有歸戶，則會回傳多筆帳號資訊
  
#建立委託單內容
order = Order(
    buy_sell = BSAction.Buy,
    symbol = "2881",
    price = "66",
    quantity =  2000,
    market_type = MarketType.Common,
    price_type = PriceType.Limit,
    time_in_force= TimeInForce.ROD,
    order_type = OrderType.Stock,
    user_def = "FromPy" # optional field
) 


sdk.stock.place_order(accounts.data[0], order)  #下單委託

```

info

整股與零股之委託，請參考各語言Libary [Python Libary](https://www.fbs.com.tw/TradeAPI/docs/trading/library/python/EnumMatrix.txt#markettype)

```js
const { FubonSDK, BSAction, TimeInForce, OrderType, PriceType, MarketType } = require('fubon-neo');

const sdk = new FubonSDK();

const accounts = sdk.login("您的身分證字號", "您的登入密碼", "您的憑證路徑" ,"您的憑證密碼");



const order = {
  buySell: BSAction.Buy,
  symbol: "2881",
  price: "66",
  quantity: 2000,
  marketType: MarketType.Common,
  priceType: PriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: OrderType.Stock,
  userDef: "fromJs"
};

sdk.stock.placeOrder(accounts.data[0],order);

```

info

整股與零股之委託，請參考各語言Libary [Node.Js Libary](https://www.fbs.com.tw/TradeAPI/docs/trading/library/nodejs/EnumMatrix.txt#markettype)

```cs

    
using FubonNeo.Sdk;

var sdk = new FubonSDK();

var accounts = sdk.Login("您的身分證字號", "您的登入密碼", "您的憑證路徑", "您的憑證密碼"); // 若有歸戶，則會回傳多筆帳號資訊

var order = new Order(
    BsAction.Buy,
    "2881",
    "66",
    2000,
    MarketType.Common,
    PriceType.Limit,
    TimeInForce.Rod,
    OrderType.Stock,
    null
);

sdk.Stock.PlaceOrder(accounts.data[0],order); // 使用阻塞委託下單

```

info

整股與零股之委託，請參考各語言Libary [C# Libary](https://www.fbs.com.tw/TradeAPI/docs/trading/library/csharp/EnumMatrix.txt#markettype)

#### 確認委託與成交回報[​](#確認委託與成交回報 "Direct link to 確認委託與成交回報")

若要確定該筆的狀態，可以依照下方範例查詢指定的委託單:

* Python
* Node.js
* C#

```python
orderResults = sdk.stock.get_order_results(accounts.data[0])
print(orderResults)

```

```js
const orderResults = sdk.stock.getOrderResults(accounts.data[0])
console.log(orderResults)

```

```cs
var orderResults = sdk.Stock.GetOrderResults(accounts.data[0]);
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
      OrderResult{
        ...
        buy_sell: Buy,   #買賣別 (BSAction)
        price: 66,       #原始委託價格 (float)
        quantity: 2000,  #原始委託數量 (int)
        after_price: 66, #有效委託價格 (float)
        after_qty: 2000, #有效委託數量 (int)
        filled_qty: 0,   #已成交數量 (int)
        filled_money: 0, #成交價金 (int)
        symbol: "2881",  #股票代號 (string)
        order_no: "bA888", #委託書號 (string)
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
      buySell: 'Buy', //買賣別 (string)
      price: 66,      //原始委託價格 (number)
      quantity: 2000, //原始委託數量 (number)
      afterPrice: 66, //有效委託價格 (number)
      afterQty: 2000, //有效委託數量 (number)
      filledQty: 0,   //已成交數量 (number)
      filledMoney: 0, //成交價金 (number)
      symbol: '2881', //股票代號 (string)
      orderNo: 'bA888', //委託書號 (string)
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
      OrderResult{
        ...
        buySell = Buy,     //買賣別 (BsAction)
        price = 66,      //原始委託價格 (double)
        quantity = 2000, //原始委託數量 (int)
        afterPrice = 66, //有效委託價格 (double)
        afterQty = 2000, //有效委託數量 (int)
        filledQty = 0,   //已成交數量 (int)
        filledMoney = 0, //成交價金 (int)
        symbol = 2881,   //股票代號 (string)
        orderNo = bA888, //委託書號 (string)
        lastTime = 10:10:10.123, //最後異動時間 (string)
        ...
      }
   ]
}


```

#### 修改委託價格[​](#修改委託價格 "Direct link to 修改委託價格")

由於原先的價格一直無法成交，我們調整原先的委託價格，改用66.50元的價格買入：

* Python
* Node.js
* C#

```python
orderResults = sdk.stock.get_order_results(accounts.data[0])

modified_pirce = sdk.stock.make_modify_price_obj(orderResults.data[0],"66.5")
sdk.stock.modify_price(accounts.data[0], modified_pirce)

```

```js
orderResults = sdk.stock.getOrderResults(accounts.data[0])

const modified_pirce = sdk.stock.makeModifyPriceObj(orderResults.data[0],"66.5")
sdk.stock.modifyPrice(accounts.data[0],modified_pirce)

```

```cs
orderResults = sdk.Stock.GetOrderResults(accounts.data[0]);

var modified_pirce =  sdk.Stock.MakeModifyPriceObj(orderResults.data[0],"66.5",null);  //將價格調整成66.5元
sdk.Stock.ModifyPrice(accounts.data[0],modified_pirce);  

```

幾分鐘後，我們再查詢一次委託狀態，發現成交了：

* Python
* Node.js
* C#

```python
orderResults = sdk.stock.get_order_results(accounts.data[0])
print(orderResults.data[0])

```

```py
[
  {
    ...
    buy_sell: Buy,     #買賣別 (BSAction)
    price: 66,         #原始委託價格 (float)
    quantity: 2000,    #原始委託數量 (int)
    after_price: 66.5, #有效委託價格 (float)
    after_qty: 1000,   #有效委託數量 (int)
    filled_qty: 1000,  #已成交數量 (int)
    filled_money: 66000, #成交價金 (int)
    symbol: "2881",    #股票代號 (string)
    order_no: "bA888", #委託書號 (string)
    last_time: "10:13:12.123", #最後異動時間 (string)
    ...
  }
]

```

```js
orderResults = sdk.stock.getOrderResults(accounts.data[0])
console.log(orderResults.data[0])

```

```json
{
    ...
    "buySell" : "Buy",     //買賣別 (string)
    "price" : 66,         //原始委託價格 (number)
    "quantity" :  2000,    //原始委託數量 (number)
    "afterPrice" : 66.5,  //有效委託價格 (number)
    "afterQty" :  1000,    //有效委託數量 (number)
    "filledQty" : 1000,   //已成交數量 (number)
    "filledMoney" : 66000, //成交價金 (number)
    "stockNo" : "2881",    //股票代號 (string)
    "orderNo" : "bA888",  //委託書號 (string)
    "lastTime" : "10:13:12.123", //最後異動時間 (string)
    ...
}

```

```cs
orderResults = sdk.Stock.GetOrderResults(accounts.data[0]);
Console.WriteLine(orderResults.data[0]);

```

```cs
{
    ...
    buySell = Buy,    //買賣別 (BsAction)
    price = 66,         //原始委託價格 (double)
    quantity =  2000,    //原始委託數量 (int)
    afterPrice = 66.5,  //有效委託價格 (double)
    afterQty =  1000,    //有效委託數量 (int)
    filledQty = 1000,   //已成交數量 (int)
    filledMoney = 66000, //成交價金 (int)
    stockNo = "2881",    //股票代號 (string)
    orderNo = "bA888",  //委託書號 (string)
    lastTime = "10:13:12.123", //最後異動時間 (string)
    ...
}

```

#### 賣出股票[​](#賣出股票 "Direct link to 賣出股票")

最後決定在收盤前，賣出一張富邦金：

* Python
* Node.js
* C#

```python
#建立委託單內容
order = Order(
    buy_sell = BSAction.Sell,
    symbol = "2881",
    price = "67",
    quantity =  1000,
    market_type = MarketType.Common,
    price_type = PriceType.Limit,
    time_in_force= TimeInForce.ROD,
    order_type = OrderType.Stock,
    user_def = "FromPy" # optional field
) 


sdk.stock.place_order(accounts.data[0], order)  #下單委託

```

```js
order = {
  buySell: BSAction.Sell,
  symbol: "2881",
  price: "66",
  quantity: 1000,
  marketType: MarketType.Common,
  priceType: PriceType.Limit,
  timeInForce: TimeInForce.ROD,
  orderType: OrderType.Stock,
  userDef: "fromJs"
};

sdk.stock.placeOrder(accounts.data[0],order)

```

```cs
order = new Order(
        BsAction.Sell,
        "2881",
        "66",
         1000,
        MarketType.Common,
        PriceType.Limit,
        TimeInForce.Rod,
        OrderType.Stock,
        null
);

sdk.Stock.PlaceOrder(accounts.data[0],order); 

```


---

