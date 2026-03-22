# NebulaPlay

A full-stack media streaming app with a Rust backend and Flutter frontend.

---

## 🚀 Features

* Movie & TV browsing (TMDB)
* Streaming source aggregation
* Cross-platform frontend (Windows/Web/etc.)

---

## 📦 Project Structure

```
NebulaPlay/
├── backend/   # Rust API server
├── frontend/  # Flutter app
```

---

## ⚙️ Prerequisites

Make sure you have installed:

* Rust (Cargo)
* Flutter SDK
* Git

---

## 🔑 Environment Variables (REQUIRED)

You must create your own API keys:

### 1. TMDB API Key

* Go to: https://www.themoviedb.org/settings/api
* Create an account and generate an API key

### 2. Real-Debrid Token

* Go to: https://real-debrid.com/apitoken
* Generate your private token

---

## 🛠️ Backend Setup (Rust)

Navigate to backend:

```bash
cd backend
```

Set environment variables (PowerShell):

```powershell
$env:TMDB_API_KEY="your_tmdb_api_key"
$env:RD_TOKEN="your_real_debrid_token"
```

Run server:

```bash
cargo run
```

Backend will start at:

```
http://127.0.0.1:8080
```

---

## 🎨 Frontend Setup (Flutter)

Navigate to frontend:

```bash
cd frontend
```

Check setup:

```bash
flutter doctor
```

Run app (Windows):

```bash
flutter run -d windows
```

---

## ⚠️ Notes

* Backend must be running before frontend
* API keys are NOT included for security reasons

---

## 🧠 Tech Stack

* Backend: Rust
* Frontend: Flutter
* APIs: TMDB, Real-Debrid

---

## 🚧 Future Improvements / Roadmap

Planned features and enhancements:

* 📺 Google TV / Android TV support (Play Store release)
* 🔐 In-app API key / token configuration (no manual setup required)
* 🎬 Improved content catalogue (Netflix-style browsing experience)
* 👤 User profiles support
* 🤖 Recommendation system based on watch history and preferences
* ⏱️ Resume playback (continue watching from last timestamp)
* 🎯 Smarter UI/UX with personalized content discovery

---

## 📌 Disclaimer

This project is for educational purposes only.
Users are responsible for how they use external APIs and services.

---
