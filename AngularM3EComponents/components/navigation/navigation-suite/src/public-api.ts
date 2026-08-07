/*
 * Public API Surface of navigation-suite
 */

export * from './lib/navigation-suite.types';
export * from './lib/navigation-suite-scaffold-defaults';
export * from './lib/navigation-suite-scaffold-state';
export * from './lib/navigation-suite-primary-action.directive';
export * from './lib/navigation-suite-item.component';
export * from './lib/navigation-suite.component';
export * from './lib/navigation-suite-scaffold.component';

import { MatNavigationSuiteComponent } from './lib/navigation-suite.component';
import { MatNavigationSuiteItemComponent } from './lib/navigation-suite-item.component';
import { MatNavigationSuitePrimaryAction } from './lib/navigation-suite-primary-action.directive';
import { MatNavigationSuiteScaffoldComponent } from './lib/navigation-suite-scaffold.component';

export const MAT_NAVIGATION_SUITE_MODULES = [
  MatNavigationSuiteScaffoldComponent,
  MatNavigationSuiteComponent,
  MatNavigationSuiteItemComponent,
  MatNavigationSuitePrimaryAction,
] as const;
