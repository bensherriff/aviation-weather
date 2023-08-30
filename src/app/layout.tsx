import React from 'react';
import RecoilRootWrapper from '@app/recoil-root-wrapper';


import '@fortawesome/fontawesome-svg-core/styles.css';
// Prevent fontawesome from adding its CSS since we did it manually above:
import { config } from '@fortawesome/fontawesome-svg-core';
config.autoAddCss = false; /* eslint-disable import/first */
import 'styles/globals.css';

export default function RootLayout({ children }: { children: React.ReactNode }) {
    return (
        <html lang="en">
            <link rel="stylesheet" href="https://unpkg.com/leaflet@1.0.1/dist/leaflet.css" />
            <body className='bg-gray-600'>
                <RecoilRootWrapper>{children}</RecoilRootWrapper>
            </body>
        </html>
    );
}
