import MetaTrader5 as mt5

def con_mt5():
    if not mt5.initialize():
        print("❌ Failed to connect to MT5")
        print(mt5.last_error())
        return False

    account = mt5.account_info()

    print("✅ Connected to MT5")
    print(f"👤 Account: {account.login}")
    print(f"💰 Balance: {account.balance}")
    print(f"💵 Equity: {account.equity}")
    print(f"🏦 Broker: {account.company}")

    return True

def dis_mt5():
    mt5.shutdown()
    print("🔌 Disconnected from MT5")

def get_symbol_info(symbol):
    info = mt5.symbol_info(symbol)
    if info is None:
        print(f"❌ Symbol {symbol} not found")
        return None

    print(f"ℹ️ Symbol Name: {info.name}")
    print(f"💰 Currency Based: {info.currency_base}/{info.currency_profit}")
    #print(f"📝 Description: {info.description}")
    print(f"📁 Path Type: {info.path}")

    return info

con_mt5()

get_symbol_info("XAUUSD")

dis_mt5()