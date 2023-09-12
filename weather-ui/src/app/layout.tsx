import React from 'react';
import RecoilRootWrapper from '@app/recoil-root-wrapper';

import '@fortawesome/fontawesome-svg-core/styles.css';
// Prevent fontawesome from adding its CSS since we did it manually above:
import { config } from '@fortawesome/fontawesome-svg-core';
config.autoAddCss = false;
import 'styles/globals.css';
import Link from 'next/link';

import 'styles/leaflet.css';

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang='en'>
      <head>
        <title>Aviation Weather</title>
      </head>
      <body className='bg-gray-600'>
        <div className='flex justify-between bg-gray-700 px-4 py-1 sm:px-6 select-none'>
          <Link href={'/'}>
            <h3 className='text-lg font-bold leading-6 text-gray-200'>Aviation Weather</h3>
          </Link>
          <Link className='text-base text-gray-200' href={'/profile'}>
            Profile
          </Link>
        </div>
        <RecoilRootWrapper>{children}</RecoilRootWrapper>
      </body>
    </html>
  );
}
