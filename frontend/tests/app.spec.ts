import { test, expect } from '@playwright/test';

test.describe('AsciiDiff App Shell', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('renders titlebar with app name', async ({ page }) => {
    await expect(page.locator('#titlebar')).toBeVisible();
    await expect(page.locator('.app-name')).toHaveText('AsciiDiff');
    await expect(page.locator('.file-name')).toBeVisible();
  });

  test('renders toolbar with all controls', async ({ page }) => {
    await expect(page.locator('#toolbar')).toBeVisible();
    // Open repo button
    await expect(page.getByText('Open repo')).toBeVisible();
    // View mode buttons
    await expect(page.getByText('Split')).toBeVisible();
    await expect(page.getByText('Unified')).toBeVisible();
    await expect(page.getByText('Preview')).toBeVisible();
    // Toggle buttons
    await expect(page.getByText('Highlight')).toBeVisible();
    await expect(page.getByText('Collapse unchanged')).toBeVisible();
    await expect(page.getByText('Sync scroll')).toBeVisible();
  });

  test('renders sidebar with empty state', async ({ page }) => {
    await expect(page.locator('#sidebar')).toBeVisible();
    await expect(page.locator('.sidebar-head')).toContainText('Changed files');
    // Empty state message
    await expect(page.getByText('Open a repository and select branches to compare')).toBeVisible();
  });

  test('renders split panels', async ({ page }) => {
    const panels = page.locator('#panels .panel');
    await expect(panels).toHaveCount(2);
  });

  test('renders statusbar', async ({ page }) => {
    await expect(page.locator('#statusbar')).toBeVisible();
    await expect(page.locator('#statusbar')).toContainText('additions');
    await expect(page.locator('#statusbar')).toContainText('deletions');
  });

  test('renders diff stats badges', async ({ page }) => {
    await expect(page.locator('.stat-add')).toBeVisible();
    await expect(page.locator('.stat-del')).toBeVisible();
    await expect(page.locator('.stat-mod')).toBeVisible();
  });
});

test.describe('View Mode Switching', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('split mode shows both panels', async ({ page }) => {
    const panels = page.locator('#panels .panel');
    await expect(panels).toHaveCount(2);
    // Both should be visible
    for (const panel of await panels.all()) {
      await expect(panel).toBeVisible();
    }
  });

  test('preview mode hides left panel', async ({ page }) => {
    await page.getByText('Preview').click();
    // Wait for state update
    await page.waitForTimeout(100);
    const panels = page.locator('#panels .panel');
    // In preview mode only 1 panel is visible
    const count = await panels.count();
    expect(count).toBe(1);
  });

  test('unified mode hides left panel', async ({ page }) => {
    await page.getByText('Unified').click();
    await page.waitForTimeout(100);
    const panels = page.locator('#panels .panel');
    const count = await panels.count();
    expect(count).toBe(1);
  });

  test('clicking split returns to split view', async ({ page }) => {
    await page.getByText('Preview').click();
    await page.waitForTimeout(100);
    await page.getByText('Split').click();
    await page.waitForTimeout(100);
    const panels = page.locator('#panels .panel');
    await expect(panels).toHaveCount(2);
  });
});

test.describe('Branch Modal', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('opens branch modal on pill click', async ({ page }) => {
    await page.locator('.branch-pill').first().click();
    await expect(page.locator('.modal-overlay')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Select branches to compare' })).toBeVisible();
  });

  test('branch modal has search inputs', async ({ page }) => {
    await page.locator('.branch-pill').first().click();
    const inputs = page.locator('.search-wrap input');
    await expect(inputs).toHaveCount(2);
  });

  test('branch modal closes on cancel', async ({ page }) => {
    await page.locator('.branch-pill').first().click();
    await expect(page.locator('.modal-overlay')).toBeVisible();
    await page.getByText('Cancel').click();
    await expect(page.locator('.modal-overlay')).not.toBeVisible();
  });

  test('branch modal closes on escape', async ({ page }) => {
    await page.locator('.branch-pill').first().click();
    await expect(page.locator('.modal-overlay')).toBeVisible();
    await page.keyboard.press('Escape');
    await expect(page.locator('.modal-overlay')).not.toBeVisible();
  });

  test('keyboard shortcut Ctrl+B opens branch modal', async ({ page }) => {
    await page.keyboard.press('Control+b');
    await expect(page.locator('.modal-overlay')).toBeVisible();
  });
});

test.describe('Settings Modal', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('opens settings via toolbar button', async ({ page }) => {
    await page.locator('.tb-btn .ti-settings').click();
    await expect(page.locator('.settings-overlay')).toBeVisible();
    await expect(page.getByText('Preferences')).toBeVisible();
  });

  test('keyboard shortcut Ctrl+, opens settings', async ({ page }) => {
    await page.keyboard.press('Control+,');
    await expect(page.locator('.settings-overlay')).toBeVisible();
  });

  test('settings has all nav tabs', async ({ page }) => {
    await page.keyboard.press('Control+,');
    await expect(page.getByRole('button', { name: 'Rendering' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Includes' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Git' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Syntax highlight' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Diff display' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Keybindings' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Appearance' })).toBeVisible();
  });

  test('settings tab switching works', async ({ page }) => {
    await page.keyboard.press('Control+,');
    await page.getByRole('button', { name: 'Git' }).click();
    await expect(page.getByText('Git integration')).toBeVisible();
    await expect(page.getByText('Default base branch')).toBeVisible();
  });

  test('settings closes on escape', async ({ page }) => {
    await page.keyboard.press('Control+,');
    await expect(page.locator('.settings-overlay')).toBeVisible();
    await page.keyboard.press('Escape');
    await expect(page.locator('.settings-overlay')).not.toBeVisible();
  });
});

test.describe('Toolbar Toggle Buttons', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('highlight button starts active', async ({ page }) => {
    const btn = page.locator('.tb-btn', { hasText: 'Highlight' });
    await expect(btn).toHaveClass(/active/);
  });

  test('clicking highlight toggles active state', async ({ page }) => {
    const btn = page.locator('.tb-btn', { hasText: 'Highlight' });
    await btn.click();
    await expect(btn).not.toHaveClass(/active/);
    await btn.click();
    await expect(btn).toHaveClass(/active/);
  });

  test('sync scroll button starts active', async ({ page }) => {
    const btn = page.locator('.tb-btn', { hasText: 'Sync scroll' });
    await expect(btn).toHaveClass(/active/);
  });

  test('collapse unchanged button starts inactive', async ({ page }) => {
    const btn = page.locator('.tb-btn', { hasText: 'Collapse unchanged' });
    await expect(btn).not.toHaveClass(/active/);
  });
});
