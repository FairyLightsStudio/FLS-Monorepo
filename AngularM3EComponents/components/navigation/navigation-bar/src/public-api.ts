/*
 * Public API Surface of navigation-bar
 */

export * from './lib/navigation-bar.component';
export * from './lib/navigation-bar-item.component';

import { MatNavigationBarComponent } from './lib/navigation-bar.component';
import { MatNavigationBarItemComponent } from './lib/navigation-bar-item.component';
import {
  MatNavigationIcon,
  MatNavigationActiveIcon,
  MatNavigationLabel,
} from '@fairylights-studio/ngx-m3-navigation-common';

export const MAT_NAVIGATION_BAR_MODULES = [
  MatNavigationBarComponent,
  MatNavigationBarItemComponent,
  MatNavigationIcon,
  MatNavigationActiveIcon,
  MatNavigationLabel,
] as const;
