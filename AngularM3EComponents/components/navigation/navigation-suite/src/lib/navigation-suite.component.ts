import {
  ChangeDetectionStrategy,
  Component,
  TemplateRef,
  computed,
  contentChildren,
  inject,
  input,
  viewChild,
} from '@angular/core';
import { NgTemplateOutlet } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';
import {
  MatNavigationBarComponent,
  MatNavigationBarItemComponent,
} from '@fairylights-studio/ngx-m3-navigation-bar';
import { MAT_NAVIGATION_RAIL_MODULES } from '@fairylights-studio/ngx-m3-navigation-rail';
import {
  MAT_NAVIGATION_WIDGET,
  MatNavigationWidget,
  MatNavigationActiveIcon,
  MatNavigationIcon,
  MatNavigationLabel,
} from '@fairylights-studio/ngx-m3-navigation-common';
import { MatNavigationSuiteItemComponent } from './navigation-suite-item.component';
import {
  MAT_NAVIGATION_SUITE_SCAFFOLD_CONTEXT,
  MatNavigationSuiteItemContent,
} from './navigation-suite.types';

/**
 * Projection adapter owned by `mat-navigation-suite-scaffold`.
 *
 * This component intentionally requires `MAT_NAVIGATION_SUITE_SCAFFOLD_CONTEXT`.
 * Do not add standalone fallback state here; scaffold is the single owner of
 * responsive navigation type, rail expansion, and primary action placement.
 */
@Component({
  selector: 'mat-navigation-suite',
  imports: [
    NgTemplateOutlet,
    MatIconModule,
    MatNavigationBarComponent,
    MatNavigationBarItemComponent,
    MAT_NAVIGATION_RAIL_MODULES,
    MatNavigationIcon,
    MatNavigationActiveIcon,
    MatNavigationLabel,
  ],
  templateUrl: './navigation-suite.component.html',
  styleUrl: './navigation-suite.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
  providers: [
    {
      provide: MAT_NAVIGATION_WIDGET,
      useExisting: MatNavigationSuiteComponent,
    },
  ],
  host: {
    class: 'mat-navigation-suite',
  },
})
export class MatNavigationSuiteComponent implements MatNavigationWidget {
  // Proxy MAT_NAVIGATION_WIDGET properties from inner component (rail or bar)
  private readonly innerWidget = viewChild(MAT_NAVIGATION_WIDGET);
  readonly placement = computed(() => this.innerWidget()?.placement() ?? ('left' as const));
  readonly size = computed(() => this.innerWidget()?.size() ?? 80);
  readonly surfaceSize = computed(() => this.innerWidget()?.surfaceSize() ?? null);

  /** Whether bar item labels remain visible when inactive. */
  readonly alwaysShowItemLabel = input(true);

  /** Accessible label passed to the active navigation landmark. */
  readonly ariaLabel = input('');

  protected readonly projectedItems = contentChildren(MatNavigationSuiteItemComponent);
  readonly itemCount = computed(() => this.projectedItems().length);

  private readonly scaffold = inject(MAT_NAVIGATION_SUITE_SCAFFOLD_CONTEXT);

  protected readonly currentVerticalArrangement = this.scaffold.verticalArrangement;
  protected readonly primaryActionTemplate = this.scaffold.primaryActionTemplate;
  protected readonly primaryActionContext = this.scaffold.primaryActionContext;
  protected readonly isBar = this.scaffold.isBar;
  protected readonly barLayout = this.scaffold.barLayout;
  protected readonly isRailExpanded = this.scaffold.isRailExpanded;
  protected readonly railShowToggle = this.scaffold.railShowToggle;
  protected readonly currentAlwaysShowItemLabel = computed(
    () => this.barLayout() === 'horizontal' || this.alwaysShowItemLabel(),
  );

  protected toggleRailExpanded(): void {
    this.scaffold.toggleRailExpanded();
  }

  protected asTemplateRef(
    value: MatNavigationSuiteItemContent | null | undefined,
  ): TemplateRef<unknown> | null {
    return value instanceof TemplateRef ? value : null;
  }

  protected asText(value: MatNavigationSuiteItemContent | null | undefined): string {
    return typeof value === 'string' ? value : '';
  }

  protected emitProjectedItemClick(item: MatNavigationSuiteItemComponent, event: Event): void {
    item.clicked.emit(event as MouseEvent);
  }
}
