import { test, expect } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  await page.goto('/swap');
});

test('swap functionality', async ({ page }) => {
  // Check initial state
  await expect(page.getByRole('banner').getByRole('heading', { name: 'slope.ski' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Swap' })).toBeVisible();
  // Select initial values and assert they are set
  await page.getByTestId('from-token-select').selectOption('BTC');
  await expect(page.getByTestId('from-token-select')).toHaveValue('BTC');
  await page.getByTestId('to-token-select').selectOption('USDT');
  await expect(page.getByTestId('to-token-select')).toHaveValue('USDT');

  // Enter amount
  await page.locator('input[type="number"]').fill('10');
  await expect(page.locator('input[type="text"][readonly]')).toHaveValue('10');

  // Swap tokens
  await page.locator('button:has-text("↓↑")').click();
  await expect(page.getByTestId('from-token-select')).toHaveValue('USDT');
  await expect(page.getByTestId('to-token-select')).toHaveValue('BTC');

  // Change token
  await page.getByTestId('from-token-select').selectOption('ETH');
  await expect(page.getByTestId('from-token-select')).toHaveValue('ETH');
});