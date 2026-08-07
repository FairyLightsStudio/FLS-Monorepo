import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatExtendedFabCollapsedDirective } from '@fairylights-studio/ngx-m3-button';
import { moduleMetadata, type Meta, type StoryObj } from '@storybook/angular';
import { expect, userEvent, within } from 'storybook/test';
import {
  MAT_NAVIGATION_SUITE_MODULES,
  MatNavigationSuiteComponent,
  MatNavigationSuiteScaffoldState,
  MatNavigationSuiteScaffoldComponent,
  MatNavigationSuiteItemComponent,
  MatNavigationSuitePrimaryAction,
  type MatNavigationSuiteType,
} from '@fairylights-studio/ngx-m3-navigation-suite';

type NavigationSuiteStoryArgs = {
  selectedIndex: number;
  navSuiteType: MatNavigationSuiteType;
  alwaysShowItemLabel: boolean;
  verticalArrangement: 'top' | 'center';
  primaryActionAlignment: 'start' | 'center' | 'end';
  railShowToggle: boolean;
  visibility?: 'visible' | 'hidden';
};

type NavigationSuiteStoryItem = {
  id: string;
  label: string;
  icon: string;
  activeIcon?: string;
};

const navItems: readonly NavigationSuiteStoryItem[] = [
  { id: 'dashboard', label: 'Dashboard', icon: 'dashboard' },
  { id: 'tasks', label: 'Tasks', icon: 'task_alt' },
  { id: 'reports', label: 'Reports', icon: 'analytics' },
  { id: 'users', label: 'Users', icon: 'people' },
  { id: 'admin', label: 'Admin', icon: 'admin_panel_settings' },
];

const meta: Meta<NavigationSuiteStoryArgs> = {
  title: 'Navigation/Navigation Suite Scaffold',
  component: MatNavigationSuiteScaffoldComponent,
  subcomponents: {
    MatNavigationSuiteComponent,
    MatNavigationSuiteItemComponent,
    MatNavigationSuitePrimaryAction,
  },
  decorators: [
    moduleMetadata({
      imports: [
        CommonModule,
        MatButtonModule,
        MatIconModule,
        MatExtendedFabCollapsedDirective,
        ...MAT_NAVIGATION_SUITE_MODULES,
      ],
    }),
  ],
  argTypes: {
    selectedIndex: {
      control: { type: 'number', min: 0, max: 4 },
    },
    navSuiteType: {
      control: 'radio',
      options: ['Auto', 'BarCompact', 'BarMedium', 'RailCollapsed', 'RailExpanded'],
    },
    alwaysShowItemLabel: {
      control: 'boolean',
    },
    verticalArrangement: {
      control: 'radio',
      options: ['top', 'center'],
    },
    primaryActionAlignment: {
      control: 'radio',
      options: ['start', 'center', 'end'],
    },
  },
  args: {
    selectedIndex: 0,
    navSuiteType: 'RailCollapsed',
    alwaysShowItemLabel: true,
    verticalArrangement: 'top',
    primaryActionAlignment: 'end',
    railShowToggle: true,
  },
};

export default meta;

type Story = StoryObj<NavigationSuiteStoryArgs>;

const renderProjectedSuite: Story['render'] = (args) => ({
  props: {
    ...args,
    navItems,
    scaffoldState: new MatNavigationSuiteScaffoldState(),
  },
  template: `
    <mat-navigation-suite-scaffold
      [navSuiteType]="navSuiteType"
      [state]="scaffoldState"
      [verticalArrangement]="verticalArrangement"
      [primaryActionAlignment]="primaryActionAlignment"
      [railShowToggle]="railShowToggle"
    >
      <ng-template matNavigationSuitePrimaryAction let-collapsed="collapsed">
        <button
          matFab
          extended
          [collapsed]="collapsed"
          aria-label="Create document"
        >
          <mat-icon aria-hidden="true">edit</mat-icon>
          Compose
        </button>
      </ng-template>

      <mat-navigation-suite [alwaysShowItemLabel]="alwaysShowItemLabel" [ariaLabel]="'Workspace'">
        @for (item of navItems; track item.id; let index = $index) {
          <mat-navigation-suite-item
            [selected]="selectedIndex === index"
            [icon]="item.icon"
            [activeIcon]="item.activeIcon ?? null"
            [label]="item.label"
            (click)="selectedIndex = index"
          />
        }
      </mat-navigation-suite>

      <section style="padding: 32px">
        <h2 style="margin: 0 0 8px">Workspace</h2>
        <p style="margin: 0; max-width: 52ch">
          The scaffold switches between navigation rail and navigation bar layouts
          based on viewport size and configured breakpoints.
        </p>
      </section>
    </mat-navigation-suite-scaffold>
  `,
});

