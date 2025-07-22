const express = require('express');
const { createProxyMiddleware } = require('http-proxy-middleware');

const app = express();

app.use('/api', createProxyMiddleware({ 
    target: 'http://127.0.0.1:3000', 
    changeOrigin: true 
}));

app.listen(3002, () => {
    console.log('Proxy server listening on port 3002');
});