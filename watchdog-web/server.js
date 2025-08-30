const express = require('express');
const path = require('path');
const { createProxyMiddleware } = require('http-proxy-middleware');

const app = express();
const PORT = process.env.PORT || 3000;

// Serve static files from the src directory
app.use(express.static(path.join(__dirname, 'src')));

// Proxy all requests to the backend service
app.use('/api', createProxyMiddleware({
    target: 'http://localhost:8080/api',
    changeOrigin: true
}));

// Proxy health check request to the backend service
app.use('/health', createProxyMiddleware({
    target: 'http://localhost:8080',
    changeOrigin: true,
    pathRewrite: {
        '^/health': ''
    }
}));

// Serve index.html for the root route
app.get('/', (req, res) => {
    res.sendFile(path.join(__dirname, 'src', 'index.html'));
});

// Start the server
app.listen(PORT, () => {
    console.log(`Watchdog Web UI is running at http://localhost:${PORT}`);
});