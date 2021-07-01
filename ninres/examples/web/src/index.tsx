import React from 'react';
import ReactDOM from 'react-dom';

import { CssBaseline, GeistProvider } from '@geist-ui/react';

import { App } from './app';

export let ninres: typeof import('../../../pkg/ninres');

(async () => {
  ninres = await import('../../../pkg/ninres');
  ninres.setupPanicHook();

  ReactDOM.render(
    <GeistProvider>
      <CssBaseline />
      <App />
    </GeistProvider>,
    document.getElementById('root')
  );
})();
