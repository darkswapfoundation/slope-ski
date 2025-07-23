const http = require('http');

const pools = [
  {
    id: '1',
    asset_a: { name: 'Ethereum', symbol: 'ETH', icon: 'eth.png' },
    asset_b: { name: 'USD Coin', symbol: 'USDC', icon: 'usdc.png' },
    total_liquidity: 1000000.0,
    volume_24h: 50000.0,
    fees_24h: 150.0,
    apr: 10.5
  },
  {
    id: '2',
    asset_a: { name: 'Bitcoin', symbol: 'BTC', icon: 'btc.png' },
    asset_b: { name: 'Tether', symbol: 'USDT', icon: 'usdt.png' },
    total_liquidity: 2000000.0,
    volume_24h: 75000.0,
    fees_24h: 225.0,
    apr: 8.2
  },
];

const gauges = [
    { id: '1', lp_token_symbol: 'ETH/USDC', apr: 12.34, total_staked: 1234567.0 },
    { id: '2', lp_token_symbol: 'BTC/USDT', apr: 8.76, total_staked: 2345678.0 },
];

const server = http.createServer((req, res) => {
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Content-Type', 'application/json');

  if (req.url === '/pools') {
    res.end(JSON.stringify(pools));
  } else if (req.url === '/gauges') {
    res.end(JSON.stringify(gauges));
  } else {
    res.statusCode = 404;
    res.end(JSON.stringify({ error: 'Not Found' }));
  }
});

server.listen(3001, () => {
  console.log('Mock API server listening on port 3001 with corrected data and root paths.');
});