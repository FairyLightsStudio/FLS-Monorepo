import {
  AfterViewInit,
  ChangeDetectionStrategy,
  Component,
  ElementRef,
  inject,
  input,
  OnDestroy,
  viewChild,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatRippleModule } from '@angular/material/core';
import { FocusMonitor, FocusOrigin } from '@angular/cdk/a11y';
import {
  MatNavigationItemBase,
  MatNavigationActiveIcon,
  MatNavigationIcon,
  MatNavigationLabel,
} from '@fairylights-studio/ngx-m3-navigation-common';

@Component({
  selector: 'mat-navigation-bar-item',
  imports: [CommonModule, MatRippleModule],
  templateUrl: './navigation-bar-item.component.html',
  styleUrl: './navigation-bar-item.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
  host: {
    class: 'mat-navigation-bar-item',
    '[class.mat-navigation-bar-item-selected]': 'active()',
    '[class.mat-navigation-bar-item-always-show-label]':
      'alwaysShowLabel() || layout() === "horizontal"',
    '[class.mat-navigation-bar-item-horizontal]': 'layout() === "horizontal"',
  },
})
export class MatNavigationBarItemComponent
  extends MatNavigationItemBase
  implements AfterViewInit, OnDestroy
{
  /** Whether the text label remains visible when the item is inactive. */
  alwaysShowLabel = input<boolean>(true);

  /** Item layout used by compact and medium navigation bar variants. */
  layout = input<'vertical' | 'horizontal'>('vertical');

  /** ARIA role applied to the internal interactive element. */
  role = input<string>('tab');

  private _focusMonitor = inject(FocusMonitor);
  private _el = inject<ElementRef<HTMLElement>>(ElementRef);
  private _button = viewChild.required<ElementRef<HTMLButtonElement>>('buttonEl');

  focus(origin?: FocusOrigin): void {
    this._button().nativeElement.focus({ preventScroll: origin === 'keyboard' });
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
