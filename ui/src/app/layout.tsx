import React from 'react';
import RecoilRootWrapper from '@app/recoil-root-wrapper';
import Sidebar from '@/components/Sidebar';
import Header from '@/components/Header';
import { Inter } from 'next/font/google';
import { MantineProvider } from '@mantine/core';
import { ModalsProvider } from '@mantine/modals';
import 'styles/globals.css';
import 'styles/leaflet.css';
import '@mantine/core/styles.css';

export const metadata = {
  title: 'Aviation Weather',
  description: ''
};

const inter = Inter({ subsets: ['latin'] });

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang='en' className='h-full bg-white'>
      <head>
        <title>Aviation Weather</title>
      </head>
      <body className={`${inter.className} wrapper h-full`}>
        <RecoilRootWrapper>
          <MantineProvider>
            <ModalsProvider>
              <Header />
              <Sidebar />
              {children}
            </ModalsProvider>
          </MantineProvider>
        </RecoilRootWrapper>
      </body>
    </html>
  );
}
