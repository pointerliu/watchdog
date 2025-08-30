# Watchdog Web UI

A beautiful, light-themed web interface for interacting with the Watchdog API, designed with a MacOS file manager style.

## Features

- Clean, modern interface inspired by MacOS
- Full API interaction for all Watchdog endpoints
- Responsive design that works on desktop and mobile
- Real-time API response display

## Installation

1. Navigate to the watchdog-web directory:
   ```bash
   cd crates/watchdog-web
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

## Usage

1. Start the web server:
   ```bash
   npm start
   ```

2. Open your browser and navigate to `http://localhost:3000`

## Development

For development with auto-reload:
```bash
npm run dev
```

## API Configuration

By default, the UI connects to the Watchdog API at `http://localhost:8000`. To change this:

1. Open `src/js/main.js`
2. Modify the `API_BASE_URL` variable at the top of the file
3. Save the file and refresh the browser

## Project Structure

```
watchdog-web/
├── src/
│   ├── index.html          # Main HTML file
│   ├── css/
│   │   └── style.css       # Styling with MacOS theme
│   ├── js/
│   │   └── main.js         # API interaction logic
│   └── assets/             # Images and other assets
├── server.js               # Express server
├── package.json            # Project dependencies
└── README.md               # This file
```

## Endpoints Covered

- Health Check
- Subscription Management
- Fetcher Management
- Notifier Management

Each endpoint includes forms for all available operations (GET, POST, DELETE).