/**
 * Auto suite type: scaffold picks the best layout for the viewport.
 * Verifies all five navigation items are rendered with correct labels.
 */
export const AutoSuiteType: Story = {
  args: {
    selectedIndex: 0,
    navSuiteType: 'Auto',
  },
  render: renderProjectedSuite,
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);

    // All five navigation destinations present (rail duplicates labels,
    // bar does not — use getAllByText to handle both cases)
    for (const item of navItems) {
      const matches = canvas.getAllByText(item.label);
      await expect(matches.length).toBeGreaterThan(0);
    }
  },
};

/**
 * BarCompact with conditional primary action button.
 * The PAB only appears when the Dashboard destination is selected.
 * Verifies PAB visibility toggles based on destination.
 */
export const BarCompactPabConditional: Story = {
  args: {
    selectedIndex: 0,
    navSuiteType: 'BarCompact',
    primaryActionAlignment: 'end',
  },
  globals: {
    viewport: { value: 'mobile2', isRotated: false },
  },
  render: (args) => ({
    props: {
      ...args,
      navItems,
      scaffoldState: new MatNavigationSuiteScaffoldState(),
    },
    template: `
      <mat-navigation-suite-scaffold
        [navSuiteType]="navSuiteType"
        [state]="scaffoldState"
        [verticalArrangement]="verticalArrangement"
        [primaryActionAlignment]="primaryActionAlignment"
        [railShowToggle]="railShowToggle"
      >
        @if (selectedIndex === 0) {
          <ng-template matNavigationSuitePrimaryAction let-collapsed="collapsed">
            <button
              matFab
              extended
              [collapsed]="collapsed"
              aria-label="Create document"
            >
              <mat-icon aria-hidden="true">edit</mat-icon>
              Compose
            </button>
          </ng-template>
        }

        <mat-navigation-suite [alwaysShowItemLabel]="alwaysShowItemLabel" [ariaLabel]="'Workspace'">
          @for (item of navItems; track item.id; let index = $index) {
            <mat-navigation-suite-item
              [selected]="selectedIndex === index"
              [icon]="item.icon"
              [activeIcon]="item.activeIcon ?? null"
              [label]="item.label"
              (click)="selectedIndex = index"
            />
          }
        </mat-navigation-suite>

        <section style="padding: 32px 16px">
          <h2 style="margin: 0 0 8px">{{ navItems[selectedIndex].label }}</h2>
          <p style="margin: 0; max-width: 52ch" data-testid="pab-status">
            @if (selectedIndex === 0) {
              PAB is visible: only the Dashboard destination shows the compose button.
            } @else {
              Navigate back to Dashboard to reveal the PAB.
            }
          </p>
        </section>
      </mat-navigation-suite-scaffold>
    `,
  }),
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);

    // Dashboard is selected: PAB visible
    const composeBtn = canvas.getByLabelText('Create document');
    await expect(composeBtn).toBeInTheDocument();

    // Status text confirms PAB visibility
    await expect(canvas.getByText(/PAB is visible/)).toBeInTheDocument();
  },
};

/**
 * Interactive test verifying PabConditional visibility toggles based on selected destination.
 */
export const PabConditionalBehavior: Story = {
  name: 'Behavior/PAB Conditional',
  args: {
    selectedIndex: 0,
    navSuiteType: 'BarCompact',
    primaryActionAlignment: 'end',
  },
  globals: {
    viewport: { value: 'mobile2', isRotated: false },
  },
  render: BarCompactPabConditional.render,
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);

    // Dashboard is selected: PAB visible
    const composeBtn = canvas.getByLabelText('Create document');
    await expect(composeBtn).toBeInTheDocument();

    // Navigate to "Tasks"
    await userEvent.click(canvas.getByText('Tasks'));

    // PAB hidden when not on Dashboard
    await expect(canvas.queryByLabelText('Create document')).not.toBeInTheDocument();

    // Navigate back to Dashboard
    await userEvent.click(canvas.getByText('Dashboard'));

    // PAB reappears
    const reappearedBtn = canvas.getByLabelText('Create document');
    await expect(reappearedBtn).toBeInTheDocument();
  },
};

