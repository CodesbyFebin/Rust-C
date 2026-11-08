# ⚡ TurboQuant AI — Quantitative Intelligence Desktop Platform

> **Professional-grade quantitative trading dashboard with embedded Claude AI.**  
> Live charts · AI signals · Market scanner · Portfolio analytics · 6 AI agent modes

---

## 🖥 Application Views

| View | Description |
|---|---|
| **Dashboard** | Live watchlist, candlestick chart with signals, portfolio metrics, order feed |
| **AI Terminal** | 6 specialist agents: Quant Analyst, Algo Trader, Risk Manager, Macro Intel, Strategy Lab, Report Writer |
| **Scanner** | AI-enhanced market scanner with confidence scores and insights |
| **Portfolio** | Holdings table, allocation donut chart, risk metrics |

---

## 🚀 Quick Start

### Option 1 — Browser (Instant, No Install)
```bash
# Just open the HTML file:
open app/index.html
# Or double-click app/index.html in Finder / Explorer
```
Set your Anthropic API key inside `app/index.html`:
```javascript
const ANTHROPIC_KEY = 'sk-ant-your-key-here';
```

### Option 2 — Web Server (Python Backend)
```bash
chmod +x start.sh
./start.sh web
# Open http://localhost:7432
```

### Option 3 — Electron Desktop App
```bash
npm install
npm start
# Or build: npm run build:mac / npm run build:win
```

---

## 🤖 AI Agent Modes

| Agent | Speciality |
|---|---|
| 📊 **Quant Analyst** | Technical signals, alpha research, trade setups |
| ⚡ **Algo Trader** | Execution strategy, momentum, stat-arb |
| 🛡 **Risk Manager** | VaR, drawdown, position limits, hedging |
| 🌐 **Macro Intel** | Fed policy, yield curve, global macro regime |
| 🔬 **Strategy Lab** | Python backtesting, factor research, code |
| 📋 **Report Writer** | Investment research, earnings analysis, commentary |

---

## ⚙️ Configuration

| Setting | Location | Default |
|---|---|---|
| Anthropic API Key | `app/index.html` line 1 | — |
| Backend Port | `TQ_PORT` env var | 7432 |
| Claude Model | `backend/server.py` | claude-opus-4-5 |

---

## 📁 File Structure

```
turboquant/
├── app/
│   └── index.html          ← Complete desktop application (single file)
├── backend/
│   ├── server.py           ← Python FastAPI server
│   └── requirements.txt
├── main.js                 ← Electron main process
├── preload.js              ← Electron security layer
├── package.json            ← Electron + build config
├── start.sh                ← Launch script
└── README.md
```

---

## 🔧 Building Desktop Executable

```bash
npm install
npm run build:mac    # → dist/TurboQuant AI.dmg
npm run build:win    # → dist/TurboQuant AI Setup.exe
npm run build:linux  # → dist/TurboQuant AI.AppImage
```

---

**⚡ Made by TurboQuant Network — Quantitative Intelligence Platform**
