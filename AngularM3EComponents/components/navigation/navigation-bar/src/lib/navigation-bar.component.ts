import {
  AfterContentInit,
  AfterViewInit,
  ChangeDetectionStrategy,
  Component,
  ContentChildren,
  ElementRef,
  inject,
  input,
  OnDestroy,
  QueryList,
  signal,
} from '@angular/core';
import { FocusKeyManager, type FocusableOption } from '@angular/cdk/a11y';
import { Directionality } from '@angular/cdk/bidi';
import { ENTER, SPACE, hasModifierKey } from '@angular/cdk/keycodes';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import {
  MAT_NAVIGATION_WIDGET,
  MatNavigationWidget,
  MatNavigationPlacement,
} from '@fairylights-studio/ngx-m3-navigation-common';
import { MatNavigationBarItemComponent } from './navigation-bar-item.component';

/** Bottom navigation container for compact and medium screen layouts. */
@Component({
  selector: 'mat-navigation-bar',
  template: `
    <div class="mat-nav-bar-content">
      <ng-content></ng-content>
    </div>
  `,
  styleUrl: './navigation-bar.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
  providers: [
    {
      provide: MAT_NAVIGATION_WIDGET,
      useExisting: MatNavigationBarComponent,
    },
  ],
  host: {
    role: 'navigation',
    '[attr.aria-label]': 'ariaLabel() || null',
    '(keydown)': '_handleKeydown($event)',
  },
})
export class MatNavigationBarComponent implements AfterContentInit, AfterViewInit, OnDestroy, MatNavigationWidget {
  readonly placement = signal<MatNavigationPlacement>('bottom').asReadonly();
  readonly size = signal<number>(80);
  readonly surfaceSize = signal<number | null>(null).asReadonly();

  private _elementRef = inject(ElementRef);
  private _resizeObserver: ResizeObserver | null = null;
  /** Accessible label for the navigation landmark. */
  ariaLabel = input<string>('');

  @ContentChildren(MatNavigationBarItemComponent, { descendants: true })
  protected _items!: QueryList<MatNavigationBarItemComponent>;

  private _keyManager!: FocusKeyManager<MatNavigationBarItemComponent>;
  private _dir = inject(Directionality, { optional: true });
  private _destroyed = new Subject<void>();

  ngAfterContentInit(): void {
    this._keyManager = new FocusKeyManager<MatNavigationBarItemComponent>(
      this._items as unknown as QueryList<MatNavigationBarItemComponent & FocusableOption>,
    )
      .withHorizontalOrientation(this._dir?.value || 'ltr')
      .withHomeAndEnd()
      .withWrap()
      .withTypeAhead();

    this._setInitialActiveItem();

    if (this._dir) {
      this._dir.change.pipe(takeUntil(this._destroyed)).subscribe((dir) => {
        this._keyManager.withHorizontalOrientation(dir);
      });
    }
  }

  ngAfterViewInit(): void {
    if (typeof window !== 'undefined') {
      const host = this._elementRef.nativeElement;

      const initialHeight = host.offsetHeight;
      if (initialHeight > 0) {
        this.size.set(initialHeight);
      }

      // Dynamically observe host height changes (e.g. layout updates) and sync
      this._resizeObserver = new ResizeObserver((entries) => {
        for (const entry of entries) {
          const height = (entry.target as HTMLElement).offsetHeight;
          if (height > 0) {
            this.size.set(height);
          }
        }
      });
      this._resizeObserver.observe(host);
    }
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
    this._resizeObserver?.disconnect();
    this._destroyed.next();
    this._destroyed.complete();
  }

  private _getEventItem(event: KeyboardEvent): MatNavigationBarItemComponent | undefined {
    const target = event.target;
    if (!(target instanceof Node)) return undefined;

    return this._items.find((item) => item._getHostElement().contains(target));
  }

  private _setInitialActiveItem(): void {
    const items = this._items.toArray();
    const activeIndex = items.findIndex((item) => item.active());
    const firstIndex = items.length > 0 ? 0 : -1;
    const initialIndex = activeIndex >= 0 ? activeIndex : firstIndex;

    if (initialIndex >= 0) {
      this._keyManager.updateActiveItem(initialIndex);
    }
  }

  private _updateActiveItem(item: MatNavigationBarItemComponent): void {
    const index = this._items.toArray().indexOf(item);
    if (index >= 0 && index !== this._keyManager.activeItemIndex) {
      this._keyManager.updateActiveItem(index);
    }
  }
}
