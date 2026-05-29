# Live-Markdown-Parser

A high-performance, real-time Markdown-to-HTML rendering application built from scratch. The project features a zero-latency, full-screen split-pane web workspace backed by a custom, multi-threaded text tokenization parser engine written in Rust.

---

## 🏗️ Architectural Blueprint

The application is structured around a strict decoupling of business rules, core state manipulation, and external network systems:

```
[ Frontend Client UI ]
        │  ▲
  (JSON)▼  │(HTML)
┌────────────────────────────────────────────────────────┐
│ Backend Server                                         │
│                                                        │
│  ├── 🌐 System Layer (Axum Router & Network Fallbacks) │
│  ├── 📋 Rule Layer   (Serde Serialization Contracts)   │
│  └── ⚙️ Engine Layer (Line-by-Line Compilation Loop)  │
└────────────────────────────────────────────────────────┘
```

### 1. System Layer (Networking & Delivery)

Powered by the **Axum** framework and the **Tokio** asynchronous runtime. It manages the server's HTTP communication channels, handles incoming compilation `POST` streams on `/api/render`, and implements a fallback file server utilizing `tower-http` to deliver static web assets safely to the browser.

### 2. Rule Layer (Data Contracts)

Defines the strict type boundaries and communication structures for data entering or exiting the backend. Using **Serde**, it handles zero-allocation deserialization of raw text inputs into type-safe request models, and serializes compiled outputs back across the network wire.

### 3. Engine Layer (The Compiler Core)

A deterministic, single-pass line-by-line parsing machine. It reads text segments, identifies leading semantic block elements (headings, list items), and implements an iterative lookahead scanner to resolve alternating inline markers (`**`) into styled emphasis blocks — completely bypassing heavy external regex engines.

---

## 🛠️ Tech Stack & Key Dependencies

- **Language & Runtime:** Rust (Modern 2021 Edition), Tokio Asynchronous Runtime (Multi-threaded)
- **Web Framework:** Axum v0.7 (Type-safe routing, extractors, and response generation)
- **Serialization:** Serde v1.0 (High-performance JSON serialization macros)
- **Asset Management:** Tower-HTTP v0.5 (Static file-system serving)
- **Frontend Canvas:** Native HTML5 semantic structures, CSS Grid (Split-Pane layout), and an asynchronous JavaScript Input Event listener.

---

## 📂 Project Structure

```
live-markdown-parser/
├── src/
│   └── main.rs             # Full backend (System, Rule, and Engine Layers)
├── static/
│   └── index.html          # Frontend Split-Pane Layout, CSS & Event Scripts
├── Cargo.toml              # Rust Dependency Specifications
└── .gitignore              # Build target & system clutter exemptions
```

---

## 🚀 Getting Started & Local Execution

### Prerequisites

Make sure you have the Rust compiler and toolchain installed on your local machine:

```bash
cargo --version
```

### 1. Clone & Navigate to the Project

```bash
cd live-markdown-parser
```

### 2. Launch the Application

Compile the dependency trees and spin up the network runtime:

```bash
cargo run
```

Once initialized, you will see the success message:
👉 **🚀 Markdown Parser running at** `http://127.0.0.1:4000`

### 3. Open the Workspace

Open your preferred web browser and navigate to:

```
http://127.0.0.1:4000
```

---

## 📝 Syntax Support Features

The custom tokenization engine parses the following subsets of markdown formatting in real-time:

| Markdown Syntax | Compiled HTML Target | Visual Style Layout |
|---|---|---|
| `# Heading 1` | `<h1>Heading 1</h1>` | Large, bold, clean bottom accent border |
| `## Heading 2` | `<h2>Heading 2</h2>` | Subheading formatting with integrated top margins |
| `**bold text**` | `<strong>bold text</strong>` | Vivid high-contrast neon matrix accent coloring |
| `- List Item` | `<li>List Item</li>` | Custom square bullet layout list formatting |
| `Plain Text` | `<p>Plain Text</p>` | Organized paragraph block heights |