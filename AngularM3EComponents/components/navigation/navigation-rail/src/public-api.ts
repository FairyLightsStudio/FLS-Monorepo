export * from './lib/navigation-rail.component';
export * from './lib/navigation-rail-header.component';
export * from './lib/navigation-rail-item.component';
export * from './lib/navigation-rail-toggle.component';

import { MatNavigationRailComponent } from './lib/navigation-rail.component';
import { MatNavigationRailHeaderComponent } from './lib/navigation-rail-header.component';
import { MatNavigationRailItemComponent } from './lib/navigation-rail-item.component';
import { MatNavigationRailToggleComponent } from './lib/navigation-rail-toggle.component';
import {
  MatNavigationIcon,
  MatNavigationActiveIcon,
  MatNavigationLabel,
} from '@fairylights-studio/ngx-m3-navigation-common';

export {
  MatNavigationIcon as MatNavigationRailIcon,
  MatNavigationActiveIcon as MatNavigationRailActiveIcon,
  MatNavigationLabel as MatNavigationRailLabel,
};

export const MAT_NAVIGATION_RAIL_MODULES = [
  MatNavigationRailComponent,
  MatNavigationRailHeaderComponent,
  MatNavigationRailItemComponent,
  MatNavigationRailToggleComponent,
  MatNavigationIcon,
  MatNavigationActiveIcon,
  MatNavigationLabel,
] as const;
