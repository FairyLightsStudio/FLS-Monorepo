import type { Context } from '@deepseek-ai/cordis'
import { defineTool } from '@deepseek-ai/dsh-tools'

export const name = 'dsh-scratch'
export const inject = ['tools']

export function apply(ctx: Context) {
  ctx.tools.register(defineTool({
    name: 'scratch_echo',
    description: 'Echo the given text back.',
    parameters: {
      text: { type: 'string', required: true, description: 'Text to echo' },
    },
    output: {
      schema: { type: 'string' },
      render: (_args, value) => [{ type: 'text', text: value }],
    },
    async execute(args) {
      return `echo: ${args.text}`
    },
  }))

  ctx.effect(() => {
    console.log('[dsh-scratch] loaded')
    return () => console.log('[dsh-scratch] unloaded')
  })
}
