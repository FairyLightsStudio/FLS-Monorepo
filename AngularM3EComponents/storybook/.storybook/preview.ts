import type { Preview } from '@storybook/angular';
import { setCompodocJson } from '@storybook/addon-docs/angular';
import { INITIAL_VIEWPORTS } from 'storybook/viewport';

// import docJson from '../documentation.json';

// setCompodocJson(docJson);

const preview: Preview = {
  tags: ['autodocs'],
  parameters: {
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
      viewport: {
        options: INITIAL_VIEWPORTS,
      },
    },
    layout: 'fullscreen',
    docs: {
      toc: true,
    },
  },
  initialGlobals: {
    viewport: { value: 'mobile1', isRotated: false },
  },
  decorators: [],
};

export default preview;
