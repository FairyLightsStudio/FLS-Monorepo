import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { moduleMetadata, type Meta, type StoryObj } from '@storybook/angular';
import { expect, userEvent, within } from 'storybook/test';
import { MatNavigationRailToggleComponent } from '@fairylights-studio/ngx-m3-navigation-rail';

type NavigationRailToggleStoryArgs = {
  expanded: boolean;
};

const meta: Meta<NavigationRailToggleStoryArgs> = {
  title: 'Navigation/Navigation Rail/Toggle',
  component: MatNavigationRailToggleComponent,
  decorators: [
    moduleMetadata({
      imports: [MatButtonModule, MatIconModule, MatNavigationRailToggleComponent],
    }),
  ],
  argTypes: {
    expanded: {
      control: 'boolean',
    },
  },
  args: {
    expanded: false,
  },
};

export default meta;

type Story = StoryObj<NavigationRailToggleStoryArgs>;

/**
 * Toggle button with expand/collapse animation.
 * Verifies initial collapsed state, aria-expanded toggling on click,
 * and aria-label updates.
 */
export const Collapsed: Story = {
  render: (args) => ({
    props: args,
    template: `
      <mat-navigation-rail-toggle
        [expanded]="expanded"
      ></mat-navigation-rail-toggle>
    `,
  }),
  args: {
    expanded: false,
  },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const toggle = canvas.getByRole('button');

    await expect(toggle).toBeInTheDocument();
    await expect(toggle).toHaveAttribute('aria-expanded', 'false');
    await expect(toggle).toHaveAttribute('aria-label', 'Expand navigation');
  },
};

export const Expanded: Story = {
  render: Collapsed.render,
  args: {
    expanded: true,
  },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const toggle = canvas.getByRole('button');

    await expect(toggle).toBeInTheDocument();
    await expect(toggle).toHaveAttribute('aria-expanded', 'true');
    await expect(toggle).toHaveAttribute('aria-label', 'Collapse navigation');
  },
};

export const ToggleBehavior: Story = {
  name: 'Behavior/Toggle',
  render: (args) => ({
    props: args,
    template: `
      <mat-navigation-rail-toggle
        [expanded]="expanded"
        (click)="expanded = !expanded"
      ></mat-navigation-rail-toggle>
    `,
  }),
  args: {
    expanded: false,
  },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const toggle = canvas.getByRole('button');

    // Initial collapsed state
    await expect(toggle).toHaveAttribute('aria-expanded', 'false');

    // Click to expand
    await userEvent.click(toggle);
    await expect(toggle).toHaveAttribute('aria-expanded', 'true');

    // Click again to collapse
    await userEvent.click(toggle);
    await expect(toggle).toHaveAttribute('aria-expanded', 'false');
  },
};

