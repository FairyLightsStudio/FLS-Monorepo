import {
  ChangeDetectionStrategy,
  Component,
  ElementRef,
  computed,
  contentChild,
  effect,
  forwardRef,
  inject,
  input,
  OnDestroy,
  signal,
  untracked,
  viewChild,
} from '@angular/core';
import { ViewportRuler } from '@angular/cdk/scrolling';
import { NgTemplateOutlet } from '@angular/common';
import { toSignal } from '@angular/core/rxjs-interop';
import { map, startWith } from 'rxjs/operators';
import {
  MAT_NAVIGATION_WIDGET,
  MatNavigationWidget,
} from '@fairylights-studio/ngx-m3-navigation-common';
import { MatNavigationSuiteComponent } from './navigation-suite.component';
import { MatNavigationSuitePrimaryAction } from './navigation-suite-primary-action.directive';
import { MatNavigationSuiteScaffoldDefaults } from './navigation-suite-scaffold-defaults';
import { MatNavigationSuiteScaffoldState } from './navigation-suite-scaffold-state';
import {
  MAT_NAVIGATION_SUITE_SCAFFOLD_CONTEXT,
  MatNavigationSuitePrimaryActionAlignment,
  MatNavigationSuiteResolvedType,
  MatNavigationSuiteScaffoldContext,
  MatNavigationSuiteType,
  MatNavigationSuiteVerticalArrangement,
  MatNavigationSuiteVisibility,
  type MatNavigationSuitePrimaryActionContext,
} from './navigation-suite.types';

const barMediumItemWidth = 168;

// `transitionend` is the primary settlement signal, but browsers can skip it
// when a transition is cancelled, reduced to zero duration, or interrupted by
// a fast state change. The timeout fallback waits slightly past the computed
// CSS duration so scaffold state promises do not hang.
const transitionEndFallbackBufferMs = 50;
const railWidthMeasurementTolerancePx = 1;

