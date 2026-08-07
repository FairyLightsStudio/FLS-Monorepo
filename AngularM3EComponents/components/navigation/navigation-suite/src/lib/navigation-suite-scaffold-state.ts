import { Signal, signal } from '@angular/core';
import { MatNavigationSuiteVisibility } from './navigation-suite.types';

export class MatNavigationSuiteScaffoldState {
  private readonly _currentValue = signal<MatNavigationSuiteVisibility>('visible');
  private readonly _targetValue = signal<MatNavigationSuiteVisibility>('visible');
  private readonly _isAnimating = signal(false);
  private pendingTransition: Promise<void> | null = null;
  private resolvePendingTransition: (() => void) | null = null;

  readonly currentValue: Signal<MatNavigationSuiteVisibility> = this._currentValue.asReadonly();
  readonly targetValue: Signal<MatNavigationSuiteVisibility> = this._targetValue.asReadonly();
  readonly isAnimating: Signal<boolean> = this._isAnimating.asReadonly();

  constructor(initialValue: MatNavigationSuiteVisibility = 'visible') {
    this._currentValue.set(initialValue);
    this._targetValue.set(initialValue);
  }

  show(): Promise<void> {
    return this.transitionTo('visible');
  }

  hide(): Promise<void> {
    return this.transitionTo('hidden');
  }

  toggle(): Promise<void> {
    return this.transitionTo(this._targetValue() === 'visible' ? 'hidden' : 'visible');
  }

  snapTo(value: MatNavigationSuiteVisibility): Promise<void> {
    this._currentValue.set(value);
    this._targetValue.set(value);
    this._isAnimating.set(false);
    this.resolvePending();

    return Promise.resolve();
  }

  /** @docs-private */
  _completeTransition(): void {
    // The scaffold owns transition timing because it can observe the real CSS
    // transition. The state object only records the settled value and resolves
    // callers waiting on show/hide/toggle.
    if (!this._isAnimating()) {
      return;
    }

    this._currentValue.set(this._targetValue());
    this._isAnimating.set(false);
    this.resolvePending();
  }

  private transitionTo(value: MatNavigationSuiteVisibility): Promise<void> {
    if (this._targetValue() === value) {
      // Repeated calls to the same target should wait for the in-flight
      // transition instead of creating competing promises.
      return this._isAnimating() ? this.currentPending() : Promise.resolve();
    }

    this._targetValue.set(value);

    if (this._currentValue() === value && !this._isAnimating()) {
      return Promise.resolve();
    }

    this._isAnimating.set(true);
    return this.currentPending();
  }

  private currentPending(): Promise<void> {
    if (this.pendingTransition !== null) {
      return this.pendingTransition;
    }

    this.pendingTransition = new Promise<void>((resolve) => {
      this.resolvePendingTransition = resolve;
    });

    return this.pendingTransition;
  }

  private resolvePending(): void {
    const resolve = this.resolvePendingTransition;

    this.pendingTransition = null;
    this.resolvePendingTransition = null;
    resolve?.();
  }
}
