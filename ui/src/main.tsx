import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import './index.css';
import App from './App.tsx';
import { createTheme, MantineProvider } from '@mantine/core';
import { Notifications } from '@mantine/notifications';
import {} from '@mantine/core';

const theme = createTheme({
  fontFamily: 'Inter, sans-serif'
});

export const metadata = {
  title: 'Aviation Weather',
  description: ''
};

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <MantineProvider theme={theme} defaultColorScheme={'dark'}>
      <Notifications />
      <App />
    </MantineProvider>
  </StrictMode>
);
