import {
  AfterContentInit,
  AfterViewInit,
  Component,
  ContentChildren,
  ElementRef,
  inject,
  input,
  OnDestroy,
  QueryList,
  signal,
  computed,
  effect,
  untracked,
} from '@angular/core';
import { FocusKeyManager, type FocusableOption } from '@angular/cdk/a11y';
import { ENTER, SPACE, hasModifierKey } from '@angular/cdk/keycodes';
import {
  MAT_NAVIGATION_WIDGET,
  MatNavigationWidget,
  MatNavigationPlacement,
} from '@fairylights-studio/ngx-m3-navigation-common';
import { MatNavigationRailItemComponent } from './navigation-rail-item.component';

export type MatNavRailIndicatorShape = 'hug' | 'fill';

/** Side navigation container for medium and expanded layouts. */
@Component({
  selector: 'mat-navigation-rail',
  imports: [],
  template: `
    <div class="mat-nav-rail-container">
      <ng-content select="mat-navigation-rail-header"></ng-content>
      <div class="mat-nav-rail-content">
        <ng-content></ng-content>
      </div>
    </div>
  `,
  styleUrl: './navigation-rail.component.scss',
  providers: [
    {
      provide: MAT_NAVIGATION_WIDGET,
      useExisting: MatNavigationRailComponent,
    },
  ],
  host: {
    role: 'navigation',
    '[attr.aria-label]': 'ariaLabel() || null',
    '[class.mat-nav-rail-expanded]': 'expanded()',
    '[class.mat-nav-rail-has-divider]': 'showDivider()',
    '[attr.data-indicator-shape]': 'indicatorShape()',
    '[attr.data-vertical-arrangement]': 'verticalArrangement()',
    '(keydown)': '_handleKeydown($event)',
    '(transitionend)': '_handleTransitionEnd($event)',
  },
})
export class MatNavigationRailComponent
  implements AfterContentInit, AfterViewInit, OnDestroy, MatNavigationWidget
{
  /** Whether the rail shows expanded labels and expanded width. */
  expanded = input<boolean>(false);

  /** Shape used for each active item indicator. */
  indicatorShape = input<MatNavRailIndicatorShape>('hug');

  /** Whether to draw the divider along the rail edge. */
  showDivider = input<boolean>(false);

  /** Vertical alignment for the rail item group. */
  verticalArrangement = input<'top' | 'center'>('top');

  /** Accessible label for the navigation landmark. */
  ariaLabel = input<string>('');

  // MatNavigationWidget interface implementation
  readonly placement = computed(() => 'left' as const);
  readonly size = signal<number>(80);
  readonly surfaceSize = signal<number | null>(null);

  private readonly _elementRef = inject(ElementRef);
  private _cachedExpandedWidth = 80;
  private _cachedCollapsedWidth = 80;
  private _previousExpanded?: boolean;

  private railSurfaceSizeResetTimeout: ReturnType<typeof setTimeout> | null = null;

  constructor() {
    effect(() => {
      const isExpanded = this.expanded();

      if (typeof window === 'undefined') {
        return;
      }

      untracked(() => {
        const isInitial = this._previousExpanded === undefined;
        const hasChanged = this._previousExpanded !== isExpanded;
        this._previousExpanded = isExpanded;

        if (isExpanded) {
          this.clearRailSurfaceSizeReset();
          // Apply pre-cached expanded width immediately, letting grid offset transition smoothly with max-width animation
          this.size.set(this._cachedExpandedWidth);
          this.surfaceSize.set(null);
        } else {
          if (this.prefersReducedMotion()) {
            this.size.set(this._cachedCollapsedWidth);
            this.surfaceSize.set(null);
          } else {
            // Freeze surfaceSize to prevent scaffold navigation container clipping during collapse
            this.surfaceSize.set(this._cachedExpandedWidth);
            this.size.set(this._cachedCollapsedWidth);

            if (isInitial || !hasChanged) {
              this.surfaceSize.set(null);
            } else {
              this.scheduleRailSurfaceSizeReset();
            }
          }
        }
      });
    });
  }

  private prefersReducedMotion(): boolean {
    return (
      typeof window !== 'undefined' &&
      window.matchMedia?.('(prefers-reduced-motion: reduce)').matches === true
    );
  }

  private measureCollapsedWidth(): number {
    const host = this._elementRef.nativeElement;
    const styles = getComputedStyle(host);
    const collapsedWidthVar = styles
      .getPropertyValue('--flight-nav-rail-container-collapsed-width')
      .trim();
    if (collapsedWidthVar) {
      const parsed = Number.parseFloat(collapsedWidthVar);
      if (Number.isFinite(parsed)) {
        return parsed;
      }
    }
    return 80;
  }

  private measureIntrinsicExpandedWidth(): number {
    const host = this._elementRef.nativeElement;

    // Backup original style states
    const originalMaxWidth = host.style.maxWidth;
    const originalTransition = host.style.transition;
    const originalWidth = host.style.width;

    const hadExpandedClass = host.classList.contains('mat-nav-rail-expanded');

    // Backup child item expanded states
    const items = Array.from(host.querySelectorAll('mat-navigation-rail-item')) as HTMLElement[];
    const itemStates = items.map((item) => ({
      el: item,
      hadClass: item.classList.contains('mat-nav-rail-item-expanded'),
    }));

    // Backup FAB collapsed states
    const fabs = Array.from(
      host.querySelectorAll('[mat-extended-fab], .mat-mdc-extended-fab'),
    ) as HTMLElement[];
    const fabStates = fabs.map((fab) => ({
      el: fab,
      hadClass: fab.classList.contains('mat-mdc-extended-fab-collapsed'),
    }));

    // Temporarily force expanded state for accurate DOM measurement
    host.classList.add('mat-nav-rail-measure-sandbox');
    host.style.transition = 'none';
    host.style.maxWidth = 'none';
    host.style.width = 'max-content';

    if (!hadExpandedClass) {
      host.classList.add('mat-nav-rail-expanded');
    }

    itemStates.forEach((state) => {
      if (!state.hadClass) {
        state.el.classList.add('mat-nav-rail-item-expanded');
      }
    });

    fabStates.forEach((state) => {
      if (state.hadClass) {
        state.el.classList.remove('mat-mdc-extended-fab-collapsed');
      }
    });

    // Perform measurement
    const intrinsicWidth = host.offsetWidth;

    // Restore original styles and classes
    host.style.maxWidth = originalMaxWidth;
    host.style.transition = originalTransition;
    host.style.width = originalWidth;

    if (!hadExpandedClass) {
      host.classList.remove('mat-nav-rail-expanded');
    }

    itemStates.forEach((state) => {
      if (!state.hadClass) {
        state.el.classList.remove('mat-nav-rail-item-expanded');
      }
    });

    fabStates.forEach((state) => {
      if (state.hadClass) {
        state.el.classList.add('mat-mdc-extended-fab-collapsed');
      }
    });

    // Force a synchronous reflow while transition is still blocked to commit the reverted collapsed layout state
    if (typeof window !== 'undefined') {
      const _triggerReflow = host.offsetWidth;
    }

    // Unblock transitions only after everything is completely reverted and layout cache is updated
    host.classList.remove('mat-nav-rail-measure-sandbox');

    return intrinsicWidth > 0 ? intrinsicWidth : 200;
  }

  private scheduleRailSurfaceSizeReset(): void {
    this.clearRailSurfaceSizeReset();
    this.railSurfaceSizeResetTimeout = setTimeout(() => {
      this.railSurfaceSizeResetTimeout = null;
      this.surfaceSize.set(null);
    }, 5000); // 5s fallback safety net in case transitionend does not fire
  }

  private clearRailSurfaceSizeReset(): void {
    if (this.railSurfaceSizeResetTimeout !== null) {
      clearTimeout(this.railSurfaceSizeResetTimeout);
      this.railSurfaceSizeResetTimeout = null;
    }
  }

  protected _handleTransitionEnd(event: TransitionEvent): void {
    if (event.propertyName === 'max-width' || event.propertyName === 'width') {
      this.clearRailSurfaceSizeReset();
      this.surfaceSize.set(null);
    }
  }

  ngAfterViewInit(): void {
    if (typeof window !== 'undefined') {
      // FIXME: 如果item在rail的生命周期内有变化，例如出现了一些更长的label，需要重新测算expandedWidth，目前只是在ngAfterViewInit中测算了一次
      // Async measure on startup to get the ideal expanded width and store in cache
      Promise.resolve().then(() => {
        const expandedWidth = this.measureIntrinsicExpandedWidth();
        this._cachedExpandedWidth = expandedWidth;

        const collapsedWidth = this.measureCollapsedWidth();
        this._cachedCollapsedWidth = collapsedWidth;

        if (this.expanded()) {
          this.size.set(expandedWidth);
        } else {
          this.size.set(collapsedWidth);
        }
      });
    }
  }

  @ContentChildren(MatNavigationRailItemComponent, { descendants: true })
  protected _items!: QueryList<MatNavigationRailItemComponent>;

  private _keyManager!: FocusKeyManager<MatNavigationRailItemComponent>;

  ngAfterContentInit(): void {
    this._keyManager = new FocusKeyManager<MatNavigationRailItemComponent>(
      this._items as unknown as QueryList<MatNavigationRailItemComponent & FocusableOption>,
    )
      .withVerticalOrientation()
      .withHomeAndEnd()
      .withWrap()
      .withTypeAhead();

    this._setInitialActiveItem();

    // Dynamically update cached expanded size when items list changes
    this._items.changes.subscribe(() => {
      if (typeof window !== 'undefined') {
        Promise.resolve().then(() => {
          const expandedWidth = this.measureIntrinsicExpandedWidth();
          this._cachedExpandedWidth = expandedWidth;
          if (this.expanded()) {
            this.size.set(expandedWidth);
          }
        });
      }
    });
  }

  protected _handleKeydown(event: KeyboardEvent): void {
    if (hasModifierKey(event)) return;

    const item = this._getEventItem(event);
    if (!item) return;
    this._updateActiveItem(item);

    switch (event.keyCode) {
      case ENTER:
      case SPACE:
        event.preventDefault();
        item._getButtonElement().click();
        break;
      default:
        this._keyManager.onKeydown(event);
    }
  }

  ngOnDestroy(): void {
    this._keyManager?.destroy();
    this.clearRailSurfaceSizeReset();
  }

  private _getEventItem(event: KeyboardEvent): MatNavigationRailItemComponent | undefined {
    const target = event.target;
    if (!(target instanceof Node)) return undefined;

    return this._items.find((item) => item._getHostElement().contains(target));
  }

  private _setInitialActiveItem(): void {
    const items = this._items.toArray();
    const activeIndex = items.findIndex((item) => item.active());
    const firstEnabledIndex = items.length > 0 ? 0 : -1;
    const initialIndex = activeIndex >= 0 ? activeIndex : firstEnabledIndex;

    if (initialIndex >= 0) {
      this._keyManager.updateActiveItem(initialIndex);
    }
  }

  private _updateActiveItem(item: MatNavigationRailItemComponent): void {
    const index = this._items.toArray().indexOf(item);
    if (index >= 0 && index !== this._keyManager.activeItemIndex) {
      this._keyManager.updateActiveItem(index);
    }
  }
}
