import { ChangeDetectionStrategy, Component, input } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';

/** Icon button that toggles a navigation rail between collapsed and expanded states. */
@Component({
  selector: 'mat-navigation-rail-toggle',
  imports: [MatButtonModule, MatIconModule],
  template: `
    <button
      mat-icon-button
      [class.expanded]="expanded()"
      [attr.aria-expanded]="expanded()"
      [attr.aria-label]="ariaLabel() || (expanded() ? 'Collapse navigation' : 'Expand navigation')"
    >
      <div class="icon-container">
        <mat-icon class="menu-icon">menu</mat-icon>
        <mat-icon class="menu-open-icon">menu_open</mat-icon>
      </div>
    </button>
  `,
  styleUrl: './navigation-rail-toggle.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class MatNavigationRailToggleComponent {
  /** Whether the controlled rail is currently expanded. */
  expanded = input<boolean>(false);

  /** Custom accessible label for the toggle button. */
  ariaLabel = input<string>('');
}
