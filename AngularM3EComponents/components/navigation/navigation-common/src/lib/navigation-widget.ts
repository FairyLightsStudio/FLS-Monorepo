import { InjectionToken, Signal } from '@angular/core';

export type MatNavigationPlacement = 'left' | 'right' | 'bottom' | 'top';

export interface MatNavigationWidget {
  /** The placement orientation of the navigation component. */
  readonly placement: Signal<MatNavigationPlacement>;

  /** The dynamic size (width for vertical layout, height for horizontal layout) that the component wants. */
  readonly size: Signal<number>;

  /** The surface size of the component, if it differs from the content offset size (useful for rails during transitions). */
  readonly surfaceSize: Signal<number | null>;
}

export const MAT_NAVIGATION_WIDGET = new InjectionToken<MatNavigationWidget>(
  'MAT_NAVIGATION_WIDGET',
);
