import {
  AfterViewInit,
  Component,
  ElementRef,
  inject,
  forwardRef,
  ChangeDetectionStrategy,
  OnDestroy,
  viewChild,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatRippleModule } from '@angular/material/core';
import { FocusMonitor, FocusOrigin } from '@angular/cdk/a11y';
import { Directionality } from '@angular/cdk/bidi';
import { MatNavigationRailComponent } from './navigation-rail.component';
import { MatNavigationItemBase } from '@fairylights-studio/ngx-m3-navigation-common';

@Component({
  selector: 'mat-navigation-rail-item',
  imports: [CommonModule, MatRippleModule],
  template: `
    <button
      class="mat-nav-rail-item-button"
      [class.active]="active()"
      [attr.dir]="dir.value"
      [attr.role]="'tab'"
      [attr.aria-selected]="active()"
      [attr.aria-current]="active() ? 'page' : null"
      #buttonEl
    >
      <div
        class="mat-nav-rail-indicator"
        [class.indicator-fill]="rail?.indicatorShape() === 'fill'"
        [class.indicator-hug]="rail?.indicatorShape() === 'hug'"
      >
        <div class="mat-nav-rail-ripple" matRipple [matRippleTrigger]="buttonEl"></div>

        <div class="mat-nav-rail-icon-box">
          @if (active() && activeIcon) {
            <ng-container *ngTemplateOutlet="activeIcon?.templateRef || null"></ng-container>
          } @else {
            <ng-container *ngTemplateOutlet="icon?.templateRef || null"></ng-container>
          }
        </div>

        <div class="mat-nav-rail-label-side">
          <div class="mat-nav-rail-label-inner">
            <ng-container *ngTemplateOutlet="label?.templateRef"></ng-container>
          </div>
        </div>
      </div>

      <div class="mat-nav-rail-label-bottom">
        <div class="mat-nav-rail-label-bottom-inner">
          <ng-container *ngTemplateOutlet="label?.templateRef"></ng-container>
        </div>
      </div>
    </button>
  `,
  styleUrl: './navigation-rail-item.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
  host: {
    '[class.mat-nav-rail-item-expanded]': 'rail?.expanded()',
  },
})
export class MatNavigationRailItemComponent
  extends MatNavigationItemBase
  implements AfterViewInit, OnDestroy
{
  /** Owning rail instance used to inherit expanded and indicator state. */
  protected rail = inject(
    forwardRef(() => MatNavigationRailComponent),
    {
      optional: true,
    },
  );
  private _focusMonitor = inject(FocusMonitor);
  private _el = inject<ElementRef<HTMLElement>>(ElementRef);
  private _button = viewChild.required<ElementRef<HTMLButtonElement>>('buttonEl');
  protected dir = inject(Directionality, { optional: true }) || { value: 'ltr' };

  focus(origin?: FocusOrigin): void {
    this._button().nativeElement.focus({
      preventScroll: origin === 'keyboard',
    });
  }

  _getHostElement(): HTMLElement {
    return this._el.nativeElement;
  }

  _getButtonElement(): HTMLButtonElement {
    return this._button().nativeElement;
  }

  getLabel(): string {
    return this._button().nativeElement.textContent?.trim() ?? '';
  }

  ngAfterViewInit(): void {
    this._focusMonitor.monitor(this._button(), true);
  }

  ngOnDestroy(): void {
    this._focusMonitor.stopMonitoring(this._button());
  }
}
