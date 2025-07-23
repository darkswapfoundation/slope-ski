import { test, expect } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  await page.goto('/pool');
  await expect(page.getByRole('heading', { name: 'Liquidity Pools' })).toBeVisible();
});

test('pool page functionality', async ({ page }) => {
  await expect(page.getByRole('heading', { name: 'Liquidity Pools' })).toBeVisible();

  const pools = page.locator('div > h3');
  await expect(pools.first()).toBeVisible();

  const firstPool = pools.first();
  const poolLinks = firstPool.locator('xpath=..').locator('a');
  await expect(poolLinks).toHaveCount(2);
  await expect(poolLinks.first()).toHaveText('Add Liquidity');
  await expect(poolLinks.last()).toHaveText('Remove Liquidity');
});