import { computed, inject, Injectable, Signal } from '@angular/core';
import { BreakpointObserver } from '@angular/cdk/layout';
import { toSignal } from '@angular/core/rxjs-interop';
import { map } from 'rxjs/operators';
import {
  MAT_NAVIGATION_SUITE_SCAFFOLD_DEFAULTS,
  MatNavigationSuiteResolvedType,
  MatNavigationSuiteType,
} from './navigation-suite.types';

const compactQuery = '(max-width: 599.98px)';
const mediumQuery = '(min-width: 600px) and (max-width: 839.98px)';

@Injectable({ providedIn: 'root' })
export class MatNavigationSuiteScaffoldDefaults {
  private readonly breakpointObserver = inject(BreakpointObserver);
  private readonly options = inject(MAT_NAVIGATION_SUITE_SCAFFOLD_DEFAULTS);

  private readonly adaptiveNavSuiteType = toSignal(
    this.breakpointObserver.observe([compactQuery, mediumQuery]).pipe(
      map((result) => {
        if (result.breakpoints[compactQuery]) {
          return 'BarCompact' satisfies MatNavigationSuiteResolvedType;
        }

        if (result.breakpoints[mediumQuery]) {
          return 'BarMedium' satisfies MatNavigationSuiteResolvedType;
        }

        return 'RailCollapsed' satisfies MatNavigationSuiteResolvedType;
      }),
    ),
    { initialValue: 'RailCollapsed' satisfies MatNavigationSuiteResolvedType },
  );

  private readonly configuredNavSuiteType = computed(() => {
    const configured = this.options.navSuiteType;
    return typeof configured === 'function' ? configured() : configured;
  });

  private readonly configuredIsAuto = computed(() => {
    const configured = this.configuredNavSuiteType();
    return configured === undefined || configured === 'Auto';
  });

  private readonly currentNavSuiteType = computed<MatNavigationSuiteResolvedType>(() => {
    const configured = this.configuredNavSuiteType();
    return configured === undefined || configured === 'Auto'
      ? this.adaptiveNavSuiteType()
      : configured;
  });

  navSuiteType(): Signal<MatNavigationSuiteResolvedType> {
    return this.currentNavSuiteType;
  }

  navSuiteTypeIsAuto(): Signal<boolean> {
    return this.configuredIsAuto;
  }
}
