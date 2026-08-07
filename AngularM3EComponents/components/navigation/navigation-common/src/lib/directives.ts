import { Directive, TemplateRef, inject } from '@angular/core';

/** Marks projected template content as the default navigation item icon. */
@Directive({
  selector: '[matNavigationIcon], [matNavIcon]',
})
export class MatNavigationIcon {
  public templateRef = inject(TemplateRef<unknown>, { optional: true });
}

/** Marks projected template content as the icon shown for active navigation items. */
@Directive({
  selector: '[matNavigationActiveIcon], [matNavActiveIcon]',
})
export class MatNavigationActiveIcon {
  public templateRef = inject(TemplateRef<unknown>, { optional: true });
}

/** Marks projected template content as the navigation item label. */
@Directive({
  selector: '[matNavigationLabel], [matNavLabel]',
})
export class MatNavigationLabel {
  public templateRef = inject(TemplateRef<unknown>, { optional: true });
}
