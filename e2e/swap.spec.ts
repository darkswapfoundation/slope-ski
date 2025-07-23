import { test, expect } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  await page.goto('/swap');
  await expect(page.getByRole('heading', { name: 'Swap' })).toBeVisible();
});

test('swap functionality', async ({ page }) => {
  // Check initial state
  await expect(page.getByRole('banner').getByRole('heading', { name: 'slope.ski' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Swap' })).toBeVisible();

  // Select initial values and assert they are set
  const fromTokenSelect = page.getByTestId('from-token-select');
  const toTokenSelect = page.getByTestId('to-token-select');

  const fromTokenOptions = await fromTokenSelect.locator('option').allTextContents();
  const toTokenOptions = await toTokenSelect.locator('option').allTextContents();

  const firstToken = fromTokenOptions[1];
  const secondToken = toTokenOptions[2];

  await fromTokenSelect.selectOption(firstToken);
  await expect(fromTokenSelect).toHaveValue(firstToken);
  await toTokenSelect.selectOption(secondToken);
  await expect(toTokenSelect).toHaveValue(secondToken);

  // Enter amount
  await page.locator('input[type="number"]').fill('10');
  await expect(page.locator('input[type="text"][readonly]')).toHaveValue('10');

  // Swap tokens
  await page.locator('button:has-text("↓↑")').click();
  await expect(fromTokenSelect).toHaveValue(secondToken);
  await expect(toTokenSelect).toHaveValue(firstToken);

  // Change token
  const thirdToken = fromTokenOptions[3];
  await fromTokenSelect.selectOption(thirdToken);
  await expect(fromTokenSelect).toHaveValue(thirdToken);
});