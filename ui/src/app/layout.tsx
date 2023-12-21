import React from 'react';
import RecoilRootWrapper from '@app/recoil-root-wrapper';
import { MantineProvider } from '@mantine/core';
import { ModalsProvider } from '@mantine/modals';
import 'styles/globals.css';
import 'styles/leaflet.css';
import '@mantine/core/styles.css';
import { Notifications } from '@mantine/notifications';
import Loader from '@/components/Loader';

export const metadata = {
  title: 'Aviation Weather',
  description: ''
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang='en'>
      <head>
        <title>Aviation Weather</title>
      </head>
      <body>
        <MantineProvider>
          <Notifications />
          <ModalsProvider>
            <RecoilRootWrapper>
              <Loader>
                {children}
              </Loader>
            </RecoilRootWrapper>
          </ModalsProvider>
        </MantineProvider>
      </body>
    </html>
  );
}
