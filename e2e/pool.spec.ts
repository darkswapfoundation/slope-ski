import { test, expect } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  await page.goto('/pool');
});

test('pool page functionality', async ({ page }) => {
  await expect(page.getByRole('heading', { name: 'Liquidity Pools' })).toBeVisible();

  const pools = page.locator('div > h3');
  await expect(pools).toHaveCount(2);

  await expect(pools.first()).toHaveText('BTC / USDC');
  await expect(pools.last()).toHaveText('ETH / USDT');

  const firstPoolLinks = page.locator('div:has-text("BTC / USDC") > a');
  await expect(firstPoolLinks).toHaveCount(2);
  await expect(firstPoolLinks.first()).toHaveText('Add Liquidity');
  await expect(firstPoolLinks.last()).toHaveText('Remove Liquidity');

  const secondPoolLinks = page.locator('div:has-text("ETH / USDT") > a');
  await expect(secondPoolLinks).toHaveCount(2);
  await expect(secondPoolLinks.first()).toHaveText('Add Liquidity');
  await expect(secondPoolLinks.last()).toHaveText('Remove Liquidity');
});