/**
 * Collapsed navigation rail on tablet viewport.
 * Verifies rail is collapsed and all items are reachable.
 */
export const RailCollapsed: Story = {
  render: renderProjectedSuite,
  globals: {
    viewport: { value: 'tablet', isRotated: false },
  },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);

    // All five items visible (rail labels appear twice)
    for (const item of navItems) {
      const matches = canvas.getAllByText(item.label);
      await expect(matches.length).toBeGreaterThan(0);
    }
    // If collapsed, Compose label is hidden
    await expect(canvas.getByText('Compose')).not.toBeVisible();
  },
};

/**
 * Expanded navigation rail on desktop viewport.
 * Verifies rail is expanded with center arrangement.
 */
export const RailExpanded: Story = {
  args: {
    selectedIndex: 1,
    navSuiteType: 'RailExpanded',
    verticalArrangement: 'center',
  },
  globals: {
    viewport: { value: 'desktop', isRotated: false },
  },
  render: renderProjectedSuite,
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);

    // All items visible (rail labels appear twice)
    for (const item of navItems) {
      const matches = canvas.getAllByText(item.label);
      await expect(matches.length).toBeGreaterThan(0);
    }
    // If expanded, Compose label is visible
    await expect(canvas.getByText('Compose')).toBeVisible();
  },
};

/**
 * Compact bottom bar on mobile viewport.
 * Verifies bar layout and item visibility.
 */
export const BarCompact: Story = {
  args: {
    selectedIndex: 2,
    navSuiteType: 'BarCompact',
    primaryActionAlignment: 'end',
  },
  globals: {
    viewport: { value: 'mobile2', isRotated: false },
  },
  render: renderProjectedSuite,
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);

    // Bar is rendered
    const bar = canvasElement.querySelector('mat-navigation-bar');
    await expect(bar).toBeInTheDocument();

    // All items accessible
    for (const item of navItems) {
      await expect(canvas.getByText(item.label)).toBeInTheDocument();
    }
  },
};

/**
 * Bar visibility state management: show, hide, toggle, and snap.
 * Verifies state controller methods update signal values correctly.
 */
export const BarVisibilityState: Story = {
  argTypes: {
    visibility: { control: 'radio', options: ['visible', 'hidden'] },
  },
  args: {
    selectedIndex: 0,
    navSuiteType: 'BarCompact',
    primaryActionAlignment: 'end',
    visibility: 'visible',
  },
  globals: {
    viewport: { value: 'mobile2', isRotated: false },
  },
  render: (args) => ({
    props: {
      ...args,
      navItems,
      scaffoldState: new MatNavigationSuiteScaffoldState(args.visibility),
    },
    template: `
      <mat-navigation-suite-scaffold
        [navSuiteType]="navSuiteType"
        [state]="scaffoldState"
        [verticalArrangement]="verticalArrangement"
        [primaryActionAlignment]="primaryActionAlignment"
        [railShowToggle]="railShowToggle"
      >
        <ng-template matNavigationSuitePrimaryAction let-collapsed="collapsed">
          <button
            matFab
            extended
            [collapsed]="collapsed"
            aria-label="Create document"
          >
            <mat-icon aria-hidden="true">edit</mat-icon>
            Compose
          </button>
        </ng-template>

        <mat-navigation-suite [alwaysShowItemLabel]="alwaysShowItemLabel" [ariaLabel]="'Workspace'">
          @for (item of navItems; track item.id; let index = $index) {
            <mat-navigation-suite-item
              [selected]="selectedIndex === index"
              [icon]="item.icon"
              [activeIcon]="item.activeIcon ?? null"
              [label]="item.label"
              (click)="selectedIndex = index"
            />
          }
        </mat-navigation-suite>

        <section style="display: grid; gap: 16px; padding: 32px 16px">
          <div>
            <h2 style="margin: 0 0 8px">Visibility state</h2>
            <p style="margin: 0; max-width: 52ch" data-testid="state-display">
              Current: {{ scaffoldState.currentValue() }} | Target:
              {{ scaffoldState.targetValue() }} | Animating:
              {{ scaffoldState.isAnimating() }}
            </p>
          </div>

          <div style="display: flex; flex-wrap: wrap; gap: 8px">
            <button type="button" data-testid="btn-show"
              (click)="scaffoldState.show(); visibility = 'visible'">Show</button>
            <button type="button" data-testid="btn-hide"
              (click)="scaffoldState.hide(); visibility = 'hidden'">Hide</button>
            <button type="button" data-testid="btn-toggle"
              (click)="scaffoldState.toggle(); visibility = scaffoldState.targetValue() === 'visible' ? 'hidden' : 'visible'">Toggle</button>
            <button type="button" data-testid="btn-snap"
              (click)="scaffoldState.snapTo('hidden'); visibility = 'hidden'">Snap hidden</button>
          </div>
        </section>
      </mat-navigation-suite-scaffold>
    `,
  }),
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stateDisplay = canvas.getByTestId('state-display');

    // Initial: visible, not animating
    await expect(stateDisplay).toHaveTextContent(/Current: visible/);
    await expect(stateDisplay).toHaveTextContent(/Target: visible/);
    await expect(stateDisplay).toHaveTextContent(/Animating: false/);
  },
};