/** Responsive scaffold that switches between navigation bar and navigation rail layouts. */
@Component({
  selector: 'mat-navigation-suite-scaffold',
  imports: [NgTemplateOutlet, MatNavigationSuiteComponent, MatNavigationSuitePrimaryAction],
  providers: [
    {
      provide: MAT_NAVIGATION_SUITE_SCAFFOLD_CONTEXT,
      useExisting: forwardRef(() => MatNavigationSuiteScaffoldComponent),
    },
  ],
  templateUrl: './navigation-suite-scaffold.component.html',
  styleUrl: './navigation-suite-scaffold.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
  host: {
    class: 'mat-navigation-suite-scaffold',
    '[class.mat-navigation-suite-scaffold--bar-compact]': 'currentNavSuiteType() === "BarCompact"',
    '[class.mat-navigation-suite-scaffold--bar-medium]': 'currentNavSuiteType() === "BarMedium"',
    '[class.mat-navigation-suite-scaffold--rail-collapsed]':
      'currentNavSuiteType() === "RailCollapsed"',
    '[class.mat-navigation-suite-scaffold--rail-expanded]':
      'currentNavSuiteType() === "RailExpanded"',
    '[class.mat-navigation-suite-scaffold--navigation-hidden]':
      'currentState().targetValue() === "hidden"',
    '[class.mat-navigation-suite-scaffold--navigation-animating]': 'currentState().isAnimating()',
  },
})
export class MatNavigationSuiteScaffoldComponent
  implements OnDestroy, MatNavigationSuiteScaffoldContext
{
  /** Explicit navigation layout, or `Auto` to use the responsive default. */
  navSuiteType = input<MatNavigationSuiteType>('Auto');

  /** External visibility/animation state controller. */
  state = input<MatNavigationSuiteScaffoldState | null>(null);

  /** Container background color token, CSS custom property, or raw CSS color. */
  containerColor = input('surface');

  /** Vertical placement of rail navigation items. */
  verticalArrangement = input<MatNavigationSuiteVerticalArrangement>('top');

  /** Placement of the primary action in bar layouts. */
  primaryActionAlignment = input<MatNavigationSuitePrimaryActionAlignment>('end');

  /** Whether the scaffold auto manages a rail expand/collapse toggle. Set to false to hide it. */
  railShowToggle = input(true);

  private readonly defaults = inject(MatNavigationSuiteScaffoldDefaults);
  private readonly viewportRuler = inject(ViewportRuler);
  private readonly defaultNavSuiteType = this.defaults.navSuiteType();
  private readonly defaultNavSuiteTypeIsAuto = this.defaults.navSuiteTypeIsAuto();
  private readonly fallbackState = new MatNavigationSuiteScaffoldState();
  private readonly primaryAction = contentChild(MatNavigationSuitePrimaryAction);
  private readonly navigationSuite = contentChild(MatNavigationSuiteComponent);
  private readonly layoutElement = viewChild<ElementRef<HTMLElement>>('layout');
  private readonly navigationElement = viewChild<ElementRef<HTMLElement>>('navigation');
  private readonly navigationWidget = contentChild(MAT_NAVIGATION_WIDGET, { descendants: true });
  private readonly requestedRailType = signal<'Collapsed' | 'Expanded' | null>(null);
  private readonly viewportWidth = toSignal(
    this.viewportRuler.change().pipe(
      startWith(null),
      map(() => this.viewportRuler.getViewportSize().width),
    ),
    { initialValue: this.viewportRuler.getViewportSize().width },
  );
  private settlementFrame: number | null = null;
  private settlementTimeout: ReturnType<typeof setTimeout> | null = null;

  currentNavSuiteType = computed<MatNavigationSuiteResolvedType>(() => {
    const requestedNavSuiteType = this.navSuiteType();
    const isAutoNavSuiteType = requestedNavSuiteType === 'Auto';
    const navSuiteType = this.resolveRequestedNavSuiteType(requestedNavSuiteType);
    const requestedRailType = this.requestedRailType();

    if (navSuiteType.startsWith('Rail') && requestedRailType !== null) {
      return requestedRailType === 'Expanded' ? 'RailExpanded' : 'RailCollapsed';
    }

    if (
      isAutoNavSuiteType &&
      this.defaultNavSuiteTypeIsAuto() &&
      this.shouldAutoBarMediumDowngrade2Compact(navSuiteType)
    ) {
      return 'BarCompact';
    }

    return navSuiteType;
  });
  currentState = computed(() => this.state() ?? this.fallbackState);
  isBar = computed(() => {
    const navSuiteType = this.currentNavSuiteType();
    return navSuiteType === 'BarCompact' || navSuiteType === 'BarMedium';
  });
  isRailExpanded = computed(() => this.currentNavSuiteType() === 'RailExpanded');
  barLayout = computed<'vertical' | 'horizontal'>(() =>
    this.currentNavSuiteType() === 'BarMedium' ? 'horizontal' : 'vertical',
  );
  primaryActionTemplate = computed(() => this.primaryAction()?.templateRef ?? null);
  primaryActionContext = computed<MatNavigationSuitePrimaryActionContext>(() => {
    const isBar = this.isBar();
    const isRailExpanded = this.isRailExpanded();
    const collapsed = isBar ? false : !isRailExpanded;

    return {
      $implicit: collapsed,
      collapsed,
      isBar,
      isRailExpanded,
    };
  });

  protected containerColorValue = computed(() => this.toCssColor(this.containerColor()));

  protected navigationSizeValue = computed(() => {
    const widget = this.navigationWidget();
    if (widget) {
      return `${widget.size()}px`;
    }
    return this.defaultNavigationSize(this.currentNavSuiteType());
  });

  protected navigationSurfaceSizeValue = computed(() => {
    const widget = this.navigationWidget();
    if (widget) {
      const surfaceSize = widget.surfaceSize();
      return surfaceSize !== null ? `${surfaceSize}px` : `${widget.size()}px`;
    }
    return this.defaultNavigationSurfaceSize(this.currentNavSuiteType());
  });

  private readonly transitionSettlementEffect = effect(() => {
    const state = this.currentState();
    const targetValue = state.targetValue();
    const isAnimating = state.isAnimating();
    const navSuiteType = this.currentNavSuiteType();

    if (isAnimating) {
      this.scheduleTransitionSettlement(state, targetValue, navSuiteType);
    } else {
      this.clearTransitionSettlement();
    }
  });

  ngOnDestroy(): void {
    this.clearTransitionSettlement();
  }

  toggleRailExpanded(): void {
    if (!this.currentNavSuiteType().startsWith('Rail')) {
      return;
    }

    if (this.isRailExpanded()) {
      this.requestedRailType.set('Collapsed');
      return;
    }

    this.requestedRailType.set('Expanded');
  }

  protected handleLayoutTransitionEnd(event: TransitionEvent): void {
    if (!this.currentState().isAnimating() || !this.isSettlementTransitionEnd(event)) {
      return;
    }

    this.completeCurrentTransition();
  }

  protected handleLayoutTransitionCancel(event: TransitionEvent): void {
    if (!this.isSettlementTransitionProperty(event)) {
      return;
    }

    const state = this.currentState();

    if (state.isAnimating()) {
      this.scheduleTransitionSettlement(state, state.targetValue(), this.currentNavSuiteType());
    }
  }

  // 如果用户放了很多的 item 在 BarMedium 中，那么在确定BarMedium撑不下去时，降级到 BarCompact，避免 item 标签重叠。
  private shouldAutoBarMediumDowngrade2Compact(
    navSuiteType: MatNavigationSuiteResolvedType,
  ): boolean {
    if (navSuiteType !== 'BarMedium') {
      return false;
    }

    const itemCount = this.navigationSuite()?.itemCount() ?? 0;
    return itemCount > 0 && this.viewportWidth() < itemCount * barMediumItemWidth;
  }

  private resolveRequestedNavSuiteType(
    navSuiteType: MatNavigationSuiteType,
  ): MatNavigationSuiteResolvedType {
    return navSuiteType === 'Auto' ? this.defaultNavSuiteType() : navSuiteType;
  }

  private scheduleTransitionSettlement(
    state: MatNavigationSuiteScaffoldState,
    targetValue: MatNavigationSuiteVisibility,
    navSuiteType: MatNavigationSuiteResolvedType,
  ): void {
    this.clearTransitionSettlement();

    if (typeof window === 'undefined') {
      state._completeTransition();
      return;
    }

    this.settlementFrame = window.requestAnimationFrame(() => {
      this.settlementFrame = null;

      if (!this.isExpectedTransition(state, targetValue)) {
        return;
      }

      const totalTransitionMs = this.settlementTransitionTotalMs(navSuiteType);

      if (this.prefersReducedMotion() || totalTransitionMs === 0) {
        this.completeCurrentTransition();
        return;
      }

      // Fallback for missing transitionend events. The state check inside the
      // callback keeps old timers from completing a newer interrupted transition.
      this.settlementTimeout = setTimeout(() => {
        if (this.isExpectedTransition(state, targetValue)) {
          this.completeCurrentTransition();
        }
      }, totalTransitionMs + transitionEndFallbackBufferMs);
    });
  }

  private clearTransitionSettlement(): void {
    if (this.settlementFrame !== null && typeof window !== 'undefined') {
      window.cancelAnimationFrame(this.settlementFrame);
      this.settlementFrame = null;
    }

    if (this.settlementTimeout !== null) {
      clearTimeout(this.settlementTimeout);
      this.settlementTimeout = null;
    }
  }

  private completeCurrentTransition(): void {
    this.clearTransitionSettlement();
    this.currentState()._completeTransition();
  }

  private isExpectedTransition(
    state: MatNavigationSuiteScaffoldState,
    targetValue: MatNavigationSuiteVisibility,
  ): boolean {
    return (
      this.currentState() === state && state.targetValue() === targetValue && state.isAnimating()
    );
  }

  private isLayoutTransitionProperty(propertyName: string): boolean {
    return (
      propertyName === 'grid-template-rows' ||
      propertyName === 'grid-template-columns' ||
      propertyName === 'all'
    );
  }

  private isTransformTransitionProperty(propertyName: string): boolean {
    return propertyName === 'transform' || propertyName === 'all';
  }

  private layoutTransitionProperty(navSuiteType: MatNavigationSuiteResolvedType): string {
    return this.isNavigationBar(navSuiteType) ? 'grid-template-rows' : 'grid-template-columns';
  }

  private isSettlementTransitionEnd(event: TransitionEvent): boolean {
    if (!this.isSettlementTransitionProperty(event)) {
      return false;
    }

    const navSuiteType = this.currentNavSuiteType();
    const eventTotalMs = this.transitionTotalMs(event.target as HTMLElement, event.propertyName);

    return eventTotalMs >= this.settlementTransitionTotalMs(navSuiteType);
  }

  private isSettlementTransitionProperty(event: TransitionEvent): boolean {
    const layoutElement = this.layoutElement()?.nativeElement;
    const navigationElement = this.navigationElement()?.nativeElement;
    const eventTarget = event.target;

    if (eventTarget === layoutElement && this.isLayoutTransitionProperty(event.propertyName)) {
      return true;
    }

    return (
      !this.isNavigationBar(this.currentNavSuiteType()) &&
      eventTarget === navigationElement &&
      this.isTransformTransitionProperty(event.propertyName)
    );
  }

  private settlementTransitionTotalMs(navSuiteType: MatNavigationSuiteResolvedType): number {
    const layoutElement = this.layoutElement()?.nativeElement;

    if (layoutElement === undefined) {
      return 0;
    }

    const layoutTotalMs = this.transitionTotalMs(
      layoutElement,
      this.layoutTransitionProperty(navSuiteType),
    );

    if (this.isNavigationBar(navSuiteType)) {
      return layoutTotalMs;
    }

    const navigationElement = this.navigationElement()?.nativeElement;
    const navigationTotalMs =
      navigationElement === undefined ? 0 : this.transitionTotalMs(navigationElement, 'transform');

    // Rail mode mirrors mat-sidenav: the surface slides with transform while
    // the content offset animates separately. State promises settle after the
    // slower of those two transitions.
    return Math.max(layoutTotalMs, navigationTotalMs);
  }

  private transitionTotalMs(element: HTMLElement, propertyName: string): number {
    const styles = getComputedStyle(element);
    const properties = styles.transitionProperty.split(',').map((property) => property.trim());
    const durations = this.parseTransitionTimeList(styles.transitionDuration);
    const delays = this.parseTransitionTimeList(styles.transitionDelay);
    let totalMs = 0;

    for (let index = 0; index < properties.length; index += 1) {
      const property = properties[index];

      if (property !== propertyName && property !== 'all') {
        continue;
      }

      const duration = durations[index % durations.length] ?? 0;
      const delay = delays[index % delays.length] ?? 0;
      totalMs = Math.max(totalMs, duration + delay);
    }

    return totalMs;
  }

  private parseTransitionTimeList(value: string): number[] {
    return value.split(',').map((time) => {
      const trimmed = time.trim();
      const parsed = Number.parseFloat(trimmed);

      if (!Number.isFinite(parsed)) {
        return 0;
      }

      return trimmed.endsWith('ms') ? parsed : parsed * 1000;
    });
  }

  private prefersReducedMotion(): boolean {
    return (
      typeof window !== 'undefined' &&
      window.matchMedia?.('(prefers-reduced-motion: reduce)').matches === true
    );
  }

  private defaultNavigationSize(navSuiteType: MatNavigationSuiteResolvedType): string {
    switch (navSuiteType) {
      case 'BarCompact':
        return 'var(--flight-nav-suite-scaffold-bar-compact-height, var(--flight-nav-bar-container-height, 80px))';
      case 'BarMedium':
        return 'var(--flight-nav-suite-scaffold-bar-medium-height, var(--flight-nav-bar-container-horizontal-height, 64px))';
      case 'RailCollapsed':
        return 'var(--flight-nav-rail-container-collapsed-width, 80px)';
      case 'RailExpanded':
        return 'var(--flight-nav-suite-scaffold-rail-expanded-width, var(--flight-nav-rail-container-collapsed-width, 80px))';
    }
  }

  private defaultNavigationSurfaceSize(navSuiteType: MatNavigationSuiteResolvedType): string {
    switch (navSuiteType) {
      case 'BarCompact':
      case 'BarMedium':
      case 'RailCollapsed':
        return this.defaultNavigationSize(navSuiteType);
      case 'RailExpanded':
        // Only the absolutely positioned surface may fall back to max-content.
        // The grid content offset uses a real length until measurement.
        return 'var(--flight-nav-suite-scaffold-rail-expanded-width, max-content)';
    }
  }

  private isNavigationBar(navSuiteType: MatNavigationSuiteResolvedType): boolean {
    return navSuiteType === 'BarCompact' || navSuiteType === 'BarMedium';
  }

  private toCssColor(color: string): string {
    const trimmed = color.trim();

    if (
      trimmed.startsWith('var(') ||
      trimmed.startsWith('#') ||
      trimmed.startsWith('rgb') ||
      trimmed.startsWith('hsl') ||
      trimmed === 'transparent'
    ) {
      return trimmed;
    }

    if (trimmed.startsWith('--')) {
      return `var(${trimmed})`;
    }

    return `var(--mat-sys-${trimmed}, ${trimmed})`;
  }
}
