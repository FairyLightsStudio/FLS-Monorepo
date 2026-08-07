import { InjectionToken, type Signal, type TemplateRef } from '@angular/core';

export type MatNavigationSuiteResolvedType =
  | 'BarCompact'
  | 'BarMedium'
  | 'RailCollapsed'
  | 'RailExpanded';

export type MatNavigationSuiteType = 'Auto' | MatNavigationSuiteResolvedType;

export type MatNavigationSuiteVerticalArrangement = 'top' | 'center';

export type MatNavigationSuitePrimaryActionAlignment = 'start' | 'center' | 'end';

export type MatNavigationSuiteVisibility = 'visible' | 'hidden';

export type MatNavigationSuiteItemContent = string | TemplateRef<unknown>;

export interface MatNavigationSuitePrimaryActionContext {
  $implicit: boolean;
  collapsed: boolean;
  isBar: boolean;
  isRailExpanded: boolean;
}

export interface MatNavigationSuiteScaffoldDefaultOptions {
  navSuiteType?: MatNavigationSuiteType | Signal<MatNavigationSuiteType>;
}

/**
 * Navigation suite state is owned by `mat-navigation-suite-scaffold`.
 * `mat-navigation-suite` requires this context and must not reimplement
 * fallback navigation type or rail expansion state.
 */
export interface MatNavigationSuiteScaffoldContext {
  currentNavSuiteType: Signal<MatNavigationSuiteResolvedType>;
  isBar: Signal<boolean>;
  isRailExpanded: Signal<boolean>;
  barLayout: Signal<'vertical' | 'horizontal'>;
  verticalArrangement: Signal<MatNavigationSuiteVerticalArrangement>;
  primaryActionTemplate: Signal<TemplateRef<MatNavigationSuitePrimaryActionContext> | null>;
  primaryActionContext: Signal<MatNavigationSuitePrimaryActionContext>;
  toggleRailExpanded(): void;
  railShowToggle: Signal<boolean>;
}

export const MAT_NAVIGATION_SUITE_SCAFFOLD_DEFAULTS =
  new InjectionToken<MatNavigationSuiteScaffoldDefaultOptions>(
    'MAT_NAVIGATION_SUITE_SCAFFOLD_DEFAULTS',
    {
      providedIn: 'root',
      factory: () => ({}),
    },
  );

export const MAT_NAVIGATION_SUITE_SCAFFOLD_CONTEXT =
  new InjectionToken<MatNavigationSuiteScaffoldContext>('MAT_NAVIGATION_SUITE_SCAFFOLD_CONTEXT');