/**
 * Interactive test verifying Bar visibility state methods: show, hide, snap, and toggle.
 */
export const BarVisibilityBehavior: Story = {
  name: 'Behavior/Bar Visibility',
  argTypes: {
    visibility: { control: 'radio', options: ['visible', 'hidden'] },
  },
  args: {
    selectedIndex: 0,
    navSuiteType: 'BarCompact',
    primaryActionAlignment: 'end',
    visibility: 'visible',
  },
  globals: {
    viewport: { value: 'mobile2', isRotated: false },
  },
  render: BarVisibilityState.render,
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stateDisplay = canvas.getByTestId('state-display');

    // Initial: visible
    await expect(stateDisplay).toHaveTextContent(/Current: visible/);

    // Click Hide
    await userEvent.click(canvas.getByTestId('btn-hide'));
    await expect(stateDisplay).toHaveTextContent(/Target: hidden/);

    // Click Show
    await userEvent.click(canvas.getByTestId('btn-show'));
    await expect(stateDisplay).toHaveTextContent(/Target: visible/);

    // Snap hidden (instant, no animation)
    await userEvent.click(canvas.getByTestId('btn-snap'));
    await expect(stateDisplay).toHaveTextContent(/Current: hidden/);

    // Toggle back to visible
    await userEvent.click(canvas.getByTestId('btn-toggle'));
    await expect(stateDisplay).toHaveTextContent(/Target: visible/);
  },
};

/**
 * Medium bottom bar on rotated tablet viewport.
 * Verifies horizontal layout with inline labels.
 */
export const BarMedium: Story = {
  args: {
    selectedIndex: 0,
    navSuiteType: 'BarMedium',
    primaryActionAlignment: 'end',
  },
  globals: {
    viewport: { value: 'tablet', isRotated: true },
  },
  render: renderProjectedSuite,
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);

    // Bar rendered
    const bar = canvasElement.querySelector('mat-navigation-bar');
    await expect(bar).toBeInTheDocument();

    // All items present
    for (const item of navItems) {
      await expect(canvas.getByText(item.label)).toBeInTheDocument();
    }
  },
};

/**
 * Rail visibility state management: show, hide, toggle, and snap.
 * When a rail layout is hidden, the primary action is part of the rail
 * header and hides with the rail. Uses `MatNavigationSuiteScaffoldState`
 * for animated transitions.
 */
