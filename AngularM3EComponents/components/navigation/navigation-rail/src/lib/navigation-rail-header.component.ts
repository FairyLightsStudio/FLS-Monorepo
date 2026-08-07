import { Component, ElementRef, AfterViewInit, OnDestroy, inject } from '@angular/core';

/**
 * Header slot for `mat-navigation-rail`, typically holding a
 * `FloatingActionButton` or a toggle button.
 *
 * Each direct child is positioned with a **fixed center offset** so that
 * it appears centered when the rail is collapsed (80px) and naturally
 * left-aligned when the rail expands — the child itself never moves.
 *
 * ## CSS variable driven centering
 *
 * The center offset is computed per-child via:
 *
 * ```
 * --_child-width      = min(
 *                         var(--_measured-width, 56px),
 *                         var(--_max-compact-width, 9999px)
 *                       )
 * --_center-offset    = calc((80px - var(--_child-width)) / 2)
 * ```
 *
 * ### Built-in FAB compact widths
 *
 * Known FAB types have their `--_max-compact-width` locked to the icon-only
 * compact size, so even if JS measurement captures the fully-extended width
 * (e.g. 136px), it is clamped back to 56px:
 *
 * | FAB variant      | Selector                     | Compact width |
 * |------------------|------------------------------|---------------|
 * | Standard         | `[mat-fab]` `.mat-mdc-fab`  | 56px          |
 * | Extended         | `[mat-extended-fab]` `.mat-mdc-extended-fab` | 56px |
 * | Small / Mini     | `[mat-mini-fab]` `.mat-mdc-mini-fab` | 40px  |
 *
 * ### Overriding for custom elements
 *
 * Set `--_measured-width` (and optionally `--_max-compact-width`) directly
 * on the element:
 *
 * ```html
 * <mat-navigation-rail-header>
 *   <button mat-fab style="--_measured-width: 40px">…</button>
 *   <my-custom-el style="--_max-compact-width: 64px">…</my-custom-el>
 * </mat-navigation-rail-header>
 * ```
 *
 * ## JS-fallback measurement
 *
 * If `--_measured-width` is not set, a **one-time** JS measurement fallback
 * runs via `ResizeObserver`. For known FAB types the measurement is capped
 * by the CSS `--_max-compact-width` (see above), so an incorrectly wide
 * measurement from an extended state is harmless.
 *
 * For unknown elements, override `--_max-compact-width` if a cap is needed.
 */
@Component({
  selector: 'mat-navigation-rail-header',
  template: `<ng-content></ng-content>`,
  styleUrls: ['./navigation-rail-header.component.scss'],
})
export class MatNavigationRailHeaderComponent implements AfterViewInit, OnDestroy {
  private el = inject<ElementRef<HTMLElement>>(ElementRef);

  private resizeObserver!: ResizeObserver;
  private mutationObserver!: MutationObserver;

  ngAfterViewInit(): void {
    this.mutationObserver = new MutationObserver((mutations) => {
      mutations.forEach((mutation) => {
        mutation.addedNodes.forEach((node) => {
          if (node.nodeType === Node.ELEMENT_NODE) {
            this.measureOnce(node as Element);
          }
        });
      });
    });

    this.mutationObserver.observe(this.el.nativeElement, { childList: true });

    Array.from(this.el.nativeElement.children).forEach((child) => {
      this.measureOnce(child);
    });
  }

  /**
   * Measure the compact width of a child element once and write it to
   * `--_measured-width`. The measurement is only performed when the element
   * has a non-zero width. After the first successful measurement, the
   * element is removed from observation.
   *
   * This method intentionally does NOT check the rail's expanded state —
   * the JS fallback is only intended for fixed-width children (see class
   * documentation), so any measurement — regardless of rail state — yields
   * the correct compact width.
   */
  private measureOnce(child: Element): void {
    const target = child as HTMLElement;
    const existing = target.style.getPropertyValue('--_measured-width').trim();
    if (existing) return;

    if (!this.resizeObserver) {
      this.resizeObserver = new ResizeObserver((entries) => {
        for (const entry of entries) {
          const el = entry.target as HTMLElement;
          const width = el.offsetWidth;
          // Only record the measured width when the element has fully rendered (avoiding intermediate 24px icon-only states)
          if (width >= 32) {
            el.style.setProperty('--_measured-width', `${width}px`);
            this.resizeObserver.unobserve(el);
          }
        }
      });
    }

    this.resizeObserver.observe(target);
  }

  ngOnDestroy(): void {
    this.resizeObserver?.disconnect();
    this.mutationObserver?.disconnect();
  }
}
