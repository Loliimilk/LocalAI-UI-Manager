[![Rust](https://img.shields.io/badge/built%20with-Rust-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Status](https://img.shields.io/badge/status-alpha-orange)]()
#### 🇷🇺 Русская версия: [README.ru.md](README.ru.md)
# LocalAI-UI-Manager
**LocalAI-UI-Manager** is a Rust-based desktop application that analyzes your system hardware, automatically selects, downloads, and configures a suitable large language model (LLM).

It provides a user-friendly graphical interface for managing and interacting with models locally.

> ⚠️ **Project status:** Active development (alpha). Core functionality is available, but many  features are still in progress.
>
> ### Current state
> - Implemented: hardware scanner and unified system data structure
> - In progress: performance recommendations and dependency installer
> - Other features are under development

## Features
- Scans your system to detect hardware capabilities
- Downloads required dependencies for running LLMs
- Selects an appropriate model based on your hardware
- Automatically configures the model for your system
- Provides a UI for model configuration
- Includes a chat interface for interacting with LLMs
- Offers an API for integration with other services

## Getting Started

To run the application locally (requires installed [Rust](https://www.rust-lang.org/tools/install)):

```bash
git clone https://github.com/Loliimilk/LocalAI-UI-Manager
cd LocalAI-UI-Manager
cargo run
