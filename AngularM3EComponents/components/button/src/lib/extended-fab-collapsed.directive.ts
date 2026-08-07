import { Directive, booleanAttribute, inject, input } from '@angular/core';
import { MatFabButton } from '@angular/material/button';

@Directive({
  selector: `
    button[matFab][extended][collapsed],
    a[matFab][extended][collapsed],
    button[mat-fab][extended][collapsed],
    a[mat-fab][extended][collapsed]
  `,
  host: {
    '[class.mat-mdc-extended-fab-collapsed]': 'matFab.extended && collapsed()',
  },
})
export class MatExtendedFabCollapsedDirective {
  readonly matFab = inject(MatFabButton);

  /** Whether the extended FAB is visually collapsed into a regular FAB shape. */
  collapsed = input(false, { transform: booleanAttribute });
}
