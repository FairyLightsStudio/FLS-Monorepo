import { Directive, TemplateRef, inject } from '@angular/core';
import type { MatNavigationSuitePrimaryActionContext } from './navigation-suite.types';

@Directive({
  selector: '[matNavigationSuitePrimaryAction]',
})
export class MatNavigationSuitePrimaryAction {
  readonly templateRef = inject<TemplateRef<MatNavigationSuitePrimaryActionContext>>(TemplateRef);

  static ngTemplateContextGuard(
    _dir: MatNavigationSuitePrimaryAction,
    context: unknown,
  ): context is MatNavigationSuitePrimaryActionContext {
    return true;
  }
}
