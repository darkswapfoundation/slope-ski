import { defineConfig } from '@playwright/test';

export default defineConfig({
  use: {
    baseURL: 'http://localhost:3000',
  },
  testDir: 'e2e',
  webServer: {
    command: '/home/ghostinthegrey/.cargo/bin/trunk serve',
    url: 'http://localhost:3000',
    reuseExistingServer: !process.env.CI,
    timeout: 180000,
  },
});