export const RailVisibilityState: Story = {
  argTypes: {
    visibility: { control: 'radio', options: ['visible', 'hidden'] },
  },
  args: {
    selectedIndex: 0,
    navSuiteType: 'RailExpanded',
    verticalArrangement: 'center',
    visibility: 'visible',
  },
  globals: {
    viewport: { value: 'desktop', isRotated: false },
  },
  render: (args) => ({
    props: {
      ...args,
      navItems,
      scaffoldState: new MatNavigationSuiteScaffoldState(args.visibility),
    },
    template: `
      <mat-navigation-suite-scaffold
        [navSuiteType]="navSuiteType"
        [state]="scaffoldState"
        [verticalArrangement]="verticalArrangement"
        [primaryActionAlignment]="primaryActionAlignment"
        [railShowToggle]="railShowToggle"
      >
        <ng-template matNavigationSuitePrimaryAction let-collapsed="collapsed">
          <button
            matFab
            extended
            [collapsed]="collapsed"
            aria-label="Create document"
          >
            <mat-icon aria-hidden="true">edit</mat-icon>
            Compose
          </button>
        </ng-template>

        <mat-navigation-suite [alwaysShowItemLabel]="alwaysShowItemLabel" [ariaLabel]="'Workspace'">
          @for (item of navItems; track item.id; let index = $index) {
            <mat-navigation-suite-item
              [selected]="selectedIndex === index"
              [icon]="item.icon"
              [activeIcon]="item.activeIcon ?? null"
              [label]="item.label"
              (click)="selectedIndex = index"
            />
          }
        </mat-navigation-suite>

        <section style="display: grid; gap: 16px; padding: 32px">
          <div>
            <h2 style="margin: 0 0 8px">Rail visibility state</h2>
            <p style="margin: 0; max-width: 52ch" data-testid="state-display">
              Current: {{ scaffoldState.currentValue() }} | Target:
              {{ scaffoldState.targetValue() }} | Animating:
              {{ scaffoldState.isAnimating() }}
            </p>
          </div>

          <div style="display: flex; flex-wrap: wrap; gap: 8px">
            <button type="button" data-testid="btn-show"
              (click)="scaffoldState.show(); visibility = 'visible'">Show</button>
            <button type="button" data-testid="btn-hide"
              (click)="scaffoldState.hide(); visibility = 'hidden'">Hide</button>
            <button type="button" data-testid="btn-toggle"
              (click)="scaffoldState.toggle(); visibility = scaffoldState.targetValue() === 'visible' ? 'hidden' : 'visible'">Toggle</button>
            <button type="button" data-testid="btn-snap"
              (click)="scaffoldState.snapTo('hidden'); visibility = 'hidden'">Snap hidden</button>
          </div>
        </section>
      </mat-navigation-suite-scaffold>
    `,
  }),
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stateDisplay = canvas.getByTestId('state-display');

    // Initial: visible, not animating
    await expect(stateDisplay).toHaveTextContent(/Current: visible/);
    await expect(stateDisplay).toHaveTextContent(/Target: visible/);
    await expect(stateDisplay).toHaveTextContent(/Animating: false/);
  },
};

/**
 * Interactive test verifying Rail visibility state methods: show, hide, snap, and toggle.
 */
export const RailVisibilityBehavior: Story = {
  name: 'Behavior/Rail Visibility',
  argTypes: {
    visibility: { control: 'radio', options: ['visible', 'hidden'] },
  },
  args: {
    selectedIndex: 0,
    navSuiteType: 'RailExpanded',
    verticalArrangement: 'center',
    visibility: 'visible',
  },
  globals: {
    viewport: { value: 'desktop', isRotated: false },
  },
  render: RailVisibilityState.render,
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stateDisplay = canvas.getByTestId('state-display');

    // Initial: visible
    await expect(stateDisplay).toHaveTextContent(/Current: visible/);

    // Click Hide — rail + primary action both hide
    await userEvent.click(canvas.getByTestId('btn-hide'));
    await expect(stateDisplay).toHaveTextContent(/Target: hidden/);

    // Click Show — rail slides back in
    await userEvent.click(canvas.getByTestId('btn-show'));
    await expect(stateDisplay).toHaveTextContent(/Target: visible/);

    // Snap hidden (instant, no animation)
    await userEvent.click(canvas.getByTestId('btn-snap'));
    await expect(stateDisplay).toHaveTextContent(/Current: hidden/);

    // Toggle back to visible
    await userEvent.click(canvas.getByTestId('btn-toggle'));
    await expect(stateDisplay).toHaveTextContent(/Target: visible/);
  },
};
