import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { moduleMetadata, type Meta, type StoryObj } from '@storybook/angular';
import { expect, userEvent, within } from 'storybook/test';
import { MatExtendedFabCollapsedDirective } from '@fairylights-studio/ngx-m3-button';

type ExtendedFabStoryArgs = {
  collapsed: boolean;
  disabled: boolean;
};

const meta: Meta<ExtendedFabStoryArgs> = {
  title: 'Button/Extended FAB',
  component: MatExtendedFabCollapsedDirective,
  decorators: [
    moduleMetadata({
      imports: [MatButtonModule, MatIconModule, MatExtendedFabCollapsedDirective],
    }),
  ],
  argTypes: {
    collapsed: {
      control: 'boolean',
    },
  },
  args: {
    collapsed: false,
  },
};

export default meta;

type Story = StoryObj<ExtendedFabStoryArgs>;

/**
 * Default expanded FAB: verifies the collapsed CSS class is absent
 * and label text is accessible.
 */
export const CollapsibleFABButton: Story = {
  render: (args) => ({
    props: args,
    template: `
      <div style="display: flex; align-items: center; gap: 24px; padding: 32px">
        <button matFab extended [collapsed]="collapsed" aria-label="Compose message">
          <mat-icon>edit</mat-icon>
          Compose
        </button>

        <button matFab extended [collapsed]="collapsed" aria-label="Search workspace">
          <mat-icon>search</mat-icon>
          Search
          <mat-icon iconPositionEnd>arrow_forward</mat-icon>
        </button>
      </div>
    `,
  }),
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);

    const composeBtn = canvas.getByLabelText('Compose message');
    const searchBtn = canvas.getByLabelText('Search workspace');

    // Both buttons rendered
    await expect(composeBtn).toBeInTheDocument();
    await expect(searchBtn).toBeInTheDocument();

    // Default collapsed=false: class must NOT be present
    await expect(composeBtn).not.toHaveClass('mat-mdc-extended-fab-collapsed');
    await expect(searchBtn).not.toHaveClass('mat-mdc-extended-fab-collapsed');

    // Label text accessible
    await expect(canvas.getByText('Compose')).toBeInTheDocument();
    await expect(canvas.getByText('Search')).toBeInTheDocument();
  },
};

/**
 * Collapsed FAB: verifies the collapsed CSS class is applied.
 */
export const Collapsed: Story = {
  args: {
    collapsed: true,
  },
  render: CollapsibleFABButton.render,
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);

    const composeBtn = canvas.getByLabelText('Compose message');
    const searchBtn = canvas.getByLabelText('Search workspace');

    // collapsed=true: class must be present
    await expect(composeBtn).toHaveClass('mat-mdc-extended-fab-collapsed');
    await expect(searchBtn).toHaveClass('mat-mdc-extended-fab-collapsed');

    // Buttons still exist in DOM
    await expect(composeBtn).toBeInTheDocument();
    await expect(searchBtn).toBeInTheDocument();
  },
};
