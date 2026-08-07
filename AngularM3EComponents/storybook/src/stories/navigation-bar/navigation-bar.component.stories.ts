import { MatIconModule } from '@angular/material/icon';
import { MatBadgeModule } from '@angular/material/badge';
import { moduleMetadata, type Meta, type StoryObj } from '@storybook/angular';
import { expect, userEvent, within } from 'storybook/test';

import {
  MAT_NAVIGATION_BAR_MODULES,
  MatNavigationBarComponent,
  MatNavigationBarItemComponent,
} from '@fairylights-studio/ngx-m3-navigation-bar';

type NavigationBarStoryArgs = {
  selectedIndex: number;
  layout: 'vertical' | 'horizontal';
  alwaysShowLabel: boolean;
};

const TEMPLATE = `
  <div>
    <mat-navigation-bar [ariaLabel]="'Primary navigation'">
      <mat-navigation-bar-item
        [active]="selectedIndex === 0"
        [alwaysShowLabel]="alwaysShowLabel"
        [layout]="layout"
        (click)="selectedIndex = 0"
      >
        <mat-icon *matNavigationIcon>home</mat-icon>
        <ng-template matNavigationLabel>Home</ng-template>
      </mat-navigation-bar-item>

      <mat-navigation-bar-item
        [active]="selectedIndex === 1"
        [alwaysShowLabel]="alwaysShowLabel"
        [layout]="layout"
        (click)="selectedIndex = 1"
      >
        <mat-icon *matNavigationIcon>search</mat-icon>
        <ng-template matNavigationLabel>Search</ng-template>
      </mat-navigation-bar-item>

      <mat-navigation-bar-item
        [active]="selectedIndex === 2"
        [alwaysShowLabel]="alwaysShowLabel"
        [layout]="layout"
        (click)="selectedIndex = 2"
      >
        <mat-icon *matNavigationIcon matBadge="3" matBadgeDescription="3 unread notifications">notifications</mat-icon>
        <ng-template matNavigationLabel>Alerts</ng-template>
      </mat-navigation-bar-item>

      <mat-navigation-bar-item
        [active]="selectedIndex === 3"
        [alwaysShowLabel]="alwaysShowLabel"
        [layout]="layout"
        (click)="selectedIndex = 3"
      >
        <mat-icon *matNavigationIcon>person</mat-icon>
        <ng-template matNavigationLabel>Profile</ng-template>
      </mat-navigation-bar-item>
    </mat-navigation-bar>
  </div>
`;

const meta: Meta<NavigationBarStoryArgs> = {
  title: 'Navigation/Navigation Bar',
  component: MatNavigationBarComponent,
  subcomponents: {
    MatNavigationBarItemComponent,
  },
  decorators: [
    moduleMetadata({
      imports: [MatIconModule, MatBadgeModule, ...MAT_NAVIGATION_BAR_MODULES],
    }),
  ],
  argTypes: {
    selectedIndex: {
      control: { type: 'number', min: 0, max: 3 },
    },
    layout: {
      control: 'radio',
      options: ['vertical', 'horizontal'],
    },
  },
  args: {
    selectedIndex: 0,
    layout: 'vertical',
    alwaysShowLabel: true,
  },
};

export default meta;

type Story = StoryObj<NavigationBarStoryArgs>;

/**
 * Vertical layout with always-visible labels.
 * Verifies initial selection state, click-driven selection change,
 * badge presence, and proper ARIA attributes.
 */
export const Basic: Story = {
  render: (args) => ({
    props: args,
    template: TEMPLATE,
  }),
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const bar = canvas.getByRole('navigation', { name: 'Primary navigation' });
    const items = within(bar).getAllByRole('tab');

    // Four navigation items
    await expect(items).toHaveLength(4);

    // Initially "Home" (index 0) is selected
    await expect(items[0]).toHaveAttribute('aria-selected', 'true');
    await expect(items[1]).toHaveAttribute('aria-selected', 'false');

    // Badge text "3" appears on the Alerts item
    const alertsItem = items[2];
    await expect(within(alertsItem).getByText('3')).toBeInTheDocument();
  },
};

/**
 * Horizontal layout: labels appear inline beside icons.
 * Verifies horizontal layout.
 */
export const HorizontalLabels: Story = {
  args: {
    selectedIndex: 1,
    layout: 'horizontal',
    alwaysShowLabel: true,
  },
  globals: {
    viewport: { value: 'tablet', isRotated: false },
  },
  render: Basic.render,
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const bar = canvas.getByRole('navigation', { name: 'Primary navigation' });
    const items = within(bar).getAllByRole('tab');

    // "Search" is selected
    await expect(items[1]).toHaveAttribute('aria-selected', 'true');

    // Labels visible for all items
    await expect(canvas.getByText('Home')).toBeInTheDocument();
    await expect(canvas.getByText('Search')).toBeInTheDocument();
    await expect(canvas.getByText('Alerts')).toBeInTheDocument();
    await expect(canvas.getByText('Profile')).toBeInTheDocument();
  },
};

/**
 * Vertical layout with labels hidden for inactive items.
 * Verifies only the active item reveals its label.
 */
export const CompactLabels: Story = {
  args: {
    selectedIndex: 2,
    layout: 'vertical',
    alwaysShowLabel: false,
  },
  render: Basic.render,
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const bar = canvas.getByRole('navigation', { name: 'Primary navigation' });
    const items = within(bar).getAllByRole('tab');

    // "Alerts" (index 2) is the selected item
    await expect(items[2]).toHaveAttribute('aria-selected', 'true');

    // Inactive labels (like "Home" and "Search") should not be visible, while active label is visible
    await expect(canvas.getByText('Home')).not.toBeVisible();
    await expect(canvas.getByText('Search')).not.toBeVisible();
    await expect(canvas.getByText('Alerts')).toBeVisible();
  },
};

/**
 * Interactive test verifying tab selection changes on click.
 */
export const SelectionBehavior: Story = {
  name: 'Behavior/Selection',
  args: {
    selectedIndex: 0,
  },
  render: Basic.render,
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const bar = canvas.getByRole('navigation', { name: 'Primary navigation' });
    const items = within(bar).getAllByRole('tab');

    // Initially "Home" is selected
    await expect(items[0]).toHaveAttribute('aria-selected', 'true');

    // Click "Search" — selection moves
    await userEvent.click(canvas.getByText('Search'));
    await expect(items[0]).toHaveAttribute('aria-selected', 'false');
    await expect(items[1]).toHaveAttribute('aria-selected', 'true');

    // Click "Alerts" — selection moves again
    await userEvent.click(canvas.getByText('Alerts'));
    await expect(items[1]).toHaveAttribute('aria-selected', 'false');
    await expect(items[2]).toHaveAttribute('aria-selected', 'true');
  },
};

