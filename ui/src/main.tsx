import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import './index.css';
import App from './App.tsx';
import { createTheme, MantineProvider } from '@mantine/core';
import { Notifications } from '@mantine/notifications';

const theme = createTheme({
  fontFamily: 'Inter, sans-serif'
});

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <MantineProvider theme={theme} defaultColorScheme={'dark'}>
      <Notifications zIndex={2000} />
      <App />
    </MantineProvider>
  </StrictMode>
);
