import React from 'react';
import RecoilRootWrapper from '@app/recoil-root-wrapper';
import Sidebar from '@/app/_components/Sidebar';
import Topbar from '@/app/_components/Topbar';
import 'styles/globals.css';
import 'styles/leaflet.css';
import StyledComponentsRegistry from '@/app/_lib/AntdRegistry';
import { Inter } from 'next/font/google';

const inter = Inter({ subsets: ['latin'] });

export const metadata = {
  title: 'Aviation Weather',
  description: ''
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang='en' className='h-full bg-white'>
      <head>
        <title>Aviation Weather</title>
      </head>
      <body className={`${inter.className} wrapper h-full`}>
        <RecoilRootWrapper>
          <StyledComponentsRegistry>
            <Topbar />
            <Sidebar />
            {children}
          </StyledComponentsRegistry>
        </RecoilRootWrapper>
      </body>
    </html>
  );
}
